use svg2web_core::{analyze, ElementType, SVGElement};
use svg2web_generator::registry::{FormatRegistry, RenderContext};

#[test]
fn test_registry_empty() {
    let registry = FormatRegistry::new();
    assert!(registry.list().is_empty());
}

#[test]
fn test_registry_register_format() {
    let registry = FormatRegistry::new();
    assert_eq!(registry.list().len(), 0);
}

#[test]
fn test_render_context_new_sets_target() {
    let root = SVGElement {
        tag: "svg".to_string(),
        id: None,
        element_type: ElementType::Group,
        attributes: [(
            "xmlns".to_string(),
            "http://www.w3.org/2000/svg".to_string(),
        )]
        .into(),
        children: vec![],
        text_content: None,
    };

    let analysis = analyze(&root);
    let context = RenderContext::new(root, analysis, vec![], "react");

    assert_eq!(context.target, "react");
}

#[test]
fn test_render_context_holds_root_and_assets() {
    let root = SVGElement {
        tag: "svg".to_string(),
        id: Some("root".to_string()),
        element_type: ElementType::Group,
        attributes: [(
            "xmlns".to_string(),
            "http://www.w3.org/2000/svg".to_string(),
        )]
        .into(),
        children: vec![],
        text_content: None,
    };

    let analysis = analyze(&root);
    let context = RenderContext::new(root, analysis, vec![], "vanilla");

    assert_eq!(context.root.id.as_deref(), Some("root"));
    assert!(context.assets.is_empty());
}
