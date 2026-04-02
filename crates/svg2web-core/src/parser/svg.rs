use super::Result;
use crate::model::{ElementType, SVGElement};
use std::collections::HashMap;
use usvg::TreeParsing;

#[derive(Debug, Default)]
pub struct UsvgParser;

impl UsvgParser {
	pub fn load(svg: &str) -> Result<usvg::Tree> {
		let options = usvg::Options::default();
		let tree = usvg::Tree::from_str(svg, &options)?;
		Ok(tree)
	}

	pub fn to_model(tree: &usvg::Tree) -> SVGElement {
		let mut attributes = HashMap::new();
		attributes.insert("width".to_string(), tree.size.width().to_string());
		attributes.insert("height".to_string(), tree.size.height().to_string());

		let children = tree.root.children().map(|node| Self::convert_node(&node)).collect();

		SVGElement {
			id: None,
			tag: "svg".to_string(),
			element_type: ElementType::Group,
			attributes,
			children,
			text_content: None,
		}
	}

	fn convert_node(node: &usvg::Node) -> SVGElement {
		let (id, tag, element_type, text_content) = {
			let borrowed = node.borrow();
			match &*borrowed {
				usvg::NodeKind::Group(group) => (
					if group.id.is_empty() {
						None
					} else {
						Some(group.id.clone())
					},
					"g".to_string(),
					ElementType::Group,
					None,
				),
				usvg::NodeKind::Path(path) => (
					if path.id.is_empty() {
						None
					} else {
						Some(path.id.clone())
					},
					"path".to_string(),
					ElementType::Path,
					None,
				),
				usvg::NodeKind::Image(image) => (
					if image.id.is_empty() {
						None
					} else {
						Some(image.id.clone())
					},
					"image".to_string(),
					ElementType::Image,
					None,
				),
				usvg::NodeKind::Text(text) => {
					let joined = text
						.chunks
						.iter()
						.map(|chunk| chunk.text.as_str())
						.collect::<Vec<_>>()
						.join(" ")
						.trim()
						.to_string();

					(
						if text.id.is_empty() {
							None
						} else {
							Some(text.id.clone())
						},
						"text".to_string(),
						ElementType::Text,
						if joined.is_empty() { None } else { Some(joined) },
					)
				}
			}
		};

		let mut attributes = HashMap::new();
		if let Some(ref id_value) = id {
			attributes.insert("id".to_string(), id_value.clone());
		}

		let children = node
			.children()
			.map(|child| Self::convert_node(&child))
			.collect();

		SVGElement {
			id,
			tag,
			element_type,
			attributes,
			children,
			text_content,
		}
	}
}
