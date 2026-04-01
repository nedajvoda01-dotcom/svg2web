```markdown
# Функции WASM

## Сигнатуры функций

### `parse`

```typescript
function parse(svg: Uint8Array | string): string
```

Парсинг SVG в промежуточный JSON.

**Параметры:**
- `svg: Uint8Array | string` — содержимое SVG файла (бинарные данные или строка)

**Возвращает:** `string` — JSON-строка с `ParseOutput`

**Пример:**
```typescript
import { parse } from 'svg2web-wasm';

// Со строкой
const svgString = '<svg xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100"/></svg>';
const result = parse(svgString);
const parsed = JSON.parse(result);
console.log(parsed.meta.canvas); // { width: 100, height: 100 }

// С Uint8Array
const encoder = new TextEncoder();
const svgBytes = encoder.encode(svgString);
const result2 = parse(svgBytes);
```

### `optimize`

```typescript
function optimize(svg: Uint8Array | string, config?: string): string
```

Оптимизация SVG.

**Параметры:**
- `svg: Uint8Array | string` — содержимое SVG
- `config?: string` — JSON-строка с `OptimizerConfig`

**Возвращает:** `string` — JSON-строка с оптимизированным `ParseOutput`

**OptimizerConfig:**
```typescript
interface OptimizerConfig {
  simplify_paths?: boolean;   // упрощение путей, по умолчанию true
  deduplicate?: boolean;      // удаление дубликатов, по умолчанию true
  minify_ids?: boolean;       // минификация id, по умолчанию true
  remove_comments?: boolean;  // удаление комментариев, по умолчанию true
  precision?: number;         // точность координат, по умолчанию 3
}
```

**Пример:**
```typescript
import { optimize } from 'svg2web-wasm';

const svg = '<svg><!-- comment --><rect id="rect1" width="100" height="100"/></svg>';

const config = JSON.stringify({
  simplify_paths: true,
  remove_comments: true,
  minify_ids: true
});

const optimized = optimize(svg, config);
const result = JSON.parse(optimized);
// result содержит оптимизированную структуру без комментариев и с минифицированными id
```

### `extractAssets`

```typescript
function extractAssets(json: string, options?: string): string
```

Извлечение ресурсов из промежуточного JSON.

**Параметры:**
- `json: string` — JSON-строка с `ParseOutput` (результат `parse`)
- `options?: string` — JSON-строка с `ExtractOptions`

**Возвращает:** `string` — JSON-строка с `ExtractedAssets`

**ExtractOptions:**
```typescript
interface ExtractOptions {
  extract_images?: boolean;      // извлекать изображения, по умолчанию true
  convert_to_webp?: boolean;     // конвертировать в WebP, по умолчанию true
  webp_quality?: number;         // качество WebP 0-100, по умолчанию 85
  extract_fonts?: boolean;       // извлекать шрифты, по умолчанию true
  resolve_external?: boolean;    // разрешать внешние ссылки, по умолчанию false
}
```

**Пример:**
```typescript
import { parse, extractAssets } from 'svg2web-wasm';

const svg = '<svg><image href="data:image/png;base64,iVBORw0KGgo..."/></svg>';
const parsed = parse(svg);

const options = JSON.stringify({
  extract_images: true,
  convert_to_webp: true,
  webp_quality: 80
});

const assets = extractAssets(parsed, options);
const extracted = JSON.parse(assets);
console.log(extracted.images); // массив извлеченных изображений в WebP
```

### `generate`

```typescript
function generate(json: string, options: string): string
```

Генерация кода из промежуточного JSON.

**Параметры:**
- `json: string` — JSON-строка с `ParseOutput`
- `options: string` — JSON-строка с `GenerateOptions`

**Возвращает:** `string` — JSON-строка с `GeneratedCode`

**GenerateOptions:**
```typescript
interface GenerateOptions {
  framework: 'react' | 'vue' | 'vanilla';
  styling: 'native' | 'tailwind' | 'scoped';
  responsive: boolean;
  image_quality: number;      // 0-100
  components: {
    detect: boolean;
    min_size: number;
    naming: 'pascal' | 'kebab' | 'snake';
  };
  output: {
    pretty: boolean;
    typescript: boolean;
    separate_css: boolean;
    separate_js: boolean;
  };
}
```

**Пример:**
```typescript
import { parse, generate } from 'svg2web-wasm';

const svg = '<svg><circle cx="50" cy="50" r="40" fill="red"/></svg>';
const parsed = parse(svg);

const options = JSON.stringify({
  framework: 'react',
  styling: 'scoped',
  responsive: true,
  output: {
    pretty: true,
    typescript: true,
    separate_css: true,
    separate_js: false
  }
});

const code = generate(parsed, options);
const generated = JSON.parse(code);
console.log(generated.html); // React компонент
```

### `convert`

```typescript
function convert(svg: Uint8Array | string, options: string): string
```

Полный цикл: парсинг → генерация кода.

**Параметры:**
- `svg: Uint8Array | string` — содержимое SVG
- `options: string` — JSON-строка с `GenerateOptions`

**Возвращает:** `string` — JSON-строка с `GeneratedCode`

**Пример:**
```typescript
import { convert } from 'svg2web-wasm';

const svg = '<svg><rect width="100" height="100" fill="blue"/></svg>';

const options = JSON.stringify({
  framework: 'vue',
  styling: 'scoped',
  components: { detect: true, min_size: 2 }
});

const result = convert(svg, options);
const output = JSON.parse(result);
console.log(output.html);     // Vue SFC
console.log(output.css);      // Scoped styles
```

### `setPanicHook`

```typescript
function setPanicHook(): void
```

Установка хука паники для отладки. Выводит стектрейс в консоль при панике в Rust.

**Пример:**
```typescript
import { setPanicHook } from 'svg2web-wasm';

// Вызвать один раз при инициализации
setPanicHook();

// Теперь при панике будет подробный вывод в консоль
try {
  parse(invalidSvg);
} catch (e) {
  // В консоли будет Rust backtrace
}
```

## Обработка ошибок

Все функции выбрасывают JavaScript ошибки при неудаче:

```typescript
import { parse, convert } from 'svg2web-wasm';

try {
  const result = parse('<invalid>svg</invalid>');
} catch (error) {
  console.error(error.type);     // 'SvgParse'
  console.error(error.message);  // 'expected opening tag'
  console.error(error.line);     // 1
}

try {
  const result = convert(svg, JSON.stringify(options));
} catch (error) {
  if (error.type === 'Unsupported') {
    console.error(`Unsupported feature: ${error.details}`);
  } else if (error.type === 'OomError') {
    console.error('Out of memory, try using StringPool');
  }
}
```

## Полный пример

```typescript
import { 
  parse, 
  optimize, 
  extractAssets, 
  generate, 
  convert,
  setPanicHook,
  StringPool 
} from 'svg2web-wasm';

// Включить отладку
setPanicHook();

async function processSVG(svgFile: File) {
  const svgText = await svgFile.text();
  
  try {
    // 1. Парсинг
    const parsed = parse(svgText);
    
    // 2. Оптимизация
    const optimized = optimize(parsed, JSON.stringify({
      simplify_paths: true,
      remove_comments: true
    }));
    
    // 3. Извлечение ресурсов
    const assets = extractAssets(optimized, JSON.stringify({
      extract_images: true,
      convert_to_webp: true,
      webp_quality: 85
    }));
    
    // 4. Генерация React компонента
    const code = generate(optimized, JSON.stringify({
      framework: 'react',
      styling: 'scoped',
      responsive: true,
      output: {
        typescript: true,
        pretty: true
      }
    }));
    
    const output = JSON.parse(code);
    return output;
    
  } catch (error) {
    console.error('Processing failed:', error);
    throw error;
  }
}

// Использование с большими файлами
async function processLargeSVG(svgFile: File) {
  const pool = new StringPool();
  const svgText = await svgFile.text();
  
  try {
    const handle = pool.allocate(svgText);
    
    // Читаем по частям
    const chunks: string[] = [];
    let offset = 0;
    const chunkSize = 1024 * 1024; // 1MB
    
    while (true) {
      const chunk = pool.read_chunk(handle, offset, chunkSize);
      if (!chunk) break;
      chunks.push(chunk);
      offset += chunkSize;
    }
    
    const result = convert(chunks.join(''), JSON.stringify({
      framework: 'vanilla',
      styling: 'tailwind'
    }));
    
    pool.free(handle);
    return JSON.parse(result);
    
  } catch (error) {
    if (error.type === 'OomError') {
      console.error('Out of memory, try reducing SVG size');
    }
    throw error;
  }
}
```
```