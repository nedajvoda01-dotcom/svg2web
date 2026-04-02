use std::fs;

fn walk_for_pattern(dir: &str, pattern: &str) -> Vec<String> {
    let mut found = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                found.extend(walk_for_pattern(path.to_str().unwrap_or(""), pattern));
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if content.contains(pattern) {
                        found.push(path.display().to_string());
                    }
                }
            }
        }
    }
    found
}

#[test]
fn architecture_compliance_contract() {
    let core = fs::read_to_string("crates/svg2web-core/src/lib.rs").unwrap_or_default();
    let generator = fs::read_to_string("crates/svg2web-generator/src/lib.rs").unwrap_or_default();

    assert!(
        !core.contains("std::fs") && !core.contains("tokio::fs"),
        "core must not depend on filesystem APIs"
    );
    assert!(
        !generator.contains("svg2web-cli") && !generator.contains("crates/svg2web-cli"),
        "generator must not depend on cli"
    );
}

// T-061: Границы крейтов — сканируем все .rs файлы в src/
#[test]
fn core_sources_no_fs_import_t061() {
    let violations = walk_for_pattern("crates/svg2web-core/src", "std::fs");
    assert!(
        violations.is_empty(),
        "svg2web-core must not use std::fs (accepts only &[u8]); found in: {:?}",
        violations
    );
}

#[test]
fn generator_sources_no_cli_or_wasm_dep_t061() {
    let cli = walk_for_pattern("crates/svg2web-generator/src", "svg2web_cli");
    let wasm = walk_for_pattern("crates/svg2web-generator/src", "svg2web_wasm");
    assert!(
        cli.is_empty() && wasm.is_empty(),
        "svg2web-generator must not depend on cli or wasm; cli: {:?}, wasm: {:?}",
        cli,
        wasm
    );
}

#[test]
fn crate_boundaries_documented_t061() {
    let d = fs::read_to_string("docs/internals/architecture.md").unwrap_or_default();
    assert!(d.contains("&[u8]"), "architecture.md must document that core works with &[u8]");
    assert!(
        d.contains("Generator не знает о CLI") || d.contains("Generator не знает"),
        "architecture.md must document generator independence from CLI"
    );
}

// T-062: Поток данных — порядок стадий в docs/internals/architecture.md
#[test]
fn data_flow_pipeline_sequence_t062() {
    let d = fs::read_to_string("docs/internals/architecture.md").unwrap_or_default();
    let stages = [
        "Parser", "Analyzer", "Optimizer", "Extractor", "Serializer", "Cache", "Generator",
    ];
    let mut last_pos = 0usize;
    for stage in stages {
        let pos = d
            .find(stage)
            .unwrap_or_else(|| panic!("pipeline stage '{stage}' not found in architecture.md"));
        assert!(
            pos >= last_pos,
            "pipeline stage '{stage}' at pos {pos} is out of order (expected after pos {last_pos})"
        );
        last_pos = pos;
    }
}
