fn parse_workspace_version() -> Option<String> {
    let root = std::fs::read_to_string("Cargo.toml").ok()?;
    let marker = "version=\"";
    let start = root.find(marker)? + marker.len();
    let rest = &root[start..];
    Some(rest.split('"').next()?.to_string())
}

#[test]
fn semver_versioning_t102() {
    let docs = std::fs::read_to_string("docs/development/releasing.md").unwrap_or_default();
    assert!(docs.contains("SemVer: MAJOR.MINOR.PATCH"));
    let version = parse_workspace_version().unwrap_or_default();
    let parts: Vec<&str> = version.split('.').collect();
    assert!(parts.len() == 3 && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit())), "workspace version must be SemVer-like, got {version}");
}

#[test]
fn changelog_format_t103() {
    let s = std::fs::read_to_string("CHANGELOG.md").unwrap_or_default();
    for section in ["### Added", "### Changed", "### Deprecated", "### Removed", "### Fixed", "### Security"] {
        assert!(s.contains(section), "CHANGELOG missing section: {section}");
    }
}

#[test]
fn multi_platform_binaries_t104() {
    let docs = std::fs::read_to_string("docs/development/releasing.md").unwrap_or_default();
    for asset in ["Linux x64", "macOS x64", "macOS arm64", "Windows x64"] {
        assert!(docs.contains(asset), "release asset missing from docs: {asset}");
    }
    assert!(docs.contains("GitHub Release"));
}

#[test]
fn docker_multi_platform_t105() {
    let docs = std::fs::read_to_string("docs/development/releasing.md").unwrap_or_default();
    assert!(docs.contains("docker buildx build"));
    assert!(docs.contains("linux/amd64,linux/arm64"));
}