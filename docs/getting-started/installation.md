```markdown
# Установка

## Способ 1: Cargo (рекомендуется)

Установка CLI инструмента через Rust package manager.

### Системные требования

- Rust 1.70 или новее
- Cargo (устанавливается вместе с Rust)
- Git

### Установка Rust

```bash
# Linux / macOS / WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Скачать rustup-init.exe с https://rustup.rs
```

### Установка svg2web-cli

```bash
cargo install svg2web-cli
```

### Обновление

```bash
cargo install --force svg2web-cli
```

### Проверка

```bash
svg2web --version
# svg2web 0.2.0
```

---

## Способ 2: Docker

### Pull образа

```bash
docker pull ghcr.io/org/svg2web:latest
```

### Запуск

```bash
# Базовый запуск
docker run --rm ghcr.io/org/svg2web:latest --version

# С volume для файлов
docker run --rm -v $(pwd):/data ghcr.io/org/svg2web:latest \
  build /data/input.svg --output /data/output --format react

# Алиас для удобства
alias svg2web='docker run --rm -v $(pwd):/data ghcr.io/org/svg2web:latest'
```

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'
services:
  svg2web:
    image: ghcr.io/org/svg2web:latest
    volumes:
      - ./:/data
    working_dir: /data
    command: --help
```

```bash
docker-compose run --rm svg2web build input.svg --output output
```

---

## Способ 3: NPM (WASM)

Установка WASM пакета для использования в браузере.

### Установка

```bash
npm install svg2web-wasm
```

### Использование

```javascript
import init, { parse_svg, analyze_svg, optimize_svg } from 'svg2web-wasm';

// Инициализация
await init();

// Использование
const svg = '<svg xmlns="http://www.w3.org/2000/svg"><circle cx="50" cy="50" r="40"/></svg>';
const parsed = parse_svg(svg);
console.log(parsed.meta.canvas);
```

### Ограничения

WASM пакет предоставляет **только**:
- `parse_svg` — парсинг SVG
- `analyze_svg` — анализ структуры
- `optimize_svg` — оптимизация
- `extract_assets` — извлечение ресурсов
- `serialize_to_json` — сериализация

**Нет** функции `generate` — для генерации кода используйте CLI.

---

## Способ 4: Сборка из исходников

### Клонирование

```bash
git clone https://github.com/org/svg2web.git
cd svg2web
```

### Сборка

```bash
# Сборка CLI
cargo build --release --bin svg2web

# Копирование в PATH
cp target/release/svg2web ~/.cargo/bin/
```

### Сборка WASM

```bash
wasm-pack build crates/svg2web-wasm --target web --release
# Результат: crates/svg2web-wasm/pkg/
```

---

## Проверка установки

### CLI

```bash
# Версия
svg2web --version

# Справка
svg2web --help

# Проверка парсинга
svg2web parse test.svg --output test.json
```

### Docker

```bash
docker run --rm ghcr.io/org/svg2web:latest --version
```

### NPM

```javascript
import init, { parse_svg } from 'svg2web-wasm';

await init();
const result = parse_svg('<svg/>');
console.log('WASM работает!');
```

---

## Системные зависимости

### Linux (Ubuntu/Debian)

```bash
# Для сборки из исходников
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libfontconfig1-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxcb1-dev
```

### Linux (Fedora)

```bash
sudo dnf install \
    gcc \
    pkg-config \
    openssl-devel \
    fontconfig-devel \
    libxcb-devel
```

### macOS

```bash
# Установка Xcode Command Line Tools
xcode-select --install

# Установка openssl
brew install openssl
export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl/lib/pkgconfig"
```

### Windows

Установите:
- [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
- [Git for Windows](https://git-scm.com/download/win)

---

## Установка дополнительных инструментов

```bash
# Для разработки WASM
cargo install wasm-pack

# Для бенчмарков
cargo install cargo-criterion

# Для проверки покрытия
cargo install cargo-tarpaulin

# Для аудита зависимостей
cargo install cargo-audit
```

---

## Удаление

### Cargo

```bash
cargo uninstall svg2web-cli
```

### Docker

```bash
docker rmi ghcr.io/org/svg2web:latest
```

### NPM

```bash
npm uninstall svg2web-wasm
```

### Из исходников

```bash
rm -rf svg2web
rm ~/.cargo/bin/svg2web
```

---

## Следующие шаги

- [Быстрый старт](/getting-started/quickstart.md) — первый проект за 5 минут
- [Полный пример](/getting-started/first-project.md) — от SVG до деплоя
- [Конфигурация](/usage/config.md) — настройка под свои нужды
```