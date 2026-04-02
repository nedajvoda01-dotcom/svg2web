pub mod context;

pub use context::RenderContext;

pub struct RenderedOutput {
	pub files: Vec<OutputFile>,
}

pub struct OutputFile {
	pub name: String,
	pub content: String,
}

pub trait FormatRenderer {
	fn name(&self) -> &str;
	fn render(&self, ctx: &RenderContext) -> crate::Result<RenderedOutput>;
}

pub struct FormatRegistry {}

impl FormatRegistry {
	pub fn new() -> Self {
		Self {}
	}

	pub fn list(&self) -> Vec<&str> {
		vec![]
	}
}

impl Default for FormatRegistry {
	fn default() -> Self {
		Self::new()
	}
}
