# Deprecations

## Таблица устареваний

| Фича | Версия устаревания | Версия удаления | Замена | Migration Path |
|---|---|---|---|---|
| component_detection поле | 0.2.0 | 0.4.0 | components.detect | sed-скрипт для .toml |
| analyze команда | 0.3.0 | 0.5.0 | parse --analyze | обновить CI-скрипты |

## Предупреждения

Как включить:

- для CLI: обычный запуск с stderr
- для Rust API: компиляция с включенными warning

Пример warning в консоли:

```text
warning: `component_detection` is deprecated since 0.2.0 and will be removed in 0.4.0
help: use `components.detect`
```

Дополнительный пример:

```text
warning: `analyze` is deprecated and will be removed in 0.5.0
help: use `parse --analyze`
```

## Автоматическая миграция

- cargo fix: для Rust API изменений (если применимо)
- IDE auto-replace: для простых переименований полей/флагов

Пример:

```bash
cargo fix --workspace
```

## Политика

- Deprecation минимум на 2 минорные версии.
- Обратная совместимость сохраняется в пределах SemVer до удаления функции.
