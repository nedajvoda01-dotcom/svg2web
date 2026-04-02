use svg2web_core::{analyze, ElementType, SVGElement};
use svg2web_generator::builders::CssBuilder;
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
fn test_css_builder_creates_variables() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let css = CssBuilder::build(&ctx).expect("CssBuilder must return css");
    assert!(css.contains(":root") || css.contains(".svg-container"));
    assert!(css.contains('{'));
    assert!(css.contains('}'));
}

#[test]
fn test_css_builder_container_styles() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let css = CssBuilder::build(&ctx).expect("CssBuilder must return css");
    assert!(css.contains(".svg-container") || css.contains("#svg-container"));
    assert!(css.contains("max-width") || css.contains("width"));
}

#[test]
fn test_css_builder_responsive_skeleton() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let css = CssBuilder::build(&ctx).expect("CssBuilder must return css");
    assert!(!css.is_empty());
}

#[test]
fn test_css_builder_no_panic_on_empty_context() {
    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let result = CssBuilder::build(&ctx);
    assert!(result.is_ok());
}
