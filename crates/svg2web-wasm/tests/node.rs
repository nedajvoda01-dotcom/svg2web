//! Tests for WASM bindings (run with: wasm-pack test --node)

use wasm_bindgen_test::*;

use svg2web_wasm::{analyze_svg, init_panic_hook, optimize_svg, parse_svg, StringPool};

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

#[wasm_bindgen_test]
fn test_analyze_svg_returns_structure() {
	init_panic_hook();

	let svg = r##"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="5" cy="5" r="5"/><circle cx="15" cy="15" r="5"/></svg>"##;
	let result = analyze_svg(svg);

	assert!(result.is_ok(), "analyze_svg should succeed on valid SVG");
	let analysis = result.unwrap();
	assert!(analysis.contains("score"), "Analysis should contain complexity score");
	assert!(analysis.contains("node_count"), "Analysis should contain node_count");
	assert!(analysis.contains("depth"), "Analysis should contain hierarchy depth");
}

#[wasm_bindgen_test]
fn test_optimize_svg_returns_result() {
	init_panic_hook();

	let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><rect x="0" y="0" width="10" height="10"/></svg>"##;
	let result = optimize_svg(svg);

	assert!(result.is_ok(), "optimize_svg should succeed on valid simple SVG");
	let optimized = result.unwrap();
	assert!(optimized.contains("svg"), "Optimized output should contain svg tag");
}

#[wasm_bindgen_test]
fn test_optimize_svg_does_not_panic_on_complex() {
	init_panic_hook();

	let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200">
		<defs><linearGradient id="g1"><stop offset="0%" stop-color="red"/><stop offset="100%" stop-color="blue"/></linearGradient></defs>
		<rect width="100" height="100" fill="url(#g1)"/>
		<circle cx="50" cy="50" r="30" fill="green"/>
	</svg>"##;

	// Must not panic — Err is acceptable, panic is not
	let _result = optimize_svg(svg);
}

#[wasm_bindgen_test]
fn test_string_pool_free_and_clear() {
	init_panic_hook();

	let mut pool = StringPool::new();
	let h1 = pool.allocate("First string");
	let h2 = pool.allocate("Second string");

	assert_eq!(pool.read(h1), "First string");
	assert_eq!(pool.read(h2), "Second string");
	assert!(pool.total_size() > 0);

	pool.free(h1);
	assert_eq!(pool.read(h1), "", "Freed handle should return empty string");
	assert_eq!(pool.read(h2), "Second string", "Other handles should remain");

	pool.clear();
	assert_eq!(pool.total_size(), 0, "Pool should be empty after clear");
	assert_eq!(pool.read(h2), "", "All handles should be gone after clear");
}