use std::error::Error;

use noter::configs::Configuration;
use noter::{configuration, variant};

#[test]
fn test_parse_rust_config() -> Result<(), Box<dyn Error>> {
    // load in a rust-format configuration file
    let raw_config = std::fs::read_to_string("tests/test_data/rust_config.toml")?;

    // parse as a configuration
    let config: Configuration = toml::from_str(&raw_config)?;

    // what we expect our configuration to look like
    let expected_configuration = configuration!(
        "unreleased_notes",
        "ReleaseNotes.rst",
        "v{version} - {project_date}",
        "`{issue} <https://www.example.com/{issue}>`_",
        variant!("breaking", "Incompatible Changes", true),
        variant!("build", "Build and Packaging", true),
        variant!("feature", "Features", true),
        variant!("bugfix", "Bugfixes", true),
        variant!("doc", "Improved Documentation", true),
        variant!("deprecation", "Deprecations", true),
        variant!("refactor", "Refactors", true)
    );

    assert_eq!(config, expected_configuration);

    Ok(())
}
