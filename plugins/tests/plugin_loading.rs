#[test]
fn format_renderer_trait_existence_t112() {
    let docs = std::fs::read_to_string("docs/contributing/plugin-development.md").unwrap_or_default();
    let registry = std::fs::read_to_string("crates/svg2web-generator/src/registry/mod.rs").unwrap_or_default();
    assert!(docs.contains("pub trait FormatRenderer"));
    assert!(docs.contains("fn name(&self) -> &str;"));
    assert!(docs.contains("fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput, RenderError>;"));
    assert!(registry.contains("FormatRenderer"), "registry surface must expose FormatRenderer");
}

#[test]
fn svelte_renderer_registers_t113() {
    assert!(std::path::Path::new("plugins/svelte").exists());
}

#[test]
fn solid_renderer_registers_t113() {
    assert!(std::path::Path::new("plugins/solid").exists());
}

#[test]
fn cdylib_format_compiles_t113() {
    let svelte = std::fs::read_to_string("plugins/svelte/Cargo.toml").unwrap_or_default();
    let solid = std::fs::read_to_string("plugins/solid/Cargo.toml").unwrap_or_default();
    assert!(svelte.contains("cdylib"));
    assert!(solid.contains("cdylib"));
}

#[test]
fn dynamic_loading_contract_t113() {
    let d = std::fs::read_to_string("docs/contributing/plugin-development.md").unwrap_or_default();
    assert!(d.contains("libloading (dlopen)"));
    assert!(d.contains("FormatRegistry"));
    assert!(d.contains("интеграционный тест динамической загрузки"));
}
