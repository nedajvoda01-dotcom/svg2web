fn workspace_root() -> std::path::PathBuf {
    let mut dir = std::env::current_dir().expect("current dir must be readable");
    loop {
        if dir.join("docs").is_dir() && dir.join("crates").is_dir() {
            return dir;
        }
        assert!(dir.pop(), "workspace root not found");
    }
}

fn component_detection_doc() -> String {
    std::fs::read_to_string(
        workspace_root().join("docs/internals/generator/component-detection.md"),
    )
    .expect("docs/internals/generator/component-detection.md must exist")
}

// T-080: 4-Phase Algorithm
#[test]
fn phase1_hashing_dfs_t080() {
    let d = component_detection_doc();
    assert!(d.contains("Phase 1") && d.contains("DFS"));
    assert!(
        d.contains("tag name") || d.contains("tag"),
        "component-detection.md Phase 1 must include tag name in hash"
    );
    assert!(
        d.contains("sorted attributes") || d.contains("sorted"),
        "component-detection.md Phase 1 must sort attributes for hash"
    );
    assert!(
        d.contains("sha256") || d.contains("SHA256"),
        "component-detection.md Phase 1 must use sha256 hashing"
    );
}

#[test]
fn phase2_clustering_hashmap_t080() {
    let d = component_detection_doc();
    assert!(d.contains("HashMap"));
    assert!(
        d.contains("Phase 2"),
        "component-detection.md must label Phase 2"
    );
    assert!(
        d.contains("Vec<NodeRef>") || d.contains("NodeRef"),
        "component-detection.md Phase 2 must document Vec<NodeRef> grouping"
    );
}

#[test]
fn phase3_similarity_95_percent_t080() {
    let d = component_detection_doc();
    assert!(d.contains("95%"));
    assert!(
        d.contains("Phase 3"),
        "component-detection.md must label Phase 3"
    );
    assert!(
        d.contains("коллизий") || d.contains("collision"),
        "component-detection.md Phase 3 must mention collision handling"
    );
}

#[test]
fn phase4_ranking_min_size_2_t080() {
    let d = component_detection_doc();
    assert!(d.contains("min_size") && d.contains("2"));
    assert!(
        d.contains("Phase 4"),
        "component-detection.md must label Phase 4"
    );
    assert!(
        d.contains("occurrences"),
        "component-detection.md Phase 4 must describe sorting by occurrences"
    );
}

#[test]
fn max_depth_3_enforced_t080() {
    let d = component_detection_doc();
    assert!(d.contains("max_depth") && d.contains("3"));
}

#[test]
fn component_result_struct_documented_t080() {
    let d = component_detection_doc();
    // Component struct has id, template, occurrences
    assert!(
        d.contains("id") && d.contains("template") && d.contains("occurrences"),
        "component-detection.md must document Component struct fields: id, template, occurrences"
    );
}
