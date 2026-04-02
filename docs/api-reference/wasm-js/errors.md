# Обработка ошибок в JS

## Формат ошибок из Rust

Ошибки из Rust конвертируются в JavaScript объекты с единой структурой:

```typescript
interface RustError {
  type: 'SvgParse' | 'InvalidSvg' | 'Unsupported' | 'NetworkError' | 'OomError';
  message: string;
  line?: number;           // номер строки для ошибок парсинга
  details?: string;        // дополнительная информация
}
```

**Пример объекта ошибки:**
```javascript
{
  type: 'SvgParse',
  message: 'unexpected closing tag </rect> at line 42',
  line: 42
}
```

## Кастомные классы ошибок

### `ParseError`

Ошибка парсинга SVG синтаксиса.

```typescript
class ParseError extends Error {
  constructor(message: string, public line?: number) {
    super(message);
    this.name = 'ParseError';
  }
}
```

### `ValidationError`

Ошибка валидации SVG (отсутствие обязательных атрибутов).

```typescript
class ValidationError extends Error {
  constructor(message: string, public element?: string) {
    super(message);
    this.name = 'ValidationError';
  }
}
```

### `NotFoundError`

Ресурс не найден (шрифт, изображение).

```typescript
class NotFoundError extends Error {
  constructor(message: string, public resource?: string) {
    super(message);
    this.name = 'NotFoundError';
  }
}
```

### `UnsupportedFeatureError`

Неподдерживаемая SVG фича.

```typescript
class UnsupportedFeatureError extends Error {
  constructor(message: string, public feature?: string) {
    super(message);
    this.name = 'UnsupportedFeatureError';
  }
}
```

### `OomError`

Ошибка нехватки памяти (Out of Memory).

```typescript
class OomError extends Error {
  constructor(message: string, public size?: number) {
    super(message);
    this.name = 'OomError';
  }
}
```

## Преобразование ошибок

```typescript
function toJsError(rustError: RustError): Error {
  switch (rustError.type) {
    case 'SvgParse':
      return new ParseError(rustError.message, rustError.line);
    case 'InvalidSvg':
      return new ValidationError(rustError.message);
    case 'Unsupported':
      return new UnsupportedFeatureError(rustError.message, rustError.details);
    case 'NetworkError':
      return new NotFoundError(rustError.message);
    case 'OomError':
      return new OomError(rustError.message);
    default:
      return new Error(rustError.message);
  }
}
```

## Retry стратегии

### NetworkError для внешних ресурсов

```typescript
async function fetchWithRetry<T>(
  fn: () => Promise<T>,
  maxRetries: number = 3,
  delay: number = 1000
): Promise<T> {
  for (let i = 0; i < maxRetries; i++) {
    try {
      return await fn();
    } catch (error) {
      if (error instanceof NotFoundError && i < maxRetries - 1) {
        await new Promise(resolve => setTimeout(resolve, delay * Math.pow(2, i)));
        continue;
      }
      throw error;
    }
  }
  throw new Error('Max retries exceeded');
}

// Использование
const result = await fetchWithRetry(() => extract_assets(svg));
```

### OOM для больших SVG через StringPool

```typescript
import { parse_svg, StringPool } from 'svg2web-wasm';

function parseLargeSVG(svg: string, pool: StringPool): any {
  try {
    return parse_svg(svg);
  } catch (error) {
    if (error instanceof OomError) {
      // Используем StringPool для чанковой обработки
      const handle = pool.allocate(svg);
      
      try {
        // Читаем и обрабатываем по частям
        const chunks: string[] = [];
        const chunkSize = 1024 * 1024; // 1MB
        let offset = 0;
        
        while (true) {
          const chunk = pool.read_chunk(handle, offset, chunkSize);
          if (!chunk) break;
          chunks.push(chunk);
          offset += chunkSize;
        }
        
        return parse_svg(chunks.join(''));
      } finally {
        pool.free(handle);
      }
    }
    throw error;
  }
}
```

## Примеры try/catch

### Базовый пример

```typescript
import { parse_svg, analyze_svg, optimize_svg } from 'svg2web-wasm';

try {
  const parsed = parse_svg(svgContent);
  const analysis = analyze_svg(svgContent);
  const optimized = optimize_svg(svgContent, { simplify_paths: true });
  
  console.log('Success:', { parsed, analysis, optimized });
} catch (error) {
  if (error instanceof ParseError) {
    console.error(`Parse error at line ${error.line}: ${error.message}`);
  } else if (error instanceof ValidationError) {
    console.error(`Invalid SVG: ${error.message}`);
  } else if (error instanceof UnsupportedFeatureError) {
    console.error(`Unsupported feature: ${error.feature} - ${error.message}`);
  } else {
    console.error('Unexpected error:', error);
  }
}
```

### С типизацией и guard-функциями

```typescript
interface ErrorWithType {
  type: string;
  message: string;
  line?: number;
}

function isParseError(error: unknown): error is ParseError {
  return error instanceof Error && error.name === 'ParseError';
}

function isValidationError(error: unknown): error is ValidationError {
  return error instanceof Error && error.name === 'ValidationError';
}

function isUnsupportedError(error: unknown): error is UnsupportedFeatureError {
  return error instanceof Error && error.name === 'UnsupportedFeatureError';
}

async function safeParse(svg: string) {
  try {
    return { success: true, data: parse_svg(svg) };
  } catch (err) {
    const error = err as ErrorWithType;
    
    if (isParseError(err)) {
      return {
        success: false,
        error: 'PARSE_ERROR',
        message: error.message,
        line: error.line
      };
    }
    
    if (isValidationError(err)) {
      return {
        success: false,
        error: 'VALIDATION_ERROR',
        message: error.message
      };
    }
    
    if (isUnsupportedError(err)) {
      return {
        success: false,
        error: 'UNSUPPORTED_FEATURE',
        message: error.message
      };
    }
    
    return {
      success: false,
      error: 'UNKNOWN_ERROR',
      message: error.message
    };
  }
}

// Использование
const result = await safeParse(largeSvg);
if (!result.success) {
  switch (result.error) {
    case 'PARSE_ERROR':
      // Показать пользователю строку с ошибкой
      highlightErrorLine(result.line!);
      break;
    case 'UNSUPPORTED_FEATURE':
      // Предложить альтернативу
      suggestAlternative();
      break;
    default:
      showErrorMessage(result.message);
  }
}
```

### С обработкой в Web Worker

```typescript
// worker.ts
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
      case 'OPTIMIZE':
        result = optimize_svg(svg, e.data.config);
        break;
    }
    
    self.postMessage({ id, success: true, result });
  } catch (error) {
    self.postMessage({
      id,
      success: false,
      error: {
        type: error.type || 'UnknownError',
        message: error.message,
        line: error.line,
        details: error.details
      }
    });
  }
});

// main.ts
class SVGProcessor {
  private worker: Worker;
  private pending = new Map<number, { resolve: Function; reject: Function }>();
  private nextId = 0;

  constructor() {
    this.worker = new Worker(new URL('./worker.ts', import.meta.url));
    this.worker.addEventListener('message', this.handleMessage.bind(this));
  }

  private handleMessage(e: MessageEvent) {
    const { id, success, result, error } = e.data;
    const pending = this.pending.get(id);
    
    if (pending) {
      if (success) {
        pending.resolve(result);
      } else {
        const jsError = this.toJsError(error);
        pending.reject(jsError);
      }
      this.pending.delete(id);
    }
  }

  private toJsError(error: RustError): Error {
    switch (error.type) {
      case 'SvgParse':
        return new ParseError(error.message, error.line);
      case 'InvalidSvg':
        return new ValidationError(error.message);
      case 'Unsupported':
        return new UnsupportedFeatureError(error.message, error.details);
      case 'OomError':
        return new OomError(error.message);
      default:
        return new Error(error.message);
    }
  }

  async parse(svg: string): Promise<any> {
    const id = this.nextId++;
    
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.worker.postMessage({ id, type: 'PARSE', svg });
    });
  }
}
```
