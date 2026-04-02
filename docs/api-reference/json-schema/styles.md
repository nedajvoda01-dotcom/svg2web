# styles.json

Файл содержит все стилистические данные SVG: цвета, градиенты, шрифты, типографику, эффекты и обводки. Представлен в CSS-совместимом формате для прямого использования в генераторах стилей без дополнительной трансформации.

## Структура

```json
{
  "version": "0.2.0",
  "colors": {
    "primary": "#3B82F6",
    "background": "#F3F4F6",
    "text-primary": "#111827",
    "text-secondary": "#6B7280"
  },
  "gradients": {
    "hero-gradient": {
      "type": "linear",
      "x1": 0,
      "y1": 0,
      "x2": 1,
      "y2": 1,
      "stops": [
        ["#4F46E5", "0%"],
        ["#9333EA", "100%"]
      ]
    },
    "radial-glow": {
      "type": "radial",
      "x1": 0.5,
      "y1": 0.5,
      "r": 0.5,
      "stops": [
        ["#FFFFFF", "0%"],
        ["#00000000", "100%"]
      ]
    }
  },
  "fonts": [
    {
      "family": "Inter",
      "weights": [400, 500, 600, 700],
      "styles": ["normal", "italic"],
      "fallback": "system-ui, -apple-system, sans-serif"
    }
  ],
  "typography": {
    "h1": {
      "family": "Inter",
      "size": 48,
      "weight": 700,
      "line_height": 1.2,
      "letter_spacing": -0.02,
      "color": "#111827"
    },
    "body": {
      "family": "Inter",
      "size": 16,
      "weight": 400,
      "line_height": 1.5,
      "color": "#374151"
    }
  },
  "effects": {
    "card-shadow": {
      "type": "drop_shadow",
      "x": 0,
      "y": 4,
      "blur": 12,
      "spread": 0,
      "color": "#0000001A"
    },
    "backdrop-blur": {
      "type": "blur",
      "radius": 8
    }
  },
  "strokes": {
    "border-default": {
      "width": 1,
      "color": "#E5E7EB",
      "linecap": "round",
      "linejoin": "round"
    },
    "border-dashed": {
      "width": 2,
      "color": "#9CA3AF",
      "dasharray": "5,5",
      "linecap": "butt",
      "linejoin": "miter"
    }
  }
}
```

## Поля

### `version`
- **Тип**: `string` (semver)
- **Описание**: Версия схемы секции `styles`, необходимая для миграций и проверки совместимости.

### `colors`

Объект с именованными цветами, где ключ — семантическое имя (CSS-совместимое), значение — HEX/RGB/RGBA строка.

**Naming conventions:**
- Имена используют kebab-case (`primary-blue`, `text-secondary`)
- Поддерживаются форматы: `#RRGGBB`, `#RRGGBBAA`, `rgb()`, `rgba()`, `hsl()`
- Специальные ключи: `transparent`, `currentColor`, `inherit`

### `gradients`

Объект с градиентами для заливок и фонов.

**Linear Gradient:**
- `type`: "linear"
- `x1`, `y1`: number (0-1) — начальная точка в процентах от размера
- `x2`, `y2`: number (0-1) — конечная точка
- `stops`: Array<[string, string]> — пары [цвет, offset]. Offset в процентах (0%-100%) или долях (0-1)

**Radial Gradient:**
- `type`: "radial"
- `x1`, `y1`: number (0-1) — центр круга
- `r`: number (0-1) — радиус относительно размера элемента
- `stops`: аналогично linear

### `fonts`

Массив шрифтов, используемых в дизайне.

**Font Object:**
- `family`: string — имя шрифта (как в CSS font-family)
- `weights`: number[] — массив используемых весов [400, 700]
- `styles`: string[] — массив стилей ["normal", "italic"]
- `fallback`: string — fallback шрифты для CSS (через запятую)

### `typography`

Типографические стили (текстовые стили из Figma/Sketch) с CSS-параметрами.

**Typography Style Object:**
- `family`: string — ссылка на ключ из `fonts` или прямое имя шрифта
- `size`: number — размер в пикселях (16, 24, 48)
- `weight`: number | string — вес (400, "bold") или numeric (100-900)
- `line_height`: number — множитель (1.5) или абсолютное значение в px если > 3
- `letter_spacing`: number (опционально) — в em (0.05 = 5%)
- `color`: string (опционально) — ссылка на ключ `colors` или прямой HEX

Стандартные имена стилей: `h1`, `h2`, `h3`, `body`, `caption`, `button`, `label`, `overline`.

### `effects`

Визуальные эффекты: тени, размытие.

**Drop Shadow:**
- `type`: "drop_shadow"
- `x`: number — смещение по X в px
- `y`: number — смещение по Y в px  
- `blur`: number — радиус размытия
- `spread`: number — радиус распространения
- `color`: string — цвет тени (обычно с прозрачностью, например `#00000033`)

**Blur:**
- `type`: "blur"
- `radius`: number — радиус размытия в px (backdrop-filter или filter)

### `strokes`

Обводки и границы элементов.

**Stroke Object:**
- `width`: number — толщина линии в px
- `color`: string — цвет обводки (ссылка на colors или HEX)
- `dasharray`: string (опционально) — пунктирный паттерн ("5,5", "10,5,2,5")
- `linecap`: "butt" | "round" | "square" — окончание линии
- `linejoin`: "miter" | "round" | "bevel" — соединение линий

## Примечания

- **CSS Variables**: Генераторы могут преобразовывать ключи `colors` в CSS custom properties: `--color-primary: #3B82F6`
- **Gradient IDs**: При генерации SVG/HTML, градиенты получают уникальные ID на основе ключа (например, `url(#hero-gradient)`)
- **Font loading**: Поле `fonts` используется для генерации `@font-face` правил или `<link>` тегов Google Fonts (собирается уникальный URL с нужными weights)
- **Typography mapping**: При конвертации в React/Vue, стили типографики превращаются в CSS-in-JS объекты или CSS классы с миксинами
- **Effects limitations**: CSS `backdrop-filter` имеет ограниченную поддержкой в Safari, генератор может добавлять `-webkit-` префиксы или fallback
- **Stroke scaling**: При responsive адаптации `stroke-width` может потребоваться масштабирование через `vector-effect: non-scaling-stroke` для SVG иконок
