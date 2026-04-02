use std::collections::HashMap;

use wasm_bindgen::prelude::*;

/// Memory pool for managing large strings in WASM.
#[wasm_bindgen]
pub struct StringPool {
	chunks: HashMap<u32, String>,
	next_id: u32,
}

#[wasm_bindgen]
impl StringPool {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self {
			chunks: HashMap::new(),
			next_id: 1,
		}
	}

	/// Allocate a string in the pool and return a numeric handle.
	pub fn allocate(&mut self, data: &str) -> u32 {
		let id = self.next_id;
		self.chunks.insert(id, data.to_string());
		self.next_id = self.next_id.saturating_add(1);
		id
	}

	/// Read a full string by handle.
	pub fn read(&self, handle: u32) -> String {
		self.chunks.get(&handle).cloned().unwrap_or_default()
	}

	/// Free memory for a handle.
	pub fn free(&mut self, handle: u32) {
		self.chunks.remove(&handle);
	}

	/// Get total allocated size in bytes.
	pub fn total_size(&self) -> usize {
		self.chunks.values().map(std::string::String::len).sum()
	}

	/// Clear all allocations.
	pub fn clear(&mut self) {
		self.chunks.clear();
	}
}

impl Default for StringPool {
	fn default() -> Self {
		Self::new()
	}
}
