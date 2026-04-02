use crate::model::style::{Color, Gradient, LineCap, LineJoin, Stroke};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StyleParseError {
	#[error("linear gradient has an empty id")]
	EmptyGradientId,
}

pub type Result<T> = std::result::Result<T, StyleParseError>;

pub fn parse_fill(fill: Option<&usvg::Fill>) -> Option<Color> {
	let fill = fill?;
	match &fill.paint {
		usvg::Paint::Color(c) => Some(convert_color(*c, fill.opacity)),
		_ => None,
	}
}

pub fn parse_stroke(stroke: Option<&usvg::Stroke>) -> Option<Stroke> {
	let stroke = stroke?;
	let color = match &stroke.paint {
		usvg::Paint::Color(c) => convert_color(*c, stroke.opacity),
		_ => return None,
	};

	Some(Stroke {
		width: stroke.width.get() as f64,
		color,
		linecap: convert_linecap(stroke.linecap),
		linejoin: convert_linejoin(stroke.linejoin),
	})
}

pub fn parse_gradient(node: &usvg::Node) -> Result<Option<Gradient>> {
	let borrowed = node.borrow();
	let paint = match &*borrowed {
		usvg::NodeKind::Path(path) => path
			.fill
			.as_ref()
			.map(|fill| &fill.paint)
			.or_else(|| path.stroke.as_ref().map(|stroke| &stroke.paint)),
		_ => None,
	};

	let Some(paint) = paint else {
		return Ok(None);
	};

	match paint {
		usvg::Paint::LinearGradient(gradient) => {
			if gradient.id.is_empty() {
				return Err(StyleParseError::EmptyGradientId);
			}

			let stops = gradient
				.stops
				.iter()
				.map(|stop| {
					(
						convert_color(stop.color, stop.opacity),
						stop.offset.get() as f64,
					)
				})
				.collect();

			Ok(Some(Gradient::Linear {
				x1: gradient.x1 as f64,
				y1: gradient.y1 as f64,
				x2: gradient.x2 as f64,
				y2: gradient.y2 as f64,
				stops,
			}))
		}
		// Radial gradients are intentionally skipped in this iteration.
		usvg::Paint::RadialGradient(_) => Ok(None),
		_ => Ok(None),
	}
}

fn convert_color(color: usvg::Color, opacity: usvg::Opacity) -> Color {
	let alpha = (opacity.get() * 255.0).round().clamp(0.0, 255.0) as u8;
	Color {
		r: color.red,
		g: color.green,
		b: color.blue,
		a: alpha,
	}
}

fn convert_linecap(linecap: usvg::LineCap) -> LineCap {
	match linecap {
		usvg::LineCap::Butt => LineCap::Butt,
		usvg::LineCap::Round => LineCap::Round,
		usvg::LineCap::Square => LineCap::Square,
	}
}

fn convert_linejoin(linejoin: usvg::LineJoin) -> LineJoin {
	match linejoin {
		usvg::LineJoin::Miter => LineJoin::Miter,
		usvg::LineJoin::Round => LineJoin::Round,
		usvg::LineJoin::Bevel => LineJoin::Bevel,
	}
}
