```markdown
# Быстрый старт

## 1. Установка

```bash
cargo install svg2web-cli
```

## 2. Подготовьте SVG файл

Создайте `icon.svg`:

```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
  <circle cx="12" cy="12" r="10" fill="none" stroke="blue" stroke-width="2"/>
  <path d="M12 8v4l3 3" stroke="blue" stroke-width="2"/>
</svg>
```

## 3. Сгенерируйте React компонент

```bash
svg2web build icon.svg --output ./my-icon --format react
```

## 4. Посмотрите результат

```bash
cd my-icon
ls
# src/Icon.tsx   Icon.module.css   index.html
```

## 5. Откройте в браузере

```bash
open index.html
# или
start index.html  # Windows
```

## Результат

Вы увидите интерактивный React компонент с вашей иконкой.

## Готово!

За 5 минут вы сконвертировали SVG в React компонент с TypeScript и изолированными стилями.

---

## Другие форматы

```bash
# Vue компонент
svg2web build icon.svg --output ./my-icon --format vue

# Чистый HTML/CSS/JS
svg2web build icon.svg --output ./my-icon --format vanilla
```

## Следующие шаги

- [Полное руководство](/getting-started/first-project.md)
- [Настройка конфигурации](/usage/config.md)
- [Все команды CLI](/usage/cli.md)
```