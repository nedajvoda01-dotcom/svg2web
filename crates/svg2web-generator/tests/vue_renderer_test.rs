use svg2web_core::{analyze, ElementType, SVGElement};
use svg2web_generator::formats::builtin::VueRenderer;
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
fn test_vue_renderer_name() {
    let renderer = VueRenderer;
    assert_eq!(renderer.name(), "vue");
}

#[test]
fn test_vue_outputs_sfc() {
    let root = SVGElement {
        tag: "svg".to_string(),
        id: Some("icon".to_string()),
        element_type: ElementType::Group,
        attributes: Default::default(),
        children: vec![],
        text_content: None,
    };
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vue");

    let renderer = VueRenderer;
    let output = renderer.render(&ctx).expect("renderer should produce output");
    assert_eq!(output.files.len(), 1);
    let file = &output.files[0];
    assert!(file.name.ends_with(".vue"));
    assert!(file.content.contains("<template>"));
    assert!(file.content.contains("<script setup"));
    assert!(file.content.contains("defineProps"));
}

#[test]
fn test_vue_has_typescript_props() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vue");

    let renderer = VueRenderer;
    let output = renderer.render(&ctx).expect("renderer should produce output");
    let content = &output.files[0].content;

    assert!(content.contains("interface Props") || content.contains("type Props"));
    assert!(content.contains("size") || content.contains("class"));
}

#[test]
fn test_vue_no_panic() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vue");

    let renderer = VueRenderer;
    let result = renderer.render(&ctx);
    assert!(result.is_ok());
}
