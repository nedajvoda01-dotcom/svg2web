use crate::model::SVGElement;
use crate::util::signature::subtree_signature;
use std::collections::HashMap;

pub fn deduplicate_elements(root: &mut SVGElement) -> usize {
	deduplicate_recursive(root)
}

fn deduplicate_recursive(node: &mut SVGElement) -> usize {
	let mut removed = 0usize;
	for child in &mut node.children {
		removed += deduplicate_recursive(child);
	}

	// Use the signature string directly as the HashMap key.
	// This avoids any hash collision risk — no two structurally different
	// subtrees can share a key, so no silent data loss can occur.
	let mut seen: HashMap<String, usize> = HashMap::new();
	let mut unique_children: Vec<SVGElement> = Vec::with_capacity(node.children.len());

	for child in node.children.drain(..) {
		let sig = subtree_signature(&child);

		if seen.contains_key(&sig) {
			removed += 1;
		} else {
			seen.insert(sig, unique_children.len());
			unique_children.push(child);
		}
	}

	node.children = unique_children;
	removed
}
