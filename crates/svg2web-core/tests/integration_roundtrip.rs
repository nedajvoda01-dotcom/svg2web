use svg2web_core::{deserialize, parse, serialize, Format};

#[test]
fn test_roundtrip_simple_svg() {
    let fixture = include_str!("fixtures/simple.svg");
    let svg = extract_embedded_svg(fixture).unwrap_or(fixture);

    let parsed = parse(svg).expect("parse should succeed for simple fixture");
    assert_eq!(parsed.tag, "svg");
    assert!(
        !parsed.children.is_empty(),
        "parsed SVG must contain at least one child node"
    );

    let json_bytes = serialize(&parsed, Format::Json).expect("json serialization should succeed");
    let restored =
        deserialize(&json_bytes, Format::Json).expect("json deserialization should succeed");

    assert_eq!(restored.children.len(), parsed.children.len());
}

fn extract_embedded_svg(input: &str) -> Option<&str> {
    let start = input.find("<svg")?;
    let end = input.find("</svg>")? + "</svg>".len();
    input.get(start..end)
}
