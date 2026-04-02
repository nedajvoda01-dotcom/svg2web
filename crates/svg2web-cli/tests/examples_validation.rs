use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    let mut dir = std::env::current_dir().expect("current dir must be readable");
    loop {
        if dir.join("docs").is_dir() && dir.join("crates").is_dir() {
            return dir;
        }
        if !dir.pop() {
            panic!("workspace root not found");
        }
    }
}

#[test]
fn example_basic_vanilla() {
    assert!(workspace_root().join("examples/basic").exists());
}

#[test]
fn example_react_component() {
    assert!(workspace_root().join("examples/react-component").exists());
}

#[test]
fn example_responsive_breakpoints() {
    assert!(workspace_root().join("examples/responsive").exists());
}
