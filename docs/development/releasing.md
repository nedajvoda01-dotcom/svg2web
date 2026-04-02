# Релизы

## Версионирование

Используется SemVer: MAJOR.MINOR.PATCH.

Версия в Cargo.toml и git tag релиза должны совпадать и соответствовать шаблону `MAJOR.MINOR.PATCH`.

Стартовая линия проекта: 0.1.0.

## Changelog

Формат: Keep a Changelog.

Обязательные секции для каждого релиза:

- Added
- Changed
- Deprecated
- Removed
- Fixed
- Security

## Cargo publish

### Предпубликационная проверка

```bash
cargo test --workspace
cargo clippy -- -D warnings
cargo fmt --check
cargo publish --workspace --dry-run
cargo semver-checks
```

### Порядок публикации

Из-за зависимостей крейты публикуются в порядке:

1. core
2. cache
3. generator
4. cli
5. wasm

## NPM publish (WASM)

```bash
wasm-pack build crates/svg2web-wasm --target web
npm publish crates/svg2web-wasm/pkg
```

## GitHub Release

Создание релиза через gh CLI:

```bash
gh release create v0.1.0 \
  --title "v0.1.0" \
  --notes-file CHANGELOG.md
```

Прикрепляем бинарники:

- Linux x64
- macOS x64
- macOS arm64
- Windows x64

Проверка релиза считается успешной только если опубликованы бинарники для Linux, Mac и Windows.

Release notes формируются из CHANGELOG.

## Docker

Multi-platform сборка и публикация в ghcr.io:

```bash
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  --tag ghcr.io/org/svg2web:latest \
  --push .
```

Multi-platform Docker release обязан включать обе архитектуры: `linux/amd64` и `linux/arm64`.

## Пост-релиз

- обновить версию в документации и README
- подготовить анонс (блог/соцсети)
- мониторить issues на регрессии после релиза
