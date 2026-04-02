use std::process::Command;

pub fn run() -> Result<(), String> {
    let status = Command::new("cargo")
        .args(["bench", "--workspace"])
        .status()
        .map_err(|e| format!("failed to run cargo bench: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err("cargo bench --workspace failed".to_string())
    }
}
