use std::fs;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use anyhow::{anyhow, Context, Result};
use noter::configs::Configuration;

#[derive(StructOpt, Debug)]
#[structopt(name = "noter")]
struct Opt {
    /// Produce a draft
    #[structopt(short, long)]
    draft: bool,

    /// Folder containing config file to use
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,
}

fn read_config<P: AsRef<Path>>(dir: P) -> std::result::Result<Configuration, anyhow::Error> {
    for entry in fs::read_dir(dir.as_ref())? {
        if let Ok(entry) = entry {
            if entry.file_name() == "noter.toml" {
                // read in the file
                let config = fs::read(entry.path())
                    .with_context(|| format!("unable to load config {}", entry.path().display()))?;

                // parse as config
                return Ok(toml::from_slice(config.as_slice())
                    .with_context(|| "unable to parse config")?);
            }
        }
    }

    Err(anyhow!("failed to find config"))
}

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();

    // read in the configuration file
    let _config = match opt.config {
        Some(config_dir) => read_config(config_dir)?,
        None => read_config(".")?,
    };

    // create the release notes
    // TODO
    let release_notes = String::from("placeholder release notes for now");

    if opt.draft {
        // for a draft, just print the release notes to stdout
        println!("{}", release_notes);
    } else {
        // otherwise, add the release notes to the current release notes file
        // TODO
    }

    Ok(())
}
