use std::path::Path;

use svg2web_core::{Error, Result, SVGElement};

pub struct BinaryReader;

impl BinaryReader {
	#[cfg(feature = "binary-format")]
	pub fn read(path: &Path) -> Result<SVGElement> {
		let content = std::fs::read(path)
			.map_err(|e| Error::Serialize(svg2web_core::serializer::Error::Json(serde_json::Error::io(e))))?;
		bincode::deserialize(&content)
			.map_err(|e| Error::Serialize(svg2web_core::serializer::Error::Json(serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))))
	}

	#[cfg(not(feature = "binary-format"))]
	pub fn read(_path: &Path) -> Result<SVGElement> {
		Err(Error::Serialize(
			svg2web_core::serializer::Error::BinaryDisabled,
		))
	}
}
