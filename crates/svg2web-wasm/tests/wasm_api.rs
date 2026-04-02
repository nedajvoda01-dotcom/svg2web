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

// T-085: StringPool \u2014 \u0440\u0430\u0431\u043e\u0442\u0430 \u0441 \u0431\u043e\u043b\u044c\u0448\u0438\u043c\u0438 \u0441\u0442\u0440\u043e\u043a\u0430\u043c\u0438 (>1MB)
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
    assert!(d.contains(">1MB") || d.contains("1MB") || d.contains("\u0431\u043e\u043b\u044c\u0448\u0438\u0445 \u0441\u0442\u0440\u043e\u043a"));
}

#[test]
fn wasm_cache_memory_only_t042() {
    let d = std::fs::read_to_string("docs/internals/cache.md").unwrap_or_default();
    assert!(d.contains("WASM: Memory only"));
}

// T-086: \u041e\u0442\u0441\u0443\u0442\u0441\u0442\u0432\u0438\u0435 std::fs
#[test]
fn no_fs_access_panics_t086() {
    let d = std::fs::read_to_string("docs/internals/wasm-bindings.md").unwrap_or_default();
    assert!(d.contains("no std::fs") || d.contains("\u041d\u0435\u0442 \u0434\u043e\u0441\u0442\u0443\u043f\u0430 \u043a \u0444\u0430\u0439\u043b\u043e\u0432\u043e\u0439 \u0441\u0438\u0441\u0442\u0435\u043c\u0435"));
}

// T-087: Single-threaded \u043c\u043e\u0434\u0435\u043b\u044c
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
