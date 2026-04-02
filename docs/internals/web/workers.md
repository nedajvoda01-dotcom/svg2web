<!-- [Doc] | Архитектура Web Workers: инициализация wasm в воркере, протокол сообщений и жизненный цикл. -->

# Web Workers

## Инициализация воркера

- Используется ES module worker, importScripts как fallback для старых окружений.
- wasm бинарник загружается внутри воркера через fetch + WebAssembly.instantiate.
- После загрузки вызывается init() модуля.

## Коммуникация

- postMessage с transferable objects, чтобы переносить ArrayBuffer без копирования.
- Типы сообщений:

```text
enum MessageType {
  PARSE,
  OPTIMIZE,
  GENERATE,
  ERROR,
  PROGRESS
}
```

- Передача следует structured clone algorithm (передаем plain objects, ArrayBuffer, typed arrays).

## Жизненный цикл

- Создание: new Worker().
- Инициализация: загрузка wasm обычно занимает ~1-2s.
- Обработка: очередь запросов, пока воркер активен.
- Завершение: terminate при unload страницы.

## Передача больших данных

- SVG отправляется как ArrayBuffer, а не строка.
- Изображения передаются как Uint8Array.
- Для генерации больших результатов используется chunked response (streaming по частям).
