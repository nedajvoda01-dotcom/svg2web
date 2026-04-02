use std::fs;
use std::path::PathBuf;

use clap::Args;

use svg2web_core::{optimize, parse, OptimizationConfig, SVGElement};

#[derive(Args, Debug)]
pub struct OptimizeArgs {
	/// Input SVG file
	#[arg(short, long)]
	pub input: Option<PathBuf>,

	/// Output SVG file
	#[arg(short, long)]
	pub output: Option<PathBuf>,

	/// Show optimization stats without writing
	#[arg(long)]
	pub dry_run: bool,

	/// Overwrite output if exists
	#[arg(long)]
	pub overwrite: bool,
}

pub struct OptimizeCommand {
	pub input: Option<PathBuf>,
	pub output: PathBuf,
	pub dry_run: bool,
	pub overwrite: bool,
}

impl OptimizeCommand {
	pub fn from_args(args: OptimizeArgs) -> Self {
		Self {
			input: args.input,
			output: args.output.unwrap_or_else(|| PathBuf::from("optimized.svg")),
			dry_run: args.dry_run,
			overwrite: args.overwrite,
		}
	}

	pub fn validate(&self) -> Result<(), String> {
		let input = self.input.as_ref().ok_or("Input file is required")?;
		if !input.exists() {
			return Err(format!("Input file not found: {}", input.display()));
		}
		if !self.dry_run && self.output.exists() && !self.overwrite {
			return Err(format!(
				"Output file already exists. Use --overwrite to replace: {}",
				self.output.display()
			));
		}
		Ok(())
	}

	pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
		self.validate().map_err(boxed_err)?;
		let input_path = self
			.input
			.as_ref()
			.ok_or_else(|| boxed_err("Input file is required"))?;

		let original_content = fs::read_to_string(input_path)?;
		let original_size = original_content.len();

		let mut element = parse(&original_content).map_err(|e| boxed_err(format!("parse error: {e}")))?;

		let config = OptimizationConfig {
			simplify_paths: true,
			deduplicate: true,
			minify_ids: true,
			..Default::default()
		};

		optimize(&mut element, &config).map_err(|e| boxed_err(format!("Optimization failed: {e}")))?;

		let optimized_content = serialize_to_svg_string(&element);
		let optimized_size = optimized_content.len();

		let savings = original_size.saturating_sub(optimized_size);
		let savings_percent = if original_size > 0 {
			(savings as f64 / original_size as f64 * 100.0) as u32
		} else {
			0
		};

		if self.dry_run {
			println!("Optimization Report (dry run)");
			println!("=============================");
			println!("Original:  {} bytes", original_size);
			println!("Optimized: {} bytes", optimized_size);
			println!("Savings:   {} bytes ({}%)", savings, savings_percent);
		} else {
			fs::write(&self.output, optimized_content)?;
			println!("✓ Optimized SVG written to {}", self.output.display());
			println!("  Original:  {} bytes", original_size);
			println!("  Optimized: {} bytes ({}% reduction)", optimized_size, savings_percent);
		}

		Ok(())
	}
}

fn serialize_to_svg_string(root: &SVGElement) -> String {
	serialize_node(root)
}

fn serialize_node(node: &SVGElement) -> String {
	let mut out = String::new();
	out.push('<');
	out.push_str(&node.tag);

	if let Some(id) = &node.id {
		out.push_str(" id=\"");
		out.push_str(&escape_xml(id));
		out.push('"');
	}

	let mut attrs = node.attributes.iter().collect::<Vec<_>>();
	attrs.sort_by(|a, b| a.0.cmp(b.0));
	for (k, v) in attrs {
		out.push(' ');
		out.push_str(k);
		out.push_str("=\"");
		out.push_str(&escape_xml(v));
		out.push('"');
	}

	let has_text = node.text_content.as_ref().is_some_and(|t| !t.is_empty());
	let has_children = !node.children.is_empty();
	let force_explicit_close = node.tag == "svg";

	if !has_text && !has_children && !force_explicit_close {
		out.push_str("/>");
		return out;
	}

	out.push('>');
	if let Some(text) = &node.text_content {
		out.push_str(&escape_xml(text));
	}

	for child in &node.children {
		out.push_str(&serialize_node(child));
	}

	out.push_str("</");
	out.push_str(&node.tag);
	out.push('>');
	out
}

fn escape_xml(s: &str) -> String {
	s.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
	.replace('\'', "&apos;")
}

fn boxed_err(msg: impl Into<String>) -> Box<dyn std::error::Error> {
	Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg.into()))
}
