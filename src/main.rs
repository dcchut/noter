use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Local;
use structopt::StructOpt;

use anyhow::{anyhow, bail, Context, Result};
use noter::configs::{Configuration, NoteVariant};
use noter::{NoteWriter, StringWriter};

#[derive(StructOpt, Debug)]
#[structopt(name = "noter")]
struct Opt {
    /// Produce a draft
    #[structopt(short, long)]
    draft: bool,

    /// Folder containing config file to use
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,

    /// Version to use in release notes
    #[structopt(short, long)]
    version: Option<String>,
}

fn read_config<P: AsRef<Path>>(dir: P) -> std::result::Result<Configuration, anyhow::Error> {
    for entry in fs::read_dir(dir.as_ref())?.flatten() {
        if entry.file_name() == "noter.toml" {
            // read in the file
            let config = fs::read(entry.path())
                .with_context(|| format!("unable to load config {}", entry.path().display()))?;

            // parse as config
            return toml::from_slice(config.as_slice()).with_context(|| "unable to parse config");
        }
    }

    Err(anyhow!("failed to find config"))
}

fn find_variant<'cfg>(config: &'cfg Configuration, file_name: &str) -> Option<&'cfg NoteVariant> {
    config
        .variant
        .iter()
        .find(|&variant| file_name.ends_with(&variant.extension))
}

fn compile_release_notes(
    writer: &mut StringWriter,
    config: &Configuration,
    version: String,
    notes_by_variant: &HashMap<&NoteVariant, Vec<(String, String)>>,
) -> Result<String> {
    // Format the title
    let title_format = {
        let mut title_map = HashMap::new();
        title_map.insert(
            String::from("project_date"),
            Local::now().format("%Y-%m-%d").to_string(),
        );

        title_map.insert(String::from("version"), version);

        // now format the title formatting string
        strfmt::strfmt(&config.title_format, &title_map)
    }
    .with_context(|| "invalid `title_format` given")?;

    // add the title in
    writer._title(title_format);

    // now write out each variant type
    for variant in config.variant.iter() {
        if let Some(release_notes) = notes_by_variant.get(variant) {
            // write the header for this variant
            writer.variant_header(variant);

            // write each of the release notes
            for (base_file_name, note_contents) in release_notes {
                // format the issue string
                let issue = {
                    let mut issue_map = HashMap::new();
                    issue_map.insert(String::from("issue"), String::from(base_file_name));

                    strfmt::strfmt(&config.issue_format, &issue_map)
                }
                .with_context(|| "invalid `issue_format` given")?;

                writer._release_note(variant, base_file_name, note_contents, issue);
            }

            writer.variant_footer();
        }
    }

    Ok(writer.write())
}

fn version_number(version_override: Option<String>) -> Result<String> {
    if let Some(version) = version_override {
        return Ok(version);
    }

    // otherwise attempt to discover the version number using cargo
    let cmd = cargo_metadata::MetadataCommand::new();
    let metadata = cmd
        .exec()
        .with_context(|| "failed to determine version number")?;

    if let Some(resolve) = &metadata.resolve {
        if let Some(package_id) = &resolve.root {
            return Ok(metadata[package_id].version.to_string());
        }
    }

    bail!("failed to determine version number");
}

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();

    // read in the configuration file
    let (config, base_dir) = match opt.config {
        Some(config_dir) => (read_config(&config_dir)?, config_dir),
        None => (read_config(".")?, PathBuf::from(".")),
    };

    // check that the release notes directory exists
    let release_notes_dir = base_dir.join(&config.directory);

    if !release_notes_dir.exists() || !release_notes_dir.is_dir() {
        bail!(format!(
            "release notes directory {} does not exist",
            release_notes_dir.display()
        ));
    }

    let mut notes_by_variant: HashMap<&NoteVariant, Vec<(String, String)>> = HashMap::new();

    // read in all of the release notes
    for entry in fs::read_dir(&release_notes_dir)?.flatten() {
        // only consider files
        if !entry.path().is_file() {
            continue;
        }

        let file_name = entry.file_name().to_string_lossy().to_string();

        // load the file to a string
        let file_contents = fs::read_to_string(entry.path())?;

        // find the matching variant
        if let Some(variant) = find_variant(&config, &file_name) {
            // get the file_name sans the variant extension
            let base_file_name =
                String::from(&file_name[..file_name.len() - (variant.extension.len() + 1)]);

            notes_by_variant
                .entry(variant)
                .or_default()
                .push((base_file_name, file_contents));
        }
    }

    // if there are no release notes, do nothing (TODO: parameterize this?)
    if notes_by_variant.is_empty() {
        bail!("no release notes found");
    }

    let version = version_number(opt.version)?;

    // now create the actual release notes
    let release_notes: String = if config.filename.ends_with(".md") {
        compile_release_notes(
            &mut StringWriter::markdown(),
            &config,
            version,
            &notes_by_variant,
        )?
    } else if config.filename.ends_with(".rst") {
        compile_release_notes(
            &mut StringWriter::text(),
            &config,
            version,
            &notes_by_variant,
        )?
    } else {
        bail!(
            "expected `filename` ending with .md or .rst, found {}",
            config.filename
        );
    };

    if opt.draft {
        // for a draft, just print the release notes to stdout
        println!("{}", release_notes);
    } else {
        // get the path to the release notes
        let release_notes_path = base_dir.join(&config.filename);

        let content = if release_notes_path.is_file() {
            let existing_content = fs::read_to_string(&release_notes_path).with_context(|| {
                format!(
                    "failed to read release notes from {}",
                    release_notes_path.display()
                )
            })?;

            release_notes + "\n" + &existing_content
        } else {
            release_notes
        };

        // now write the output to the release notes file
        fs::write(&release_notes_path, content).with_context(|| {
            format!(
                "failed to write release notes to {}",
                release_notes_path.display()
            )
        })?;
    }

    Ok(())
}
