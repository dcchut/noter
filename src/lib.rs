use crate::configs::NoteVariant;

pub mod configs;
mod macros;

/// The `NoteWriter` trait is used to generate release notes from compiled release note files.
pub trait NoteWriter {
    /// Write out a title line.
    #[inline(always)]
    fn _title<T: AsRef<str>>(&mut self, title: T) {
        self.title(title.as_ref());
        self.spacing(1);
    }

    /// Write out a title line.
    fn title(&mut self, title: &str);

    /// Start an itemized list of release notes for a given variant.
    fn variant_header(&mut self, variant: &NoteVariant);

    /// Write a single release note
    #[inline(always)]
    fn _release_note<S: AsRef<str>, T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        variant: &NoteVariant,
        ticket: S,
        description: T,
        issue: U,
    ) {
        self.release_note(
            variant,
            ticket.as_ref(),
            description.as_ref(),
            issue.as_ref(),
        );
    }

    /// Write a single release note
    fn release_note(&mut self, variant: &NoteVariant, ticket: &str, description: &str, issue: &str);

    /// End the most recently started variant list.
    fn variant_footer(&mut self);

    /// Write out `lines` many empty lines.
    fn spacing(&mut self, lines: usize);

    /// Consumes the writer, returning the notes as a String.
    fn write(&mut self) -> String;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NoteFormatter {
    Text,
    Markdown,
}

impl NoteFormatter {
    pub fn title(self, title: &str) -> Vec<String> {
        match self {
            NoteFormatter::Text => {
                let title_length = title.len();
                vec![String::from(title), "=".repeat(title_length)]
            }
            NoteFormatter::Markdown => vec![format!("# {}", title)],
        }
    }

    pub fn variant_header(self, variant: &NoteVariant) -> Vec<String> {
        match self {
            NoteFormatter::Text => {
                let variant_title = variant.name.as_str();
                vec![String::from(variant_title), "-".repeat(variant_title.len())]
            }
            NoteFormatter::Markdown => vec![format!("## {}", &variant.name)],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringWriter {
    lines: Vec<String>,
    formatter: NoteFormatter,
}

impl StringWriter {
    pub fn new(formatter: NoteFormatter) -> Self {
        Self {
            lines: Vec::new(),
            formatter,
        }
    }

    pub fn text() -> Self {
        Self::new(NoteFormatter::Text)
    }

    pub fn markdown() -> Self {
        Self::new(NoteFormatter::Markdown)
    }
}

impl NoteWriter for StringWriter {
    #[inline(always)]
    fn title(&mut self, title: &str) {
        self.lines.extend(self.formatter.title(title));
    }

    #[inline(always)]
    fn variant_header(&mut self, variant: &NoteVariant) {
        self.lines.extend(self.formatter.variant_header(variant));
    }

    #[inline(always)]
    fn release_note(
        &mut self,
        variant: &NoteVariant,
        ticket: &str,
        description: &str,
        issue: &str,
    ) {
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

    #[inline(always)]
    fn variant_footer(&mut self) {
        // add a newline
        self.spacing(1);
    }

    #[inline(always)]
    fn spacing(&mut self, lines: usize) {
        // empty lines are blank lines
        for _ in 0..lines {
            self.lines.push(String::new());
        }
    }

    #[inline(always)]
    fn write(&mut self) -> String {
        // self.lines contains all of our output lines, so we just need to join them together with newlines
        let mut lines = Vec::new();
        std::mem::swap(&mut lines, &mut self.lines);

        lines.join("\n")
    }
}
