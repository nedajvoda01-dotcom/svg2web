```markdown
# Релизный процесс

## Версионирование

### Семантическое версионирование (SemVer)

Формат: `MAJOR.MINOR.PATCH`

| Тип | Пример | Условие |
|-----|--------|---------|
| **MAJOR** | 1.0.0 → 2.0.0 | Несовместимые изменения API |
| **MINOR** | 1.0.0 → 1.1.0 | Новая функциональность (обратно совместимо) |
| **PATCH** | 1.0.0 → 1.0.1 | Исправления багов |

### Начальная версия

Проект начинается с `0.1.0`:
- `0.1.x` — активная разработка, API нестабилен
- `0.2.0` — первый стабильный релиз

### Определение версии

```bash
# Текущая версия
grep '^version' Cargo.toml | head -1

# Увеличение PATCH (багфикс)
cargo set-version --bump patch

# Увеличение MINOR (новая фича)
cargo set-version --bump minor

# Увеличение MAJOR (breaking changes)
cargo set-version --bump major
```

---

## Changelog

### Формат Keep a Changelog

`CHANGELOG.md`:

```markdown
# Changelog

## [Unreleased]

### Added
- Поддержка SVG масок
- Новый флаг `--watch` для автоматической пересборки

### Changed
- Улучшена производительность парсера на 30%
- Обновлена зависимость usvg до 0.40

### Deprecated
- `--no-optimize` будет удален в 0.4.0, используйте `--optimize=false`

### Removed
- Устаревшая команда `svg2web convert`

### Fixed
- Исправлена паника при парсинге пустых текстовых узлов

### Security
- Обновлены зависимости с уязвимостями

## [0.2.0] - 2024-03-15

### Added
- Генерация React компонентов с TypeScript
- Детекция повторяющихся элементов

### Fixed
- Корректная обработка градиентов

## [0.1.0] - 2024-01-20

### Added
- Первый релиз
- Парсинг SVG в промежуточный формат
- Оптимизация и извлечение ресурсов
- CLI интерфейс
```

### Обновление Changelog перед релизом

```bash
# Переместить [Unreleased] в новую версию
./scripts/update_changelog.sh 0.2.0

# Добавить дату
sed -i 's/\[0.2.0\]/\[0.2.0\] - 2024-03-15/' CHANGELOG.md
```

---

## Cargo publish

### Проверка перед публикацией

```bash
# Проверка всех крейтов
cargo test --workspace
cargo clippy -- -D warnings
cargo fmt --check

# Сухая публикация
cargo publish --workspace --dry-run

# Проверка семантической совместимости
cargo semver-checks
```

### Публикация крейтов

```bash
# Публикация в порядке зависимостей
cd crates/svg2web-core
cargo publish

cd ../svg2web-cache
cargo publish

cd ../svg2web-generator
cargo publish

cd ../svg2web-cli
cargo publish

cd ../svg2web-wasm
cargo publish

# Или все сразу (сортировка по зависимостям)
cargo publish --workspace
```

### Проверка после публикации

```bash
# Проверка, что крейт доступен
cargo search svg2web-core

# Проверка документации
open https://docs.rs/svg2web-core
```

---

## NPM publish (WASM)

### Сборка

```bash
# Сборка WASM пакета
cd crates/svg2web-wasm
wasm-pack build --target web --release

# Проверка размера
ls -lh pkg/svg2web_bg.wasm
twiggy top pkg/svg2web_bg.wasm

# Проверка TypeScript типов
npx tsc --noEmit pkg/svg2web.d.ts
```

### Подготовка package.json

```json
{
  "name": "svg2web-wasm",
  "version": "0.2.0",
  "description": "Convert SVG to web components in the browser",
  "main": "svg2web.js",
  "types": "svg2web.d.ts",
  "files": [
    "svg2web_bg.wasm",
    "svg2web.js",
    "svg2web.d.ts",
    "README.md"
  ],
  "keywords": ["svg", "wasm", "converter", "react", "vue"],
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/org/svg2web"
  }
}
```

### Публикация

```bash
cd pkg

# Проверка
npm pack --dry-run

# Публикация
npm publish --access public

# Проверка
npm view svg2web-wasm
```

---

## GitHub Release

### Создание тега

```bash
# Создать тег
git tag -a v0.2.0 -m "Release v0.2.0"

# Отправить тег
git push origin v0.2.0
```

### Сборка бинарников

```bash
# Сборка для всех платформ
./scripts/build-binaries.sh

# Результат:
# target/release/svg2web-linux-x86_64
# target/release/svg2web-macos-x86_64
# target/release/svg2web-macos-aarch64
# target/release/svg2web-windows-x86_64.exe
```

**build-binaries.sh:**
```bash
#!/bin/bash
set -e

VERSION=$(git describe --tags)

# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/svg2web svg2web-linux-x86_64-$VERSION
tar czf svg2web-linux-x86_64-$VERSION.tar.gz svg2web-linux-x86_64-$VERSION

# macOS x86_64
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/svg2web svg2web-macos-x86_64-$VERSION
tar czf svg2web-macos-x86_64-$VERSION.tar.gz svg2web-macos-x86_64-$VERSION

# macOS ARM64
cargo build --release --target aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/svg2web svg2web-macos-aarch64-$VERSION
tar czf svg2web-macos-aarch64-$VERSION.tar.gz svg2web-macos-aarch64-$VERSION

# Windows x86_64
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/svg2web.exe svg2web-windows-x86_64-$VERSION.exe
zip svg2web-windows-x86_64-$VERSION.zip svg2web-windows-x86_64-$VERSION.exe
```

### Создание релиза через gh CLI

```bash
# Создать релиз
gh release create v0.2.0 \
  --title "v0.2.0" \
  --notes-file CHANGELOG.md \
  --draft

# Загрузить артефакты
gh release upload v0.2.0 \
  svg2web-linux-x86_64-*.tar.gz \
  svg2web-macos-*.tar.gz \
  svg2web-windows-x86_64-*.zip

# Опубликовать
gh release edit v0.2.0 --draft=false
```

---

## Docker image

### Dockerfile

```dockerfile
FROM rust:1.70 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin svg2web

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libc6 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/svg2web /usr/local/bin/
ENTRYPOINT ["svg2web"]
```

### Multi-platform сборка

```bash
# Создать и запустить buildx
docker buildx create --name multiarch --use

# Сборка для нескольких платформ
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  --tag ghcr.io/org/svg2web:0.2.0 \
  --tag ghcr.io/org/svg2web:latest \
  --push .
```

### Проверка

```bash
# Проверка образа
docker run --rm ghcr.io/org/svg2web:latest --version

# Запуск с volume
docker run --rm -v $(pwd):/data ghcr.io/org/svg2web:latest \
  build /data/input.svg --output /data/output
```

---

## GitHub Actions релиз

`.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Build binaries
        run: ./scripts/build-binaries.sh
      
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            svg2web-*.tar.gz
            svg2web-*.zip
          body_path: CHANGELOG.md
          draft: true
      
      - name: Publish to crates.io
        run: cargo publish --workspace
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
      
      - name: Publish to npm
        run: |
          cd crates/svg2web-wasm
          wasm-pack build --target web --release
          cd pkg
          npm publish --access public
        env:
          NPM_TOKEN: ${{ secrets.NPM_TOKEN }}
      
      - name: Build and push Docker image
        uses: docker/build-push-action@v4
        with:
          platforms: linux/amd64,linux/arm64
          push: true
          tags: |
            ghcr.io/org/svg2web:${{ github.ref_name }}
            ghcr.io/org/svg2web:latest
```

---

## Пост-релиз

### Обновление документации

```bash
# Обновить версию в README
sed -i 's/0.1.0/0.2.0/g' README.md

# Перегенерировать документацию
cargo doc --no-deps --workspace

# Обновить демо на Vercel
cd web && npm run build && vercel --prod
```

### Анонс

- **Twitter/X**: @svg2web
- **Discord**: #announcements
- **Reddit**: r/rust, r/webdev
- **Hacker News**: Show HN
- **Rust Users Forum**

### Мониторинг

```bash
# Проверка загрузок
cargo download-count svg2web-cli
npm stats svg2web-wasm

# Отслеживание issues
gh issue list --state open --label bug

# Настройка алертов в GitHub
# Settings → Alerts → Watch → Custom → Issues, Pull requests
```

---

## Релизный чеклист

### До релиза
- [ ] Все тесты проходят (`cargo test --workspace`)
- [ ] Clippy без ошибок (`cargo clippy -- -D warnings`)
- [ ] Форматирование (`cargo fmt --check`)
- [ ] Обновлен CHANGELOG.md
- [ ] Обновлена версия в Cargo.toml всех крейтов
- [ ] Обновлена версия в package.json
- [ ] Документация актуальна
- [ ] Проверены зависимости (`cargo audit`)

### Релиз
- [ ] Создан тег (`git tag -a vX.Y.Z`)
- [ ] Отправлен тег (`git push origin vX.Y.Z`)
- [ ] GitHub Actions успешно выполнен
- [ ] Проверены артефакты релиза
- [ ] Опубликованы крейты на crates.io
- [ ] Опубликован WASM пакет на npm
- [ ] Docker image доступен на ghcr.io

### После релиза
- [ ] Обновлена документация на docs.rs
- [ ] Обновлено демо-приложение
- [ ] Сделаны анонсы в соцсетях
- [ ] Мониторинг ошибок в первую неделю
- [ ] Созданы issue для next релиза
```