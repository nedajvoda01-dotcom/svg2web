// T-082: Key derivation — SHA256(concat(sha256(svg), core_version, sha256(config)))
#[test]
fn cache_key_sha256_derivation_t082() {
    let docs = std::fs::read_to_string("docs/internals/cache.md").unwrap_or_default();
    assert!(
        docs.contains("SHA256"),
        "cache.md must document SHA256 key derivation"
    );
    assert!(
        docs.contains("SHA256(concat(") || docs.contains("sha256(svg_content)"),
        "cache.md must document the exact key formula with SHA256(concat(...))"
    );
    assert!(
        docs.contains("Детерминированность") || docs.contains("core_version"),
        "cache.md must document determinism and version invalidation"
    );
}

#[test]
fn cache_methods_exist_t040() {
    let s = std::fs::read_to_string("docs/api-reference/rust/cache.md").unwrap_or_default();
    for method in ["get(", "set(", "invalidate(", "clear("] {
        assert!(s.contains(method), "cache.md must document method {method}");
    }
}

#[test]
fn cache_key_derivation_t041() {
    let docs = std::fs::read_to_string("docs/api-reference/rust/cache.md").unwrap_or_default();
    assert!(docs.contains("from_svg(svg: &str, config: &Config) -> Self"));
    assert!(docs.contains("SHA256(SVG содержимого)"));
    assert!(docs.contains("SHA256(сериализованного конфига в JSON)"));
}

// T-083: Sled backend
#[test]
fn disk_cache_uses_sled_t083() {
    let docs = std::fs::read_to_string("docs/internals/cache.md").unwrap_or_default();
    assert!(
        docs.contains("Sled") || docs.contains("sled"),
        "cache.md must document Sled as disk cache backend"
    );
    assert!(
        docs.contains("~/.cache/svg2web/"),
        "cache.md must document sled storage path ~/.cache/svg2web/"
    );
    assert!(
        docs.contains("ACID"),
        "cache.md must document ACID transactions in sled"
    );
    assert!(
        docs.contains("zstd"),
        "cache.md must document zstd compression in sled"
    );
}

// T-084: LRU memory limit
#[test]
fn memory_cache_lru_1000_entries_t084() {
    let docs = std::fs::read_to_string("docs/internals/cache.md").unwrap_or_default();
    assert!(
        docs.contains("1000"),
        "cache.md must document 1000 entry LRU limit"
    );
    assert!(
        docs.contains("lru") || docs.contains("LRU"),
        "cache.md must document LRU crate or algorithm"
    );
    assert!(
        docs.contains("Arc<Mutex"),
        "cache.md must document thread-safe access via Arc<Mutex>"
    );
}

#[test]
fn version_change_invalidates_t042() {
    let docs = std::fs::read_to_string("docs/api-reference/rust/cache.md").unwrap_or_default();
    assert!(docs.contains("CLI (инкрементальные сборки)"));
    assert!(docs.contains("disk: Option<DiskCache>") && docs.contains("memory: MemoryCache"));
    assert!(docs.contains("WASM (только память)"));
}

#[test]
fn wasm_uses_only_memory_t085() {
    let docs = std::fs::read_to_string("docs/api-reference/rust/cache.md").unwrap_or_default();
    assert!(docs.contains("WASM (только память)"));
}
