use svg2web_core::analyzer::components::detect_components;
use svg2web_core::parse;

#[test]
fn test_detects_repeated_component() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 10">
        <g id="icon"><circle cx="5" cy="5" r="5"/></g>
        <g id="icon2"><circle cx="5" cy="5" r="5"/></g>
    </svg>"#;

    let element = parse(svg).expect("parse should succeed");
    let components = detect_components(&element);

    assert!(
        components.iter().any(|component| component.occurrences.len() >= 2),
        "should detect at least one repeated component with two or more occurrences"
    );
}
