//! Tests for WASM bindings (run with: wasm-pack test --node)

use wasm_bindgen_test::*;

use svg2web_wasm::{init_panic_hook, parse_svg, StringPool};

#[wasm_bindgen_test]
fn test_string_pool_allocate_and_read() {
	init_panic_hook();

	let mut pool = StringPool::new();
	let data = "Hello, WASM!";
	let handle = pool.allocate(data);

	let retrieved = pool.read(handle);
	assert_eq!(retrieved, data);
}

#[wasm_bindgen_test]
fn test_string_pool_large_data() {
	init_panic_hook();

	let mut pool = StringPool::new();
	let large_data = "x".repeat(1024 * 1024);
	let handle = pool.allocate(&large_data);

	let retrieved = pool.read(handle);
	assert_eq!(retrieved.len(), large_data.len());
	assert_eq!(retrieved, large_data);
}

#[wasm_bindgen_test]
fn test_parse_svg_valid() {
	init_panic_hook();

	let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><rect width="10" height="10"/></svg>"#;

	let result = parse_svg(svg);
	assert!(result.is_ok(), "Should parse valid SVG");

	let json = result.expect("parse_svg should return JSON for valid SVG");
	assert!(json.contains("\"tag\":\"svg\""));
	assert!(json.contains("\"children\""));
}

#[wasm_bindgen_test]
fn test_parse_svg_invalid() {
	init_panic_hook();

	let result = parse_svg("not valid xml");
	assert!(result.is_err(), "Should fail on invalid SVG");

	let err = result.expect_err("parse_svg should fail for invalid SVG");
	let lowered = err.as_string().unwrap_or_default().to_lowercase();
	assert!(lowered.contains("parse") || lowered.contains("error"));
}