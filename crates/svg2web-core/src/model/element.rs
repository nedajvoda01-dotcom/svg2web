use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVGElement {
	pub id: Option<String>,
	pub tag: String,
	pub element_type: ElementType,
	pub attributes: HashMap<String, String>,
	pub children: Vec<SVGElement>,
	pub text_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementType {
	Group,
	Path,
	Rect,
	Circle,
	Ellipse,
	Line,
	Polyline,
	Polygon,
	Text,
	Image,
	LinearGradient,
	RadialGradient,
	Defs,
	ClipPath,
	Mask,
	Use,
	Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
	pub element: SVGElement,
}
