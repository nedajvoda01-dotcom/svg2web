use crate::builders::{CssBuilder, HtmlBuilder, JsBuilder};
use crate::registry::{FormatRenderer, OutputFile, RenderContext, RenderedOutput};
use crate::{Error, Result};

pub struct VanillaRenderer;

impl FormatRenderer for VanillaRenderer {
	fn name(&self) -> &str {
		"vanilla"
	}

	fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput> {
		let html = HtmlBuilder::build(ctx).map_err(|e| Error::RenderError(e.to_string()))?;
		let css = CssBuilder::build(ctx).map_err(|e| Error::RenderError(e.to_string()))?;
		let js = JsBuilder::build(ctx).map_err(|e| Error::RenderError(e.to_string()))?;

		Ok(RenderedOutput {
			files: vec![
				OutputFile {
					name: "index.html".to_string(),
					content: html,
				},
				OutputFile {
					name: "styles.css".to_string(),
					content: css,
				},
				OutputFile {
					name: "script.js".to_string(),
					content: js,
				},
			],
		})
	}
}
