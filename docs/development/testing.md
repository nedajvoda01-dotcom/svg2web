# Тестирование

## Стратегия

Документ описывает что, где и как тестируется в svg2web.

## Unit тесты

Локации:

- crates/*/src/*.rs (inline tests через #[cfg(test)])
- crates/*/tests/

Покрытие:

- core/model структуры
- сериализация (roundtrip)

Цель:

- >= 80% покрытие для core/model.

Примеры запуска:

```bash
cargo test --workspace --lib
cargo test -p svg2web-core
```

## Интеграционные тесты

Тестовые данные:

- fixtures в tests/fixtures/ (сложные и проблемные SVG)
- golden outputs в tests/snapshots/

Визуальная регрессия:

- рендер исходного и результата через resvg
- pixel diff с допустимым порогом отклонения < 1%

## Snapshot тесты

Инструмент:

- insta для React/Vue/Vanilla выходов и parser golden files

Локации:

- tests/fixtures/ для сложных SVG
- tests/snapshots/ для golden files и parser snapshots

Команды:

```bash
# Просмотр и ревью изменений снапшотов
cargo insta review

# Принять изменения
cargo insta accept
```

Что проверяем:

- структура и стабильность сгенерированного кода
- отсутствие случайных изменений шаблонов

## E2E тесты

CLI:

- assert_cmd: запуск бинарника, проверка exit code, stdout/stderr, выходных файлов

Web:

- Playwright: drag & drop SVG, ожидание preview, проверка экспорта

## CI

GitHub Actions:

- test matrix: Rust stable/beta
- OS: Linux, macOS, Windows

Coverage:

- cargo-tarpaulin с порогом >= 80% для svg2web-core model/core путей
- upload в Codecov

Fail условия:

- покрытие core/model < 80%
- clippy warnings считаются ошибками

## Рекомендуемый пайплайн

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test --workspace
cargo tarpaulin --workspace --packages svg2web-core --out Xml --fail-under 80
```
