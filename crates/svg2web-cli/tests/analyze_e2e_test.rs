use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_analyze_outputs_json_report() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
        <rect x="0" y="0" width="10" height="10"/>
        <rect x="0" y="0" width="10" height="10"/>
        <circle cx="50" cy="50" r="25"/>
    </svg>"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("analyze")
        .arg("--input")
        .arg(&input)
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"score\""))
        .stdout(predicate::str::contains("\"node_count\""))
        .stdout(predicate::str::contains("\"component_count\""));
}

#[test]
fn test_analyze_human_format() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="10" height="10"/></svg>"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("analyze")
        .arg("--input")
        .arg(&input)
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Score:"))
        .stdout(predicate::str::contains("Nodes:"));
}

#[test]
fn test_analyze_detects_components() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg">
        <g id="icon"><rect width="10" height="10"/></g>
        <g id="icon2"><rect width="10" height="10"/></g>
    </svg>"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("analyze")
        .arg("--input")
        .arg(&input)
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"components\""));
}

#[test]
fn test_analyze_invalid_svg_fails() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("bad.svg");

    fs::write(&input, "not valid xml").unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("analyze").arg("--input").arg(&input);

    cmd.assert().failure();
}