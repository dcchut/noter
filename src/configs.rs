use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NoteVariant {
    /// The extension (e.g. .breaking) to identify this variant of notes.
    pub extension: String,

    /// A description of this variant.
    pub name: String,

    /// Whether the content of this variant should be included in the release notes.
    pub show_content: bool,
}

impl NoteVariant {
    pub fn new<S, T>(extension: S, name: T, show_content: bool) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            extension: extension.into(),
            name: name.into(),
            show_content,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Configuration {
    /// The directory (relative to the configuration file) in which release notes are stored.
    pub directory: String,

    /// The name of the release notes file.
    pub filename: String,

    /// The format string used to generate the title line of each release note.
    pub title_format: String,

    /// The format string used to display next to the ticket name.
    pub issue_format: String,

    /// A list of release note variants.
    pub variant: Vec<NoteVariant>,
}

impl Configuration {
    pub fn new<S, T, U, V>(
        directory: S,
        filename: T,
        title_format: U,
        issue_format: V,
        variants: Vec<NoteVariant>,
    ) -> Self
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        Self {
            directory: directory.into(),
            filename: filename.into(),
            title_format: title_format.into(),
            issue_format: issue_format.into(),
            variant: variants,
        }
    }
}
