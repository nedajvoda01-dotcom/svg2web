```markdown
# svg2web-core

Библиотека для парсинга, анализа и оптимизации SVG. Ядро системы SVG2Web, предоставляющее низкоуровневый API для обработки SVG без зависимости от фреймворка или платформы.

## Публичные функции

### `parse`

```rust
pub fn parse(svg_bytes: &[u8]) -> Result<ParseOutput, Error>
```

Парсит SVG из байтового массива и возвращает структурированное промежуточное представление.

### `parse_file`

```rust
pub fn parse_file(path: &Path) -> Result<ParseOutput, Error>
```

Читает SVG из файла по указанному пути и парсит его. Обёртка над `parse` с файловым I/O.

### `parse_str`

```rust
pub fn parse_str(svg_str: &str) -> Result<ParseOutput, Error>
```

Парсит SVG из строки (UTF-8). Удобно для работы с SVG, полученным из текстовых источников.

### `optimize`

```rust
pub fn optimize(input: ParseOutput, config: &OptimizerConfig) -> Result<ParseOutput, Error>
```

Оптимизирует промежуточное представление SVG согласно конфигурации. Включает упрощение путей, дедупликацию элементов и минификацию.

### `extract_assets`

```rust
pub fn extract_assets(input: &ParseOutput, options: &ExtractOptions) -> Result<ExtractedAssets, Error>
```

Извлекает встроенные ресурсы из SVG: base64-изображения, шрифты и ссылки на внешние ресурсы.

### `serialize_to_json`

```rust
pub fn serialize_to_json(input: &ParseOutput) -> Result<String, Error>
```

Сериализует промежуточное представление в JSON-строку (6-секционный формат: meta, structure, geometry, styles, assets, content).

## Типы

### `ParseOutput`

Структура промежуточного представления SVG, содержащая все данные, необходимые для генерации кода.

```rust
pub struct ParseOutput {
    pub meta: Meta,           // Метаданные (source, parsed_at, canvas, viewBox)
    pub structure: Structure, // Дерево DOM элементов
    pub geometry: Geometry,   // Границы, пути, трансформации
    pub styles: Styles,       // Цвета, градиенты, шрифты
    pub assets: Assets,       // Изображения, иконки, шрифты
    pub content: Content,     // Текстовый контент для i18n
}
```

### `OptimizerConfig`

Конфигурация оптимизации SVG.

```rust
pub struct OptimizerConfig {
    pub simplify_paths: bool,      // Упрощение путей (Douglas-Peucker)
    pub deduplicate: bool,         // Удаление дубликатов элементов
    pub minify: bool,              // Минификация ID и атрибутов
    pub remove_comments: bool,     // Удаление XML комментариев
    pub precision: u8,             // Точность координат (знаков после запятой)
}
```

**Значения по умолчанию:**
- `simplify_paths`: true
- `deduplicate`: true
- `minify`: true
- `remove_comments`: true
- `precision`: 3

### `ExtractOptions`

Настройки извлечения ресурсов.

```rust
pub struct ExtractOptions {
    pub extract_images: bool,       // Извлекать base64 изображения
    pub convert_to_webp: bool,      // Конвертировать растровые изображения в WebP
    pub webp_quality: u8,           // Качество WebP (0-100, по умолчанию 85)
    pub extract_fonts: bool,        // Детектировать и извлекать информацию о шрифтах
    pub resolve_external: bool,     // Загружать внешние ресурсы по URL
}
```

## Модели

### `SVGElement`

Базовый элемент SVG DOM.

```rust
pub struct SVGElement {
    pub id: Option<String>,
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<SVGElement>,
    pub text_content: Option<String>,
}
```

### `Color`

Цвет в RGBA формате.

```rust
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
```

### `Gradient`

Градиентная заливка.

```rust
pub enum Gradient {
    Linear {
        x1: f64, y1: f64,
        x2: f64, y2: f64,
        stops: Vec<(Color, f64)>,
    },
    Radial {
        cx: f64, cy: f64,
        r: f64,
        stops: Vec<(Color, f64)>,
    },
}
```

### `Shadow`

Тень (drop shadow).

```rust
pub struct Shadow {
    pub x: f64,
    pub y: f64,
    pub blur: f64,
    pub color: Color,
}
```

### `Stroke`

Обводка элемента.

```rust
pub struct Stroke {
    pub width: f64,
    pub color: Color,
    pub linecap: LineCap,   // Butt, Round, Square
    pub linejoin: LineJoin, // Miter, Round, Bevel
}
```

### `Font`

Шрифт и типографические параметры.

```rust
pub struct Font {
    pub family: String,
    pub size: f64,
    pub weight: FontWeight, // 100-900 или Normal, Bold
    pub style: FontStyle,   // Normal, Italic, Oblique
}
```

### `ImageAsset`

Извлечённое изображение.

```rust
pub struct ImageAsset {
    pub id: String,
    pub format: ImageFormat, // Png, Jpeg, WebP, Svg
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub source: ImageSource, // Inline, External
}
```

### `FontAsset`

Обнаруженный шрифт.

```rust
pub struct FontAsset {
    pub family: String,
    pub weights: Vec<u16>,
    pub styles: Vec<String>,
    pub source: FontSource, // System, Google, Custom(File)
}
```

## Пример использования

```rust
use svg2web_core::{parse_str, optimize, extract_assets, serialize_to_json};
use svg2web_core::{OptimizerConfig, ExtractOptions};

fn process_svg(svg_content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Парсинг
    let parsed = parse_str(svg_content)?;
    
    // Оптимизация
    let config = OptimizerConfig {
        simplify_paths: true,
        deduplicate: true,
        minify: true,
        remove_comments: false,
        precision: 2,
    };
    let optimized = optimize(parsed, &config)?;
    
    // Извлечение ресурсов
    let extract_opts = ExtractOptions {
        extract_images: true,
        convert_to_webp: true,
        webp_quality: 85,
        extract_fonts: true,
        resolve_external: false,
    };
    let _assets = extract_assets(&optimized, &extract_opts)?;
    
    // Сериализация в JSON
    let json = serialize_to_json(&optimized)?;
    Ok(json)
}
```
```