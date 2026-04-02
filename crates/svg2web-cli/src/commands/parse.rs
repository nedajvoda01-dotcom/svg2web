use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Args;

use svg2web_core::parse;

#[derive(Args, Debug)]
pub struct ParseArgs {
	/// Input SVG file
	#[arg(short, long)]
	pub input: Option<PathBuf>,

	/// Output JSON file (stdout if not specified)
	#[arg(short, long)]
	pub output: Option<PathBuf>,
}

pub struct ParseCommand {
	pub input: Option<PathBuf>,
	pub output: Option<PathBuf>,
}

impl ParseCommand {
	pub fn from_args(args: ParseArgs) -> Self {
		Self {
			input: args.input,
			output: args.output,
		}
	}

	pub fn validate(&self) -> Result<(), String> {
		let input = self.input.as_ref().ok_or("Input file is required")?;
		if !input.exists() {
			return Err(format!("Input file not found: {}", input.display()));
		}
		Ok(())
	}

	pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
		self.validate().map_err(boxed_err)?;
		let input_path = self
			.input
			.as_ref()
			.ok_or_else(|| boxed_err("Input file is required"))?;

		let svg_content = fs::read_to_string(input_path)?;

		let element = parse(&svg_content).map_err(|e| boxed_err(format!("parse error: {e}")))?;

		let json = serde_json::to_string_pretty(&element)
			.map_err(|e| boxed_err(format!("Failed to serialize: {e}")))?;

		match &self.output {
			Some(path) => {
				fs::write(path, json)?;
			}
			None => {
				io::stdout().write_all(json.as_bytes())?;
				println!();
			}
		}

		Ok(())
	}
}

fn boxed_err(msg: impl Into<String>) -> Box<dyn std::error::Error> {
	Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg.into()))
}
