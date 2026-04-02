use std::path::Path;

#[test]
fn unit_test_coverage_t095() {
    let docs = std::fs::read_to_string("docs/development/testing.md").unwrap_or_default();
    assert!(docs.contains("cargo-tarpaulin"));
    assert!(docs.contains(">= 80% покрытие для core/model"));
    assert!(docs.contains("--fail-under 80"));
}

#[test]
fn golden_files_for_parser_t096() {
    let docs = std::fs::read_to_string("docs/development/testing.md").unwrap_or_default();
    assert!(docs.contains("tests/fixtures/"));
    assert!(docs.contains("tests/snapshots/"));
    assert!(docs.contains("insta"));
    assert!(Path::new("tests/fixtures").is_dir(), "tests/fixtures must exist");
    assert!(Path::new("tests/snapshots").is_dir(), "tests/snapshots must exist");
}

#[test]
fn resvg_visual_regression_t097() {
    let docs = std::fs::read_to_string("docs/development/testing.md").unwrap_or_default();
    assert!(docs.contains("resvg"));
    assert!(docs.contains("< 1%"), "visual diff threshold must be documented as < 1%");
}