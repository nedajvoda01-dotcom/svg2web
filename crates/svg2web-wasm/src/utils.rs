use wasm_bindgen::prelude::*;

/// Initialize panic hook for better debugging in browser console.
#[wasm_bindgen]
pub fn init_panic_hook() {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();
}

/// Placeholder for future logger wiring.
#[wasm_bindgen]
pub fn init_logging() {
	// Hook for wasm_logger initialization in future iterations.
}
