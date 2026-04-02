use crate::model::geometry::{Bounds, Rect, Size, Transform};
use usvg::NodeExt;

pub fn extract_viewbox(tree: &usvg::Tree) -> Option<Rect> {
	Some(convert_non_zero_rect(tree.view_box.rect))
}

pub fn extract_size(tree: &usvg::Tree) -> Size {
	Size {
		width: tree.size.width() as f64,
		height: tree.size.height() as f64,
	}
}

pub fn compute_bounds(element: &usvg::Node) -> Bounds {
	if let Some(bbox) = element.calculate_bbox() {
		Bounds {
			x: bbox.x() as f64,
			y: bbox.y() as f64,
			width: bbox.width() as f64,
			height: bbox.height() as f64,
		}
	} else {
		Bounds {
			x: 0.0,
			y: 0.0,
			width: 0.0,
			height: 0.0,
		}
	}
}

pub fn convert_transform(ts: usvg::Transform) -> Transform {
	Transform {
		a: ts.sx as f64,
		b: ts.ky as f64,
		c: ts.kx as f64,
		d: ts.sy as f64,
		e: ts.tx as f64,
		f: ts.ty as f64,
	}
}

pub fn convert_rect(rect: usvg::Rect) -> Rect {
	Rect {
		x: rect.x() as f64,
		y: rect.y() as f64,
		width: rect.width() as f64,
		height: rect.height() as f64,
	}
}

fn convert_non_zero_rect(rect: usvg::NonZeroRect) -> Rect {
	Rect {
		x: rect.x() as f64,
		y: rect.y() as f64,
		width: rect.width() as f64,
		height: rect.height() as f64,
	}
}
