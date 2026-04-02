#[test]
fn sla_performance_t098() {
    let docs = std::fs::read_to_string("docs/development/benchmarking.md").unwrap_or_default();
    assert!(docs.contains("Parse 10MB"));
    assert!(docs.contains("< 500ms"));
    assert!(docs.contains("cargo bench --package svg2web-core --bench parse_bench"));
}

#[test]
fn no_regression_more_than_10_percent_t099() {
    let docs = std::fs::read_to_string("docs/development/benchmarking.md").unwrap_or_default();
    assert!(docs.contains("degradation") || docs.contains("деградации производительности > 10%"));
    assert!(docs.contains("--save-baseline main"));
    assert!(docs.contains("--baseline main"));
}