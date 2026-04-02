use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn parse_workspace_version() -> (u64, u64, u64) {
    let root = fs::read_to_string("Cargo.toml").unwrap_or_default();
    let marker = "version=\"";
    if let Some(start) = root.find(marker) {
        let rest = &root[start + marker.len()..];
        let version = rest.split('"').next().unwrap_or("0.0.0");
        let parts = version.split('.').collect::<Vec<_>>();
        let major = parts.first().and_then(|v| v.parse().ok()).unwrap_or(0);
        let minor = parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0);
        let patch = parts.get(2).and_then(|v| v.parse().ok()).unwrap_or(0);
        return (major, minor, patch);
    }
    (0, 0, 0)
}

#[test]
fn changelog_date_format_iso8601_t114() {
    let s = fs::read_to_string("docs/migration/changelog.md").unwrap_or_default();
    for line in s.lines() {
        if line.starts_with("## [") && line.contains(" - ") {
            let date = line.split(" - ").nth(1).unwrap_or_default().trim();
            let parts = date.split('-').collect::<Vec<_>>();
            assert!(parts.len() == 3, "date must use YYYY-MM-DD: {date}");
            assert!(parts[0].len() == 4 && parts[1].len() == 2 && parts[2].len() == 2, "date must be ISO 8601 date: {date}");
            assert!(parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit())), "date must contain only digits and hyphens: {date}");
        }
    }
}

#[test]
fn migration_script_execution_t115() {
    let docs = fs::read_to_string("docs/migration/v0.1-to-v0.2.md").unwrap_or_default();
    assert!(docs.contains("scripts/migrations/v0.1-to-v0.2.sh"));
    assert!(docs.contains("component_detection -> components.detect"));
    assert!(docs.contains("min_component_size -> components.min_size"));

    let script = PathBuf::from("scripts/migrations/v0.1-to-v0.2.sh");
    assert!(script.exists(), "migration script must exist");

    let temp_dir = std::env::temp_dir().join("svg2web-migration-test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("failed to create temp dir");
    let config_path = temp_dir.join("svg2web.toml");
    fs::write(
        &config_path,
        "component_detection = true\nmin_component_size = 2\ncmd = 'svg2web analyze input.svg'\n",
    )
    .expect("failed to write temp config");

    let status = Command::new("sh")
        .arg(script)
        .arg(&config_path)
        .status()
        .expect("failed to execute migration script");
    assert!(status.success(), "migration script must succeed");

    let migrated = fs::read_to_string(&config_path).expect("failed to read migrated config");
    assert!(migrated.contains("components.detect"));
    assert!(migrated.contains("components.min_size"));
    assert!(migrated.contains("svg2web parse --analyze input.svg"));
}

#[test]
fn deprecation_warning_display_t116() {
    let s = fs::read_to_string("docs/migration/deprecations.md").unwrap_or_default();
    assert!(s.contains("component_detection"));
    assert!(s.contains("components.detect"));
    assert!(s.contains("warning: `component_detection` is deprecated since 0.2.0 and will be removed in 0.4.0"));
    assert!(s.contains("help: use `components.detect`"));
}

#[test]
fn removal_schedule_checked_t117() {
    let s = fs::read_to_string("docs/migration/deprecations.md").unwrap_or_default();
    assert!(s.contains("0.4.0"), "deprecations must declare removal schedule for component_detection");

    let version = parse_workspace_version();
    if version >= (0, 4, 0) {
        let config_docs = fs::read_to_string("docs/usage/config.md").unwrap_or_default();
        assert!(!config_docs.contains("component_detection"), "deprecated field must be removed in 0.4.0+");
    }
}
