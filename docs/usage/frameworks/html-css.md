<!-- [Doc] | Специфика генерации HTML/CSS для статических сайтов: структура файлов, CSS variables, адаптивность, шрифты и интеграции в Next.js/WordPress. -->

# HTML/CSS генерация

## Структура выходных файлов

```text
output/
  index.html
  styles.css
  script.js
```

### Назначение файлов

- index.html: семантическая разметка, inline критический CSS и defer script.
- styles.css: CSS-переменные в :root и медиа-запросы.
- script.js: минимальный vanilla JS для интерактива (если он есть в исходном SVG).

Это три основных выходных файла HTML/CSS-режима. Дополнительные ассеты, если они извлекались отдельно, считаются вспомогательными и не входят в основной контракт структуры.

## CSS переменные

Типовые переменные, которые генерируются:

- --color-primary
- --color-secondary
- --spacing-unit
- --radius-base
- --font-size-base

Пример:

```css
:root {
  --color-primary: #2563eb;
  --color-secondary: #475569;
  --spacing-unit: 8px;
  --radius-base: 8px;
  --font-size-base: 16px;
}
```

Как переопределить:

- подключить свой CSS после styles.css;
- задать inline style на контейнере.

Пример inline override:

```html
<div class="svg2web-root" style="--color-primary:#0f766e; --spacing-unit:10px;"></div>
```

Темная тема:

```css
@media (prefers-color-scheme: dark) {
  :root {
    --color-primary: #60a5fa;
    --color-secondary: #cbd5e1;
  }
}
```

## Адаптивность

Mobile-first breakpoints:

- 320px
- 768px
- 1024px

Пример:

```css
.svg2web-root { width: 100%; }

@media (min-width: 768px) {
  .svg2web-root { max-width: 720px; }
}

@media (min-width: 1024px) {
  .svg2web-root { max-width: 980px; }
}
```

Как контролировать через viewBox и CSS:

- корректный viewBox в SVG задает масштабирование без потери пропорций;
- width: 100% + height: auto обеспечивает адаптивный рендер;
- object-fit и container constraints помогают встроить SVG в сетки.

## Шрифты

Подключение через Google Fonts:

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600&display=swap" rel="stylesheet">
```

Подключение локальных файлов:

```css
@font-face {
  font-family: "Inter";
  src: url("./assets/fonts/Inter-Regular.woff2") format("woff2");
  font-display: swap;
}
```

Fallback стек:

```css
font-family: 'Inter', system-ui, sans-serif;
```

## Когда использовать

- Лендинги: быстрый первый рендер и минимум зависимостей.
- Встраивание в существующий HTML: через iframe или shadow DOM.
- Email-рассылки: ограниченная поддержка, требуется проверка в почтовых клиентах.

## Интеграция

### Next.js

```tsx
export default function Hero({ html }: { html: string }) {
  return <section dangerouslySetInnerHTML={{ __html: html }} />;
}
```

### WordPress shortcode

```php
function svg2web_embed_shortcode($atts) {
  $path = get_stylesheet_directory() . '/svg2web/output/index.html';
  return file_exists($path) ? file_get_contents($path) : '';
}
add_shortcode('svg2web_embed', 'svg2web_embed_shortcode');
```
