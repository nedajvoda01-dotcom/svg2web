use crate::registry::RenderContext;

use svg2web_core::Result;

pub struct CssBuilder;

impl CssBuilder {
	pub fn build(_ctx: &RenderContext) -> Result<String> {
		let mut css = String::new();

		css.push_str(":root {\n");
		css.push_str("  --svg-bg: #ffffff;\n");
		css.push_str("  --svg-fg: #000000;\n");
		css.push_str("}\n\n");

		css.push_str(".svg-container {\n");
		css.push_str("  width: 100%;\n");
		css.push_str("  max-width: 100%;\n");
		css.push_str("  height: auto;\n");
		css.push_str("  display: flex;\n");
		css.push_str("  justify-content: center;\n");
		css.push_str("  align-items: center;\n");
		css.push_str("}\n\n");

		css.push_str(".svg-container svg {\n");
		css.push_str("  max-width: 100%;\n");
		css.push_str("  height: auto;\n");
		css.push_str("}\n\n");

		css.push_str("@media (max-width: 768px) {\n");
		css.push_str("  .svg-container {\n");
		css.push_str("    /* Responsive adjustments placeholder */\n");
		css.push_str("  }\n");
		css.push_str("}\n");

		Ok(css)
	}
}
