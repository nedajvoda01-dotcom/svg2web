use crate::model::style::{Font, FontStyle, FontWeight};

pub fn parse_font(span: &usvg::TextSpan) -> Option<Font> {
	let family = span.font.families.first()?.clone();

	Some(Font {
		family,
		size: span.font_size.get() as f64,
		weight: map_font_weight(span.font.weight),
		style: map_font_style(span.font.style),
	})
}

pub fn extract_text_content(node: &usvg::Node) -> Option<String> {
	let borrowed = node.borrow();
	let usvg::NodeKind::Text(text) = &*borrowed else {
		return None;
	};

	let content = text
		.chunks
		.iter()
		.map(|chunk| chunk.text.as_str())
		.collect::<Vec<_>>()
		.join(" ")
		.trim()
		.to_string();

	if content.is_empty() {
		None
	} else {
		Some(content)
	}
}

fn map_font_weight(weight: u16) -> FontWeight {
	match weight {
		0..=200 => FontWeight::Thin,
		201..=300 => FontWeight::Light,
		301..=450 => FontWeight::Normal,
		451..=550 => FontWeight::Medium,
		551..=650 => FontWeight::Semibold,
		651..=850 => FontWeight::Bold,
		_ => FontWeight::Black,
	}
}

fn map_font_style(style: usvg::FontStyle) -> FontStyle {
	match style {
		usvg::FontStyle::Normal => FontStyle::Normal,
		usvg::FontStyle::Italic => FontStyle::Italic,
		usvg::FontStyle::Oblique => FontStyle::Oblique,
	}
}
