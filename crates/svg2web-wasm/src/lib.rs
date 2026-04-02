mod memory;
mod utils;

use wasm_bindgen::prelude::*;

pub use memory::StringPool;
pub use utils::init_panic_hook;

use svg2web_core::{analyze, optimize, parse, OptimizationConfig, SVGElement};

/// Parse SVG string and return JSON representation of the DOM.
#[wasm_bindgen]
pub fn parse_svg(svg_str: &str) -> Result<String, JsValue> {
	let element = parse(svg_str).map_err(|e| JsValue::from_str(&format!("Parse error: {e}")))?;
	serde_json::to_string(&element)
		.map_err(|e| JsValue::from_str(&format!("Serialization error: {e}")))
}

/// Analyze SVG and return metrics as JSON.
#[wasm_bindgen]
pub fn analyze_svg(svg_str: &str) -> Result<String, JsValue> {
	let element = parse(svg_str).map_err(|e| JsValue::from_str(&format!("Parse error: {e}")))?;
	let analysis = analyze(&element);
	serde_json::to_string(&analysis)
		.map_err(|e| JsValue::from_str(&format!("Serialization error: {e}")))
}

/// Optimize SVG and return optimized SVG string.
#[wasm_bindgen]
pub fn optimize_svg(svg_str: &str) -> Result<String, JsValue> {
	let mut element = parse(svg_str).map_err(|e| JsValue::from_str(&format!("Parse error: {e}")))?;

	let config = OptimizationConfig {
		simplify_paths: true,
		deduplicate: true,
		minify_ids: true,
		..Default::default()
	};

	optimize(&mut element, &config).map_err(|e| JsValue::from_str(&format!("Optimize error: {e}")))?;

	Ok(serialize_to_svg_string(&element))
}

fn serialize_to_svg_string(root: &SVGElement) -> String {
	serialize_node(root)
}

fn serialize_node(node: &SVGElement) -> String {
	let mut out = String::new();
	out.push('<');
	out.push_str(&node.tag);

	if let Some(id) = &node.id {
		out.push_str(" id=\"");
		out.push_str(&escape_xml(id));
		out.push('"');
	}

	let mut attrs = node.attributes.iter().collect::<Vec<_>>();
	attrs.sort_by(|a, b| a.0.cmp(b.0));
	for (k, v) in attrs {
		out.push(' ');
		out.push_str(k);
		out.push_str("=\"");
		out.push_str(&escape_xml(v));
		out.push('"');
	}

	let has_text = node.text_content.as_ref().is_some_and(|t| !t.is_empty());
	let has_children = !node.children.is_empty();
	let force_explicit_close = node.tag == "svg";

	if !has_text && !has_children && !force_explicit_close {
		out.push_str("/>");
		return out;
	}

	out.push('>');
	if let Some(text) = &node.text_content {
		out.push_str(&escape_xml(text));
	}

	for child in &node.children {
		out.push_str(&serialize_node(child));
	}

	out.push_str("</");
	out.push_str(&node.tag);
	out.push('>');
	out
}

fn escape_xml(s: &str) -> String {
	s.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
		.replace('\'', "&apos;")
}
