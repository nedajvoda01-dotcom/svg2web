//! Headless SVG rendering and PSNR-based visual diff.
//!
//! Uses `resvg` (wraps `usvg` + `tiny-skia`) for pure-Rust pixel rendering —
//! no headless browser required. Compatible with WASM (for web preview).
//!
//! # Example
//! ```rust,ignore
//! use svg2web_core::validator::visual::visual_diff;
//! let psnr = visual_diff(original_svg, optimized_svg, 512, 512);
//! assert!(psnr.map_or(false, |p| p > 50.0));
//! ```

use resvg::tiny_skia;
use resvg::usvg;
use usvg::TreeParsing;

/// RGBA pixel buffer produced by [`render_svg_to_rgba`].
#[derive(Debug, Clone)]
pub struct RenderResult {
	/// Raw RGBA bytes (4 bytes per pixel, row-major).
	pub data: Vec<u8>,
	/// Rendered width in pixels.
	pub width: u32,
	/// Rendered height in pixels.
	pub height: u32,
}

/// Render an SVG string to an RGBA pixel buffer at the given dimensions.
///
/// Returns `None` if the SVG is invalid, dimensions are zero, or memory
/// allocation fails.
pub fn render_svg_to_rgba(svg: &str, width: u32, height: u32) -> Option<RenderResult> {
	let opt = usvg::Options::default();
	let usvg_tree = usvg::Tree::from_str(svg, &opt).ok()?;
	let render_tree = resvg::Tree::from_usvg(&usvg_tree);
	let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
	render_tree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());
	Some(RenderResult {
		data: pixmap.data().to_vec(),
		width,
		height,
	})
}

/// Calculate PSNR (Peak Signal-to-Noise Ratio) between two equal-length RGBA buffers.
///
/// - Returns `f32::INFINITY` for identical buffers (MSE = 0).
/// - Returns `0.0` for mismatched lengths or empty buffers.
/// - Formula: `PSNR = 10 * log10(255² / MSE)`
pub fn calculate_psnr(original: &[u8], processed: &[u8]) -> f32 {
	if original.len() != processed.len() || original.is_empty() {
		return 0.0;
	}
	let mse = mean_square_error(original, processed);
	if mse == 0.0 {
		return f32::INFINITY;
	}
	10.0 * (255.0_f32.powi(2) / mse).log10()
}

/// Render both SVGs at the given resolution and compute PSNR between them.
///
/// Returns `None` if either SVG fails to render (invalid input or zero dimensions).
pub fn visual_diff(original_svg: &str, result_svg: &str, width: u32, height: u32) -> Option<f32> {
	let orig = render_svg_to_rgba(original_svg, width, height)?;
	let proc = render_svg_to_rgba(result_svg, width, height)?;
	Some(calculate_psnr(&orig.data, &proc.data))
}

fn mean_square_error(a: &[u8], b: &[u8]) -> f32 {
	let sum: f64 = a
		.iter()
		.zip(b.iter())
		.map(|(&x, &y)| {
			let diff = x as f64 - y as f64;
			diff * diff
		})
		.sum();
	(sum / a.len() as f64) as f32
}
