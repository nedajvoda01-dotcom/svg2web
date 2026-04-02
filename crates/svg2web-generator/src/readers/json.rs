use std::path::Path;

use svg2web_core::{deserialize, Error, Format, Result, SVGElement};

pub struct JsonReader;

impl JsonReader {
	pub fn read(path: &Path) -> Result<SVGElement> {
		let content = std::fs::read(path)
			.map_err(|_| Error::Serialize(svg2web_core::serializer::Error::BinaryDisabled))?;
		deserialize(&content, Format::Json)
	}

	pub fn read_str(json: &str) -> Result<SVGElement> {
		deserialize(json.as_bytes(), Format::Json)
	}
}
