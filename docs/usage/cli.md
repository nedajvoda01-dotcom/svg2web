<!-- [Doc] | CLI man-page: полный справочник команд parse/generate/build/optimize, флаги, exit codes и сценарии CI/batch. Без описания JSON-схем и внутренних алгоритмов. -->

# CLI

## Обзор команд

| Команда | Что делает | Когда использовать |
|---|---|---|
| parse | Преобразует SVG в JSON и ассеты без генерации кода | Нужен промежуточный результат для анализа или последующей генерации |
| generate | Генерирует код из готового JSON | JSON уже получен и требуется код под конкретный фреймворк |
| build | Выполняет parse + generate одной командой | Нужен полный цикл в локальной работе и CI |
| optimize | Оптимизирует SVG без генерации кода | Проверки и подготовка SVG в CI/CD |

## Команда parse

Назначение: SVG -> JSON + assets (без генерации кода).

### Синтаксис

```bash
svg2web parse <input.svg> [flags]
```

### Флаги

- --extract-images: извлекает встроенные изображения в отдельные файлы.
- --webp: конвертирует извлеченные изображения в WebP.
- --quality <0-100>: качество изображений при экспорте.
- --split-output: пишет результат в 6 файлов вместо 1 объединенного файла.
- --output, -o <path>: путь вывода (по умолчанию ./output).

### Примеры

```bash
svg2web parse design.svg --output ./json
svg2web parse design.svg --extract-images --webp --quality 85
```

### Выходные файлы

Без --split-output:

```text
json/
  output.json
  assets/
```

С --split-output:

```text
json/
  document.json
  nodes.json
  styles.json
  gradients.json
  metadata.json
  manifest.json
  assets/
```

## Команда generate

Назначение: JSON -> Code.

### Синтаксис

```bash
svg2web generate <input.json> [flags]
```

### Флаги

- --framework <react|vue|vanilla>
- --styling <native|tailwind|scoped>
- --responsive
- --config <path/to.toml>
- --output, -o <path>

### Пример

```bash
svg2web generate ./json/output.json --framework react --styling tailwind
```

## Команда build

Назначение: parse + generate (полный цикл).

### Синтаксис

```bash
svg2web build <input.svg> [flags]
```

### Флаги

Команда принимает флаги из parse и generate, включая:

- --extract-images, --webp, --quality, --split-output
- --framework, --styling, --responsive
- --config, --output, --verbose, --quiet

### Пример полного цикла с кастомным конфигом

```bash
svg2web build design.svg --config ./ci.toml --framework react --styling scoped --output ./dist
```

## Команда optimize

Назначение: оптимизация SVG без генерации кода (для CI/CD проверки).

### Синтаксис

```bash
svg2web optimize <input.svg> [flags]
```

### Флаги

- --simplify
- --deduplicate
- --minify
- --precision <int>
- --output, -o <path>

### Пример

```bash
svg2web optimize design.svg --simplify --deduplicate --minify --precision 2 --output ./optimized.svg
```

## Общие флаги

- --config path/to.toml: путь к конфигу, приоритет над дефолтами.
- --output, -o: путь вывода, дефолт ./output.
- --verbose, -v: подробные логи. Уровни: info, debug, trace.
- --quiet, -q: только ошибки.

## Exit codes

| Код | Значение |
|---|---|
| 0 | Успешное выполнение |
| 1 | Ошибка парсинга |
| 2 | Ошибка I/O |
| 3 | Невалидный конфиг |

## Сложные сценарии

### CI pipeline

```bash
svg2web build --config ci.toml --quiet && exit $?
```

### Batch processing

```bash
for file in *.svg; do svg2web parse "$file"; done
```

## См. также

- [Конфигурация](config.md)
