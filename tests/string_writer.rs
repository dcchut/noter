use noter::variant;
use noter::{NoteWriter, StringWriter};

#[test]
fn test_string_writer() {
    let mut writer = StringWriter::new();

    writer.title("A title string");
    writer.spacing(1);

    writer.start_variant_list(variant!("basic", "Basic notes", true));
    writer.note(
        "TICKET0001",
        "Improve something or other",
        "<www.google.com>",
    );
    writer.note(
        "TICKET0002",
        "Improve something else too",
        "<www.google.com>",
    );
    writer.end_variant();

    assert_eq!(
        writer.write(),
        r#"A title string
--------------

Basic notes
===========
 - TICKET0001: Improve something or other <www.google.com>
 - TICKET0002: Improve something else too <www.google.com>
"#
    );
}
