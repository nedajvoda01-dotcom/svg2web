use svg2web_core::{analyze, ElementType, SVGElement};
use svg2web_generator::builders::JsBuilder;
use svg2web_generator::registry::RenderContext;

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
fn test_js_builder_creates_iife() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let js = JsBuilder::build(&ctx).expect("JsBuilder must return script");
    assert!(js.contains("function") || js.contains("const") || js.contains("let"));
    assert!(js.contains("DOMContentLoaded") || js.contains("document.addEventListener"));
}

#[test]
fn test_js_builder_event_listeners_skeleton() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let js = JsBuilder::build(&ctx).expect("JsBuilder must return script");
    assert!(
        js.contains("click") || js.contains("hover") || js.contains("mouse") || js.contains("addEventListener")
    );
}

#[test]
fn test_js_builder_no_global_pollution() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let js = JsBuilder::build(&ctx).expect("JsBuilder must return script");
    assert!(js.contains("\"use strict\"") || js.contains("export") || js.contains("module"));
}

#[test]
fn test_js_builder_handles_empty_context() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let result = JsBuilder::build(&ctx);
    assert!(result.is_ok());
    assert!(!result.expect("script should exist").is_empty());
}
