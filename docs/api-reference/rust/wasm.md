```markdown
# svg2web-wasm

WebAssembly биндинги для использования svg2web в браузере.

## Экспортируемые функции

### `parse_svg`

```rust
#[wasm_bindgen]
pub fn parse_svg(svg: &str) -> Result<JsValue, JsValue>
```

Парсинг SVG в промежуточное представление.

**Параметры:**
- `svg: &str` — содержимое SVG файла

**Возвращает:** `Result<JsValue, JsValue>` — объект `ParseOutput` в виде JS объекта или ошибку

### `analyze_svg`

```rust
#[wasm_bindgen]
pub fn analyze_svg(svg: &str) -> Result<JsValue, JsValue>
```

Анализ структуры SVG.

**Параметры:**
- `svg: &str` — содержимое SVG файла

**Возвращает:** `Result<JsValue, JsValue>` — объект `AnalysisResult` (компоненты, метрики сложности, иерархия)

### `optimize_svg`

```rust
#[wasm_bindgen]
pub fn optimize_svg(svg: &str, config: &JsValue) -> Result<JsValue, JsValue>
```

Оптимизация SVG.

**Параметры:**
- `svg: &str` — содержимое SVG файла
- `config: &JsValue` — объект конфигурации:
  ```javascript
  {
    simplify_paths: boolean,   // упрощение путей
    deduplicate: boolean,      // удаление дубликатов
    minify_ids: boolean,       // минификация id
    remove_comments: boolean   // удаление комментариев
  }
  ```

**Возвращает:** `Result<JsValue, JsValue>` — оптимизированный `ParseOutput`

### `extract_assets`

```rust
#[wasm_bindgen]
pub fn extract_assets(svg: &str) -> Result<JsValue, JsValue>
```

Извлечение ресурсов (изображения, шрифты).

**Параметры:**
- `svg: &str` — содержимое SVG файла

**Возвращает:** `Result<JsValue, JsValue>` — объект `ExtractedAssets`

### `serialize_to_json`

```rust
#[wasm_bindgen]
pub fn serialize_to_json(svg: &str) -> Result<String, JsValue>
```

Сериализация SVG в JSON.

**Параметры:**
- `svg: &str` — содержимое SVG файла

**Возвращает:** `Result<String, JsValue>` — JSON строка

### `set_panic_hook`

```rust
#[wasm_bindgen(start)]
pub fn set_panic_hook()
```

Установка хука паники для отладки. Вызывается автоматически при инициализации модуля.

## Типы данных

### `StringPool`

Управление памятью для больших строк, предотвращение OOM.

```rust
#[wasm_bindgen]
pub struct StringPool;

#[wasm_bindgen]
impl StringPool {
    pub fn new() -> StringPool;
    
    /// Выделить строку в пуле, вернуть handle
    pub fn allocate(&mut self, s: String) -> u32;
    
    /// Прочитать часть строки по handle
    pub fn read_chunk(&self, handle: u32, offset: usize, len: usize) -> JsValue;
    
    /// Освободить строку
    pub fn free(&mut self, handle: u32);
}
```

**Пример использования:**
```javascript
import { StringPool } from 'svg2web-wasm';

const pool = new StringPool();
const handle = pool.allocate(largeSvgString);

// Читаем по частям
const chunk = pool.read_chunk(handle, 0, 1024);
const result = parse_svg(chunk);

// Освобождаем память
pool.free(handle);
```

### `WasmCache`

Обертка над in-memory LRU кэшем.

```rust
#[wasm_bindgen]
pub struct WasmCache;

#[wasm_bindgen]
impl WasmCache {
    pub fn new(max_size: usize) -> WasmCache;
    
    pub fn get(&mut self, key: &str) -> Option<JsValue>;
    
    pub fn set(&mut self, key: String, value: JsValue);
}
```

**Пример:**
```javascript
const cache = new WasmCache(100);

// Кэшируем результат
cache.set(svgHash, parseResult);

// Получаем из кэша
const cached = cache.get(svgHash);
if (cached) {
  // используем кэшированный результат
}
```

## Ограничения

### Нет файловой системы
- Нет доступа к `std::fs`
- Все данные передаются через параметры функций
- Ресурсы извлекаются в память, возвращаются как массивы байтов

### Нет сетевых запросов
- Нет доступа к HTTP клиенту
- Внешние ресурсы (xlink:href) не могут быть загружены автоматически
- Для загрузки внешних ресурсов нужно использовать fetch из JS

### Single-threaded
- WASM выполняется в основном потоке
- Рекомендуется использовать Web Workers для неблокирующей работы

## Использование в Web Worker

**worker.js:**
```javascript
import init, { parse_svg, analyze_svg, optimize_svg } from 'svg2web-wasm';

let wasmReady = false;

await init();
wasmReady = true;

self.addEventListener('message', async (e) => {
  const { id, type, svg, config } = e.data;
  
  try {
    let result;
    switch (type) {
      case 'PARSE':
        result = parse_svg(svg);
        break;
      case 'ANALYZE':
        result = analyze_svg(svg);
        break;
      case 'OPTIMIZE':
        result = optimize_svg(svg, config);
        break;
    }
    
    self.postMessage({ id, result, error: null });
  } catch (error) {
    self.postMessage({ id, result: null, error: error.message });
  }
});
```

**main.js:**
```javascript
const worker = new Worker('worker.js');

function parseSVG(svg) {
  return new Promise((resolve, reject) => {
    const id = Math.random();
    
    const handler = (e) => {
      if (e.data.id === id) {
        worker.removeEventListener('message', handler);
        e.data.error ? reject(e.data.error) : resolve(e.data.result);
      }
    };
    
    worker.addEventListener('message', handler);
    worker.postMessage({ id, type: 'PARSE', svg });
  });
}
```

## Обработка ошибок

Ошибки возвращаются как `JsValue` с сообщением:

```javascript
try {
  const result = parse_svg(invalidSvg);
} catch (error) {
  // error.message — описание ошибки
  // error.type — 'ParseError' | 'InvalidSvg' | 'UnsupportedFeature'
  console.error(`${error.type}: ${error.message}`);
}
```

## Типы возвращаемых объектов

### ParseOutput (JavaScript)
```typescript
interface ParseOutput {
  meta: {
    source: string;
    canvas: { width: number; height: number };
    viewBox: [number, number, number, number];
  };
  structure: SVGElement;
  geometry: Geometry;
  styles: Styles;
  assets: Assets;
}
```

### AnalysisResult (JavaScript)
```typescript
interface AnalysisResult {
  components: Array<{
    id: string;
    occurrences: Array<{ path: string }>;
  }>;
  complexity: {
    score: number;
    nodeCount: number;
    pathCount: number;
    gradientCount: number;
  };
  hierarchy: {
    depth: number;
    maxWidth: number;
  };
}
```
```