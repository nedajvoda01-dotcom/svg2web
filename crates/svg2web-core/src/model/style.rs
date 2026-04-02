use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gradient {
	Linear {
		x1: f64,
		y1: f64,
		x2: f64,
		y2: f64,
		stops: Vec<(Color, f64)>,
	},
	Radial {
		cx: f64,
		cy: f64,
		r: f64,
		stops: Vec<(Color, f64)>,
	},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
	pub fill: Option<Color>,
	pub stroke: Option<Stroke>,
	pub opacity: f64,
	pub gradient: Option<Gradient>,
	pub font: Option<Font>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
	pub width: f64,
	pub color: Color,
	pub linecap: LineCap,
	pub linejoin: LineJoin,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LineCap {
	Butt,
	Round,
	Square,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LineJoin {
	Miter,
	Round,
	Bevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
	pub family: String,
	pub size: f64,
	pub weight: FontWeight,
	pub style: FontStyle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FontWeight {
	Thin,
	Light,
	Normal,
	Medium,
	Semibold,
	Bold,
	Black,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FontStyle {
	Normal,
	Italic,
	Oblique,
}
