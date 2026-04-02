use svg2web_core::{analyze, ElementType, SVGElement};
use svg2web_generator::formats::builtin::ReactRenderer;
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
fn test_react_renderer_name() {
    let renderer = ReactRenderer;
    assert_eq!(renderer.name(), "react");
}

#[test]
fn test_react_outputs_tsx() {
    let root = SVGElement {
        tag: "svg".to_string(),
        id: Some("icon".to_string()),
        element_type: ElementType::Group,
        attributes: [("viewBox".to_string(), "0 0 24 24".to_string())].into(),
        children: vec![],
        text_content: None,
    };
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "react");

    let renderer = ReactRenderer;
    let output = renderer.render(&ctx).expect("renderer should produce output");

    assert_eq!(output.files.len(), 1);
    let file = &output.files[0];
    assert!(file.name.ends_with(".tsx"));
    assert!(file.content.contains("import React"));
    assert!(file.content.contains("export function"));
    assert!(file.content.contains("interface"));
}

#[test]
fn test_react_has_props_interface() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "react");

    let renderer = ReactRenderer;
    let output = renderer.render(&ctx).expect("renderer should produce output");
    let content = &output.files[0].content;

    assert!(content.contains("interface") || content.contains("type"));
    assert!(content.contains("size") || content.contains("className") || content.contains("style"));
}

#[test]
fn test_react_no_panic() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "react");

    let renderer = ReactRenderer;
    let result = renderer.render(&ctx);
    assert!(result.is_ok());
}
