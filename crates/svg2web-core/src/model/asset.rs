use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Asset {
	Image(ImageAsset),
	Font(FontAsset),
	External(ExternalAsset),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAsset {
	pub id: String,
	pub format: ImageFormat,
	pub data: Vec<u8>,
	pub width: u32,
	pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
	Png,
	Jpeg,
	Webp,
	Gif,
	Svg,
	Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontAsset {
	pub family: String,
	pub source: FontSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontSource {
	Embedded(Vec<u8>),
	Local(String),
	Url(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAsset {
	pub url: String,
	pub data: Option<Vec<u8>>,
}
