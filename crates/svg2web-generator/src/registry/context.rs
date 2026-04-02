use svg2web_core::{AnalysisResult, Asset, SVGElement};

#[derive(Debug, Clone)]
pub struct RenderContext {
	pub root: SVGElement,
	pub analysis: AnalysisResult,
	pub assets: Vec<Asset>,
	pub target: String,
}

impl RenderContext {
	pub fn new(root: SVGElement, analysis: AnalysisResult, assets: Vec<Asset>, target: &str) -> Self {
		Self {
			root,
			analysis,
			assets,
			target: target.to_string(),
		}
	}
}
