# Отладка

## Логирование

### tracing subscriber

Рекомендуемый базовый setup:

```rust
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logging() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .with_level(true)
        .init();
}
```

### Фильтры

```bash
RUST_LOG=svg2web_core=debug cargo run --bin svg2web -- build input.svg
```

Ожидаемый результат: structured logs с полями уровня, target и сообщения от tracing subscriber.

### Форматы вывода

- pretty: для локальной отладки.
- json: для production/агрегации логов.

Пример json-режима:

```rust
tracing_subscriber::fmt().json().init();
```

## Отладка WASM

### Chrome DevTools

Путь:

- Sources -> Page -> wasm bundle

Для Rust breakpoints используйте:

```bash
wasm-pack build crates/svg2web-wasm --target web --debug
```

Сборка в режиме `--debug` должна генерировать source maps, доступные в Chrome DevTools.

Что смотреть:

- breakpoints в .rs через source maps
- Memory tab -> Linear Memory
- Console panic traces через set_panic_hook

Паника с трейсом:

```rust
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
```

## Отладка Web Workers

Путь в DevTools:

- Sources -> Threads -> worker

Практика:

- логируйте postMessage вход/выход
- проверяйте порядок событий PARSE/OPTIMIZE/GENERATE/ERROR/PROGRESS

## Типичные проблемы

### OOM в WASM

Проблема:

- большие SVG приводят к росту heap и OOM.

Решение:

- использовать StringPool
- уменьшать размер SVG до обработки

### CORS для шрифтов

Проблема:

- внешние шрифты/ресурсы могут блокироваться браузером.

Решение:

- проксирование через backend
- fallback на локальные/системные шрифты

### Stack overflow

Проблема:

- глубокая рекурсия в parse.

Решение:

- переход на итеративный обход
- ограничение глубины дерева
