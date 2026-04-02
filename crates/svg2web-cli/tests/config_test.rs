use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use svg2web_cli::config::load_config;

fn unique_temp_dir(name: &str) -> std::path::PathBuf {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("svg2web-cli-{name}-{ts}"))
}

#[test]
fn load_config_without_path_returns_defaults() {
    let cfg = load_config(None).expect("default config should load");
    assert_eq!(cfg.build.format, "vanilla");
    assert_eq!(cfg.build.output_dir, "dist");
}

#[test]
fn load_config_reads_toml_values() {
    let root = unique_temp_dir("config-ok");
    fs::create_dir_all(&root).expect("failed to create temp root");

    let path = root.join("svg2web.toml");
    fs::write(
        &path,
        r#"
[build]
format = "react"
output_dir = "web-output"
"#,
    )
    .expect("failed to write config");

    let cfg = load_config(Some(Path::new(&path))).expect("config should parse");
    assert_eq!(cfg.build.format, "react");
    assert_eq!(cfg.build.output_dir, "web-output");

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn load_config_fails_for_invalid_toml() {
    let root = unique_temp_dir("config-bad");
    fs::create_dir_all(&root).expect("failed to create temp root");

    let path = root.join("bad.toml");
    fs::write(&path, "[build\nformat='react'").expect("failed to write invalid config");

    let err = load_config(Some(Path::new(&path))).expect_err("invalid toml must fail");
    let msg = err.to_string().to_lowercase();
    assert!(msg.contains("toml") || msg.contains("parse"));

    let _ = fs::remove_dir_all(&root);
}
