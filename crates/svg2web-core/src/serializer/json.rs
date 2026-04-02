use super::{Error, Result};
use crate::model::SVGElement;

#[derive(Debug, Default)]
pub struct JsonSerializer;

impl JsonSerializer {
	pub fn serialize(element: &SVGElement, pretty: bool) -> Result<String> {
		if pretty {
			serde_json::to_string_pretty(element).map_err(Error::from)
		} else {
			serde_json::to_string(element).map_err(Error::from)
		}
	}

	pub fn deserialize(json: &str) -> Result<SVGElement> {
		serde_json::from_str(json).map_err(Error::from)
	}
}
