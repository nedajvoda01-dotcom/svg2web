```markdown
# Настройка окружения

## Системные требования

| Компонент | Минимальная версия | Рекомендуемая версия |
|-----------|-------------------|---------------------|
| Rust | 1.70 | 1.75+ |
| Node.js | 18 | 20 LTS |
| npm | 9 | 10 |
| wasm-pack | 0.12 | latest |
| Git | 2.30 | latest |

---

## Клонирование репозитория

```bash
# Клонирование
git clone https://github.com/org/svg2web.git
cd svg2web

# Проверка подмодулей (если есть)
git submodule update --init --recursive
```

---

## Установка Rust toolchain

### Установка rustup

```bash
# Linux / macOS / WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Скачать и запустить rustup-init.exe с https://rustup.rs
```

### Установка версии

```bash
# Установить стабильную версию
rustup install stable

# Установить nightly (для некоторых фич)
rustup install nightly

# Установить WASM target
rustup target add wasm32-unknown-unknown

# Проверка
rustc --version  # rustc 1.75.0
cargo --version  # cargo 1.75.0
```

### Дополнительные инструменты

```bash
# Установка wasm-pack
cargo install wasm-pack

# Установка cargo-watch для разработки
cargo install cargo-watch

# Установка cargo-expand для макросов
cargo install cargo-expand

# Установка cargo-tarpaulin для покрытия кода
cargo install cargo-tarpaulin

# Установка cargo-audit для проверки безопасности
cargo install cargo-audit
```

---

## Установка Node.js

### Linux (Ubuntu/Debian)

```bash
# Через nvm (рекомендуется)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20

# Или через apt
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs
```

### macOS

```bash
# Через Homebrew
brew install node@20

# Через nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
```

### Windows

```bash
# Скачать с https://nodejs.org/
# Или через winget
winget install OpenJS.NodeJS.LTS
```

### Проверка

```bash
node --version  # v20.11.0
npm --version   # 10.2.4
```

---

## Сборка проекта

### Первичная сборка

```bash
# Сборка всех крейтов
cargo build --workspace

# Сборка WASM
wasm-pack build crates/svg2web-wasm --target web

# Установка зависимостей web интерфейса
cd web
npm install
cd ..
```

### Режимы сборки

```bash
# Debug сборка (быстрая)
cargo build

# Release сборка (оптимизированная)
cargo build --release

# Сборка с флагами для отладки WASM
wasm-pack build crates/svg2web-wasm --target web --debug

# Сборка с оптимизацией размера
wasm-pack build crates/svg2web-wasm --target web --release
```

---

## Запуск

### CLI

```bash
# Из исходников
cargo run --bin svg2web -- --help

# Сборка и запуск
cargo build --release --bin svg2web
./target/release/svg2web build input.svg --output ./output --format react
```

### Web интерфейс

```bash
cd web

# Запуск dev сервера
npm run dev

# Сборка для production
npm run build

# Предпросмотр сборки
npm run preview
```

---

## Проверки

### Тесты

```bash
# Все тесты
cargo test --workspace

# Тесты конкретного крейта
cargo test -p svg2web-core

# Тесты с выводом
cargo test -- --nocapture

# WASM тесты
wasm-pack test crates/svg2web-wasm --headless --chrome

# Web тесты
cd web && npm test
```

### Форматирование

```bash
# Проверка
cargo fmt --check

# Автоисправление
cargo fmt
```

### Линтинг

```bash
# Проверка
cargo clippy -- -D warnings

# Автоисправление некоторых проблем
cargo clippy --fix --allow-dirty
```

### Проверка зависимостей

```bash
# Аудит безопасности
cargo audit

# Неиспользуемые зависимости
cargo udeps

# Проверка лицензий
cargo deny check
```

---

## IDE настройка

### Visual Studio Code

**Расширения:**

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "tamasfe.even-better-toml",
    "serayuzgur.crates",
    "vadimcn.vscode-lldb",
    "bradlc.vscode-tailwindcss",
    "esbenp.prettier-vscode"
  ]
}
```

**Настройки `.vscode/settings.json`:**

```json
{
  "rust-analyzer.cargo.target": "wasm32-unknown-unknown",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
  "rust-analyzer.procMacro.enable": true,
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "[toml]": {
    "editor.defaultFormatter": "tamasfe.even-better-toml"
  }
}
```

**launch.json для отладки:**

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug CLI",
      "program": "${workspaceFolder}/target/debug/svg2web",
      "args": ["build", "input.svg", "--format", "react"],
      "cwd": "${workspaceFolder}"
    },
    {
      "type": "chrome",
      "request": "launch",
      "name": "Debug Web",
      "url": "http://localhost:5173",
      "webRoot": "${workspaceFolder}/web",
      "sourceMapPathOverrides": {
        "webpack:///./src/*": "${webRoot}/src/*"
      }
    },
    {
      "type": "lldb",
      "request": "attach",
      "name": "Attach to WASM",
      "pid": "${command:pickProcess}"
    }
  ]
}
```

### IntelliJ / CLion

1. Установить Rust plugin
2. Настройки → Languages & Frameworks → Rust
3. Добавить target: `wasm32-unknown-unknown`
4. Настроить внешние инструменты для `wasm-pack`

---

## Переменные окружения

### Для разработки

```bash
# Увеличить уровень логирования
export RUST_LOG=debug

# Включить backtrace
export RUST_BACKTRACE=1

# Использовать sparse протокол для быстрой загрузки
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Путь к кэшу
export SVG2WEB_CACHE_DIR=~/.cache/svg2web-dev
```

### Для WASM

```bash
# Включить консольный паник хук
export RUSTFLAGS="--cfg console_error_panic_hook"

# Увеличить размер стека
export RUSTFLAGS="-C link-arg=-zstack-size=8388608"
```

---

## Docker окружение

### Development контейнер

```dockerfile
# Dockerfile.dev
FROM rust:1.75

RUN apt-get update && apt-get install -y \
    nodejs \
    npm \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install wasm-pack

WORKDIR /workspace
```

### Использование

```bash
# Сборка образа
docker build -f Dockerfile.dev -t svg2web-dev .

# Запуск с volume
docker run -it --rm \
  -v $(pwd):/workspace \
  -p 5173:5173 \
  svg2web-dev bash
```

---

## Устранение проблем

### Ошибка: linker `cc` not found

```bash
# Ubuntu/Debian
sudo apt install build-essential

# macOS
xcode-select --install

# Fedora
sudo dnf install gcc
```

### Ошибка: failed to run custom build command for `openssl-sys`

```bash
# Ubuntu/Debian
sudo apt install pkg-config libssl-dev

# macOS
brew install openssl
export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl/lib/pkgconfig"

# Fedora
sudo dnf install openssl-devel
```

### Ошибка: `wasm32-unknown-unknown` target not found

```bash
rustup target add wasm32-unknown-unknown
```

### Ошибка: `wasm-pack` command not found

```bash
cargo install wasm-pack

# Или через npm
npm install -g wasm-pack
```

### Ошибка: `npm` permission denied

```bash
# Не используйте sudo, настройте права
mkdir ~/.npm-global
npm config set prefix '~/.npm-global'
echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc
source ~/.bashrc
```

---

## Быстрый старт

```bash
# 1. Клонирование
git clone https://github.com/org/svg2web.git
cd svg2web

# 2. Установка зависимостей
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# 3. Сборка
cargo build --workspace
wasm-pack build crates/svg2web-wasm --target web

# 4. Установка web зависимостей
cd web && npm install && cd ..

# 5. Запуск тестов
cargo test --workspace

# 6. Запуск CLI
cargo run --bin svg2web -- --help

# 7. Запуск web интерфейса
cd web && npm run dev
```
```