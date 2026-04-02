use crate::registry::{FormatRenderer, OutputFile, RenderContext, RenderedOutput};
use crate::Result;

pub struct VueRenderer;

impl FormatRenderer for VueRenderer {
	fn name(&self) -> &str {
		"vue"
	}

	fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput> {
		let component_name = ctx.root.id.as_deref().unwrap_or("SvgComponent");
		let vue = r#"<template>
  <svg
    :width="size"
    :height="size"
    :class="className"
    viewBox="0 0 24 24"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <!-- SVG content placeholder -->
    <rect x="0" y="0" width="24" height="24" fill="currentColor" />
  </svg>
</template>

<script setup lang="ts">
interface Props {
  size?: number
  className?: string
}

const props = withDefaults(defineProps<Props>(), {
  size: 24,
  className: '',
})
</script>

<style scoped>
svg {
  display: block;
  max-width: 100%;
}
</style>
"#;

		Ok(RenderedOutput {
			files: vec![OutputFile {
				name: format!("{}.vue", to_kebab_case(component_name)),
				content: vue.to_string(),
			}],
		})
	}
}

fn to_kebab_case(s: &str) -> String {
	let mut out = String::new();
	for c in s.chars() {
		if c.is_uppercase() {
			if !out.is_empty() {
				out.push('-');
			}
			out.push(c.to_ascii_lowercase());
		} else {
			out.push(c);
		}
	}
	out
}
