use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
	pub x: f64,
	pub y: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size {
	pub width: f64,
	pub height: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bounds {
	pub x: f64,
	pub y: f64,
	pub width: f64,
	pub height: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform {
	pub a: f64,
	pub b: f64,
	pub c: f64,
	pub d: f64,
	pub e: f64,
	pub f: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
	pub x: f64,
	pub y: f64,
	pub width: f64,
	pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathSegment {
	MoveTo(Point),
	LineTo(Point),
	CurveTo {
		control1: Point,
		control2: Point,
		to: Point,
	},
	ClosePath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathData {
	pub segments: Vec<PathSegment>,
}
