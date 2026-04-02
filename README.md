# SVG2Web

Конвертер SVG в веб-компоненты. Превращайте SVG-файлы из Figma/Illustrator в готовые React, Vue или Vanilla JS компоненты с оптимизацией и извлечением ресурсов.

[![CI](https://github.com/nedajvoda01-dotcom/svg2web/actions/workflows/ci.yml/badge.svg)](https://github.com/nedajvoda01-dotcom/svg2web/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/svg2web-cli)](https://crates.io/crates/svg2web-cli)
[![npm](https://img.shields.io/npm/v/svg2web-wasm)](https://www.npmjs.com/package/svg2web-wasm)

## Быстрый старт

```bash
# Установка CLI
cargo install svg2web-cli

# Конвертация SVG в React-компонент
svg2web build input.svg --framework react

# Или в Vue SFC
svg2web build input.svg --framework vue --output ./components
```

**Онлайн-демо**: [svg2web.vercel.app](https://svg2web.vercel.app)

## Фичи

- **Парсинг SVG** — полная поддержка спецификации через `usvg`, обработка сложных градиентов, маски и текстов
- **Анализ структуры** — автоматическая детекция повторяющихся компонентов, метрики сложности, критические пути рендеринга
- **Оптимизация** — упрощение путей (Douglas-Peucker), дедупликация элементов, минификация ID, конвертация в WebP
- **Генерация кода** — React (TSX + hooks), Vue 3 (SFC + script setup), Vanilla JS (ES modules/Web Components)
- **Извлечение ресурсов** — автоматический вынос base64-изображений, детекция шрифтов, загрузка внешних ресурсов
- **Кэширование** — инкрементальные сборки с Sled (диск) и LRU (память), версионирование по хешу содержимого

## Архитектура

Модульное ядро (`svg2web-core`) без side-effects → Trait-based генератор (`svg2web-generator`) с поддержкой плагинов → WASM-биндинги для браузера (`svg2web-wasm`) с Web Workers.

```
SVG → [Parser] → [Analyzer] → [Optimizer] → [Serializer] → [Cache] → [Generator] → React/Vue/Vanilla
```

## Установка

### Cargo (рекомендуется)
```bash
cargo install svg2web-cli
svg2web --version
```

### NPM (WASM-версия, browser-only)
```bash
npm install svg2web-wasm
# Поддерживает parse/analyze/optimize, генерация только в Node/CLI версии
```

### Docker
```bash
docker pull ghcr.io/org/svg2web:latest
docker run -v $(pwd):/work ghcr.io/org/svg2web build /work/input.svg --framework react
```

## Использование

### CLI команды
- `svg2web parse input.svg --output model.json` — парсинг в промежуточный формат
- `svg2web generate model.json --framework react` — генерация кода из JSON
- `svg2web build input.svg --framework vue --config svg2web.toml` — полный pipeline
- `svg2web optimize input.svg --output optimized.svg` — только оптимизация

### Конфигурация (`svg2web.toml`)
```toml
[generate]
framework = "react"
styling = "tailwind"
responsive = true

[generate.components]
detect = true
min_size = 3
naming = "pascal"

[optimizer]
simplify_paths = true
deduplicate = true

[extractor]
extract_images = true
convert_to_webp = true
webp_quality = 85
```

## Ссылки

- **Демо**: [svg2web.vercel.app](https://svg2web.vercel.app)
- **Документация**: [docs/README.md](docs/README.md)
- **Crates.io**: [crates.io/crates/svg2web-cli](https://crates.io/crates/svg2web-cli)
- **NPM**: [npmjs.com/package/svg2web-wasm](https://www.npmjs.com/package/svg2web-wasm)

## Лицензия

MIT License — см. [LICENSE](LICENSE)