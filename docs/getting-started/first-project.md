# Первый проект

## Шаг 1: Подготовка SVG

1. Экспортируйте из Figma:
   - Выберите объект → **Export** → SVG.
   - Включите опции "Remove unused styles" и "Outline text".

2. Очистите SVG:

```bash
svg2web optimize input.svg --output optimized.svg
```

## Шаг 2: Выбор фреймворка

| Фреймворк | Когда использовать          |
|-----------|-----------------------------|
| React     | Для сложных интерфейсов     |
| Vue       | Для интеграции в Vue-проект |
| HTML/CSS  | Для статических сайтов      |

## Шаг 3: Конфигурация

Создайте файл `svg2web.toml`:

```toml
[build]
output = "./output"
framework = "react"
```

## Шаг 4: Сборка

```bash
svg2web build input.svg
```

## Шаг 5: Интеграция

React:

```javascript
import Logo from './output/components/Logo';
```

HTML:

```html
<link rel="stylesheet" href="output/styles.css">
<script src="output/components/Logo.js"></script>
```

## Шаг 6: Деплой

- **Vercel**:

```bash
vercel ./output
```

- **Netlify**:

```bash
netlify deploy ./output
```