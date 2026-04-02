<!-- [Doc] | Входная точка документации. Карта разделов: быстрые ссылки на getting-started для новичков, usage для ежедневной работы, api-reference для интеграторов, internals для контрибьюторов. Поиск по документации, версионирование, как сообщить об ошибке в доках -->

# Документация svg2web

svg2web — это инструмент для конвертации SVG в компоненты React, Vue или HTML.

---

## 👶 Новичок

- [Быстрый старт](getting-started/quickstart.md)
- [Установка](getting-started/installation.md)
- [Первый проект](getting-started/first-project.md)

## 🛠️ Пользователь

- [CLI](usage/cli.md)
- [Конфигурация](usage/config.md)
- [Фреймворки](usage/frameworks/react.md)

## 🔌 Интегратор

- [Rust API](api-reference/rust/core.md)
- [WASM/JS API](api-reference/wasm-js/index.md)
- [JSON Schema](api-reference/json-schema/overview.md)

## 🔧 Разработчик

- [Архитектура](internals/architecture.md)
- [Контрибуция](contributing/guidelines.md)
- [Тестирование](development/testing.md)

---

## Версионирование

- ![Stable](https://img.shields.io/badge/version-stable-brightgreen)
- ![Nightly](https://img.shields.io/badge/version-nightly-orange)

[Changelog](migration/changelog.md)

---

## Поиск

Для поиска используйте Ctrl+F или оглавление.

---

## Контрибьютинг

Нашли ошибку? [Сообщите об этом](https://github.com/nedajvoda01-dotcom/svg2web/issues/new?template=documentation.md).

---

## Архитектура

svg2web преобразует SVG в JSON, а затем в код (React/Vue/HTML).
