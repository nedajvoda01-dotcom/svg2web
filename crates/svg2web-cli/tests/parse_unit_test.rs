use std::fs;
use std::path::PathBuf;

use tempfile::tempdir;

use svg2web_cli::commands::parse::ParseCommand;

#[test]
fn test_parse_command_validate_missing_input() {
    let cmd = ParseCommand {
        input: None,
        output: None,
    };

    assert!(cmd.validate().is_err(), "Should fail without input");
}

#[test]
fn test_parse_command_validate_nonexistent_file() {
    let cmd = ParseCommand {
        input: Some(PathBuf::from("/nonexistent/file.svg")),
        output: None,
    };

    assert!(cmd.validate().is_err(), "Should fail if file doesn't exist");
}

#[test]
fn test_parse_command_validate_valid() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("test.svg");
    fs::write(&input, "<svg/>").unwrap();

    let cmd = ParseCommand {
        input: Some(input),
        output: None,
    };

    assert!(cmd.validate().is_ok(), "Should pass with valid input");
}