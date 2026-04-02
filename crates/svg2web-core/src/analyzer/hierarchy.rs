use crate::model::analysis::HierarchyInfo;
use crate::model::SVGElement;

pub fn analyze_hierarchy(element: &SVGElement) -> HierarchyInfo {
	let mut max_depth = 0usize;
	let mut max_width = 0usize;
	let mut critical_paths: Vec<Vec<String>> = Vec::new();
	let mut current_path = Vec::new();

	dfs_hierarchy(
		element,
		1,
		&mut max_depth,
		&mut max_width,
		&mut current_path,
		&mut critical_paths,
	);

	HierarchyInfo {
		depth: max_depth,
		max_width,
		critical_paths,
	}
}

fn dfs_hierarchy(
	node: &SVGElement,
	depth: usize,
	max_depth: &mut usize,
	max_width: &mut usize,
	current_path: &mut Vec<String>,
	critical_paths: &mut Vec<Vec<String>>,
) {
	let segment = node
		.id
		.clone()
		.unwrap_or_else(|| format!("{}:{}", node.tag, depth));
	current_path.push(segment);

	if node.children.len() > *max_width {
		*max_width = node.children.len();
	}

	if depth > *max_depth {
		*max_depth = depth;
		critical_paths.clear();
		critical_paths.push(current_path.clone());
	} else if depth == *max_depth && node.children.is_empty() {
		critical_paths.push(current_path.clone());
	}

	for child in &node.children {
		dfs_hierarchy(
			child,
			depth + 1,
			max_depth,
			max_width,
			current_path,
			critical_paths,
		);
	}

	let _ = current_path.pop();
}
