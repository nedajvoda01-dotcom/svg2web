```markdown
# Отладка

## Логирование

### Настройка tracing

В `main.rs` CLI:

```rust
use tracing_subscriber::{fmt, EnvFilter};

fn init_logging() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .with_level(true)
        .init();
}
```

### Уровни логирования

```rust
use tracing::{trace, debug, info, warn, error};

trace!("подробная информация о входных данных");
debug!("отладочная информация, промежуточные состояния");
info!("ключевые этапы: парсинг начат, генерация завершена");
warn!("некритичные проблемы: неподдерживаемый атрибут");
error!("ошибки: не удалось распарсить SVG");
```

### Фильтры

```bash
# Только ошибки
RUST_LOG=error cargo run

# Инфо для всех крейтов
RUST_LOG=info cargo run

# Дебаг для core
RUST_LOG=svg2web_core=debug cargo run

# Дебаг для core и инфо для остальных
RUST_LOG=svg2web_core=debug,info cargo run

# Трассировка для парсера
RUST_LOG=svg2web_core::parser=trace cargo run

# В WASM (через console.log)
RUST_LOG=debug wasm-pack build --dev
```

### Структурированное логирование

```rust
use tracing::info;

info!(
    svg_size = svg.len(),
    elements_count = elements.len(),
    has_gradients = gradients_exist,
    "SVG parsed successfully"
);
```

### Интеграция с console в WASM

```rust
use web_sys::console;

fn debug_wasm(message: &str) {
    console::log_1(&JsValue::from_str(message));
}

fn debug_wasm_with_value(key: &str, value: &JsValue) {
    console::log_2(&JsValue::from_str(key), value);
}
```

---

## Отладка WASM в Chrome DevTools

### Сборка с source maps

```bash
# Режим разработки с source maps
wasm-pack build --target web --debug

# Или с дополнительными флагами
RUSTFLAGS='-C debuginfo=2' wasm-pack build --target web
```

### Настройка source maps в Chrome

1. Откройте DevTools (F12)
2. Settings (⚙️) → Experiments
3. Включите "WebAssembly Debugging: Enable DWARF support"
4. Перезагрузите DevTools

### Точки останова в Rust коде

```rust
// В Chrome DevTools можно поставить breakpoint на эту строку
#[wasm_bindgen]
pub fn parse_svg(svg: &str) -> Result<JsValue, JsValue> {
    // Breakpoint здесь
    let result = svg2web_core::parse(svg, ParseOptions::default())?;
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
```

### Просмотр памяти

```javascript
// В консоли Chrome
const wasm = await import('svg2web-wasm');

// Просмотр экспортированных функций
console.log(Object.keys(wasm));

// Проверка памяти
const memory = wasm.memory;
console.log(memory.buffer.byteLength); // размер в байтах
```

---

## Отладка Web Workers

### Инструменты Chrome

1. **Sources** → **Threads** → выберите воркер
2. **Console** → выберите контекст воркера в выпадающем списке
3. **Network** → фильтр по воркеру

### Пример с отладочными сообщениями

```javascript
// worker.js
import init, { parse_svg } from 'svg2web-wasm';

await init();

self.addEventListener('message', async (e) => {
    console.log('[Worker] Received:', e.data);
    
    try {
        const start = performance.now();
        const result = parse_svg(e.data.svg);
        const duration = performance.now() - start;
        
        console.log(`[Worker] Parsed in ${duration.toFixed(2)}ms`);
        
        self.postMessage({
            id: e.data.id,
            success: true,
            result,
            duration
        });
    } catch (error) {
        console.error('[Worker] Error:', error);
        self.postMessage({
            id: e.data.id,
            success: false,
            error: error.message
        });
    }
});
```

### Отладка сообщений

```javascript
// main.js
const worker = new Worker('worker.js');

worker.addEventListener('message', (e) => {
    console.log('[Main] Received from worker:', e.data);
});

worker.addEventListener('error', (e) => {
    console.error('[Main] Worker error:', e);
});

// Отслеживание всех сообщений
worker.addEventListener('message', (e) => {
    if (e.data.duration) {
        console.timeEnd(`parse-${e.data.id}`);
    }
});
```

---

## Типичные проблемы

### OOM в WASM

**Проблема:** WASM память ограничена 2GB, большие SVG вызывают OOM.

**Решение:** Использовать StringPool для чанковой обработки.

```javascript
import { parse_svg, StringPool } from 'svg2web-wasm';

function parseLargeSVG(svgString) {
    const pool = new StringPool();
    const handle = pool.allocate(svgString);
    
    try {
        // Читаем по частям
        let result = '';
        let offset = 0;
        const chunkSize = 1024 * 1024; // 1MB
        
        while (true) {
            const chunk = pool.read_chunk(handle, offset, chunkSize);
            if (!chunk || chunk.length === 0) break;
            
            // Обрабатываем chunk
            result += parse_svg(chunk);
            offset += chunkSize;
        }
        
        return result;
    } finally {
        pool.free(handle);
    }
}
```

### CORS для шрифтов

**Проблема:** Внешние шрифты блокируются CORS политикой.

**Решение:** Проксирование через свой сервер или использование CDN с CORS заголовками.

```rust
// В Rust: отключаем загрузку внешних ресурсов
let options = ExtractOptions {
    resolve_external: false,  // не пытаемся загружать
    ..Default::default()
};
```

```javascript
// В JS: загружаем через fetch с правильными заголовками
async function fetchFont(url) {
    const response = await fetch(url, {
        mode: 'cors',
        credentials: 'omit'
    });
    
    if (!response.ok) {
        throw new Error(`Failed to fetch font: ${response.status}`);
    }
    
    const blob = await response.blob();
    return URL.createObjectURL(blob);
}
```

```nginx
# Настройка CORS в nginx
location /fonts/ {
    add_header Access-Control-Allow-Origin *;
    add_header Access-Control-Allow-Methods GET;
    add_header Access-Control-Allow-Headers Content-Type;
}
```

### Stack overflow в глубокой рекурсии

**Проблема:** SVG с глубокой вложенностью (>1000 уровней) вызывает stack overflow.

**Решение:** Увеличить размер стека или переписать рекурсию на итерацию.

```rust
// В Cargo.toml
[profile.release]
stack-size = 8388608  # 8MB

// Или использование итеративного обхода
fn traverse_iterative(root: &SVGElement) {
    let mut stack = vec![root];
    
    while let Some(element) = stack.pop() {
        // Обработка элемента
        for child in element.children.iter().rev() {
            stack.push(child);
        }
    }
}
```

### Паника в WASM

**Проблема:** Паника в Rust коде не показывает стектрейс.

**Решение:** Установить panic hook.

```rust
#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}
```

```javascript
// В JS: перехват паники
try {
    const result = parse_svg(invalidSvg);
} catch (error) {
    // error.stack содержит Rust стектрейс
    console.error(error.stack);
}
```

---

## Профилирование WASM

### Chrome Performance Tab

1. Откройте DevTools → Performance
2. Нажмите Record
3. Выполните операцию
4. Остановите запись
5. Фильтр по "wasm"

### CPU профилирование

```javascript
// Добавление меток
performance.mark('parse-start');
const result = parse_svg(svg);
performance.mark('parse-end');
performance.measure('parse', 'parse-start', 'parse-end');

console.log(performance.getEntriesByName('parse'));
```

### Memory профилирование

```javascript
// Мониторинг памяти
let lastMemory = performance.memory?.usedJSHeapSize || 0;

function checkMemory() {
    const current = performance.memory?.usedJSHeapSize || 0;
    const delta = (current - lastMemory) / 1024 / 1024;
    console.log(`Memory delta: ${delta.toFixed(2)} MB`);
    lastMemory = current;
}

setInterval(checkMemory, 1000);
```

---

## Отладка CLI

### RUST_BACKTRACE

```bash
# Полный стектрейс при панике
RUST_BACKTRACE=1 svg2web parse input.svg

# С символьной информацией
RUST_BACKTRACE=full svg2web build input.svg --format react
```

### GDB

```bash
# Запуск под gdb
gdb --args target/debug/svg2web parse input.svg

# Точки останова
break svg2web_core::parser::parse
run

# Стек вызовов
backtrace
```

### Valgrind (Linux)

```bash
# Проверка утечек памяти
valgrind --leak-check=full target/debug/svg2web parse input.svg

# Профилирование
valgrind --tool=cachegrind target/debug/svg2web parse input.svg
```

---

## Отладка тестов

```bash
# Запуск конкретного теста с выводом
cargo test test_parse_simple -- --nocapture

# С логированием
RUST_LOG=debug cargo test test_parse_simple -- --nocapture

# Запуск теста под отладчиком
rust-gdb --args target/debug/deps/svg2web_core-* test_parse_simple
```

---

## Полезные инструменты

| Инструмент | Назначение |
|------------|------------|
| `cargo expand` | Просмотр макросов |
| `cargo tree` | Дерево зависимостей |
| `wasm2wat` | Декомпиляция WASM в текст |
| `twiggy` | Анализ размера WASM |
| `cargo bloat` | Анализ размера бинарника |
| `cargo udeps` | Неиспользуемые зависимости |
| `cargo audit` | Проверка уязвимостей |
```