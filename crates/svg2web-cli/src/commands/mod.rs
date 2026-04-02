use anyhow::{bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

use crate::config::load_config;

#[derive(Debug, Parser)]
#[command(name = "svg2web", version, about = "Convert SVG assets to web-ready output")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	Build(BuildArgs),
}

#[derive(Debug, Args)]
pub struct BuildArgs {
	pub input: PathBuf,

	#[arg(long)]
	pub output: Option<PathBuf>,

	#[arg(long)]
	pub format: Option<String>,

	#[arg(long)]
	pub config: Option<PathBuf>,
}

pub async fn run(cli: Cli) -> Result<()> {
	match cli.command {
		Commands::Build(args) => run_build(args),
	}
}

fn run_build(args: BuildArgs) -> Result<()> {
	if !args.input.exists() {
		bail!("Input SVG not found: {}", args.input.display());
	}

	let config = load_config(args.config.as_deref())?;
	let format = args.format.unwrap_or(config.build.format);
	if !matches!(format.as_str(), "vanilla" | "react" | "vue") {
		bail!("Unsupported format: {format}");
	}

	let output_dir = args
		.output
		.unwrap_or_else(|| PathBuf::from(config.build.output_dir));

	fs::create_dir_all(&output_dir)
		.with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

	let build_info = format!(
		"input={}\nformat={}\n",
		args.input.display(),
		format
	);
	fs::write(output_dir.join("build-info.txt"), build_info)
		.context("Failed to write build metadata")?;

	Ok(())
}
