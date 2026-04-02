use std::fs;

pub fn run() -> Result<(), String> {
    // T-003 workspace_deps_consistent
    let root = fs::read_to_string("Cargo.toml").map_err(|e| e.to_string())?;
    if !root.contains("workspace.dependencies") {
        return Err("Cargo.toml must declare [workspace.dependencies]".to_string());
    }

    // T-001 + T-004 are executed by tests, here we do quick smoke checks.
    let docs = fs::read_to_string("docs/README.md").map_err(|e| e.to_string())?;
    if !docs.contains("getting-started") {
        return Err("docs/README.md missing getting-started entry".to_string());
    }

    let changelog = fs::read_to_string("CHANGELOG.md").map_err(|e| e.to_string())?;
    if !changelog.contains("## [Unreleased]") {
        return Err("CHANGELOG.md must have Unreleased section".to_string());
    }

    Ok(())
}
