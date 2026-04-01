```markdown
# svg2web-generator

Крейт для генерации веб-кода из промежуточного представления SVG.

## Публичные функции

### `generate`

```rust
pub fn generate(parse_output: &ParseOutput, options: GenerateOptions) -> Result<GeneratedCode, Error>
```

Основная функция генерации кода.

**Параметры:**
- `parse_output: &ParseOutput` — результат парсинга из `svg2web-core`
- `options: GenerateOptions` — конфигурация генерации

**Возвращает:** `Result<GeneratedCode, Error>`

### `generate_from_file`

```rust
pub fn generate_from_file(json_path: &Path, options: GenerateOptions) -> Result<GeneratedCode, Error>
```

Генерация из JSON-файла промежуточного формата.

**Параметры:**
- `json_path: &Path` — путь к JSON-файлу с `ParseOutput`
- `options: GenerateOptions` — конфигурация генерации

## Типы данных

### `GenerateOptions`

```rust
pub struct GenerateOptions {
    pub framework: Framework,
    pub styling: Styling,
    pub responsive: bool,
    pub image_quality: u8,
    pub components: ComponentOptions,
    pub output: OutputOptions,
}
```

### `Framework`

```rust
pub enum Framework {
    React,      // TypeScript + functional components
    Vue,        // SFC + script setup
    Vanilla,    // чистый HTML/CSS/JS
}
```

### `Styling`

```rust
pub enum Styling {
    NativeCss,   // нативные CSS классы
    Tailwind,    // Tailwind CSS утилиты
    ScopedCss,   // Scoped CSS (Vue) или CSS Modules (React)
}
```

### `ComponentOptions`

```rust
pub struct ComponentOptions {
    pub detect: bool,           // включить детекцию компонентов
    pub min_size: usize,        // минимальный размер (узлов)
    pub naming: NamingScheme,   // схема именования
    pub max_depth: usize,       // макс. глубина вложенности
}

pub enum NamingScheme {
    PascalCase,  // IconButton
    KebabCase,   // icon-button
    SnakeCase,   // icon_button
}
```

### `OutputOptions`

```rust
pub struct OutputOptions {
    pub pretty: bool,           // форматировать код
    pub typescript: bool,       // генерировать TypeScript
    pub separate_css: bool,     // CSS в отдельный файл
    pub separate_js: bool,      // JS в отдельный файл
    pub source_maps: bool,      // source maps
}
```

### `GeneratedCode`

```rust
pub struct GeneratedCode {
    pub html: String,                           // HTML или основной компонент
    pub css: Option<String>,                   // CSS стили
    pub js: Option<String>,                    // JavaScript код
    pub components: Vec<ComponentCode>,        // сгенерированные компоненты
    pub assets: Vec<AssetFile>,                // ресурсы
    pub metadata: GenerationMetadata,          // метаданные
}
```

### `ComponentCode`

```rust
pub struct ComponentCode {
    pub name: String,
    pub framework: Framework,
    pub code: String,
    pub path: String,
    pub dependencies: Vec<String>,
}
```

### `AssetFile`

```rust
pub struct AssetFile {
    pub filename: String,
    pub data: Vec<u8>,
    pub mime_type: String,
    pub path: String,
}
```

### `GenerationMetadata`

```rust
pub struct GenerationMetadata {
    pub generated_at: DateTime<Utc>,
    pub version: String,
    pub components_count: usize,
    pub total_size_bytes: u64,
}
```

## Примеры

### React компонент

```rust
let output = generate(&parsed, GenerateOptions {
    framework: Framework::React,
    styling: Styling::ScopedCss,
    responsive: true,
    output: OutputOptions {
        typescript: true,
        ..Default::default()
    },
    ..Default::default()
})?;
```

### Vue SFC с детекцией компонентов

```rust
let output = generate(&parsed, GenerateOptions {
    framework: Framework::Vue,
    components: ComponentOptions {
        detect: true,
        min_size: 3,
        ..Default::default()
    },
    ..Default::default()
})?;
```

### Vanilla JS с Tailwind

```rust
let output = generate(&parsed, GenerateOptions {
    framework: Framework::Vanilla,
    styling: Styling::Tailwind,
    responsive: true,
    ..Default::default()
})?;
```
```