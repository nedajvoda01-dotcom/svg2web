fn read_serial_doc() -> String {
    std::fs::read_to_string("docs/internals/core/serialization.md").unwrap_or_default()
}

// T-074: Zero-copy сериализация
#[test]
fn zero_copy_cow_documented_t074() {
    let d = read_serial_doc();
    assert!(
        d.contains("zero-copy"),
        "serialization.md must document zero-copy approach"
    );
    assert!(
        d.contains("Cow"),
        "serialization.md must mention Cow for zero-copy serialization"
    );
}

#[test]
fn serde_used_for_serialization_t074() {
    let d = read_serial_doc();
    assert!(d.contains("serde"), "serialization.md must document serde usage");
}

// T-075: Streaming для больших файлов
#[test]
fn streaming_for_large_files_t075() {
    let d = read_serial_doc();
    assert!(
        d.contains("10MB+"),
        "serialization.md must document 10MB+ threshold for streaming"
    );
    assert!(
        d.contains("streaming") || d.contains("streaming-подход"),
        "serialization.md must document streaming approach for large files"
    );
}

// T-076: Split output — 6 файлов
#[test]
fn split_output_creates_6_files_t076() {
    let d = read_serial_doc();
    assert!(
        d.contains("Split-output") || d.contains("split-output") || d.contains("--split-output"),
        "serialization.md must document --split-output flag"
    );
    assert!(
        d.contains("6 файлов"),
        "serialization.md must document that split-output produces 6 files"
    );
}

#[test]
fn split_output_file_names_documented_t076() {
    let d = read_serial_doc();
    // All 6 schema file names must be referenced
    for name in ["meta", "structure", "geometry", "styles", "assets", "content"] {
        assert!(
            d.contains(name),
            "serialization.md must reference split output file '{name}'"
        );
    }
}
