use clap::Parser;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use svg2web_cli::commands::{run, Cli, Commands};

fn unique_temp_dir(name: &str) -> std::path::PathBuf {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("svg2web-cli-{name}-{ts}"))
}

#[test]
fn parses_build_command_with_common_flags() {
    let cli = Cli::parse_from([
        "svg2web",
        "build",
        "./input.svg",
        "--output",
        "./dist",
        "--format",
        "react",
    ]);

    match cli.command {
        Commands::Build(args) => {
            assert_eq!(args.input, std::path::PathBuf::from("./input.svg"));
            assert_eq!(args.output, Some(std::path::PathBuf::from("./dist")));
            assert_eq!(args.format.as_deref(), Some("react"));
        }
    }
}

#[tokio::test]
async fn run_build_creates_output_directory() {
    let root = unique_temp_dir("run-build");
    fs::create_dir_all(&root).expect("failed to create temp root");

    let input = root.join("icon.svg");
    fs::write(
        &input,
        r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><rect x=\"1\" y=\"1\" width=\"14\" height=\"14\"/></svg>"#,
    )
    .expect("failed to write input svg");

    let output = root.join("dist");
    let cli = Cli::parse_from([
        "svg2web",
        "build",
        input.to_string_lossy().as_ref(),
        "--output",
        output.to_string_lossy().as_ref(),
    ]);

    run(cli).await.expect("build command should succeed");

    assert!(output.exists(), "build should create output directory");

    let _ = fs::remove_dir_all(&root);
}
