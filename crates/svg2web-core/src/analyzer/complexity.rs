use crate::model::analysis::ComplexityMetrics;
use crate::model::{ElementType, SVGElement};

pub fn compute_complexity(element: &SVGElement) -> ComplexityMetrics {
	let mut node_count = 0usize;
	let mut path_count = 0usize;
	let mut gradient_count = 0usize;
	let mut text_count = 0usize;
	let mut image_count = 0usize;
	let mut path_complexity = 0.0f64;

	walk(
		element,
		&mut node_count,
		&mut path_count,
		&mut gradient_count,
		&mut text_count,
		&mut image_count,
		&mut path_complexity,
	);

	let raw_score = (node_count as f64 * 1.0)
		+ (path_count as f64 * 2.0)
		+ (gradient_count as f64 * 0.5)
		+ (text_count as f64 * 0.8)
		+ (image_count as f64 * 1.2)
		+ path_complexity;

	let score = raw_score.round().clamp(0.0, 100.0) as u8;

	ComplexityMetrics {
		score,
		node_count,
		path_count,
		path_complexity,
		gradient_count,
		text_count,
		image_count,
	}
}

fn walk(
	node: &SVGElement,
	node_count: &mut usize,
	path_count: &mut usize,
	gradient_count: &mut usize,
	text_count: &mut usize,
	image_count: &mut usize,
	path_complexity: &mut f64,
) {
	*node_count += 1;

	match node.element_type {
		ElementType::Path => {
			*path_count += 1;
			if let Some(d) = node.attributes.get("d") {
				let commands = d
					.chars()
					.filter(|c| c.is_ascii_alphabetic())
					.count() as f64;
				*path_complexity += commands * 0.1;
			}
		}
		ElementType::LinearGradient | ElementType::RadialGradient => {
			*gradient_count += 1;
		}
		ElementType::Text => {
			*text_count += 1;
		}
		ElementType::Image => {
			*image_count += 1;
		}
		_ => {}
	}

	for child in &node.children {
		walk(
			child,
			node_count,
			path_count,
			gradient_count,
			text_count,
			image_count,
			path_complexity,
		);
	}
}
