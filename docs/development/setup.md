# Настройка окружения

## Цель

Onboarding для нового разработчика: подготовить toolchain, собрать проект, запустить проверки и настроить IDE.

## 1) Клонирование

```bash
git clone https://github.com/org/svg2web.git
cd svg2web

# Если в проекте есть submodules
git submodule update --init --recursive
```

## 2) Toolchain

### Rust 1.70+

```bash
rustup show active-toolchain
rustc --version
```

### Node 18+

```bash
node --version
npm --version
```

### wasm-pack

```bash
cargo install wasm-pack
wasm-pack --version
```

## 3) Сборка

### Debug

```bash
cargo build --workspace
```

### Release

```bash
cargo build --workspace --release
```

### WASM

```bash
wasm-pack build crates/svg2web-wasm --target web
```

### Web UI

```bash
cd web
npm install
npm run dev
```

## 4) Проверка качества

```bash
# Tests
cargo test --workspace

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt --check

# WASM tests
wasm-pack test --headless
```

## 5) IDE setup

### VS Code extensions

- rust-analyzer
- Even Better TOML
- ESLint

### Пример settings.json

```json
{
  "editor.formatOnSave": true,
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "[toml]": {
    "editor.defaultFormatter": "tamasfe.even-better-toml"
  },
  "[javascript]": {
    "editor.defaultFormatter": "dbaeumer.vscode-eslint"
  },
  "[typescript]": {
    "editor.defaultFormatter": "dbaeumer.vscode-eslint"
  }
}
```

### Пример launch.json (Rust + WASM)

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug svg2web-cli",
      "type": "lldb",
      "request": "launch",
      "cargo": {
        "args": ["build", "--bin", "svg2web"],
        "filter": {
          "name": "svg2web",
          "kind": "bin"
        }
      },
      "args": ["build", "input.svg", "--output", "./output", "--framework", "react"],
      "cwd": "${workspaceFolder}"
    },
    {
      "name": "Debug WASM Web",
      "type": "pwa-chrome",
      "request": "launch",
      "url": "http://localhost:5173",
      "webRoot": "${workspaceFolder}/web"
    }
  ]
}
```
