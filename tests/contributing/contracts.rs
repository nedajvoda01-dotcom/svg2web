use std::path::Path;
use std::process::Command;

fn has_cmd(name: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {} >/dev/null 2>&1", name))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[test]
fn conventional_commits_regex_t106() {
    let docs = std::fs::read_to_string("docs/contributing/guidelines.md").unwrap_or_default();
    assert!(docs.contains("^(feat|fix|docs|test|refactor|chore)(\\(.+\\))?: .+"));

    for valid in [
        "feat(parser): add svg normalization",
        "fix: handle empty gradients",
        "docs(api): clarify cache behavior",
        "test(core): add parser snapshots",
        "refactor(generator): simplify registry flow",
        "chore: update fixtures",
    ] {
        let status = Command::new("sh")
            .arg("scripts/ci/check-commit-message.sh")
            .arg(valid)
            .status()
            .expect("failed to run commit message check");
        assert!(status.success(), "valid commit message rejected: {valid}");
    }

    let invalid = Command::new("sh")
        .arg("scripts/ci/check-commit-message.sh")
        .arg("bad commit message")
        .status()
        .expect("failed to run invalid commit message check");
    assert!(!invalid.success(), "invalid commit message must be rejected");
}

#[test]
fn branch_naming_convention_t107() {
    let docs = std::fs::read_to_string("docs/contributing/guidelines.md").unwrap_or_default();
    assert!(docs.contains("^(feature|fix|docs)\\/.+"));

    for branch in ["feature/react-props-cleanup", "fix/issue-123", "docs/typo-fix"] {
        let status = Command::new("sh")
            .arg("scripts/ci/check-branch-name.sh")
            .arg(branch)
            .status()
            .expect("failed to run branch naming check");
        assert!(status.success(), "valid branch name rejected: {branch}");
    }

    let invalid = Command::new("sh")
        .arg("scripts/ci/check-branch-name.sh")
        .arg("hotfix/bad-name")
        .status()
        .expect("failed to run invalid branch naming check");
    assert!(!invalid.success(), "invalid branch name must be rejected");
}

#[test]
fn rustfmt_config_compliance_t108() {
    let docs = std::fs::read_to_string("docs/contributing/code-style.md").unwrap_or_default();
    let rustfmt = std::fs::read_to_string("rustfmt.toml").unwrap_or_default();
    assert!(docs.contains("line_width = 100"));
    assert!(docs.contains("tab_spaces = 4"));
    assert!(rustfmt.contains("line_width=100"));
    assert!(rustfmt.contains("tab_spaces=4"));

    if has_cmd("cargo") {
        let status = Command::new("cargo")
            .args(["fmt", "--check"])
            .status()
            .expect("failed to start cargo fmt --check");
        assert!(status.success(), "cargo fmt --check must succeed");
    }
}

#[test]
fn clippy_deny_rules_t109() {
    let docs = std::fs::read_to_string("docs/contributing/code-style.md").unwrap_or_default();
    let cargo = std::fs::read_to_string("Cargo.toml").unwrap_or_default();
    assert!(docs.contains("-D clippy::unwrap_used"));
    assert!(docs.contains("-D clippy::expect_used"));
    assert!(cargo.contains("unwrap_used=\"deny\""));
    assert!(cargo.contains("expect_used=\"deny\""));
}

#[test]
fn documentation_examples_runnable_t110() {
    let docs = std::fs::read_to_string("docs/contributing/code-style.md").unwrap_or_default();
    assert!(docs.contains("cargo test --doc"));
    assert!(docs.contains("# Examples"));

    if has_cmd("cargo") {
        let status = Command::new("cargo")
            .args(["test", "--doc"])
            .status()
            .expect("failed to start cargo test --doc");
        assert!(status.success(), "cargo test --doc must succeed");
    }
}

#[test]
fn adr_implementation_compliance_t111() {
    let docs = std::fs::read_to_string("docs/contributing/architecture-decisions.md").unwrap_or_default();
    let parser = std::fs::read_to_string("crates/svg2web-core/src/parser/svg.rs").unwrap_or_default();
    assert!(docs.contains("основной парсер: usvg"));
    assert!(docs.contains("не roxmltree напрямую") || docs.contains("не используется как прямой production parser API"));
    assert!(parser.contains("usvg::Tree"), "parser must document usvg usage");
    assert!(Path::new("crates/svg2web-generator/src/formats/tera_adapter.rs").exists());
    assert!(!Path::new("crates/svg2web-generator/src/formats/handlebars.rs").exists());
    for file in [
        "docs/api-reference/json-schema/meta.md",
        "docs/api-reference/json-schema/structure.md",
        "docs/api-reference/json-schema/geometry.md",
        "docs/api-reference/json-schema/styles.md",
        "docs/api-reference/json-schema/assets.md",
        "docs/api-reference/json-schema/content.md",
    ] {
        assert!(Path::new(file).exists(), "missing split-output section doc: {file}");
    }
}
