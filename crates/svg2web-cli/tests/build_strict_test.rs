use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_build_strict_passes_good_quality() {
	let temp = tempdir().unwrap();
	let input = temp.path().join("good.svg");
	let output = temp.path().join("out");

	fs::write(
		&input,
		r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10"/></svg>"#,
	)
	.unwrap();

	let mut cmd = Command::cargo_bin("svg2web").unwrap();
	cmd.arg("build")
		.arg("--input")
		.arg(&input)
		.arg("--output")
		.arg(&output)
		.arg("--format")
		.arg("vanilla")
		.arg("--strict");

	cmd.assert().success();
}

#[test]
fn test_build_strict_fails_poor_quality() {
	let temp = tempdir().unwrap();
	let input = temp.path().join("bad.svg");
	let output = temp.path().join("out");

	fs::write(
		&input,
		r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" filter="url(#blur)"><defs><filter id="blur"><feGaussianBlur stdDeviation="5"/></filter></defs><rect width="100" height="100" fill="black"/></svg>"#,
	)
	.unwrap();

	let mut cmd = Command::cargo_bin("svg2web").unwrap();
	cmd.arg("build")
		.arg("--input")
		.arg(&input)
		.arg("--output")
		.arg(&output)
		.arg("--strict");

	cmd.assert().failure().stderr(
		predicate::str::contains("Quality gate failed").or(predicate::str::contains("PSNR")),
	);
}

#[test]
fn test_build_without_strict_allows_poor_quality() {
	let temp = tempdir().unwrap();
	let input = temp.path().join("bad.svg");
	let output = temp.path().join("out");

	fs::write(
		&input,
		r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" filter="url(#blur)"><defs><filter id="blur"><feGaussianBlur stdDeviation="5"/></filter></defs><rect width="100" height="100" fill="black"/></svg>"#,
	)
	.unwrap();

	let mut cmd = Command::cargo_bin("svg2web").unwrap();
	cmd.arg("build")
		.arg("--input")
		.arg(&input)
		.arg("--output")
		.arg(&output);

	cmd.assert().success();
}
