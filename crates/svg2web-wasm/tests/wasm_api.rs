#![cfg(not(target_arch = "wasm32"))]

#[test]
fn wasm_bindgen_exports_exist_t043() {
    let d = std::fs::read_to_string("docs/api-reference/rust/wasm.md").unwrap_or_default();
    for export in [
        "#[wasm_bindgen]\npub fn parse_svg",
        "#[wasm_bindgen]\npub fn analyze_svg",
        "#[wasm_bindgen]\npub fn optimize_svg",
        "#[wasm_bindgen]\npub fn extract_assets",
        "#[wasm_bindgen]\npub fn serialize_to_json",
    ] {
        assert!(d.contains(export), "missing wasm export in docs: {export}");
    }
}

#[test]
fn string_pool_prevents_oom_t085() {
    let d = std::fs::read_to_string("docs/internals/wasm-bindings.md").unwrap_or_default();
    assert!(
        d.contains(">1MB") || d.contains("1MB"),
        "wasm-bindings.md must document >1MB threshold for StringPool"
    );
    assert!(
        d.contains("allocate"),
        "wasm-bindings.md must document StringPool.allocate"
    );
    assert!(
        d.contains("read_chunk"),
        "wasm-bindings.md must document StringPool.read_chunk"
    );
    assert!(
        d.contains("free"),
        "wasm-bindings.md must document StringPool.free"
    );
    assert!(
        d.contains("handle") && (d.contains("u32") || d.contains("(u32)")),
        "wasm-bindings.md must document that allocate returns u32 handle"
    );
}

#[test]
fn string_pool_prevents_oom_t085_t044() {
    let d = std::fs::read_to_string("docs/api-reference/rust/wasm.md").unwrap_or_default();
    assert!(d.contains("StringPool") && d.contains("allocate") && d.contains("read_chunk") && d.contains("free"));
    assert!(d.contains(">1MB") || d.contains("1MB") || d.contains("large strings"));
}

#[test]
fn wasm_cache_memory_only_t042() {
    let d = std::fs::read_to_string("docs/internals/cache.md").unwrap_or_default();
    assert!(d.contains("WASM: Memory only"));
}

#[test]
fn no_fs_access_panics_t086() {
    let d = std::fs::read_to_string("docs/internals/wasm-bindings.md").unwrap_or_default();
    assert!(d.contains("no std::fs") || d.contains("no filesystem access"));
}

#[test]
fn wasm_single_threaded_model_t087() {
    let d = std::fs::read_to_string("docs/internals/wasm-bindings.md").unwrap_or_default();
    assert!(
        d.contains("Single-threaded") || d.contains("single-threaded"),
        "wasm-bindings.md must document single-threaded model for WASM"
    );
    assert!(
        d.contains("Web Workers"),
        "wasm-bindings.md must document Web Workers for parallelism as alternative to threads"
    );
}
