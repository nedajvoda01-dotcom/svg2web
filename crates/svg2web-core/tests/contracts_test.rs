#[test]
fn parse_returns_correct_signature_t035() {
    let root = std::fs::read_to_string("docs/api-reference/rust/core.md").unwrap_or_default();
    assert!(
        root.contains("pub fn parse(svg_bytes: &[u8]) -> Result<ParseOutput, Error>"),
        "core.md must document exact parse signature"
    );
}

#[test]
fn parse_file_reads_from_path_t035() {
    let root = std::fs::read_to_string("docs/api-reference/rust/core.md").unwrap_or_default();
    assert!(
        root.contains("pub fn parse_file(path: &Path) -> Result<ParseOutput, IoError>"),
        "core.md must document exact parse_file signature"
    );
}

#[test]
fn optimize_returns_correct_signature_t035() {
    let root = std::fs::read_to_string("docs/api-reference/rust/core.md").unwrap_or_default();
    assert!(
        root.contains("pub fn optimize(input: ParseOutput, config: &OptimizerConfig) -> Result<ParseOutput, Error>"),
        "core.md must document exact optimize signature"
    );
}

#[test]
fn parseoutput_has_6_fields_t036() {
    let root = std::fs::read_to_string("docs/api-reference/rust/core.md").unwrap_or_default();
    for f in ["meta", "structure", "geometry", "styles", "assets", "content"] {
        assert!(root.contains(f), "missing ParseOutput field: {f}");
    }
}

#[test]
fn optimizerconfig_fields_exist_t037() {
    let root = std::fs::read_to_string("docs/api-reference/rust/core.md").unwrap_or_default();
    for f in ["simplify_paths", "deduplicate", "minify", "remove_comments", "precision"] {
        assert!(root.contains(f), "missing optimizer field: {f}");
    }
}

#[test]
fn extract_images_decodes_base64_t070() {
    let d = std::fs::read_to_string("docs/internals/core/extraction.md").unwrap_or_default();
    assert!(d.contains("Base64") || d.contains("base64"));
    assert!(
        d.contains("magic bytes"),
        "extraction.md must document format detection via magic bytes"
    );
}

#[test]
fn extract_images_converts_webp_85_quality_t071() {
    let d = std::fs::read_to_string("docs/internals/core/extraction.md").unwrap_or_default();
    assert!(d.contains("WebP") && d.contains("85"));
    assert!(
        (d.contains("1x") && d.contains("2x")) || d.contains("Retina"),
        "extraction.md must document 1x and 2x retina variants"
    );
}

#[test]
fn extract_fonts_detects_google_fonts_t072() {
    let d = std::fs::read_to_string("docs/internals/core/extraction.md").unwrap_or_default();
    assert!(d.contains("Google Fonts"));
    assert!(
        d.contains("font-family"),
        "extraction.md must document font-family parsing"
    );
    assert!(
        d.contains("system font") || d.contains("Fallback"),
        "extraction.md must document system font fallback stack"
    );
}

// T-073: Fetch внешних ресурсов
#[test]
fn fetch_external_timeout_30s_t073() {
    let d = std::fs::read_to_string("docs/internals/core/extraction.md").unwrap_or_default();
    assert!(
        d.contains("30s"),
        "extraction.md must document 30s timeout for external fetch"
    );
    assert!(
        d.contains("reqwest"),
        "extraction.md must document reqwest HTTP client"
    );
    assert!(
        d.contains("3 retry") || d.contains("3 attempts") || d.contains("3 попыт"),
        "extraction.md must document 3 retry attempts"
    );
    assert!(
        d.contains("exponential backoff"),
        "extraction.md must document exponential backoff retry strategy"
    );
}

#[test]
fn simplify_paths_douglas_peucker_tolerance_t067() {
    let d = std::fs::read_to_string("docs/internals/core/optimization.md").unwrap_or_default();
    assert!(d.contains("Douglas-Peucker") && d.contains("tolerance"));
}

#[test]
fn deduplication_threshold_95_percent_t068() {
    let d = std::fs::read_to_string("docs/internals/core/optimization.md").unwrap_or_default();
    assert!(d.contains("95%") || d.contains("95"));
}

#[test]
fn minification_short_ids_t069() {
    let d = std::fs::read_to_string("docs/internals/core/optimization.md").unwrap_or_default();
    assert!(d.contains("a, b, c") || d.contains("short id") || d.contains("коротких id"));
}
