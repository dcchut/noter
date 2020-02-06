use crate::configs::NoteVariant;

pub mod configs;
mod macros;

/// The `NoteWriter` trait is used to generate release notes from compiled release note files.
pub trait NoteWriter {
    /// Write out a title line
    fn title<T: Into<String>>(&mut self, title: T);

    /// Start an itemized list of release notes for a given variant
    fn start_variant_list(&mut self, variant: NoteVariant);

    /// Write a single release note
    fn note<S: AsRef<str>, T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        ticket: S,
        description: T,
        issue: U,
    );

    /// End the most recently started variant list.
    fn end_variant(&mut self);

    /// Write out `lines` many empty lines.
    fn spacing(&mut self, lines: usize);

    /// Consumes the writer, returning the notes as a String.
    fn write(&mut self) -> String;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StringWriter {
    // just a really basic unformatted text string
    lines: Vec<String>,

    current_variant: Option<NoteVariant>,
}

impl StringWriter {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            current_variant: None,
        }
    }
}

impl NoteWriter for StringWriter {
    fn title<T: Into<String>>(&mut self, title: T) {
        let title = title.into();
        let title_length = title.len();

        self.lines.push(title);
        self.lines.push(vec!["-"; title_length].join(""));
    }

    fn start_variant_list(&mut self, variant: NoteVariant) {
        let variant_title = variant.name.as_str();
        self.lines.push(String::from(variant_title));
        self.lines.push(vec!["="; variant_title.len()].join(""));
        self.current_variant = Some(variant);
    }

    fn note<S: AsRef<str>, T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        ticket: S,
        description: T,
        issue: U,
    ) {
        let ticket = ticket.as_ref();
        let description = description.as_ref();
        let issue = issue.as_ref();
        // it's okay to panic if we're not in a variant here
        let variant = self.current_variant.as_ref().unwrap();

        // construct our line
        let line = {
            let mut current_line =
                String::with_capacity(6 + ticket.len() + description.len() + issue.len());
            current_line.push_str(" - ");
            current_line.push_str(ticket);
            current_line.push_str(": ");
            if variant.show_content {
                current_line.push_str(description);
                current_line.push(' ');
            }
            current_line.push_str(issue);

            current_line
        };

        self.lines.push(line);
    }

    fn end_variant(&mut self) {
        // add a newline
        self.spacing(1);
        // clear out the current variant info
        self.current_variant.take();
    }

    fn spacing(&mut self, lines: usize) {
        // empty lines are blank lines
        for _ in 0..lines {
            self.lines.push(String::new());
        }
    }

    fn write(&mut self) -> String {
        // self.lines contains all of our output lines, so we just need to join them together with newlines
        let mut lines = Vec::new();
        std::mem::swap(&mut lines, &mut self.lines);

        lines.join("\n")
    }
}
