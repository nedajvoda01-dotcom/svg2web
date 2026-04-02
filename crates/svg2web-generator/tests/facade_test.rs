use svg2web_core::{analyze, ElementType, SVGElement};
use svg2web_generator::registry::RenderContext;
use svg2web_generator::GeneratorBuilder;

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
fn test_builder_creates_generator() {
    let result = GeneratorBuilder::new().format("vanilla").build();

    assert!(result.is_ok());
    let generator = result.expect("builder should return generator");
    assert!(generator.is_ready());
}

#[test]
fn test_generator_renders_output() {
    let generator = GeneratorBuilder::new()
        .format("vanilla")
        .build()
        .expect("builder should return generator");

    let root = empty_svg();
    let analysis = analyze(&root);
    let ctx = RenderContext::new(root, analysis, vec![], "vanilla");

    let output = generator.render(&ctx).expect("render should succeed");
    assert!(!output.files.is_empty(), "Should produce output files");
    assert!(
        output.files[0].name.ends_with(".html")
            || output.files[0].name.ends_with(".tsx")
            || output.files[0].name.ends_with(".vue")
    );
}

#[test]
fn test_invalid_format_returns_error() {
    let result = GeneratorBuilder::new().format("invalid_format_xyz").build();

    assert!(result.is_err(), "Should fail on unknown format");
}

#[test]
fn test_all_formats_available() {
    for format in ["vanilla", "react", "vue"] {
        let result = GeneratorBuilder::new().format(format).build();
        assert!(result.is_ok(), "Format {} should be available", format);
    }
}
