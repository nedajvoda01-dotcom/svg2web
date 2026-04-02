use crate::formats::{ReactRenderer, VanillaRenderer, VueRenderer};
use crate::registry::{FormatRenderer, RenderContext, RenderedOutput};
use crate::{Error, Result};

pub struct Generator {
    renderer: Box<dyn FormatRenderer>,
}

pub struct GeneratorBuilder {
    format: Option<String>,
}

impl GeneratorBuilder {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(mut self, name: &str) -> Self {
        self.format = Some(name.to_string());
        self
    }

    pub fn build(self) -> Result<Generator> {
        let format = self
            .format
            .ok_or_else(|| Error::render_error("Format not specified"))?;

        let renderer: Box<dyn FormatRenderer> = match format.as_str() {
            "vanilla" => Box::new(VanillaRenderer),
            "react" => Box::new(ReactRenderer),
            "vue" => Box::new(VueRenderer),
            _ => {
                return Err(Error::render_error(&format!(
                    "Unknown format: {}. Available: vanilla, react, vue",
                    format
                )))
            }
        };

        Ok(Generator { renderer })
    }
}

impl Default for GeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator {
    pub fn is_ready(&self) -> bool {
        true
    }

    pub fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput> {
        self.renderer.render(ctx)
    }
}
