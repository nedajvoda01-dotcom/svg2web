# Contributing to SVG2Web

Thank you for your interest in contributing to SVG2Web! This document provides guidelines and steps for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/<your-username>/svg2web.git
   cd svg2web

   # Add upstream remote for syncing with main repository
   git remote add upstream https://github.com/nedajvoda01-dotcom/svg2web.git
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/my-feature
   # or
   git checkout -b fix/issue-123
   ```

## Development Workflow

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/). Use these prefixes:

- `feat:` — New feature
- `fix:` — Bug fix
- `docs:` — Documentation changes
- `test:` — Adding or updating tests
- `refactor:` — Code refactoring without functional changes
- `chore:` — Maintenance tasks, dependencies, tooling

Example:
```
feat: add Svelte plugin support

Implement FormatRenderer for Svelte framework with
script setup syntax and scoped styles.
```

### Code Quality Checks

Before submitting a PR, ensure your code passes:

```bash
# Format code
cargo fmt

# Run linter (zero warnings policy)
cargo clippy -- -D warnings

# Run all tests
cargo test --workspace

# Check WASM build
wasm-pack build crates/svg2web-wasm --target web
```

## Submitting Changes

1. **Push your branch** to your fork:
   ```bash
   git push origin feature/my-feature
   ```

2. **Create a Pull Request** to the `main` branch with:
   - Clear description of changes
   - Reference to related issues (if any)
   - Screenshots/examples for UI changes
   - Updated documentation if needed

3. **CI Requirements**:
   - All GitHub Actions jobs must pass (test matrix: Ubuntu, Windows, macOS)
   - Code coverage must not decrease significantly
   - No clippy warnings allowed

4. **Review Process**:
   - At least one maintainer approval required
   - Address review comments promptly
   - Keep PR focused and reasonably sized

## Merge Strategy

- **Features**: Squash and merge (clean history, single commit per feature)
- **Hotfixes**: Rebase and merge (preserve linear history for critical fixes)

## Code of Conduct

- **Be respectful** — Treat everyone with respect and professionalism
- **Be constructive** — Provide helpful feedback and accept it gracefully
- **Be collaborative** — Work together toward the best solution
- **Respect diversity** — Welcome contributors from all backgrounds and skill levels

## Questions?

- Open an issue for bug reports or feature requests: https://github.com/nedajvoda01-dotcom/svg2web/issues
- Join discussions in existing issues
- Contact maintainers via GitHub Discussions: https://github.com/nedajvoda01-dotcom/svg2web/discussions

---

By contributing to SVG2Web, you agree that your contributions will be licensed under the MIT License.