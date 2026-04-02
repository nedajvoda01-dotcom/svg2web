use svg2web_core::{analyze, ElementType, SVGElement};
use svg2web_generator::builders::HtmlBuilder;
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
fn test_html_builder_creates_doctype() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "react");

    let html = HtmlBuilder::build(&ctx).expect("HtmlBuilder must return html");
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("<html"));
    assert!(html.contains("</html>"));
}

#[test]
fn test_html_builder_contains_viewport_meta() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "react");

    let html = HtmlBuilder::build(&ctx).expect("HtmlBuilder must return html");
    assert!(html.contains("viewport"));
    assert!(html.contains("width=device-width"));
}

#[test]
fn test_html_builder_embeds_svg_element() {
    let svg = SVGElement {
        tag: "svg".to_string(),
        id: Some("root".to_string()),
        element_type: ElementType::Group,
        attributes: [("viewBox".to_string(), "0 0 100 100".to_string())].into(),
        children: vec![],
        text_content: None,
    };

    let analysis = analyze(&svg);
    let ctx = RenderContext::new(svg, analysis, vec![], "vanilla");
    let html = HtmlBuilder::build(&ctx).expect("HtmlBuilder must return html");

    assert!(html.contains(r#"<div id="svg-container">"#) || html.contains("<svg"));
    assert!(html.contains("root") || html.contains("viewBox"));
}

#[test]
fn test_html_builder_error_handling() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "react");

    let result = HtmlBuilder::build(&ctx);
    assert!(result.is_ok());
}
