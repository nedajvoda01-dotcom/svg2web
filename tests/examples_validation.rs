use std::path::Path;

#[test]
fn examples_dirs_exist() {
    for dir in [
        "examples/basic",
        "examples/react-component",
        "examples/vue-component",
        "examples/responsive",
    ] {
        assert!(Path::new(dir).exists(), "Missing example dir: {}", dir);
    }
}
