use noter::variant;
use noter::{NoteWriter, StringWriter};

fn basic_writer_example(writer: &mut StringWriter) {
    writer.title("A title string");
    writer.spacing(1);

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
fn test_string_writer_text() {
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
fn test_string_writer_markdown() {
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
