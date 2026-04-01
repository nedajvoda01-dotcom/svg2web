# SVG2Web

**Конвертер SVG-макетов в production-ready веб-компоненты.**  
Парсинг → Анализ → Оптимизация → Генерация кода для React, Vue, Svelte, Solid и Vanilla JS.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/org/svg2web/actions/workflows/ci.yml/badge.svg)](https://github.com/org/svg2web/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/svg2web-cli.svg)](https://crates.io/crates/svg2web-cli)
[![Docs](https://img.shields.io/badge/docs-GitHub%20Pages-green)](https://svg2web.vercel.app)

---

## ✨ Ключевые возможности

- **🎯 Точный парсинг** — полная поддержка SVG через `usvg`: градиенты, маски, трансформации, шрифты
- **🧩 Детекция компонентов** — алгоритм хеширования поддеревьев находит повторяющиеся UI-блоки (кнопки, карточки) с порогом 95% схожести
- **⚡ Оптимизация** — упрощение путей (Douglas-Peucker), конвертация изображений в WebP (1x/2x), дедупликация элементов, минификация ID
- **🎨 Генерация кода** — React (TSX), Vue (SFC), Vanilla (ES modules), Svelte и Solid (через плагины)
- **💾 Инкрементальные сборки** — кэширование на диске (Sled) и в памяти (LRU) для повторных запусков <100мс
- **🌐 Браузерный интерфейс** — WebAssembly + Web Workers, drag-drop, анализ сложности в реальном времени
- **📦 Промежуточный формат** — 6 JSON-секций (meta, structure, geometry, styles, assets, content) для частичного чтения и кэширования

---

## 🚀 Быстрый старт

### CLI (рекомендуется)
```bash
# Установка
cargo install svg2web-cli

# Полный цикл: parse → analyze → optimize → generate
svg2web build design.svg -o ./my-project --format react

# Результат в my-project/
# ├── index.html
# ├── components/
# │   ├── Button.tsx      # детектированные компоненты
# │   └── Icon.tsx
# ├── styles.css          # CSS переменные из SVG
# └── assets/
#     └── hero.webp       # оптимизированные изображения
```

### Веб-интерфейс
```bash
# Локальный запуск
git clone https://github.com/org/svg2web.git
cd svg2web/web && npm install && npm run dev

# Или используй онлайн-версию: https://svg2web.vercel.app
```

### NPM/WASM (браузер/node)
```bash
npm install svg2web-wasm
```

---

## 📦 Установка

| Способ | Команда | Комментарий |
|--------|---------|-------------|
| **Cargo** | `cargo install svg2web-cli` | Полный функционал, кэш на диске |
| **NPM** | `npm install svg2web-wasm` | Только парсинг/анализ/оптимизация (WASM) |
| **Docker** | `docker pull ghcr.io/org/svg2web` | CI/CD, изолированные сборки |
| **Из исходников** | `cargo build --release --bin svg2web` | Для разработки |

**Требования:** Rust 1.70+, для WASM — Node.js 18+.

---

## 🎮 Использование

### Команды CLI

```bash
# Только парсинг в 6 JSON-секций
svg2web parse input.svg -o ./assets --split-output

# Анализ структуры (сложность, компоненты)
svg2web analyze input.svg --json

# Оптимизация без генерации
svg2web optimize input.svg -o optimized.svg --webp-quality 85

# Генерация из промежуточного формата
svg2web generate ./assets -o ./build --format vue

# Полный цикл с прогресс-баром и кэшированием
svg2web build input.svg -o ./output --format react --config svg2web.toml
```

### Конфигурация (`svg2web.toml`)

```toml
[parse]
extract_images = true
extract_fonts = true
image_quality = "high"  # high, medium, low

[generate]
framework = "react"           # react, vue, vanilla, svelte (plugin), solid (plugin)
typescript = true
styling = "css-modules"       # native-css, tailwind, styled-components
responsive = true           # media queries по breakpoints

[optimizer]
simplify_paths = true
tolerance = 0.5             # Douglas-Peucker tolerance
deduplicate = true
minify_ids = true

[cache]
disk = true                 # Sled embedded DB
memory_size = 1000          # LRU entries
```

---

## 🏗️ Архитектура

```
SVG → [svg2web-core] → JSON (6 секций) → [Cache] → [Generator] → Output
         ↓                                    ↑
    [Parser] → [Analyzer] → [Optimizer]   [Disk: Sled]
    (usvg)      (хеши)      (Douglas-Peucker)  [Memory: LRU]
```

**5 крейтов:**

| Крейт | Назначение | Особенности |
|-------|------------|-------------|
| `svg2web-core` | Парсинг, анализ, оптимизация | `no-std` compatible, zero-copy где возможно |
| `svg2web-cache` | Инкрементальность | Sled (disk) + LRU (memory), версионирование кэша |
| `svg2web-generator` | Кодогенерация | Плагиновая система (FormatRenderer trait), Tera шаблоны |
| `svg2web-cli` | Интерфейс | clap, env_logger, progress bars |
| `svg2web-wasm` | Браузер | Web Workers, StringPool (>1MB SVG), bindgen |

**Плагины:**  
Динамическая загрузка через `cdylib` (Svelte, Solid). См. [`plugins/README.md`](./plugins/README.md).

**WASM ограничения:**
- Нет доступа к файловой системе (только drag-drop)
- Для SVG >1MB используется StringPool (ручное управление памятью)
- Все вычисления в Web Workers (не блокирует UI)

---

## 📂 Структура проекта

```
svg2web/
├── crates/
│   ├── svg2web-core/       # Ядро (парсинг, модели, оптимизация)
│   ├── svg2web-cache/      # Кэширование (Sled + LRU)
│   ├── svg2web-generator/  # Генераторы кода + плагиновая система
│   ├── svg2web-cli/        # Командная строка
│   └── svg2web-wasm/       # WebAssembly bindings
├── plugins/                # Официальные плагины (Svelte, Solid)
├── web/                    # React + Vite frontend
├── examples/               # Примеры использования
│   ├── basic/              # Vanilla HTML/CSS
│   ├── react-component/      # React + TypeScript
│   ├── vue-component/        # Vue SFC
│   ├── responsive/           # Адаптивность
│   └── advanced/             # Шрифты, внешние изображения, анимации
└── docs/                   # Полная документация
    ├── getting-started/      # Для новичков
    ├── usage/                # CLI, конфиги, фреймворки
    ├── api-reference/        # Rust API, WASM API, JSON Schema
    ├── internals/            # Архитектура, алгоритмы
    └── development/          # Тестирование, бенчмарки, релизы
```

---

## 📚 Документация

- **[Быстрый старт](docs/getting-started/quickstart.md)** — первый проект за 5 минут
- **[CLI Reference](docs/usage/cli.md)** — все команды и флаги
- **[Конфигурация](docs/usage/config.md)** — полный справочник по `svg2web.toml`
- **[JSON Schema](docs/api-reference/json-schema/)** — спецификация 6 промежуточных файлов
- **[Разработка плагинов](docs/contributing/plugin-development.md)** — создание рендереров для новых фреймворков
- **[Архитектура](docs/internals/architecture.md)** — почему 5 крейтов, почему 6 JSON файлов

---

## 🧪 Разработка

```bash
# Клонирование
git clone https://github.com/org/svg2web.git
cd svg2web

# Сборка всего workspace + WASM + web
cargo xtask build-all

# Или по частям:
cargo build --workspace
wasm-pack build crates/svg2web-wasm --target web
cd web && npm install && npm run dev

# Тестирование
cargo xtask test-all      # Rust + WASM + Web
cargo bench --workspace   # Criterion бенчмарки

# Проверка качества кода (CI строгий: deny unwrap/expect)
cargo clippy --workspace -- -D warnings
cargo fmt --check
cargo deny check
```

**Требования к коду:**
- `unwrap`/`expect` запрещены (clippy.toml: `unwrap_used = "deny"`)
- Тестовое покрытие ≥80% для нового кода
- Conventional Commits

См. [CONTRIBUTING.md](CONTRIBUTING.md) и [docs/development/](docs/development/).

---

## 📊 Производительность

Целевые метрики на CI:

| Операция | Время (10MB SVG) | Реализация |
|----------|------------------|------------|
| Парсинг | <500ms | `usvg` + zero-copy |
| Анализ | <100ms | DFS + хеширование |
| Оптимизация | <200ms | Douglas-Peucker |
| Генерация | <100ms | Tera шаблоны |

Инкрементальная сборка (кэш hit): **<100ms**.

---

## 🤝 Контрибуция

1. Форкните репозиторий
2. Создайте ветку: `git checkout -b feature/my-feature`
3. Коммиты по [Conventional Commits](https://conventionalcommits.org/): `feat:`, `fix:`, `docs:` и т.д.
4. Убедитесь что CI зелёный: `cargo xtask test-all`
5. Откройте PR

Обсуждения и баг-репорты в [Issues](https://github.com/org/svg2web/issues).

---

## 📄 Лицензия

MIT License — см. [LICENSE](LICENSE).  
Генерируемый код (React/Vue компоненты) принадлежит вам, ограничений на использование нет.

---

**Сделано с ❤️ на Rust.**  
[Демо](https://svg2web.vercel.app) • [Документация](docs/) • [Crates.io](https://crates.io/crates/svg2web-cli)