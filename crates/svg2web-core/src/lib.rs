pub mod analyzer;
pub mod error;
pub mod extractor;
pub mod model;
pub mod optimizer;
pub mod parser;
pub mod serializer;
pub mod validator;

pub use analyzer::AnalysisResult;
pub use error::{Error, Result};
pub use extractor::{ExtractOptions, ExtractedAssets};
pub use model::*;
pub use optimizer::OptimizationConfig;
pub use parser::{ParseOptions, Parser};
pub use serializer::{Format, SerializeOptions};
pub use validator::ValidationError;
pub use validator::performance::{check_timing, measure, TimingBudget};
pub use validator::strict::{check_strict, StrictConfig, StrictViolation, PSNR_THRESHOLD};
pub use validator::visual::{calculate_psnr, render_svg_to_rgba, visual_diff};

pub fn parse(svg: &str) -> Result<SVGElement> {
	parser::parse(svg).map_err(Error::from)
}

pub fn validate_svg(svg: &str) -> Result<()> {
	validator::validate_svg(svg).map_err(Error::from)
}

pub fn analyze(element: &SVGElement) -> AnalysisResult {
	analyzer::analyze(element)
}

pub fn optimize(element: &mut SVGElement, config: &OptimizationConfig) -> Result<()> {
	optimizer::optimize(element, config).map_err(Error::from)
}

pub fn extract_assets(element: &SVGElement, options: &ExtractOptions) -> Result<ExtractedAssets> {
	extractor::extract_assets(element, options).map_err(Error::from)
}

pub fn serialize(element: &SVGElement, format: Format) -> Result<Vec<u8>> {
	serializer::serialize(element, format).map_err(Error::from)
}

pub fn deserialize(data: &[u8], format: Format) -> Result<SVGElement> {
	serializer::deserialize(data, format).map_err(Error::from)
}
