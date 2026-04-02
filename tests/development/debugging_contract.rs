#[test]
fn tracing_subscriber_format_t100() {
    let docs = std::fs::read_to_string("docs/development/debugging.md").unwrap_or_default();
    assert!(docs.contains("tracing_subscriber"));
    assert!(docs.contains("RUST_LOG=svg2web_core=debug"));
    assert!(docs.contains("structured logs"));
}

#[test]
fn source_maps_for_wasm_t101() {
    let docs = std::fs::read_to_string("docs/development/debugging.md").unwrap_or_default();
    assert!(docs.contains("wasm-pack build crates/svg2web-wasm --target web --debug"));
    assert!(docs.contains("source maps"));
}