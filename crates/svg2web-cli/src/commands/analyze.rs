use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;

use clap::{Args, ValueEnum};

use svg2web_core::{analyze, parse};

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
	Json,
	Human,
}

impl Display for OutputFormat {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			OutputFormat::Json => write!(f, "json"),
			OutputFormat::Human => write!(f, "human"),
		}
	}
}

#[derive(Args, Debug)]
pub struct AnalyzeArgs {
	/// Input SVG file
	#[arg(short, long)]
	pub input: Option<PathBuf>,

	/// Output format
	#[arg(short, long, value_enum, default_value = "human")]
	pub format: OutputFormat,
}

pub struct AnalyzeCommand {
	pub input: Option<PathBuf>,
	pub format: OutputFormat,
}

impl AnalyzeCommand {
	pub fn from_args(args: AnalyzeArgs) -> Self {
		Self {
			input: args.input,
			format: args.format,
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

		let analysis = analyze(&element);

		match self.format {
			OutputFormat::Json => {
				let report = serde_json::json!({
					"score": analysis.complexity.score,
					"node_count": analysis.complexity.node_count,
					"path_count": analysis.complexity.path_count,
					"component_count": analysis.components.len(),
					"components": analysis
						.components
						.iter()
						.map(|c| {
							serde_json::json!({
								"id": c.id,
								"occurrences": c.occurrences.len(),
							})
						})
						.collect::<Vec<_>>(),
					"hierarchy": {
						"depth": analysis.hierarchy.depth,
						"width": analysis.hierarchy.max_width,
					}
				});
				println!("{}", serde_json::to_string_pretty(&report)?);
			}
			OutputFormat::Human => {
				println!("SVG Analysis Report");
				println!("==================");
				println!("Score: {}/100", analysis.complexity.score);
				println!("Nodes: {}", analysis.complexity.node_count);
				println!("Paths: {}", analysis.complexity.path_count);
				println!("Components: {}", analysis.components.len());
				if !analysis.components.is_empty() {
					println!("Component list:");
					for c in &analysis.components {
						println!("  - {} ({} occurrences)", c.id, c.occurrences.len());
					}
				}
				println!("Hierarchy depth: {}", analysis.hierarchy.depth);
				println!("Max width: {}", analysis.hierarchy.max_width);
			}
		}

		Ok(())
	}
}

fn boxed_err(msg: impl Into<String>) -> Box<dyn std::error::Error> {
	Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg.into()))
}
