use std::fs;
use std::process::Command;

pub fn run() -> Result<(), String> {
    let status = Command::new("cargo")
        .arg("clean")
        .status()
        .map_err(|e| format!("failed to run cargo clean: {e}"))?;
    if !status.success() {
        return Err("cargo clean failed".to_string());
    }

    let _ = fs::remove_dir_all("web/node_modules");
    let _ = fs::remove_dir_all("web/dist");
    let _ = fs::remove_dir_all("crates/svg2web-wasm/pkg");
    Ok(())
}
