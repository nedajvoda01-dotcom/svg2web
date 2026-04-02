use std::fs;
use std::path::PathBuf;

use tempfile::tempdir;

use svg2web_cli::commands::build::BuildCommand;

#[test]
fn test_build_command_validate_missing_input() {
    let cmd = BuildCommand {
        input: None,
        output: PathBuf::from("out"),
        format: "vanilla".to_string(),
        cache: true,
        strict: false,
    };

    assert!(cmd.validate().is_err(), "Should fail without input");
}

#[test]
fn test_build_command_validate_missing_file() {
    let cmd = BuildCommand {
        input: Some(PathBuf::from("/nonexistent/file.svg")),
        output: PathBuf::from("out"),
        format: "vanilla".to_string(),
        cache: true,
        strict: false,
    };

    assert!(cmd.validate().is_err(), "Should fail if file doesn't exist");
}

#[test]
fn test_build_command_validate_valid() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("test.svg");
    fs::write(&input, "<svg/>").unwrap();

    let cmd = BuildCommand {
        input: Some(input),
        output: PathBuf::from("out"),
        format: "vanilla".to_string(),
        cache: true,
        strict: false,
    };

    assert!(cmd.validate().is_ok(), "Should pass with valid input");
}
