<!-- [Doc] | Vanilla-генерация для максимальной совместимости: форматы ESM/UMD/Web Component, API, события, CSS и интеграции. -->

# Vanilla JS генерация

## Форматы выхода

- ES Module: основной контракт генерации, .js + .css для современных сборок через import/export.
- UMD: подключение через <script src="...">.
- Web Component: .js с customElements.define.

Пример структуры:

```text
output/
  esm/
    logo.js
    logo.css
  umd/
    logo.umd.js
  wc/
    logo-element.js
```

## API компонента

Типовой программный интерфейс:

- init(options)
- update(options)
- destroy()

data-атрибуты для инициализации:

```html
<div data-svg2web="logo" data-size="32" data-color="#2563eb"></div>
```

События через CustomEvent:

- svg2web:ready
- svg2web:click
- svg2web:error

Пример:

```javascript
element.addEventListener('svg2web:click', (e) => {
  console.log(e.detail);
});
```

## CSS

Поддерживаемые режимы:

- CSS-in-JS (опционально).
- Shadow DOM для инкапсуляции.
- классические CSS-классы с префиксом (BEM-like), например .svg2web-logo__path.

## Размер и зависимости

- Runtime overhead обычно 0-4 KB gzip.
- Zero-dependency mode поддерживается для базовых компонентов.

## Интеграция

- В существующий сайт: можно подключать рядом с jQuery-скриптами.
- Webflow: вставка через custom code embed.
- Webhook-ориентированные платформы: генерация и публикация как статический ассет.
- WordPress: подключение через enqueue_script/enqueue_style без плагинов.

## См. также

- [CLI](../cli.md)
- [Конфигурация](../config.md)
