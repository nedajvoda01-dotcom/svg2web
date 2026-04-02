pub mod binary;
pub mod json;

use crate::model::SVGElement;
pub use binary::BinarySerializer;
pub use json::JsonSerializer;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum Format {
	Json,
	Binary,
}

#[derive(Debug, Clone, Copy)]
pub struct SerializeOptions {
	pub pretty: bool,
}

impl Default for SerializeOptions {
	fn default() -> Self {
		Self { pretty: false }
	}
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("json serialization error: {0}")]
	Json(#[from] serde_json::Error),
	#[cfg(feature = "binary-format")]
	#[error("binary serialization error: {0}")]
	Binary(#[from] bincode::Error),
	#[error("binary serialization is disabled; enable feature `binary-format`")]
	BinaryDisabled,
	#[error("utf-8 decode error: {0}")]
	Utf8(#[from] std::str::Utf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn serialize(element: &SVGElement, format: Format) -> Result<Vec<u8>> {
	serialize_with_options(element, format, SerializeOptions::default())
}

pub fn serialize_with_options(
	element: &SVGElement,
	format: Format,
	options: SerializeOptions,
) -> Result<Vec<u8>> {
	match format {
		Format::Json => Ok(JsonSerializer::serialize(element, options.pretty)?.into_bytes()),
		Format::Binary => BinarySerializer::serialize(element),
	}
}

pub fn deserialize(data: &[u8], format: Format) -> Result<SVGElement> {
	match format {
		Format::Json => {
			let json = std::str::from_utf8(data)?;
			JsonSerializer::deserialize(json)
		}
		Format::Binary => BinarySerializer::deserialize(data),
	}
}
