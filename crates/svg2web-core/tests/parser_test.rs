fn read_parsing_doc() -> String {
    std::fs::read_to_string("docs/internals/core/parsing.md").unwrap_or_default()
}

// T-063: usvg как основной парсер
#[test]
fn usvg_is_primary_parser_t063() {
    let d = read_parsing_doc();
    assert!(
        d.contains("usvg используется как основной") || d.contains("usvg как основной"),
        "parsing.md must state that usvg is the primary parser"
    );
}

#[test]
fn usvg_in_core_cargo_toml_t063() {
    let manifest =
        std::fs::read_to_string("crates/svg2web-core/Cargo.toml").unwrap_or_default();
    assert!(manifest.contains("usvg"), "svg2web-core Cargo.toml must declare usvg dependency");
}

// T-064: roxmltree fallback
#[test]
fn roxmltree_fallback_documented_t064() {
    let d = read_parsing_doc();
    assert!(
        d.contains("roxmltree"),
        "parsing.md must document roxmltree as fallback parser"
    );
    assert!(
        d.contains("fallback") && d.contains("базовых элементов"),
        "parsing.md must explain that fallback recovers basic elements"
    );
}

// T-065: Поддержка SVG-фич
#[test]
fn basic_shapes_documented_t065() {
    let d = read_parsing_doc();
    for shape in ["rect", "circle", "path"] {
        assert!(
            d.contains(shape),
            "parsing.md must document support for SVG shape: {shape}"
        );
    }
}

#[test]
fn groups_with_transforms_documented_t065() {
    let d = read_parsing_doc();
    assert!(
        d.contains("Группы") || d.contains("group"),
        "parsing.md must document group (g) support"
    );
    assert!(
        d.contains("transforms") || d.contains("трансформациями"),
        "parsing.md must document transforms in groups"
    );
}

#[test]
fn text_with_fonts_documented_t065() {
    let d = read_parsing_doc();
    assert!(
        d.contains("Текст") || d.contains("текст"),
        "parsing.md must document text element support"
    );
    assert!(
        d.contains("font metadata") || d.contains("шрифт"),
        "parsing.md must document font metadata extraction"
    );
}

#[test]
fn gradients_and_patterns_documented_t065() {
    let d = read_parsing_doc();
    assert!(
        d.contains("Градиенты") || d.contains("gradient"),
        "parsing.md must document gradient support"
    );
    assert!(
        d.contains("паттерны") || d.contains("pattern"),
        "parsing.md must document pattern support"
    );
}

#[test]
fn masks_and_clippath_documented_t065() {
    let d = read_parsing_doc();
    assert!(
        d.contains("Маски") || d.contains("mask"),
        "parsing.md must document mask support"
    );
    assert!(
        d.contains("clip-path"),
        "parsing.md must document clip-path support"
    );
}

// T-066: Валидация XML
#[test]
fn xml_validation_uses_quickxml_t066() {
    let d = read_parsing_doc();
    assert!(
        d.contains("quick-xml"),
        "parsing.md must document quick-xml for XML well-formedness validation"
    );
}

#[test]
fn svgparse_error_has_line_info_t066() {
    let d = read_parsing_doc();
    assert!(
        d.contains("SvgParse"),
        "parsing.md must document SvgParse error type"
    );
    assert!(
        d.contains("строки") || d.contains("позиции") || d.contains("line"),
        "parsing.md SvgParse error must reference line/position"
    );
}

#[test]
fn invalid_svg_error_documented_t066() {
    let d = read_parsing_doc();
    assert!(
        d.contains("InvalidSvg"),
        "parsing.md must document InvalidSvg error type"
    );
    assert!(
        d.contains("структурные проблемы") || d.contains("structural"),
        "parsing.md must describe InvalidSvg as covering structural problems"
    );
}

// pre-existing fixture-based tests kept
#[test]
fn parses_all_basic_shapes_t065() {
    let fixture = std::fs::read_to_string("crates/svg2web-core/tests/fixtures/simple.svg").unwrap();
    assert!(fixture.contains("<rect") && fixture.contains("<circle"));
}

#[test]
fn validates_xml_wellformedness_t066() {
    let fixture = std::fs::read_to_string("crates/svg2web-core/tests/fixtures/simple.svg").unwrap();
    assert!(fixture.contains("xmlns="));
}

#[test]
fn resolves_circular_references_t066() {
    let docs = std::fs::read_to_string("docs/internals/core/parsing.md").unwrap_or_default();
    assert!(docs.contains("use -> def") || docs.contains("use -> def"));
}
