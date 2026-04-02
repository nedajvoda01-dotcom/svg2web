use super::element::SVGElement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
	pub id: String,
	pub template: SVGElement,
	pub occurrences: Vec<Occurrence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occurrence {
	pub node_id: Option<String>,
	pub path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
	pub score: u8,
	pub node_count: usize,
	pub path_count: usize,
	pub path_complexity: f64,
	pub gradient_count: usize,
	pub text_count: usize,
	pub image_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyInfo {
	pub depth: usize,
	pub max_width: usize,
	pub critical_paths: Vec<Vec<String>>,
}
