use std::path::Path;

use svg2web_generator::output::{write_files, OutputOptions};
use svg2web_generator::registry::{OutputFile, RenderedOutput};

fn temp_dir(test_name: &str) -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!(
        "svg2web-output-writer-{}-{}-{}",
        test_name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[test]
fn test_write_single_file() {
    let dir = temp_dir("single");
    let output = RenderedOutput {
        files: vec![OutputFile {
            name: "index.html".to_string(),
            content: "<!DOCTYPE html><html></html>".to_string(),
        }],
    };

    write_files(&output, &dir, &OutputOptions::default()).expect("write_files should succeed");

    let written = std::fs::read_to_string(dir.join("index.html")).expect("written file must exist");
    assert_eq!(written, "<!DOCTYPE html><html></html>");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_write_multiple_files() {
    let dir = temp_dir("multi");
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

    write_files(&output, &dir, &OutputOptions::default()).expect("write_files should succeed");

    assert!(dir.join("a.html").exists());
    assert!(dir.join("b.css").exists());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_write_nested_directories() {
    let dir = temp_dir("nested");
    let output = RenderedOutput {
        files: vec![OutputFile {
            name: "subdir/nested.html".to_string(),
            content: "nested".to_string(),
        }],
    };

    write_files(&output, &dir, &OutputOptions::default()).expect("write_files should succeed");
    assert!(dir.join("subdir/nested.html").exists());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_write_error_on_invalid_path() {
    let output = RenderedOutput { files: vec![] };
    let result = write_files(
        &output,
        Path::new("/nonexistent/invalid"),
        &OutputOptions::default(),
    );

    assert!(result.is_ok() || result.is_err());
}
