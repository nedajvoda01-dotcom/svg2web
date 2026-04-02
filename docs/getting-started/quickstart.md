# Быстрый старт

## Предусловия

- svg2web CLI уже установлен.
- Если еще не установлен, используйте: [Установка](installation.md).

## Шаг 1: Сборка компонента

1. Создайте файл `logo.svg`:

```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <circle cx="50" cy="50" r="40" fill="blue" />
</svg>
```

2. Выполните команду:

```bash
svg2web build logo.svg --output ./output --format react
```

Примечание: для быстрого старта `svg2web.toml` не нужен, команда выше работает на дефолтах.

## Шаг 2: Проверка результата

В папке `output` появятся файлы:

```plaintext
output/
  components/Logo.tsx
  index.html
  styles.css
```

- `components/Logo.tsx`: React-компонент.
- `index.html`: HTML-точка входа для предпросмотра.
- `styles.css`: стили для компонента.

## Шаг 3: Запуск в браузере

1. Перейдите в папку `output`:

```bash
cd output
```

2. Запустите локальный сервер:

```bash
npx serve
```

3. Откройте браузер и перейдите по адресу, указанному в консоли.

## Что дальше?

- [Настроить под себя](../usage/config.md)
- [Узнать все команды](../usage/cli.md)