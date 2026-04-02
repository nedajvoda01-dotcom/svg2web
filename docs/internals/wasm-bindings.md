<!-- [Doc] | WASM FFI слой: wasm-bindgen, serde_wasm_bindgen, StringPool и ограничения браузерного рантайма. -->

# WASM Bindings

## wasm-bindgen

Используемые элементы:

- #[wasm_bindgen] для экспорта Rust-функций в JS.
- #[wasm_bindgen(start)] для инициализации, включая panic hook.

Поддерживаемые типы на границе:

- числа
- строки
- ArrayBuffer
- JsValue

## serde_wasm_bindgen

- Конвертирует структуры Rust в JS-объекты и обратно.
- Для производительности минимизируются копирования (avoid cloning where possible).

## StringPool

Проблема:

- строки >1MB в WASM heap могут приводить к OOM.

Решение:

- allocate -> handle (u32)
- read_chunk(offset, len)
- free(handle)

Пример JS использования:

```javascript
const handle = wasm.allocate(largeString);
const chunk = wasm.read_chunk(handle, 0, 65536);
wasm.free(handle);
```

## Ограничения

- Нет доступа к файловой системе (no std::fs).
- Сетевые запросы ограничены CORS-политиками браузера.
- Single-threaded модель в базовом режиме; для параллелизма используется Web Workers.
