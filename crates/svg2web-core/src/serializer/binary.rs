use super::{Error, Result};
use crate::model::SVGElement;

#[derive(Debug, Default)]
pub struct BinarySerializer;

impl BinarySerializer {
	#[cfg(feature = "binary-format")]
	pub fn serialize(element: &SVGElement) -> Result<Vec<u8>> {
		bincode::serialize(element).map_err(Error::from)
	}

	#[cfg(not(feature = "binary-format"))]
	pub fn serialize(_element: &SVGElement) -> Result<Vec<u8>> {
		Err(Error::BinaryDisabled)
	}

	#[cfg(feature = "binary-format")]
	pub fn deserialize(data: &[u8]) -> Result<SVGElement> {
		bincode::deserialize(data).map_err(Error::from)
	}

	#[cfg(not(feature = "binary-format"))]
	pub fn deserialize(_data: &[u8]) -> Result<SVGElement> {
		Err(Error::BinaryDisabled)
	}
}
