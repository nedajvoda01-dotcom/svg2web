fn workspace_root() -> std::path::PathBuf {
    let mut dir = std::env::current_dir().expect("current dir must be readable");
    loop {
        if dir.join("docs").is_dir() && dir.join("crates").is_dir() {
            return dir;
        }
        assert!(dir.pop(), "workspace root not found");
    }
}

fn generator_doc() -> String {
    std::fs::read_to_string(workspace_root().join("docs/api-reference/rust/generator.md"))
        .expect("docs/api-reference/rust/generator.md must exist")
}

#[test]
fn generate_options_enums_t038() {
    let d = generator_doc();
    assert!(d.contains("pub enum Framework"));
    for variant in ["React", "Vue", "Vanilla"] {
        assert!(d.contains(variant), "missing Framework variant: {variant}");
    }

    assert!(d.contains("pub enum Styling"));
    for variant in ["NativeCss", "Tailwind", "ScopedCss"] {
        assert!(d.contains(variant), "missing Styling variant: {variant}");
    }
}

#[test]
fn generated_code_fields_t039() {
    let d = generator_doc();
    for field in [
        "pub html: String",
        "pub css: Option<String>",
        "pub js: Option<String>",
        "pub components: Vec<ComponentCode>",
        "pub assets: Vec<AssetFile>",
    ] {
        assert!(d.contains(field), "missing GeneratedCode field: {field}");
    }
}