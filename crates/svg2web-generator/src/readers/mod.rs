use std::path::Path;

use svg2web_core::{Result, SVGElement};

pub mod binary;
pub mod json;

pub use binary::BinaryReader;
pub use json::JsonReader;

pub fn read_json(path: &Path) -> Result<SVGElement> {
	JsonReader::read(path)
}

pub fn read_binary(path: &Path) -> Result<SVGElement> {
	BinaryReader::read(path)
}
