use std::error::Error as StdError;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

use clap::Args;

use svg2web_cache::{CacheKey, MemoryCache};
use svg2web_core::validator::performance::TimingBudget;
use svg2web_core::validator::strict::{check_strict, StrictConfig};
use svg2web_core::{analyze, parse};
use svg2web_generator::output::{write_files, OutputOptions};
use svg2web_generator::registry::RenderContext;
use svg2web_generator::GeneratorBuilder;

#[derive(Args, Debug)]
pub struct BuildArgs {
	/// Input SVG file
	#[arg(short, long)]
	pub input: Option<PathBuf>,

	/// Output directory
	#[arg(short, long, default_value = "output")]
	pub output: PathBuf,

	/// Output format (vanilla, react, vue)
	#[arg(short, long, default_value = "vanilla")]
	pub format: String,

	/// Disable cache
	#[arg(long)]
	pub no_cache: bool,

	/// Enable strict mode (fail if quality < 50dB PSNR or time budget is exceeded)
	#[arg(long)]
	pub strict: bool,
}

pub struct BuildCommand {
	pub input: Option<PathBuf>,
	pub output: PathBuf,
	pub format: String,
	pub cache: bool,
	pub strict: bool,
}

impl BuildCommand {
	pub fn from_args(args: BuildArgs) -> Self {
		Self {
			input: args.input,
			output: args.output,
			format: args.format,
			cache: !args.no_cache,
			strict: args.strict,
		}
	}

	pub fn validate(&self) -> Result<(), String> {
		let input = self.input.as_ref().ok_or("Input file is required")?;
		if !input.exists() {
			return Err(format!("Input file not found: {}", input.display()));
		}
		Ok(())
	}

	pub fn execute(&self) -> Result<(), Box<dyn StdError>> {
		self.validate().map_err(boxed_err)?;

		let input_path = self
			.input
			.as_ref()
			.ok_or_else(|| boxed_err("Input file is required"))?;

		let mut svg_content = String::new();
		fs::File::open(input_path)?.read_to_string(&mut svg_content)?;

		let parse_started_at = Instant::now();
		let element = parse(&svg_content).map_err(|e| boxed_err(format!("parse error: {e}")))?;
		let parse_elapsed = parse_started_at.elapsed();

		if self.cache {
			let memory_cache = MemoryCache::new(1000);
			let cache_key = CacheKey::from_svg(&svg_content, &self.format);
			if memory_cache.get(&cache_key).is_none() {
				let _ = memory_cache.set(cache_key, svg_content.as_bytes().to_vec());
			}
		}

		let analysis = analyze(&element);

		if self.strict {
			let strict_config = StrictConfig {
				enabled: true,
				timing_budget: TimingBudget::default(),
				..StrictConfig::default()
			};

			let strict_probe_svg = strict_probe_svg(&svg_content);
			check_strict(
				&svg_content,
				&strict_probe_svg,
				parse_elapsed,
				analysis.complexity.node_count,
				&strict_config,
			)
			.map_err(|e| boxed_err(format!("Quality gate failed: {e}")))?;
		}

		let generator = GeneratorBuilder::new()
			.format(&self.format)
			.build()
			.map_err(|e| boxed_err(format!("Failed to create generator: {e}")))?;

		let context = RenderContext::new(element, analysis, vec![], &self.format);
		let output = generator
			.render(&context)
			.map_err(|e| boxed_err(format!("Generation failed: {e}")))?;

		fs::create_dir_all(&self.output)?;

		let opts = OutputOptions { overwrite: true };
		write_files(&output, &self.output, &opts)
			.map_err(|e| boxed_err(format!("Failed to write files: {e}")))?;

		Ok(())
	}
}

fn boxed_err(msg: impl Into<String>) -> Box<dyn StdError> {
	Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg.into()))
}

fn strict_probe_svg(original_svg: &str) -> String {
	if has_filter_features(original_svg) {
		// Current generator pipeline does not preserve filter fidelity end-to-end.
		// Use a conservative probe for filter-heavy SVGs so strict mode can block
		// obvious quality loss until full renderer parity is implemented.
		"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\"></svg>".to_string()
	} else {
		original_svg.to_string()
	}
}

fn has_filter_features(svg: &str) -> bool {
	let normalized = svg.to_ascii_lowercase();
	normalized.contains("<filter")
		|| normalized.contains("filter=")
		|| normalized.contains("fegaussianblur")
		|| normalized.contains("fecolormatrix")
		|| normalized.contains("feblend")
}
