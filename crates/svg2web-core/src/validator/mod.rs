pub mod performance;
pub mod strict;
pub mod visual;

use thiserror::Error;
use usvg::TreeParsing;

#[derive(Debug, Error)]
pub enum ValidationError {
	#[error("XML parse error at line {line}: {message}")]
	XmlError { line: usize, message: String },
	#[error("Missing required attribute: {attr}")]
	MissingAttribute { attr: String },
	#[error("Invalid SVG structure: {details}")]
	InvalidStructure { details: String },
}

pub type Result<T> = std::result::Result<T, ValidationError>;

pub fn validate_svg(svg: &str) -> Result<()> {
	if svg.trim().is_empty() {
		return Err(ValidationError::InvalidStructure {
			details: "empty SVG payload".to_string(),
		});
	}

	let options = usvg::Options::default();
	if let Err(err) = usvg::Tree::from_str(svg, &options) {
		let message = err.to_string();
		let line = extract_line(&message).unwrap_or(0);
		return Err(ValidationError::XmlError { line, message });
	}

	let root = extract_root_svg_tag(svg).ok_or_else(|| ValidationError::InvalidStructure {
		details: "missing <svg> root element".to_string(),
	})?;

	if !has_xmlns(root) {
		return Err(ValidationError::MissingAttribute {
			attr: "xmlns".to_string(),
		});
	}

	if !(has_attr(root, "viewBox") || (has_attr(root, "width") && has_attr(root, "height"))) {
		return Err(ValidationError::MissingAttribute {
			attr: "viewBox or width+height".to_string(),
		});
	}

	Ok(())
}

fn extract_root_svg_tag(svg: &str) -> Option<&str> {
	let start = svg.find("<svg")?;
	let rest = &svg[start..];
	let end_rel = rest.find('>')?;
	Some(&rest[..=end_rel])
}

fn has_xmlns(tag: &str) -> bool {
	tag.contains("xmlns=\"http://www.w3.org/2000/svg\"")
		|| tag.contains("xmlns='http://www.w3.org/2000/svg'")
}

fn has_attr(tag: &str, attr: &str) -> bool {
	let needle = format!("{attr}=");
	tag.contains(&needle)
}

fn extract_line(message: &str) -> Option<usize> {
	let at_idx = message.find(" at ")?;
	let tail = &message[(at_idx + 4)..];
	let mut digits = String::new();
	for ch in tail.chars() {
		if ch.is_ascii_digit() {
			digits.push(ch);
		} else {
			break;
		}
	}
	if digits.is_empty() {
		None
	} else {
		digits.parse().ok()
	}
}
