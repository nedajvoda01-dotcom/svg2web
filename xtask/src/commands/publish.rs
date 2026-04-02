use std::process::Command;

fn run_cmd(program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|e| format!("failed to run {program}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command failed: {} {}", program, args.join(" ")))
    }
}

pub fn run() -> Result<(), String> {
    run_cmd("cargo", &["publish", "--workspace"])?;
    run_cmd("npm", &["publish", "crates/svg2web-wasm/pkg"])?;
    Ok(())
}
