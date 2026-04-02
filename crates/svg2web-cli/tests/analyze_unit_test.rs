use std::fs;
use std::path::PathBuf;

use tempfile::tempdir;

use svg2web_cli::commands::analyze::{AnalyzeCommand, OutputFormat};

#[test]
fn test_analyze_command_validate_missing_input() {
    let cmd = AnalyzeCommand {
        input: None,
        format: OutputFormat::Json,
    };

    assert!(cmd.validate().is_err());
}

#[test]
fn test_analyze_command_validate_valid() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("test.svg");
    fs::write(&input, "<svg/>").unwrap();

    let cmd = AnalyzeCommand {
        input: Some(input),
        format: OutputFormat::Json,
    };

    assert!(cmd.validate().is_ok());
}

#[test]
fn test_output_format_json() {
    let format = OutputFormat::Json;
    assert_eq!(format.to_string(), "json");
}

#[test]
fn test_output_format_human() {
    let format = OutputFormat::Human;
    assert_eq!(format.to_string(), "human");
}