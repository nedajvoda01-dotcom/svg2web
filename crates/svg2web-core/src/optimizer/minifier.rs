use crate::model::SVGElement;
use std::collections::HashMap;

pub fn minify_ids(root: &mut SVGElement) {
	let mut map = HashMap::new();
	let mut counter = 0usize;

	collect_and_rewrite_ids(root, &mut map, &mut counter);
	rewrite_id_references(root, &map);
}

fn collect_and_rewrite_ids(
	node: &mut SVGElement,
	map: &mut HashMap<String, String>,
	counter: &mut usize,
) {
	if let Some(current_id) = node.id.clone() {
		let short = map
			.entry(current_id.clone())
			.or_insert_with(|| next_short_id(counter))
			.clone();
		node.id = Some(short.clone());
		node.attributes.insert("id".to_string(), short);
	}

	for child in &mut node.children {
		collect_and_rewrite_ids(child, map, counter);
	}
}

fn rewrite_id_references(node: &mut SVGElement, map: &HashMap<String, String>) {
	let keys = ["href", "xlink:href", "fill", "stroke", "clip-path", "mask"];
	for key in keys {
		if let Some(value) = node.attributes.get_mut(key) {
			if let Some(id) = extract_id_reference(value) {
				if let Some(new_id) = map.get(id) {
					if value.starts_with("url(#") {
						*value = format!("url(#{new_id})");
					} else {
						*value = format!("#{new_id}");
					}
				}
			}
		}
	}

	for child in &mut node.children {
		rewrite_id_references(child, map);
	}
}

fn extract_id_reference(value: &str) -> Option<&str> {
	if value.starts_with("url(#") && value.ends_with(')') {
		return Some(&value[5..value.len() - 1]);
	}

	value.strip_prefix('#')
}

fn next_short_id(counter: &mut usize) -> String {
	let mut index = *counter;
	*counter += 1;

	let mut chars = Vec::new();
	loop {
		let rem = (index % 26) as u8;
		chars.push((b'a' + rem) as char);
		if index < 26 {
			break;
		}
		index = (index / 26) - 1;
	}

	chars.iter().rev().collect()
}
