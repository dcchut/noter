use crate::configs::NoteVariant;
use serde::export::PhantomData;

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

#[derive(Debug, Clone, PartialEq)]
pub struct StringWriter<F>
where
    F: NoteFormatter<Output = Vec<String>>,
{
    lines: Vec<String>,
    formatter: PhantomData<F>,
}

impl<F> StringWriter<F>
where
    F: NoteFormatter<Output = Vec<String>>,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            formatter: Default::default(),
        }
    }
}

impl StringWriter<MarkdownNoteFormatter> {
    pub fn markdown() -> Self {
        Self::new()
    }
}

impl StringWriter<TextNoteFormatter> {
    pub fn text() -> Self {
        Self::new()
    }
}

pub trait NoteFormatter {
    type Output;

    fn title(title: &str) -> Self::Output;
    fn variant_header(variant: &NoteVariant) -> Self::Output;
    fn release_note(
        ticket: &str,
        description: &str,
        issue: &str,
        variant: &NoteVariant,
    ) -> Self::Output;
}

pub struct TextNoteFormatter {}

impl NoteFormatter for TextNoteFormatter {
    type Output = Vec<String>;

    #[inline(always)]
    fn title(title: &str) -> Self::Output {
        let title_length = title.len();
        vec![String::from(title), vec!["="; title_length].join("")]
    }

    #[inline(always)]
    fn variant_header(variant: &NoteVariant) -> Self::Output {
        let variant_title = variant.name.as_str();
        vec![
            String::from(variant_title),
            vec!["-"; variant_title.len()].join(""),
        ]
    }

    #[inline(always)]
    fn release_note(
        ticket: &str,
        description: &str,
        issue: &str,
        variant: &NoteVariant,
    ) -> Self::Output {
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

        vec![line]
    }
}

pub struct MarkdownNoteFormatter {}

impl NoteFormatter for MarkdownNoteFormatter {
    type Output = Vec<String>;

    #[inline(always)]
    fn title(title: &str) -> Self::Output {
        vec![format!("# {}", title)]
    }

    #[inline(always)]
    fn variant_header(variant: &NoteVariant) -> Self::Output {
        vec![format!("## {}", &variant.name)]
    }

    #[inline(always)]
    fn release_note(
        ticket: &str,
        description: &str,
        issue: &str,
        variant: &NoteVariant,
    ) -> Self::Output {
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

        vec![line]
    }
}

impl<F> NoteWriter for StringWriter<F>
where
    F: NoteFormatter<Output = Vec<String>>,
{
    #[inline(always)]
    fn title(&mut self, title: &str) {
        self.lines.extend(F::title(title));
    }

    #[inline(always)]
    fn variant_header(&mut self, variant: &NoteVariant) {
        self.lines.extend(F::variant_header(variant));
    }

    #[inline(always)]
    fn release_note(
        &mut self,
        variant: &NoteVariant,
        ticket: &str,
        description: &str,
        issue: &str,
    ) {
        self.lines
            .extend(F::release_note(ticket, description, issue, variant));
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
