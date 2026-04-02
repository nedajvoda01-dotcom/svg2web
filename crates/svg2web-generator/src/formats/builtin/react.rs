use crate::registry::{FormatRenderer, OutputFile, RenderContext, RenderedOutput};
use crate::Result;

pub struct ReactRenderer;

impl FormatRenderer for ReactRenderer {
	fn name(&self) -> &str {
		"react"
	}

	fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput> {
		let svg_id = ctx.root.id.as_deref().unwrap_or("SvgComponent");
		let component_name = to_pascal_case(svg_id);
		let tsx = format!(
			"import React from 'react';\n\ninterface {component_name}Props {{\n  size?: number;\n  className?: string;\n  style?: React.CSSProperties;\n}}\n\nexport function {component_name}(props: {component_name}Props) {{\n  const {{ size = 24, className = '', style }} = props;\n\n  return (\n    <svg\n      viewBox=\"0 0 24 24\"\n      width={{size}}\n      height={{size}}\n      className={{className}}\n      style={{style}}\n      fill=\"none\"\n      xmlns=\"http://www.w3.org/2000/svg\"\n    >\n      {{/* SVG content placeholder - will be generated from children */}}\n      <rect x=\"0\" y=\"0\" width=\"24\" height=\"24\" fill=\"currentColor\" />\n    </svg>\n  );\n}}\n"
		);

		Ok(RenderedOutput {
			files: vec![OutputFile {
				name: format!("{}.tsx", component_name.to_lowercase()),
				content: tsx,
			}],
		})
	}
}

fn to_pascal_case(s: &str) -> String {
	s.split('-')
		.filter(|segment| !segment.is_empty())
		.map(|word| {
			let mut chars = word.chars();
			match chars.next() {
				Some(first) => {
					first.to_uppercase().collect::<String>() + chars.as_str()
				}
				None => String::new(),
			}
		})
		.collect::<String>()
}
