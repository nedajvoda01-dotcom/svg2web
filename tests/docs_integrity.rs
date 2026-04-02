use std::fs;
use std::path::{Path, PathBuf};

fn docs_root() -> PathBuf {
    PathBuf::from("docs")
}

fn contains_any_file(root: &Path) -> bool {
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                return true;
            }
            if path.is_dir() {
                stack.push(path);
            }
        }
    }
    false
}

fn extract_md_links(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = content.as_bytes();
    let mut i = 0usize;
    while i + 1 < bytes.len() {
        if bytes[i] == b']' && bytes[i + 1] == b'(' {
            let start = i + 2;
            if let Some(end_rel) = content[start..].find(')') {
                let raw = &content[start..start + end_rel];
                if !raw.starts_with("http://")
                    && !raw.starts_with("https://")
                    && !raw.starts_with("mailto:")
                    && !raw.starts_with('#')
                {
                    out.push(raw.to_string());
                }
            }
        }
        i += 1;
    }
    out
}

#[test]
fn readme_links_valid_t001() {
    let readme_path = docs_root().join("README.md");
    let content = fs::read_to_string(&readme_path).expect("docs/README.md must exist");
    for link in extract_md_links(&content) {
        assert!(
            link.ends_with(".md"),
            "docs/README.md contains non-markdown internal link: {}",
            link
        );
        let p = Path::new("docs").join(link);
        assert!(
            p.exists(),
            "docs/README.md contains dead link: {}",
            p.display()
        );
    }
}

#[test]
fn section_completeness_t002() {
    let required = [
        "getting-started",
        "usage",
        "api-reference",
        "internals",
        "development",
        "contributing",
        "migration",
    ];

    for dir in required {
        let path = docs_root().join(dir);
        assert!(path.is_dir(), "Required docs section missing: {}", path.display());

        let has_any_file = contains_any_file(&path);

        assert!(
            has_any_file,
            "Docs section must contain at least one file: {}",
            path.display()
        );
    }
}
