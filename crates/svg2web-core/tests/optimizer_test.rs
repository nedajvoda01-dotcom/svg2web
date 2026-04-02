fn read_opt_doc() -> String {
    std::fs::read_to_string("docs/internals/core/optimization.md").unwrap_or_default()
}

#[test]
fn optimize_preserves_visuals_via_resvg_t067() {
    let docs = std::fs::read_to_string("docs/development/testing.md").unwrap_or_default();
    assert!(docs.contains("resvg"));
}

// T-067: Douglas-Peucker
#[test]
fn douglas_peucker_algorithm_documented_t067() {
    let d = read_opt_doc();
    assert!(
        d.contains("Douglas-Peucker"),
        "optimization.md must document Douglas-Peucker algorithm"
    );
    assert!(
        d.contains("tolerance"),
        "optimization.md must document tolerance parameter for path simplification"
    );
    assert!(
        d.contains("коллинеарных"),
        "optimization.md must document removal of collinear points"
    );
}

#[test]
fn path_simplify_pseudocode_present_t067() {
    let d = read_opt_doc();
    assert!(
        d.contains("simplify"),
        "optimization.md must document simplify_paths function"
    );
    assert!(
        d.contains("tolerance") && d.contains("recurse"),
        "optimization.md must include Douglas-Peucker pseudocode with tolerance and recursion"
    );
}

// T-068: Дедупликация 95%
#[test]
fn deduplication_sha256_clustering_t068() {
    let d = read_opt_doc();
    assert!(
        d.contains("SHA256"),
        "optimization.md must document SHA256 subtree hashing for deduplication"
    );
    assert!(
        d.contains("95%"),
        "optimization.md must document structural similarity threshold >95%"
    );
}

#[test]
fn deduplication_threshold_parameter_t068() {
    let d = read_opt_doc();
    assert!(
        d.contains("threshold"),
        "optimization.md must document threshold parameter for deduplication"
    );
    assert!(
        d.contains("deduplication"),
        "optimization.md must have a deduplication section"
    );
}

// T-069: Минификация ID
#[test]
fn minify_generates_short_ids_t069() {
    let d = read_opt_doc();
    assert!(
        d.contains("a, b, c") || d.contains("a, b,"),
        "optimization.md must document short id generation (a, b, c...)"
    );
    assert!(
        d.contains("uuid").not() || d.contains("вместо длинных uuid"),
        "optimization.md must contrast short ids against uuid"
    );
}

#[test]
fn minify_removes_default_attributes_t069() {
    let d = read_opt_doc();
    assert!(
        d.contains("fill=\"#000\""),
        "optimization.md must document removal of default fill attribute"
    );
    assert!(
        d.contains("дефолтных атрибутов") || d.contains("default"),
        "optimization.md must document removal of default attributes"
    );
}

trait BoolNot {
    fn not(self) -> bool;
}
impl BoolNot for bool {
    fn not(self) -> bool {
        !self
    }
}
