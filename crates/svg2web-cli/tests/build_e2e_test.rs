use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_build_creates_output_files() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");
    let output = temp.path().join("out");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
        <rect x="0" y="0" width="10" height="10" fill="red"/>
    </svg>"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("build")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .arg("--format")
        .arg("vanilla");

    cmd.assert().success();

    assert!(output.exists(), "Output dir should be created");
    let files: Vec<_> = fs::read_dir(&output)
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    assert!(!files.is_empty(), "Should generate at least one file");

    let has_html = files
        .iter()
        .any(|f| f.to_string_lossy().ends_with(".html"));
    assert!(has_html, "Should generate HTML file for vanilla format");
}

#[test]
fn test_build_react_format() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("icon.svg");
    let output = temp.path().join("dist");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/></svg>"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("build")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .arg("--format")
        .arg("react");

    cmd.assert().success();

    let files: Vec<_> = fs::read_dir(&output)
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    let has_tsx = files
        .iter()
        .any(|f| f.to_string_lossy().ends_with(".tsx"));
    assert!(has_tsx, "Should generate TSX for React format");
}

#[test]
fn test_build_invalid_svg_shows_error() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("bad.svg");
    let output = temp.path().join("out");

    fs::write(&input, "not valid xml").unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("build")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("parse").or(predicate::str::contains("error")));
}

#[test]
fn test_build_uses_cache_on_second_run() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");
    let output = temp.path().join("out");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10"/></svg>"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();

    // First run
    cmd.arg("build")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .arg("--format")
        .arg("vanilla");
    cmd.assert().success();

    // Second run should be faster (cache hit) - проверяем что не падает
    let mut cmd2 = Command::cargo_bin("svg2web").unwrap();
    cmd2.arg("build")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .arg("--format")
        .arg("vanilla");
    cmd2.assert().success();
}
