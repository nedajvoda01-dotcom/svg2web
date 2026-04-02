use std::fs;

fn must_contain(path: &str, needles: &[&str]) {
    let s = fs::read_to_string(path).unwrap_or_default();
    for n in needles {
        assert!(s.contains(n), "{} must contain {}", path, n);
    }
}

#[test]
fn meta_json_schema_t054() {
    must_contain(
        "docs/api-reference/json-schema/meta.md",
        &["version", "source", "parsed_at", "ISO 8601", "canvas", "width", "height", "viewBox", "title", "description"],
    );
}

#[test]
fn structure_json_schema_t055() {
    must_contain(
        "docs/api-reference/json-schema/structure.md",
        &["version", "root", "id", "type", "tag", "children", "flat", "elements"],
    );
}

#[test]
fn geometry_json_schema_t056() {
    must_contain(
        "docs/api-reference/json-schema/geometry.md",
        &["version", "bounds", "x", "y", "width", "height", "rx", "ry", "paths", "d", "fill_rule", "transforms", "translate", "rotate", "scale"],
    );
}

#[test]
fn styles_json_schema_t057() {
    must_contain(
        "docs/api-reference/json-schema/styles.md",
        &["version", "colors", "gradients", "linear", "radial", "fonts", "typography", "effects", "strokes"],
    );
}

#[test]
fn assets_json_schema_t058() {
    must_contain(
        "docs/api-reference/json-schema/assets.md",
        &["version", "images", "original", "variants", "1x", "2x", "icons", "fonts", "woff2", "woff", "external"],
    );
}

#[test]
fn content_json_schema_t059() {
    must_contain(
        "docs/api-reference/json-schema/content.md",
        &["version", "logo", "tagline", "navigation", "contact", "hero", "cards", "footer", "localization"],
    );
}

#[test]
fn version_field_present_t060() {
    for path in [
        "docs/api-reference/json-schema/meta.md",
        "docs/api-reference/json-schema/structure.md",
        "docs/api-reference/json-schema/geometry.md",
        "docs/api-reference/json-schema/styles.md",
        "docs/api-reference/json-schema/assets.md",
        "docs/api-reference/json-schema/content.md",
        "docs/api-reference/json-schema/overview.md",
    ] {
        must_contain(path, &["version"]);
    }
}
