use std::fs;
use std::path::PathBuf;

use tempfile::tempdir;

use svg2web_cli::commands::optimize::OptimizeCommand;

#[test]
fn test_optimize_command_validate_missing_input() {
    let cmd = OptimizeCommand {
        input: None,
        output: PathBuf::from("out.svg"),
        dry_run: false,
        overwrite: false,
    };

    assert!(cmd.validate().is_err());
}

#[test]
fn test_optimize_command_validate_missing_file() {
    let cmd = OptimizeCommand {
        input: Some(PathBuf::from("/nonexistent/file.svg")),
        output: PathBuf::from("out.svg"),
        dry_run: false,
        overwrite: false,
    };

    assert!(cmd.validate().is_err());
}

#[test]
fn test_optimize_command_validate_valid() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("test.svg");
    fs::write(&input, "<svg/>").unwrap();

    let cmd = OptimizeCommand {
        input: Some(input),
        output: PathBuf::from("out.svg"),
        dry_run: false,
        overwrite: false,
    };

    assert!(cmd.validate().is_ok());
}