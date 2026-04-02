use std::fs;
use std::path::Path;

fn collect_rs_files(dir: &Path, out: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                collect_rs_files(&p, out);
            } else if p.extension().and_then(|x| x.to_str()) == Some("rs") {
                out.push(p.display().to_string());
            }
        }
    }
}

pub fn run() -> Result<(), String> {
    // T-005 license_headers (lightweight linter)
    let mut files = Vec::new();
    collect_rs_files(Path::new("crates"), &mut files);
    collect_rs_files(Path::new("xtask"), &mut files);

    for f in files {
        let s = fs::read_to_string(&f).map_err(|e| e.to_string())?;
        if !s.contains("MIT") {
            return Err(format!("Missing MIT marker in: {}", f));
        }
    }

    Ok(())
}
