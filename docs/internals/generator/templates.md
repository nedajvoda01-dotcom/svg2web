<!-- [Doc] | Tera-шаблоны генератора: структура, контекст, фильтры, наследование и кэш compiled templates. -->

# Templates

## Структура директорий

```text
templates/
  base/
  react/
  vue/
  vanilla/
```

Явные директории шаблонов:

- templates/base/
- templates/react/
- templates/vue/
- templates/vanilla/

- templates/base/: общие блоки и макросы.
- templates/react|vue|vanilla/: целевые шаблоны по платформам.

## Контекст шаблонов

Данные ParseOutput преобразуются в контекст Tera в два слоя:

- Глобальные переменные: colors, fonts, настройки проекта.
- Локальные переменные: конкретный element и его атрибуты.

## Фильтры

Типовые фильтры:

- to_camel_case
- to_pascal_case
- css_value (Color -> CSS string)
- px (f64 -> "10px")

## Наследование

Используется базовый шаблон с блоками:

- head
- body
- imports

Дочерние шаблоны переопределяют только нужные блоки и наследуют общую структуру.

## Кэширование

Compiled templates хранятся в LRU-кэше, чтобы избежать повторной компиляции при многократной генерации.
