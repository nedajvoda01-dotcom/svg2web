# WASM API Обзор

## Инициализация

```typescript
import init from 'svg2web-wasm';

// Инициализация WASM модуля
await init();
```

Инициализация через `await init()` обязательна перед любым вызовом `parse_svg`, `analyze_svg`, `optimize_svg`, `extract_assets` или `serialize_to_json`.

## Основные функции

### `parse_svg`

```typescript
function parse_svg(svg: string): ParseOutput
```

Парсинг SVG в промежуточную модель.

**Параметры:**
- `svg: string` — содержимое SVG файла

**Возвращает:** строку JSON, которая затем преобразуется в `ParseOutput` на стороне JavaScript/TypeScript.

### `analyze_svg`

```typescript
function analyze_svg(svg: string): AnalysisResult
```

Структурный анализ SVG.

**Параметры:**
- `svg: string` — содержимое SVG файла

**Возвращает:** `AnalysisResult` — компоненты, метрики сложности, иерархия

### `optimize_svg`

```typescript
function optimize_svg(svg: string, config?: string): string
```

Оптимизация SVG.

**Параметры:**
- `svg: string` — содержимое SVG файла
- `config?: string` — JSON-строка с `OptimizerConfig`

**Возвращает:** JSON-строку с оптимизированной моделью `ParseOutput`

### `extract_assets`

```typescript
function extract_assets(svg: string): Asset[]
```

Извлечение ресурсов.

**Параметры:**
- `svg: string` — содержимое SVG файла

**Возвращает:** `Asset[]` — массив ресурсов (изображения, шрифты)

### `serialize_to_json`

```typescript
function serialize_to_json(svg: string): string
```

Сериализация в JSON.

**Параметры:**
- `svg: string` — содержимое SVG файла

**Возвращает:** `string` — JSON строка с промежуточным представлением

## Типы данных

### Примитивные типы

Rust примитивы автоматически конвертируются в JavaScript:

| Rust | JavaScript |
|------|------------|
| `bool` | `boolean` |
| `i32`, `u32`, `i64`, `u64` | `number` |
| `f32`, `f64` | `number` |
| `String` | `string` |
| `Vec<T>` | `Array<T>` |
| `Option<T>` | `T \| null` |

Сложные структуры передаются через `serde_wasm_bindgen`, а бинарные данные передаются как `Uint8Array`.

### Структуры через serde_wasm_bindgen

Сложные структуры конвертируются в JavaScript объекты:

```typescript
interface ParseOutput {
  meta: {
    source: string;
    canvas: { width: number; height: number };
    viewBox: [number, number, number, number];
    title: string | null;
    description: string | null;
  };
  structure: SVGElement;
  geometry: Geometry;
  styles: Styles;
  assets: Assets;
}

interface SVGElement {
  tag: string;
  attributes: Record<string, string>;
  children: SVGElement[];
  id?: string;
  class: string[];
  text_content?: string;
}

interface AnalysisResult {
  components: Component[];
  complexity: ComplexityMetrics;
  hierarchy: HierarchyInfo;
}

interface Component {
  id: string;
  template: SVGElement;
  occurrences: Occurrence[];
}

interface ComplexityMetrics {
  node_count: number;
  path_count: number;
  path_complexity: number;
  gradient_count: number;
  score: number;
}

interface Asset {
  type: 'image' | 'font' | 'external';
  id: string;
  data?: Uint8Array;
  url?: string;
  mime_type?: string;
}
```

### Бинарные данные

Бинарные данные передаются как `Uint8Array`:

```typescript
interface ImageAsset {
  id: string;
  format: 'png' | 'jpeg' | 'webp' | 'svg';
  data: Uint8Array;        // бинарные данные изображения
  width: number;
  height: number;
}
```

## Ограничения

### Нет доступа к файловой системе

- Все данные передаются через параметры функций
- Нельзя читать/писать файлы напрямую
- Ресурсы возвращаются как массивы байтов

### Нет сетевых запросов

- Нельзя делать HTTP запросы из WASM
- Внешние ресурсы (`xlink:href`) не загружаются автоматически
- Для загрузки внешних ресурсов используйте `fetch()` из JavaScript

### Single-threaded выполнение

- WASM выполняется в основном потоке
- Длительные операции блокируют UI
- Используйте Web Workers для фоновой обработки

## Использование в Web Worker

**worker.js:**
```javascript
import init, { parse_svg, analyze_svg } from 'svg2web-wasm';

await init();

self.addEventListener('message', async (e) => {
  const { id, svg, type } = e.data;
  
  try {
    let result;
    switch (type) {
      case 'PARSE':
        result = parse_svg(svg);
        break;
      case 'ANALYZE':
        result = analyze_svg(svg);
        break;
    }
    
    self.postMessage({ id, success: true, result });
  } catch (error) {
    self.postMessage({ id, success: false, error: error.message });
  }
});
```

**main.js:**
```javascript
const worker = new Worker('worker.js');

function parseSVG(svg) {
  return new Promise((resolve, reject) => {
    const id = Math.random();
    
    worker.addEventListener('message', function handler(e) {
      if (e.data.id === id) {
        worker.removeEventListener('message', handler);
        e.data.success ? resolve(e.data.result) : reject(e.data.error);
      }
    });
    
    worker.postMessage({ id, type: 'PARSE', svg });
  });
}

// Использование
const result = await parseSVG(svgContent);
console.log(result.meta.canvas);
```

## Примеры использования

### Базовый пример

```javascript
import init, { parse_svg, analyze_svg } from 'svg2web-wasm';

await init();

const svg = '<svg xmlns="http://www.w3.org/2000/svg"><circle cx="50" cy="50" r="40" fill="red"/></svg>';

// Парсинг
const parsed = parse_svg(svg);
console.log(parsed.meta.canvas); // { width: 100, height: 100 }

// Анализ
const analysis = analyze_svg(svg);
console.log(analysis.complexity.score); // числовая оценка сложности
```

### Оптимизация с конфигурацией

```javascript
import { parse_svg, optimize_svg } from 'svg2web-wasm';

const svg = '<svg><g><rect id="rect1" width="100" height="100"/></g></svg>';

const optimized = optimize_svg(svg, {
  simplify_paths: true,
  deduplicate: true,
  minify_ids: true
});
```

### Извлечение ресурсов

```javascript
import { extract_assets } from 'svg2web-wasm';

const svg = '<svg><image href="data:image/png;base64,..."/></svg>';
const assets = extract_assets(svg);

assets.forEach(asset => {
  if (asset.type === 'image') {
    console.log(`Image: ${asset.format}, ${asset.data.length} bytes`);
    // asset.data — Uint8Array
  }
});
```

### Сериализация в JSON

```javascript
import { serialize_to_json } from 'svg2web-wasm';

const json = serialize_to_json(svg);
const parsed = JSON.parse(json);
localStorage.setItem('svg-cache', json);
```

## Полный цикл с Web Worker

```javascript
// converter.worker.js
import init, { convert } from 'svg2web-wasm';

await init();

self.addEventListener('message', async (e) => {
  const { id, svg, options } = e.data;
  
  try {
    const result = convert(svg, JSON.stringify(options));
    self.postMessage({ id, success: true, result: JSON.parse(result) });
  } catch (error) {
    self.postMessage({ id, success: false, error: error.message });
  }
});

// app.js
class SVGConverter {
  constructor() {
    this.worker = new Worker('converter.worker.js');
    this.callbacks = new Map();
    
    this.worker.addEventListener('message', (e) => {
      const { id, success, result, error } = e.data;
      const cb = this.callbacks.get(id);
      if (cb) {
        success ? cb.resolve(result) : cb.reject(error);
        this.callbacks.delete(id);
      }
    });
  }
  
  async convert(svg, options) {
    const id = Math.random();
    
    return new Promise((resolve, reject) => {
      this.callbacks.set(id, { resolve, reject });
      this.worker.postMessage({ id, svg, options });
    });
  }
  
  terminate() {
    this.worker.terminate();
  }
}

// Использование
const converter = new SVGConverter();
const result = await converter.convert(svg, {
  framework: 'react',
  styling: 'scoped'
});
console.log(result.html);
```
