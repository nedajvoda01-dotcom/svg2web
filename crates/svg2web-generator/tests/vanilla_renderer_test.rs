use svg2web_core::{analyze, ElementType, SVGElement};
use svg2web_generator::formats::VanillaRenderer;
use svg2web_generator::registry::{FormatRenderer, RenderContext};

fn empty_svg() -> SVGElement {
    SVGElement {
        tag: "svg".to_string(),
        id: None,
        element_type: ElementType::Group,
        attributes: Default::default(),
        children: vec![],
        text_content: None,
    }
}

#[test]
fn test_vanilla_renderer_name() {
    let renderer = VanillaRenderer;
    assert_eq!(renderer.name(), "vanilla");
}

#[test]
fn test_vanilla_renderer_outputs_three_files() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let renderer = VanillaRenderer;
    let output = renderer.render(&ctx).expect("renderer must return output");

    assert!(!output.files.is_empty());
    let has_html = output
        .files
        .iter()
        .any(|f| f.name.ends_with(".html") || f.content.contains("<!DOCTYPE"));
    let has_css = output
        .files
        .iter()
        .any(|f| f.name.ends_with(".css") || f.content.contains(".svg-container"));
    let has_js = output
        .files
        .iter()
        .any(|f| f.name.ends_with(".js") || f.content.contains("function"));

    assert!(has_html, "Should output HTML");
    assert!(has_css, "Should output CSS");
    assert!(has_js, "Should output JS");
}

#[test]
fn test_vanilla_renderer_no_panic() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let renderer = VanillaRenderer;
    let result = renderer.render(&ctx);
    assert!(result.is_ok());
}

#[test]
fn test_vanilla_uses_builders() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let renderer = VanillaRenderer;
    let output = renderer.render(&ctx).expect("renderer must return output");
    let combined = output
        .files
        .iter()
        .map(|f| f.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(combined.contains("svg-container") || combined.contains("DOMContentLoaded"));
}
