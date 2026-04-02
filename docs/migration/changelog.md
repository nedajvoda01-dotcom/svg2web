# Changelog

Формат: Keep a Changelog.
Версионирование: Semantic Versioning.

## [Unreleased]

### Added
- Добавлен экспорт метрик анализа в JSON ([#301](https://github.com/nedajvoda01-dotcom/svg2web/pull/301), [#298](https://github.com/nedajvoda01-dotcom/svg2web/issues/298))

### Changed
- Улучшена стабильность генерации Vue-шаблонов ([#304](https://github.com/nedajvoda01-dotcom/svg2web/pull/304), [#300](https://github.com/nedajvoda01-dotcom/svg2web/issues/300))

### Deprecated
- Флаг --no-optimize помечен устаревшим, используйте --optimize=false ([#305](https://github.com/nedajvoda01-dotcom/svg2web/pull/305), [#289](https://github.com/nedajvoda01-dotcom/svg2web/issues/289))

### Removed
- Удалена устаревшая команда convert ([#306](https://github.com/nedajvoda01-dotcom/svg2web/pull/306), [#287](https://github.com/nedajvoda01-dotcom/svg2web/issues/287))

### Fixed
- Исправлена ошибка обработки вложенных group transforms ([#307](https://github.com/nedajvoda01-dotcom/svg2web/pull/307), [#292](https://github.com/nedajvoda01-dotcom/svg2web/issues/292))

### Security
- Обновлены зависимости с security-патчами ([#308](https://github.com/nedajvoda01-dotcom/svg2web/pull/308), [#294](https://github.com/nedajvoda01-dotcom/svg2web/issues/294))

## [0.2.0] - 2024-02-15

### Added
- Генерация React/Vue/Vanilla форматов ([#210](https://github.com/nedajvoda01-dotcom/svg2web/pull/210), [#182](https://github.com/nedajvoda01-dotcom/svg2web/issues/182))

### Changed
- Обновлен пайплайн оптимизации SVG ([#214](https://github.com/nedajvoda01-dotcom/svg2web/pull/214), [#190](https://github.com/nedajvoda01-dotcom/svg2web/issues/190))

### Deprecated
- Детали депрекаций и breaking changes вынесены в [v0.1-to-v0.2.md](v0.1-to-v0.2.md)

### Removed
- Сводка удалений для миграции вынесена в [v0.1-to-v0.2.md](v0.1-to-v0.2.md)

### Fixed
- Исправлена сериализация градиентов ([#220](https://github.com/nedajvoda01-dotcom/svg2web/pull/220), [#197](https://github.com/nedajvoda01-dotcom/svg2web/issues/197))

### Security
- Закрыты предупреждения по транзитивным зависимостям ([#222](https://github.com/nedajvoda01-dotcom/svg2web/pull/222), [#199](https://github.com/nedajvoda01-dotcom/svg2web/issues/199))

[Unreleased]: https://github.com/nedajvoda01-dotcom/svg2web/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/nedajvoda01-dotcom/svg2web/compare/v0.1.0...v0.2.0
