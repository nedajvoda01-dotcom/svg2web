pub mod geometry;
pub mod styles;
pub mod svg;
pub mod text;

use crate::model::SVGElement;
use std::path::Path;
pub use svg::UsvgParser;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ParseOptions {
	pub normalize: bool,
	pub keep_comments: bool,
}

impl Default for ParseOptions {
	fn default() -> Self {
		Self {
			normalize: true,
			keep_comments: false,
		}
	}
}

#[derive(Debug, Error)]
pub enum ParseError {
	#[error("failed to parse SVG with usvg: {0}")]
	Usvg(#[from] usvg::Error),
	#[error("empty SVG payload")]
	EmptyInput,
	#[error("file parsing is disabled in core; provide SVG bytes/string from caller")]
	FileParsingDisabled,
}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Default)]
pub struct Parser {
	pub options: ParseOptions,
}

impl Parser {
	pub fn new() -> Self {
		Self {
			options: ParseOptions::default(),
		}
	}

	pub fn with_options(options: ParseOptions) -> Self {
		Self { options }
	}

	pub fn parse(&self, svg: &str) -> Result<SVGElement> {
		parse_str(svg, self.options.clone())
	}
}

pub fn parse(svg: &str) -> Result<SVGElement> {
	parse_str(svg, ParseOptions::default())
}

pub fn parse_file(_path: &Path) -> Result<SVGElement> {
	Err(ParseError::FileParsingDisabled)
}

pub fn parse_str(svg: &str, _options: ParseOptions) -> Result<SVGElement> {
	if svg.trim().is_empty() {
		return Err(ParseError::EmptyInput);
	}

	let tree = UsvgParser::load(svg)?;
	Ok(UsvgParser::to_model(&tree))
}
