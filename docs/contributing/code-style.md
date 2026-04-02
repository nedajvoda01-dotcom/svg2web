# Стиль кода

## rustfmt

Конфигурация:

```toml
line_width = 100
tab_spaces = 4
reorder_imports = true
group_imports = "StdExternalCrate"
```

Проверка:

```bash
cargo fmt --check
```

CI считает конфигурацию корректной только если `cargo fmt --check` использует `line_width = 100` и `tab_spaces = 4` из корневого rustfmt.toml.

## Clippy

Политика:

- Deny: unwrap_used, expect_used (в production коде)
- Allow: too_many_arguments (до 7)
- cognitive_complexity_threshold = 30
- zero warnings policy

Обязательные deny-флаги в CI:

```bash
cargo clippy -- -D warnings -D clippy::unwrap_used -D clippy::expect_used
```

## Обработка ошибок

- core/generator: thiserror (типизированные ошибки с контекстом)
- cli: anyhow (только верхнеуровневый слой)
- unwrap() запрещен в библиотечном коде
- unwrap() разрешен только в тестах и main.rs

## Документация

Требования:

- rustdoc для всех pub API
- секция # Examples с запускаемыми примерами
- README.md для каждого крейта с кратким описанием

Проверка примеров:

```bash
cargo test --doc
```

## Hard no

- println! в библиотеках (используйте tracing)
- хардкод путей (используйте std::env или аргументы)
- unsafe без explicit approve в PR
