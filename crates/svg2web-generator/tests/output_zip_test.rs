use std::io::Cursor;

use svg2web_generator::output::write_zip;
use svg2web_generator::registry::{OutputFile, RenderedOutput};
use zip::ZipArchive;

#[test]
fn test_zip_single_file() {
    let output = RenderedOutput {
        files: vec![OutputFile {
            name: "test.txt".to_string(),
            content: "hello".to_string(),
        }],
    };

    let mut buf = Cursor::new(Vec::new());
    write_zip(&output, &mut buf).expect("write_zip should succeed");

    buf.set_position(0);
    let archive = ZipArchive::new(buf).expect("zip archive must be readable");
    assert_eq!(archive.len(), 1);
}

#[test]
fn test_zip_multiple_files() {
    let output = RenderedOutput {
        files: vec![
            OutputFile {
                name: "a.html".to_string(),
                content: "A".to_string(),
            },
            OutputFile {
                name: "b.css".to_string(),
                content: "B".to_string(),
            },
        ],
    };

    let mut buf = Cursor::new(Vec::new());
    write_zip(&output, &mut buf).expect("write_zip should succeed");

    buf.set_position(0);
    let archive = ZipArchive::new(buf).expect("zip archive must be readable");
    assert_eq!(archive.len(), 2);
}
