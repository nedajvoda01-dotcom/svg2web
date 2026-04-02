use crate::model::SVGElement;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn deduplicate_elements(root: &mut SVGElement) -> usize {
	deduplicate_recursive(root)
}

fn deduplicate_recursive(node: &mut SVGElement) -> usize {
	let mut removed = 0usize;
	for child in &mut node.children {
		removed += deduplicate_recursive(child);
	}

	let mut seen: HashMap<u64, usize> = HashMap::new();
	let mut unique_children: Vec<SVGElement> = Vec::with_capacity(node.children.len());

	for child in node.children.drain(..) {
		let signature = subtree_signature(&child);
		let hash = stable_hash(&signature);

		if seen.contains_key(&hash) {
			removed += 1;
		} else {
			seen.insert(hash, unique_children.len());
			unique_children.push(child);
		}
	}

	node.children = unique_children;
	removed
}

fn subtree_signature(element: &SVGElement) -> String {
	let mut pairs: Vec<(&str, &str)> = element
		.attributes
		.iter()
		.map(|(k, v)| (k.as_str(), v.as_str()))
		.collect();
	pairs.sort_unstable_by(|a, b| a.0.cmp(b.0).then(a.1.cmp(b.1)));

	let attrs = pairs
		.iter()
		.map(|(k, v)| format!("{k}={v}"))
		.collect::<Vec<_>>()
		.join(";");

	let children = element
		.children
		.iter()
		.map(subtree_signature)
		.collect::<Vec<_>>()
		.join("|");

	format!(
		"id={:?};tag={};etype={:?};attrs={};text={:?};children=[{}]",
		element.id, element.tag, element.element_type, attrs, element.text_content, children
	)
}

fn stable_hash(input: &str) -> u64 {
	let mut hasher = DefaultHasher::new();
	input.hash(&mut hasher);
	hasher.finish()
}
