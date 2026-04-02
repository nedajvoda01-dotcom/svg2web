use super::{Error, Result};
use crate::model::asset::ExternalAsset;
use crate::model::SVGElement;

pub fn fetch_external_resource(_url: &str) -> Result<Vec<u8>> {
	Err(Error::Unsupported(
		"network fetch is not available in core; use CLI integration",
	))
}

pub fn resolve_external_references(element: &SVGElement) -> Vec<ExternalAsset> {
	let mut external = Vec::new();
	collect_external(element, &mut external);
	external
}

fn collect_external(element: &SVGElement, external: &mut Vec<ExternalAsset>) {
	for key in ["href", "xlink:href", "src"] {
		if let Some(value) = element.attributes.get(key) {
			if is_external_url(value) {
				external.push(ExternalAsset {
					url: value.clone(),
					data: None,
				});
			}
		}
	}

	for child in &element.children {
		collect_external(child, external);
	}
}

fn is_external_url(value: &str) -> bool {
	value.starts_with("http://") || value.starts_with("https://") || value.starts_with("//")
}
