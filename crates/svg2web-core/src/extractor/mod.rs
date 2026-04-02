pub mod external;
pub mod fonts;
pub mod images;

use crate::model::asset::{ExternalAsset, FontAsset, ImageAsset};
use crate::model::SVGElement;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ExtractOptions {
	pub extract_images: bool,
	pub convert_to_webp: bool,
	pub extract_fonts: bool,
	pub fetch_external: bool,
}

impl Default for ExtractOptions {
	fn default() -> Self {
		Self {
			extract_images: true,
			convert_to_webp: false,
			extract_fonts: true,
			fetch_external: false,
		}
	}
}

#[derive(Debug, Clone, Default)]
pub struct ExtractedAssets {
	pub images: Vec<ImageAsset>,
	pub fonts: Vec<FontAsset>,
	pub external: Vec<ExternalAsset>,
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("unsupported operation in core extractor: {0}")]
	Unsupported(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn extract_assets(element: &SVGElement, options: &ExtractOptions) -> Result<ExtractedAssets> {
	let mut extracted = ExtractedAssets::default();

	if options.extract_images {
		extracted.images = images::extract_base64_images(element);
		if options.convert_to_webp {
			extracted
				.images
				.iter_mut()
				.for_each(images::mark_for_webp_conversion);
		}
	}

	if options.extract_fonts {
		extracted.fonts = fonts::detect_font_families(element);
	}

	extracted.external = external::resolve_external_references(element);

	if options.fetch_external {
		return Err(Error::Unsupported(
			"fetching external resources is disabled in core; use CLI layer",
		));
	}

	Ok(extracted)
}
