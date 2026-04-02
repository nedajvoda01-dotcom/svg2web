use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_parse_outputs_valid_json() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");
    let output = temp.path().join("output.json");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
        <rect x="0" y="0" width="10" height="10" fill="red"/>
    </svg>"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("parse")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output);

    cmd.assert().success();

    assert!(output.exists());
    let content = fs::read_to_string(&output).unwrap();

    assert!(content.contains("\"tag\""));
    assert!(content.contains("\"svg\""));
    assert!(content.contains("\"children\""));

    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["tag"], "svg");
}

#[test]
fn test_parse_roundtrip() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");
    let output = temp.path().join("output.json");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="5" cy="5" r="5"/></svg>"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("parse")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output);
    cmd.assert().success();

    let content = fs::read_to_string(&output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json.get("children").is_some());
    assert!(json.get("attributes").is_some());
}

#[test]
fn test_parse_invalid_svg_fails() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("bad.svg");
    let output = temp.path().join("output.json");

    fs::write(&input, "not valid xml").unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("parse")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("parse").or(predicate::str::contains("error")));
}

#[test]
fn test_parse_stdout_if_no_output() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");

    fs::write(&input, r#"<svg xmlns="http://www.w3.org/2000/svg"/>"#).unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("parse").arg("--input").arg(&input);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"tag\""));
}