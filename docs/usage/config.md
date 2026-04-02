<!-- [Doc] | Полная спецификация svg2web.toml: секции parse/generate/generate.components/optimizer/extractor, ENV overrides, валидация и готовые примеры конфигов. -->

# Конфигурация

## Общая структура toml

Файл поддерживает 5 секций:

- [parse]
- [generate]
- [generate.components]
- [optimizer]
- [extractor]

Пример пустого конфига:

```toml
[parse]

[generate]

[generate.components]

[optimizer]

[extractor]
```

## Секция [parse]

- extract_images (bool, default: true) — извлекать ли base64-картинки.
- image_quality (int 0-100, default: 85) — качество WebP.
- resolve_external (bool, default: false) — загружать ли внешние URL-ресурсы (шрифты, изображения).

Пример:

```toml
[parse]
extract_images = true
image_quality = 85
resolve_external = false
```

Ошибка валидации при quality = 150:

```text
Configuration error at [parse].image_quality: expected 0..100, got 150
```

## Секция [generate]

- framework (enum: "react", "vue", "vanilla", "html", required)
- styling (enum: "native", "tailwind", "scoped", default: "native")
- responsive (bool, default: true) — генерировать media queries.
- image_quality (enum: "low", "medium", "high", "original")

Пример:

```toml
[generate]
framework = "react"
styling = "tailwind"
responsive = true
image_quality = "high"
```

## Секция [generate.components]

- detect (bool, default: true) — авто-детекция переиспользуемых компонентов.
- min_size (int, default: 2) — минимум использований для выделения в компонент.
- naming (string template, default: "{id}") — шаблон имени компонента.

Пример:

```toml
[generate.components]
detect = true
min_size = 2
naming = "{id}"
```

## Секция [optimizer]

- simplify_paths.enabled (bool, default: true)
- simplify_paths.tolerance (float, default: 0.5)
- deduplicate.enabled (bool, default: true)
- deduplicate.threshold (float, default: 0.95)
- minify (bool, default: true)
- remove_comments (bool, default: true)
- precision (int, default: 2) — знаков после запятой в координатах.

Пример:

```toml
[optimizer]
minify = true
remove_comments = true
precision = 2

[optimizer.simplify_paths]
enabled = true
tolerance = 0.5

[optimizer.deduplicate]
enabled = true
threshold = 0.95
```

## Секция [extractor]

- fonts_dir (string, optional) — куда сохранять шрифты.
- images_dir (string, optional) — куда сохранять картинки.

Пример:

```toml
[extractor]
fonts_dir = "./output/assets/fonts"
images_dir = "./output/assets/images"
```

## ENV переменные

- SVG2WEB_FRAMEWORK — переопределяет generate.framework.
- SVG2WEB_IMAGE_QUALITY — переопределяет parse.image_quality.

Приоритет значений:

1. ENV
2. config file
3. defaults

Пример:

```bash
export SVG2WEB_FRAMEWORK=vue
export SVG2WEB_IMAGE_QUALITY=78
svg2web build input.svg --config ./svg2web.toml
```

## Примеры конфигов

### 1) Минимальный (только framework)

```toml
[generate]
framework = "react"
```

### 2) React + Tailwind + авто-детекция компонентов

```toml
[parse]
extract_images = true
image_quality = 85
resolve_external = false

[generate]
framework = "react"
styling = "tailwind"
responsive = true
image_quality = "high"

[generate.components]
detect = true
min_size = 2
naming = "{id}"
```

### 3) Production (оптимизации, WebP, кастомные пути)

```toml
[parse]
extract_images = true
image_quality = 82
resolve_external = false

[generate]
framework = "html"
styling = "native"
responsive = true
image_quality = "original"

[generate.components]
detect = true
min_size = 3
naming = "prod-{id}"

[optimizer]
minify = true
remove_comments = true
precision = 2

[optimizer.simplify_paths]
enabled = true
tolerance = 0.4

[optimizer.deduplicate]
enabled = true
threshold = 0.97

[extractor]
fonts_dir = "./dist/fonts"
images_dir = "./dist/images"
```

## Валидация и ошибки

При ошибке в конфиге CLI завершает выполнение и выводит красное сообщение с номером строки и полем.

Пример:

```text
Error: invalid value in svg2web.toml at line 12, column 17
[parse].image_quality must be between 0 and 100
```

## См. также

- [CLI](cli.md)
