# Типы JS/TS

## ParseOutput

```typescript
/**
 * Результат парсинга SVG в промежуточное представление
 */
interface ParseOutput {
  /** Метаданные SVG */
  meta: Meta;
  
  /** Древовидная структура SVG элементов */
  structure: SVGElement;
  
  /** Геометрические данные элементов */
  geometry: Geometry;
  
  /** Стили и визуальные атрибуты */
  styles: Styles;
  
  /** Извлеченные ресурсы */
  assets: Assets;
  
  /** Структурированный контент */
  content: Content;
}

/**
 * Метаданные SVG документа
 */
interface Meta {
  /** Имя исходного файла */
  source: string;
  
  /** Время парсинга (ISO 8601) */
  parsed_at: string;
  
  /** Размеры холста */
  canvas: {
    width: number;
    height: number;
  };
  
  /** ViewBox SVG [x, y, width, height] */
  viewBox: [number, number, number, number];
  
  /** Заголовок SVG */
  title: string | null;
  
  /** Описание SVG */
  description: string | null;
}

/**
 * SVG элемент
 */
interface SVGElement {
  /** Имя тега (rect, circle, path, g и т.д.) */
  tag: string;
  
  /** Атрибуты элемента */
  attributes: Record<string, string>;
  
  /** Дочерние элементы */
  children: SVGElement[];
  
  /** Уникальный идентификатор */
  id?: string;
  
  /** CSS классы */
  class: string[];
  
  /** Текстовое содержимое (для text элементов) */
  text_content?: string;
}

/**
 * Геометрические данные
 */
interface Geometry {
  /** Границы элементов по id */
  bounds: Record<string, Bounds>;
  
  /** Данные path элементов */
  paths: Record<string, PathData>;
  
  /** Трансформации элементов */
  transforms: Record<string, Transform>;
  
  /** Именованные точки */
  points: Record<string, Point>;
}

interface Bounds {
  x: number;
  y: number;
  width: number;
  height: number;
  rx?: number;
  ry?: number;
}

interface PathData {
  type: 'path';
  d: string;
  fill_rule?: 'nonzero' | 'evenodd';
  bounds: [number, number, number, number];
}

interface Transform {
  translate?: [number, number];
  rotate?: number;
  scale?: [number, number];
  matrix?: [number, number, number, number, number, number];
}

interface Point {
  x: number;
  y: number;
}

/**
 * Стили и визуальные атрибуты
 */
interface Styles {
  /** Цвета (CSS переменные) */
  colors: Record<string, string>;
  
  /** Градиенты */
  gradients: Record<string, Gradient>;
  
  /** Используемые шрифты */
  fonts: FontInfo[];
  
  /** Типографические стили */
  typography: Record<string, TypographyStyle>;
  
  /** Эффекты (тени, размытие) */
  effects: Record<string, Effect>;
  
  /** Обводки */
  strokes: Record<string, Stroke>;
}

interface Gradient {
  type: 'linear' | 'radial';
  x1?: number;
  y1?: number;
  x2?: number;
  y2?: number;
  cx?: number;
  cy?: number;
  r?: number;
  stops: Array<[string, number]>; // [color, offset]
}

interface FontInfo {
  family: string;
  weights: number[];
  styles: string[];
  fallback: string[];
}

interface TypographyStyle {
  family: string;
  size: number;
  weight: number;
  line_height: number;
  letter_spacing?: number;
  color?: string;
}

interface Effect {
  type: 'drop_shadow' | 'blur';
  blur_radius?: number;
  offset_x?: number;
  offset_y?: number;
  color?: string;
}

interface Stroke {
  width: number;
  color: string;
  dasharray?: number[];
  linecap: 'butt' | 'round' | 'square';
  linejoin: 'miter' | 'round' | 'bevel';
}

/**
 * Извлеченные ресурсы
 */
interface Assets {
  /** Изображения */
  images: Record<string, ImageAsset>;
  
  /** Иконки */
  icons: Record<string, IconAsset>;
  
  /** Шрифты */
  fonts: Record<string, FontAsset>;
  
  /** Внешние ресурсы */
  external: ExternalAssets;
}

interface ImageAsset {
  original: {
    data: string;        // base64
    mime: string;
    width: number;
    height: number;
    size_kb: number;
  };
  variants: {
    '1x'?: Variant;
    '2x'?: Variant;
    default?: Variant;
  };
  used_by: string[];     // id элементов, использующих изображение
}

interface Variant {
  file?: string;         // имя файла для сохранения
  inline?: string;       // base64 для inline использования
  width?: number;
  height?: number;
  size_kb?: number;
}

interface IconAsset {
  source: string;
  type: 'sprite' | 'external' | 'inline';
  used_by: string[];
}

interface FontAsset {
  files: {
    woff2?: string;
    woff?: string;
    ttf?: string;
  };
  weights: number[];
  styles: string[];
}

interface ExternalAssets {
  stylesheets: string[];
  scripts: string[];
}

/**
 * Структурированный контент
 */
interface Content {
  /** Логотип */
  logo?: string;
  
  /** Слоган */
  tagline?: string;
  
  /** Навигация */
  navigation?: {
    main: NavItem[];
    secondary?: NavItem[];
  };
  
  /** Контактная информация */
  contact?: {
    phone?: string;
    phone_formatted?: string;
    email?: string;
    address?: string;
    work_hours?: string;
  };
  
  /** Hero секция */
  hero?: {
    title: string;
    subtitle?: string;
    description?: string;
    cta?: string;
    cta_link?: string;
    secondary_cta?: string;
    image_alt?: string;
  };
  
  /** Карточки */
  cards?: Card[];
  
  /** Футер */
  footer?: {
    copyright?: string;
    social?: SocialLink[];
    legal?: LegalLink[];
  };
  
  /** Локализация */
  localization?: {
    locale: string;
    currency: string;
    currency_symbol: string;
    date_format: string;
  };
}

interface NavItem {
  label: string;
  href: string;
  active?: boolean;
  icon?: string;
}

interface Card {
  id: string;
  title: string;
  description: string;
  icon?: string;
  cta?: string;
  badge?: string;
  stats?: Record<string, string | number>;
}

interface SocialLink {
  platform: string;
  url: string;
}

interface LegalLink {
  label: string;
  href: string;
}
```

## GenerateOptions

```typescript
/**
 * Опции генерации кода
 */
interface GenerateOptions {
  /** Целевой фреймворк */
  framework: 'react' | 'vue' | 'vanilla';
  
  /** Способ стилизации */
  styling: 'native' | 'tailwind' | 'scoped';
  
  /** Генерировать адаптивную верстку */
  responsive: boolean;
  
  /** Качество изображений */
  imageQuality: 'low' | 'medium' | 'high' | 'original';
  
  /** Настройки детекции компонентов */
  components: {
    /** Включить автоматическую детекцию */
    detect: boolean;
    
    /** Минимальный размер компонента (количество узлов) */
    minSize: number;
    
    /** Схема именования: 'pascal', 'kebab', 'snake' */
    naming: string;
  };
  
  /** Настройки вывода */
  output: {
    /** Форматировать код */
    pretty: boolean;
    
    /** Генерировать TypeScript типы */
    typescript: boolean;
    
    /** CSS в отдельный файл */
    separateCss: boolean;
    
    /** JS в отдельный файл */
    separateJs: boolean;
    
    /** Source maps */
    sourceMaps: boolean;
  };
}
```

## GeneratedCode

```typescript
/**
 * Результат генерации кода
 */
interface GeneratedCode {
  /** HTML контент или основной компонент */
  html: string;
  
  /** CSS стили */
  css: string | null;
  
  /** JavaScript код */
  js: string | null;
  
  /** Сгенерированные компоненты */
  components: ComponentCode[];
  
  /** Извлеченные ресурсы */
  assets: AssetFile[];
  
  /** Метаданные генерации */
  metadata: {
    generated_at: string;
    version: string;
    components_count: number;
    total_size_bytes: number;
  };
}

/**
 * Сгенерированный компонент
 */
interface ComponentCode {
  /** Имя компонента */
  name: string;
  
  /** Целевой фреймворк */
  framework: 'react' | 'vue' | 'vanilla';
  
  /** Исходный код компонента */
  code: string;
  
  /** Относительный путь для сохранения */
  path: string;
  
  /** Зависимости компонента */
  dependencies: string[];
}

/**
 * Файл ресурса
 */
interface AssetFile {
  /** Имя файла */
  filename: string;
  
  /** Бинарные данные (base64 или Uint8Array) */
  data: string | Uint8Array;
  
  /** MIME тип */
  mime_type: string;
  
  /** Относительный путь */
  path: string;
}
```

## AnalysisResult

```typescript
/**
 * Результат анализа SVG структуры
 */
interface AnalysisResult {
  /** Детектированные компоненты */
  components: Component[];
  
  /** Метрики сложности */
  complexity: ComplexityMetrics;
  
  /** Информация об иерархии */
  hierarchy: HierarchyInfo;
}

/**
 * Детектированный компонент
 */
interface Component {
  /** Уникальный идентификатор компонента */
  id: string;
  
  /** Шаблон компонента (SVG структура) */
  template: SVGElement;
  
  /** Места использования компонента */
  occurrences: Occurrence[];
}

interface Occurrence {
  /** Путь к элементу в дереве */
  path: string;
  
  /** Позиция в исходном SVG (если доступна) */
  position?: {
    line: number;
    column: number;
  };
}

/**
 * Метрики сложности SVG
 */
interface ComplexityMetrics {
  /** Общее количество узлов */
  node_count: number;
  
  /** Количество path элементов */
  path_count: number;
  
  /** Сложность path (сумма команд) */
  path_complexity: number;
  
  /** Количество градиентов */
  gradient_count: number;
  
  /** Итоговая оценка сложности (0-100) */
  score: number;
  
  /** Оценочное время рендеринга (мс) */
  estimated_render_time_ms: number;
}

/**
 * Информация об иерархии
 */
interface HierarchyInfo {
  /** Максимальная глубина дерева */
  depth: number;
  
  /** Максимальная ширина (количество элементов на уровне) */
  max_width: number;
  
  /** Критические пути рендеринга */
  critical_paths: string[][];
  
  /** Проблемные места (глубокая вложенность) */
  bottlenecks?: string[];
}
```

## OptimizerConfig

```typescript
/**
 * Конфигурация оптимизации
 */
interface OptimizerConfig {
  /** Упрощение путей (Douglas-Peucker) */
  simplify_paths?: boolean;
  
  /** Удаление дублирующихся элементов */
  deduplicate?: boolean;
  
  /** Минификация id */
  minify_ids?: boolean;
  
  /** Удаление комментариев */
  remove_comments?: boolean;
  
  /** Точность координат (количество знаков) */
  precision?: number;
  
  /** Порог упрощения (0-1, меньше = агрессивнее) */
  simplification_tolerance?: number;
}
```

## ExtractOptions

```typescript
/**
 * Конфигурация извлечения ресурсов
 */
interface ExtractOptions {
  /** Извлекать изображения */
  extract_images?: boolean;
  
  /** Конвертировать в WebP */
  convert_to_webp?: boolean;
  
  /** Качество WebP (0-100) */
  webp_quality?: number;
  
  /** Извлекать шрифты */
  extract_fonts?: boolean;
  
  /** Разрешать внешние ссылки */
  resolve_external?: boolean;
  
  /** Таймаут для внешних запросов (мс) */
  timeout_ms?: number;
}
```
