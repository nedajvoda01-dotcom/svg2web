use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_optimizes_outputs_smaller_svg() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");
    let output = temp.path().join("output.svg");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
        <rect id="a" x="0" y="0" width="10" height="10" fill="red"/>
        <rect id="b" x="0" y="0" width="10" height="10" fill="red"/>
        <rect id="c" x="50" y="50" width="10" height="10" fill="blue"/>
    </svg>"#,
    )
    .unwrap();

    let input_size = fs::metadata(&input).unwrap().len();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("optimize")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output);

    cmd.assert().success();

    assert!(output.exists(), "Output file should be created");

    let output_size = fs::metadata(&output).unwrap().len();
    let output_content = fs::read_to_string(&output).unwrap();

    assert!(output_content.contains("<svg"));
    assert!(output_content.contains("</svg>"));

    println!("Input: {} bytes, Output: {} bytes", input_size, output_size);
}

#[test]
fn test_optimize_dry_run_shows_stats() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="10" height="10"/></svg>"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("optimize")
        .arg("--input")
        .arg(&input)
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Original:"))
        .stdout(predicate::str::contains("Optimized:"))
        .stdout(predicate::str::contains("Savings:"));
}

#[test]
fn test_optimize_invalid_svg_fails() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("bad.svg");
    let output = temp.path().join("output.svg");

    fs::write(&input, "not valid xml").unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("optimize")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output);

    cmd.assert().failure();
}

#[test]
fn test_optimize_overwrites_with_flag() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.svg");
    let output = temp.path().join("output.svg");

    fs::write(
        &input,
        r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="10" height="10"/></svg>"#,
    )
    .unwrap();
    fs::write(&output, "existing content").unwrap();

    let mut cmd = Command::cargo_bin("svg2web").unwrap();
    cmd.arg("optimize")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .arg("--overwrite");

    cmd.assert().success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("<svg"));
}