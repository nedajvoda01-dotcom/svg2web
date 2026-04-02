use std::fs;

#[test]
fn changelog_format_t004() {
    let s = fs::read_to_string("CHANGELOG.md").expect("CHANGELOG.md missing");
    assert!(s.contains("## [Unreleased]"), "Missing Unreleased section");
    assert!(s.contains("### Added"), "Missing Added section");
    assert!(s.contains("### Changed"), "Missing Changed section");
    assert!(s.contains("### Deprecated"), "Missing Deprecated section");
    assert!(s.contains("### Removed"), "Missing Removed section");
    assert!(s.contains("### Fixed"), "Missing Fixed section");
    assert!(s.contains("### Security"), "Missing Security section");
}
