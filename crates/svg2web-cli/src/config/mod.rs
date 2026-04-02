use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
	pub build: BuildConfig,
}

#[derive(Debug, Clone)]
pub struct BuildConfig {
	pub format: String,
	pub output_dir: String,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			build: BuildConfig::default(),
		}
	}
}

impl Default for BuildConfig {
	fn default() -> Self {
		Self {
			format: "vanilla".to_string(),
			output_dir: "dist".to_string(),
		}
	}
}

pub fn load_config(path: Option<&Path>) -> Result<Config> {
	let Some(path) = path else {
		return Ok(Config::default());
	};

	let raw = fs::read_to_string(path)
		.with_context(|| format!("Failed to read config: {}", path.display()))?;

	parse_config_toml(&raw)
		.with_context(|| format!("Failed to parse TOML config: {}", path.display()))
}

fn parse_config_toml(raw: &str) -> Result<Config> {
	let mut cfg = Config::default();
	let mut section = String::new();

	for line in raw.lines() {
		let trimmed = line.trim();
		if trimmed.is_empty() || trimmed.starts_with('#') {
			continue;
		}

		if trimmed.starts_with('[') {
			if !(trimmed.ends_with(']') && trimmed.len() >= 2) {
				anyhow::bail!("Invalid section header: {trimmed}");
			}
			section = trimmed[1..trimmed.len() - 1].trim().to_string();
			continue;
		}

		let Some((key, value)) = trimmed.split_once('=') else {
			anyhow::bail!("Invalid key/value line: {trimmed}");
		};

		if section == "build" {
			let key = key.trim();
			let value = parse_string_value(value.trim())?;
			match key {
				"format" => cfg.build.format = value,
				"output_dir" => cfg.build.output_dir = value,
				_ => {}
			}
		}
	}

	Ok(cfg)
}

fn parse_string_value(value: &str) -> Result<String> {
	if value.len() >= 2 {
		let first = value.as_bytes()[0] as char;
		let last = value.as_bytes()[value.len() - 1] as char;
		if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
			return Ok(value[1..value.len() - 1].to_string());
		}
	}

	anyhow::bail!("Expected quoted string value, got: {value}")
}
