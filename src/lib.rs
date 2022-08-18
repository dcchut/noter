use crate::configs::NoteVariant;

pub mod configs;
mod macros;

/// The `NoteWriter` trait is used to generate release notes from compiled release note files.
pub trait NoteWriter {
    /// Write out a title line.
    #[inline(always)]
    fn _title<T: AsRef<str>>(&mut self, title: T) {
        self.title(title.as_ref());
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
                vec![String::from(title), "=".repeat(title_length), String::new()]
            }
            NoteFormatter::Markdown => vec![format!("# {}", title), String::new()],
        }
    }

    pub fn variant_header(self, variant: &NoteVariant) -> Vec<String> {
        match self {
            NoteFormatter::Text => {
                let variant_title = variant.name.as_str();
                vec![
                    String::from(variant_title),
                    "-".repeat(variant_title.len()),
                    String::new(),
                ]
            }
            NoteFormatter::Markdown => vec![format!("## {}", &variant.name), String::new()],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
                String::with_capacity(5 + ticket.len() + description.len() + issue.len());
            current_line.push_str("- ");
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
        // if spacing is called when self.lines is empty, it won't actually make a new line in the output
        // so add an extra empty line in, if thats the case.
        let total_lines = if self.lines.is_empty() {
            lines + 1
        } else {
            lines
        };

        // empty lines are blank lines
        for _ in 0..total_lines {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_writer_text() {
        let mut writer = StringWriter::text();
        writer.spacing(2);
        assert_eq!(writer.write(), "\n\n");

        let mut writer = StringWriter::text();
        writer.title("Hello world");
        writer.spacing(1);
        assert_eq!(writer.write(), "Hello world\n===========\n\n");

        let mut writer = StringWriter::text();
        writer.title("TITLE");
        let variant = variant!("basic", "Basic Notes", true);
        writer.variant_header(&variant);
        writer.release_note(&variant, "ticket", "description", "issue");
        writer.variant_footer();
        assert_eq!(
            writer.write(),
            "TITLE\n=====\n\nBasic Notes\n-----------\n\n- ticket: description issue\n"
        );
    }

    #[test]
    fn test_string_writer_markdown() {
        let mut writer = StringWriter::markdown();
        writer.spacing(2);
        assert_eq!(writer.write(), "\n\n");

        let mut writer = StringWriter::markdown();
        writer.title("Hello world");
        writer.spacing(2);
        assert_eq!(writer.write(), "# Hello world\n\n\n");

        let mut writer = StringWriter::markdown();
        writer.title("TITLE");
        let variant = variant!("basic", "Basic Notes", true);
        writer.variant_header(&variant);
        writer.release_note(&variant, "ticket", "description", "issue");
        writer.variant_footer();
        assert_eq!(
            writer.write(),
            "# TITLE\n\n## Basic Notes\n\n- ticket: description issue\n"
        );
    }

    fn basic_writer_example(writer: &mut StringWriter) {
        writer.title("A title string");

        let variant = variant!("basic", "Basic notes", true);

        writer.variant_header(&variant);
        writer.release_note(
            &variant,
            "TICKET0001",
            "Improve something or other",
            "<www.google.com>",
        );
        writer.release_note(
            &variant,
            "TICKET0002",
            "Improve something else too",
            "<www.google.com>",
        );
        writer.variant_footer();
    }

    #[test]
    fn test_string_writer_text_2() {
        let mut writer = StringWriter::text();
        basic_writer_example(&mut writer);

        assert_eq!(
            writer.write(),
            r#"A title string
==============

Basic notes
-----------

- TICKET0001: Improve something or other <www.google.com>
- TICKET0002: Improve something else too <www.google.com>
"#
        );
    }

    #[test]
    fn test_string_writer_markdown_2() {
        let mut writer = StringWriter::markdown();
        basic_writer_example(&mut writer);

        assert_eq!(
            writer.write(),
            r#"# A title string

## Basic notes

- TICKET0001: Improve something or other <www.google.com>
- TICKET0002: Improve something else too <www.google.com>
"#
        );
    }
}
