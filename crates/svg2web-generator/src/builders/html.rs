use crate::registry::RenderContext;

use svg2web_core::Result;

pub struct HtmlBuilder;

impl HtmlBuilder {
	pub fn build(ctx: &RenderContext) -> Result<String> {
		let svg_element = &ctx.root;
		let svg_id = svg_element.id.as_deref().unwrap_or("svg-root");
		let view_box = svg_element
			.attributes
			.get("viewBox")
			.map(String::as_str)
			.unwrap_or("0 0 100 100");

		let mut html = String::new();
		html.push_str("<!DOCTYPE html>\n");
		html.push_str("<html lang=\"en\">");
		html.push_str("<head>");
		html.push_str("<meta charset=\"UTF-8\">");
		html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">");
		html.push_str("<title>SVG2Web Output</title>");
		html.push_str("</head>");
		html.push_str("<body>");
		html.push_str("<div id=\"svg-container\">");
		html.push_str(&format!("<svg id=\"{}\" viewBox=\"{}\"></svg>", svg_id, view_box));
		html.push_str("</div>");
		html.push_str("</body>");
		html.push_str("</html>");

		Ok(html)
	}
}
