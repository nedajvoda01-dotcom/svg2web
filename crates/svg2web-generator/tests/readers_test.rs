use std::io::Write;

use svg2web_core::{ElementType, SVGElement};
use svg2web_generator::readers::{read_binary, read_json, JsonReader};

fn temp_path(name: &str) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "svg2web-generator-{}-{}-{}",
        name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    path
}

#[test]
fn test_read_json_roundtrip() {
    let element = SVGElement {
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

    let json = svg2web_core::serialize(&element, svg2web_core::Format::Json)
        .expect("serialize element to json bytes");
    let temp = temp_path("roundtrip.json");
    let mut file = std::fs::File::create(&temp).expect("create temp file");
    file.write_all(&json).expect("write json to temp file");

    let result = read_json(&temp).expect("read_json should parse valid file");
    assert_eq!(result.tag, "svg");
    assert!(result.attributes.contains_key("xmlns"));

    let json_text = String::from_utf8(json).expect("json bytes must be utf-8");
    let result_from_str = JsonReader::read_str(&json_text).expect("read_str should parse valid json");
    assert_eq!(result_from_str.tag, "svg");

    let _ = std::fs::remove_file(&temp);
}

#[test]
fn test_read_json_invalid_fails() {
    let temp = temp_path("invalid.json");
    let mut file = std::fs::File::create(&temp).expect("create temp file");
    file.write_all(b"not json")
        .expect("write invalid json to temp file");

    let result = read_json(&temp);
    assert!(result.is_err());

    let _ = std::fs::remove_file(&temp);
}

#[test]
fn test_read_binary_when_feature_disabled() {
    let result = read_binary(std::path::Path::new("/tmp/test.bin"));
    assert!(result.is_err());
}