//! Browser-specific tests (run with: wasm-pack test --headless --firefox/chrome)

use wasm_bindgen_test::*;

use svg2web_wasm::init_panic_hook;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_panic_hook_in_browser() {
	init_panic_hook();
	assert!(true);
}