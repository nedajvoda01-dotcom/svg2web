use crate::model::analysis::{Component, Occurrence};
use crate::model::SVGElement;
use crate::util::signature::subtree_signature;
use std::collections::HashMap;

pub fn detect_components(element: &SVGElement) -> Vec<Component> {
	let mut buckets: HashMap<String, Vec<(SVGElement, Occurrence)>> = HashMap::new();
	let mut path = Vec::new();
	collect_subtrees(element, &mut path, &mut buckets);

	let mut keys: Vec<String> = buckets.keys().cloned().collect();
	keys.sort();

	let mut components = Vec::new();
	for (idx, key) in keys.into_iter().enumerate() {
		let Some(group) = buckets.remove(&key) else {
			continue;
		};

		// Keep only repeating structures. Exact signature match implies 100% similarity.
		if group.len() < 2 {
			continue;
		}

		let template = group[0].0.clone();
		let occurrences = group.into_iter().map(|(_, occ)| occ).collect();
		components.push(Component {
			id: format!("component-{}", idx + 1),
			template,
			occurrences,
		});
	}

	components
}

fn collect_subtrees(
	element: &SVGElement,
	path: &mut Vec<String>,
	buckets: &mut HashMap<String, Vec<(SVGElement, Occurrence)>>,
) {
	let segment = element
		.id
		.clone()
		.unwrap_or_else(|| format!("{}:{}", element.tag, path.len()));
	path.push(segment);

	let signature = subtree_signature(element);
	let occurrence = Occurrence {
		node_id: element.id.clone(),
		path: path.clone(),
	};

	buckets
		.entry(signature)
		.or_default()
		.push((element.clone(), occurrence));

	for child in &element.children {
		collect_subtrees(child, path, buckets);
	}

	let _ = path.pop();
}
