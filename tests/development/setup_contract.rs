use std::net::TcpStream;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

fn has_cmd(name: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {} >/dev/null 2>&1", name))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn parse_semver_like(value: &str) -> Option<(u64, u64, u64)> {
    let cleaned = value.trim().trim_start_matches('v');
    let mut parts = cleaned.split('.');
    Some((
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.split('-').next()?.parse().ok()?,
    ))
}

#[test]
fn toolchain_versions_t093() {
    let docs = std::fs::read_to_string("docs/development/setup.md").unwrap_or_default();
    assert!(docs.contains("Rust 1.70+"));
    assert!(docs.contains("Node 18+"));
    assert!(docs.contains("cargo install wasm-pack"));

    if has_cmd("rustc") {
        let out = Command::new("rustc").arg("--version").output().expect("rustc --version failed");
        let text = String::from_utf8_lossy(&out.stdout);
        let version = text.split_whitespace().nth(1).and_then(parse_semver_like).unwrap_or((0, 0, 0));
        assert!(version >= (1, 70, 0), "Rust 1.70+ required, got {}", text.trim());
    }

    if has_cmd("node") {
        let out = Command::new("node").arg("--version").output().expect("node --version failed");
        let text = String::from_utf8_lossy(&out.stdout);
        let version = parse_semver_like(text.trim()).unwrap_or((0, 0, 0));
        assert!(version >= (18, 0, 0), "Node 18+ required, got {}", text.trim());
    }

    if has_cmd("wasm-pack") {
        let status = Command::new("wasm-pack").arg("--version").status().expect("wasm-pack --version failed");
        assert!(status.success(), "wasm-pack must be installed");
    }
}

#[test]
fn build_commands_work_t094() {
    let docs = std::fs::read_to_string("docs/development/setup.md").unwrap_or_default();
    assert!(docs.contains("cargo build --workspace"));
    assert!(docs.contains("wasm-pack build crates/svg2web-wasm --target web"));
    assert!(docs.contains("npm install") && docs.contains("npm run dev"));

    if has_cmd("cargo") {
        let status = Command::new("cargo")
            .arg("build")
            .arg("--workspace")
            .status()
            .expect("cargo build --workspace failed to start");
        assert!(status.success(), "cargo build --workspace must succeed");
    }

    if has_cmd("wasm-pack") {
        let status = Command::new("wasm-pack")
            .args(["build", "crates/svg2web-wasm", "--target", "web"])
            .status()
            .expect("wasm-pack build failed to start");
        assert!(status.success(), "wasm-pack build must succeed");
    }

    if has_cmd("npm") {
        let mut child = Command::new("sh")
            .arg("-lc")
            .arg("cd web && npm install && npm run dev -- --port 3000")
            .spawn()
            .expect("failed to start web dev server");
        let started = Instant::now();
        let mut listening = false;
        while started.elapsed() < Duration::from_secs(30) {
            if TcpStream::connect("127.0.0.1:3000").is_ok() {
                listening = true;
                break;
            }
            thread::sleep(Duration::from_millis(500));
        }
        let _ = child.kill();
        let _ = child.wait();
        assert!(listening, "web dev server must listen on port 3000");
    }
}