```markdown
# assets.json

Файл содержит все ресурсы, извлеченные из SVG: растровые изображения, иконки, шрифты и ссылки на внешние ресурсы. Используется генератором для создания оптимизированного выхода.

## Структура

```json
{
  "images": {
    "img-001": {
      "original": {
        "data": "base64encoded...",
        "mime": "image/png",
        "width": 800,
        "height": 600
      },
      "variants": {
        "1x": {
          "file": "assets/img-001-1x.webp",
          "width": 400,
          "height": 300,
          "size_kb": 45
        },
        "2x": {
          "file": "assets/img-001-2x.webp",
          "width": 800,
          "height": 600,
          "size_kb": 120
        },
        "default": {
          "inline": true,
          "data": "base64webp...",
          "width": 200,
          "height": 150
        }
      },
      "used_by": ["Frame 1", "Card Background"]
    }
  },
  "icons": {
    "icon-arrow": {
      "source": "<svg>...</svg>",
      "type": "sprite",
      "used_by": ["Button Primary", "Link External"]
    }
  },
  "fonts": {
    "Inter": {
      "files": {
        "woff2": "assets/fonts/Inter-Regular.woff2",
        "woff": "assets/fonts/Inter-Regular.woff"
      },
      "weights": [400, 500, 600],
      "styles": ["normal", "italic"]
    }
  },
  "external": {
    "stylesheets": ["https://fonts.googleapis.com/css2?family=Inter"],
    "scripts": []
  }
}
```

## Поля

### `images`

Объект, где ключ — уникальный ID изображения (обычно генерируется из ID элемента в SVG или хеша), значение — метаданные и варианты.

**Image Object:**
- `original` — исходные данные
  - `data`: string (base64) — закодированное содержимое файла
  - `mime`: string — MIME-тип (image/png, image/jpeg, image/svg+xml)
  - `width`: number — ширина в пикселях
  - `height`: number — высота в пикселях
- `variants` — оптимизированные версии для разных DPI/размеров
  - `1x`, `2x`, `default` — ключи вариантов
    - `file`: string (опционально) — путь к файлу относительно output, если ресурс выносится в файл
    - `inline`: boolean (опционально) — true если встроено в CSS/HTML как data-uri
    - `data`: string (опционально) — base64 данных для inline-варианта
    - `width`: number — ширина варианта
    - `height`: number — высота варианта  
    - `size_kb`: number — размер в килобайтах (для сортировки по эффективности)
- `used_by`: string[] — список ID SVG-элементов, использующих это изображение

### `icons`

Объект со значками SVG, которые могут быть объединены в спрайт или оставлены inline.

**Icon Object:**
- `source`: string — SVG разметка элемента (viewBox + path)
- `type`: "sprite" | "external" | "inline" — стратегия встраивания
  - `sprite` — будет добавлен в SVG-спрайт и использоваться через `<use>`
  - `external` — вынесен в отдельный файл, ссылается через URL
  - `inline` — встроен непосредственно в HTML
- `used_by`: string[] — элементы, использующие иконку

### `fonts`

Шрифты, обнаруженные в SVG (через CSS font-family или атрибуты).

**Font Object:**
- `files`: object — доступные форматы
  - `woff2`: string (опционально) — путь к WOFF2 файлу
  - `woff`: string (опционально) — путь к WOFF файлу
- `weights`: number[] — массив используемых насыщенностей (400, 700)
- `styles`: string[] — массив стилей (normal, italic)

### `external`

Внешние ресурсы, на которые ссылается SVG (через `<link>`, `@import`, или `xlink:href`).

- `stylesheets`: string[] — URL внешних CSS (Google Fonts, CDN)
- `scripts`: string[] — URL внешних JavaScript

## Примечания

- **Конвертация форматов**: Растровые изображения автоматически конвертируются в WebP при включенной оптимизации (`webp` feature)
- **Inline порог**: Изображения менее 4KB обычно inline'ятся для сокращения HTTP-запросов (конфигурируется через `embed_threshold`)
- **Шрифты**: Системные шрифты (Arial, sans-serif) не включаются в этот файл, только кастомные/внешние
- **Целостность**: При генерации кода генератор проверяет наличие всех файлов из `variants` перед созданием ссылок
```