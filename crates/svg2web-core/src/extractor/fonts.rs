use crate::model::asset::{FontAsset, FontSource};
use crate::model::SVGElement;
use std::collections::BTreeSet;

pub fn detect_font_families(element: &SVGElement) -> Vec<FontAsset> {
	let mut names = BTreeSet::new();
	collect_font_families(element, &mut names);

	names
		.into_iter()
		.map(|family| FontAsset {
			source: FontSource::Local(family.clone()),
			family,
		})
		.collect()
}

fn collect_font_families(element: &SVGElement, names: &mut BTreeSet<String>) {
	if let Some(family) = element.attributes.get("font-family") {
		for part in family.split(',') {
			let cleaned = part.trim().trim_matches('"').trim_matches('\'');
			if !cleaned.is_empty() {
				names.insert(cleaned.to_string());
			}
		}
	}

	if let Some(style) = element.attributes.get("style") {
		if let Some(family) = style
			.split(';')
			.find_map(|entry| entry.trim().strip_prefix("font-family:"))
		{
			for part in family.split(',') {
				let cleaned = part.trim().trim_matches('"').trim_matches('\'');
				if !cleaned.is_empty() {
					names.insert(cleaned.to_string());
				}
			}
		}
	}

	for child in &element.children {
		collect_font_families(child, names);
	}
}
