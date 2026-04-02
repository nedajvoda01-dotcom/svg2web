# Установка

Цель: выбрать подходящий способ установки под вашу среду и ограничения.

## Сравнение способов

| Способ | Требования | Команда | Ограничения |
|---|---|---|---|
| Cargo (рекомендуется) | Rust 1.70+, cargo | cargo install svg2web-cli --locked | Нет ограничений по функциям |
| NPM/WASM | Node.js 18+ | npm install svg2web-wasm | Только parse/analyze/optimize, нет generate, practical limit: SVG < 50MB |
| Docker | Docker | docker pull ghcr.io/org/svg2web:latest | Нужен volume mount для входных/выходных файлов |

## Cargo (полный функционал)

```bash
cargo install svg2web-cli --locked
```

Проверка:

```bash
svg2web --version
# expected: svg2web 0.x.x
```

## NPM (WASM)

```bash
npm install svg2web-wasm
```

Минимальный запуск:

```javascript
import init from 'svg2web-wasm';
await init();
```

Ограничения:

- Нет команды generate (только parse/analyze/optimize).
- Ограничение памяти браузера, для стабильной работы рекомендуется SVG < 50MB.

## Docker

```bash
docker pull ghcr.io/org/svg2web:latest
```

Запуск с volume:

```bash
docker run --rm -v $(pwd):/workdir ghcr.io/org/svg2web:latest \
  build /workdir/input.svg --output /workdir/output
```

Когда использовать:

- CI/CD
- воспроизводимые сборки в изолированной среде

## Из исходников (для разработки)

См. подробности в [development/setup.md](../development/setup.md).

## Фичи по платформам

| Фича | Cargo | NPM/WASM | Docker |
|---|---|---|---|
| Parse | ✅ | ✅ | ✅ |
| Generate | ✅ | ❌ | ✅ |
| Cache | ✅ | ❌ (memory only) | ✅ |
