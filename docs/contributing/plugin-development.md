```markdown
# Разработка плагинов

## FormatRenderer Trait

```rust
use svg2web_generator::registry::{FormatRenderer, RenderContext, RenderedOutput};
use svg2web_generator::Result;

pub struct MyRenderer;

impl FormatRenderer for MyRenderer {
    /// Имя формата (используется в CLI: --format myformat)
    fn name(&self) -> &str {
        "myformat"
    }
    
    /// Генерация кода из контекста
    fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput> {
        // Генерация файлов
        let files = vec![
            OutputFile {
                name: "index.html".to_string(),
                content: generate_html(ctx)?,
                path: "".to_string(),
            },
            OutputFile {
                name: "component.js".to_string(),
                content: generate_js(ctx)?,
                path: "src/".to_string(),
            },
        ];
        
        Ok(RenderedOutput {
            files,
            main_entry: "index.html".to_string(),
        })
    }
}
```

## Структура плагина

### Cargo.toml

```toml
[package]
name = "svg2web-svelte"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Для динамической загрузки

[dependencies]
svg2web-generator = { path = "../../crates/svg2web-generator" }
anyhow = "1.0"
```

### src/lib.rs

```rust
use svg2web_generator::registry::{FormatRenderer, RenderContext, RenderedOutput};
use svg2web_generator::Result;

struct SvelteRenderer;

impl FormatRenderer for SvelteRenderer {
    fn name(&self) -> &str {
        "svelte"
    }
    
    fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput> {
        let mut files = Vec::new();
        
        // Генерация Svelte компонента
        let component = generate_svelte_component(ctx)?;
        files.push(OutputFile {
            name: "Icon.svelte".to_string(),
            content: component,
            path: "src/".to_string(),
        });
        
        // Генерация индекса
        let index = generate_index_html(ctx)?;
        files.push(OutputFile {
            name: "index.html".to_string(),
            content: index,
            path: "".to_string(),
        });
        
        Ok(RenderedOutput {
            files,
            main_entry: "src/Icon.svelte".to_string(),
        })
    }
}

fn generate_svelte_component(ctx: &RenderContext) -> Result<String> {
    let mut code = String::new();
    
    // <script>
    code.push_str("<script>\n");
    code.push_str("  export let size = 24;\n");
    code.push_str("  export let color = 'currentColor';\n");
    code.push_str("</script>\n\n");
    
    // <template>
    code.push_str("<svg\n");
    code.push_str("  width={size}\n");
    code.push_str("  height={size}\n");
    code.push_str("  viewBox=\"0 0 100 100\"\n");
    code.push_str("  fill={color}\n");
    code.push_str(">\n");
    
    // Вставляем SVG элементы
    for element in &ctx.elements {
        code.push_str(&render_element(element, 1)?);
    }
    
    code.push_str("</svg>\n\n");
    
    // <style>
    code.push_str("<style>\n");
    code.push_str("  svg {\n");
    code.push_str("    display: inline-block;\n");
    code.push_str("  }\n");
    code.push_str("</style>\n");
    
    Ok(code)
}

// Экспорт для динамической загрузки
#[no_mangle]
pub extern "C" fn create_renderer() -> *mut dyn FormatRenderer {
    let renderer = Box::new(SvelteRenderer);
    Box::into_raw(renderer)
}

#[no_mangle]
pub extern "C" fn destroy_renderer(ptr: *mut dyn FormatRenderer) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}
```

## Сборка плагина

```bash
# Сборка release версии
cargo build --release

# Результат: target/release/libsvg2web_svelte.so (Linux)
#           target/release/libsvg2web_svelte.dylib (macOS)
#           target/release/svg2web_svelte.dll (Windows)
```

## Динамическая загрузка

### Загрузка в Rust

```rust
use libloading::{Library, Symbol};

fn load_plugin(path: &str) -> Result<Box<dyn FormatRenderer>> {
    unsafe {
        let lib = Library::new(path)?;
        
        let create: Symbol<fn() -> *mut dyn FormatRenderer> = lib.get(b"create_renderer")?;
        let destroy: Symbol<fn(*mut dyn FormatRenderer)> = lib.get(b"destroy_renderer")?;
        
        let renderer_ptr = create();
        let renderer = Box::from_raw(renderer_ptr);
        
        // Сохраняем библиотеку и destroy функцию для cleanup
        // ...
        
        Ok(renderer)
    }
}
```

### Регистрация в FormatRegistry

```rust
use svg2web_generator::registry::FormatRegistry;

fn register_plugins(registry: &mut FormatRegistry) -> Result<()> {
    // Статическая регистрация (встроенные плагины)
    registry.register(Box::new(ReactRenderer));
    registry.register(Box::new(VueRenderer));
    
    // Динамическая загрузка из директории
    let plugins_dir = std::env::var("SVG2WEB_PLUGINS")
        .unwrap_or_else(|_| "./plugins".to_string());
    
    for entry in std::fs::read_dir(plugins_dir)? {
        let path = entry?.path();
        if path.extension().map_or(false, |ext| ext == "so" || ext == "dylib" || ext == "dll") {
            if let Ok(renderer) = load_plugin(path.to_str().unwrap()) {
                registry.register(renderer);
            }
        }
    }
    
    Ok(())
}
```

## Пример: Svelte плагин

```rust
// plugins/svelte/src/lib.rs

use svg2web_generator::registry::{FormatRenderer, RenderContext, RenderedOutput, OutputFile};
use svg2web_generator::Result;

pub struct SvelteRenderer;

impl FormatRenderer for SvelteRenderer {
    fn name(&self) -> &str {
        "svelte"
    }
    
    fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput> {
        let mut files = Vec::new();
        
        // Генерация .svelte файла
        let svelte_code = format!(
            r#"<script>
  export let size = {};
  export let color = '{}';
</script>

<svg width={{size}} height={{size}} viewBox="0 0 {} {}" fill={{color}}>
{}
</svg>

<style>
  svg {{
    display: inline-block;
  }}
</style>"#,
            ctx.options.default_size.unwrap_or(24),
            ctx.options.default_color.unwrap_or("currentColor".to_string()),
            ctx.canvas.width,
            ctx.canvas.height,
            render_svg_elements(ctx)?
        );
        
        files.push(OutputFile {
            name: "Icon.svelte".to_string(),
            content: svelte_code,
            path: "src/".to_string(),
        });
        
        Ok(RenderedOutput {
            files,
            main_entry: "src/Icon.svelte".to_string(),
        })
    }
}

#[no_mangle]
pub extern "C" fn create_renderer() -> *mut dyn FormatRenderer {
    let renderer = Box::new(SvelteRenderer);
    Box::into_raw(renderer)
}
```

## Пример: Solid плагин

```rust
// plugins/solid/src/lib.rs

use svg2web_generator::registry::{FormatRenderer, RenderContext, RenderedOutput, OutputFile};
use svg2web_generator::Result;

pub struct SolidRenderer;

impl FormatRenderer for SolidRenderer {
    fn name(&self) -> &str {
        "solid"
    }
    
    fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput> {
        let mut files = Vec::new();
        
        // Генерация .jsx файла с Solid signals
        let solid_code = format!(
            r#"import {{ createSignal }} from 'solid-js';

export function Icon(props) {{
  const [size] = createSignal(props.size || {});
  const [color] = createSignal(props.color || '{}');

  return (
    <svg width={{size()}} height={{size()}} viewBox="0 0 {} {}" fill={{color()}}>
      {}
    </svg>
  );
}}"#,
            ctx.options.default_size.unwrap_or(24),
            ctx.options.default_color.unwrap_or("currentColor".to_string()),
            ctx.canvas.width,
            ctx.canvas.height,
            render_jsx_elements(ctx)?
        );
        
        files.push(OutputFile {
            name: "Icon.jsx".to_string(),
            content: solid_code,
            path: "src/".to_string(),
        });
        
        Ok(RenderedOutput {
            files,
            main_entry: "src/Icon.jsx".to_string(),
        })
    }
}

#[no_mangle]
pub extern "C" fn create_renderer() -> *mut dyn FormatRenderer {
    let renderer = Box::new(SolidRenderer);
    Box::into_raw(renderer)
}
```

## Тестирование плагинов

### Интеграционный тест

```rust
// tests/plugin_test.rs

use svg2web_generator::registry::{FormatRegistry, RenderContext};
use svg2web_core::{parse, ParseOptions};

#[test]
fn test_svelte_plugin() {
    // Загружаем плагин
    let mut registry = FormatRegistry::new();
    let plugin = load_plugin("target/release/libsvg2web_svelte.so").unwrap();
    registry.register(plugin);
    
    // Создаем тестовый контекст
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100" fill="red"/></svg>"#;
    let parsed = parse(svg, ParseOptions::default()).unwrap();
    
    let ctx = RenderContext {
        elements: vec![parsed.structure],
        analysis: parsed.analysis,
        options: Default::default(),
        assets: parsed.assets,
    };
    
    // Генерируем код
    let renderer = registry.get("svelte").unwrap();
    let output = renderer.render(&ctx).unwrap();
    
    // Проверяем результат
    assert_eq!(output.files.len(), 1);
    assert!(output.files[0].content.contains("<script>"));
    assert!(output.files[0].content.contains("export let size"));
    assert!(output.files[0].content.contains("<svg"));
}
```

### Snapshot тесты с insta

```rust
#[test]
fn test_svelte_snapshot() {
    let ctx = create_test_context();
    let renderer = SvelteRenderer;
    let output = renderer.render(&ctx).unwrap();
    
    insta::assert_snapshot!("svelte-component", output.files[0].content);
}
```

## CLI использование

```bash
# Сборка плагина
cd plugins/svelte
cargo build --release

# Использование с CLI
svg2web build input.svg --format svelte --output ./output

# С указанием директории плагинов
SVG2WEB_PLUGINS=./plugins svg2web build input.svg --format solid
```

## API для плагинов

### RenderContext

```rust
pub struct RenderContext {
    /// SVG элементы
    pub elements: Vec<SVGElement>,
    
    /// Результат анализа
    pub analysis: AnalysisResult,
    
    /// Опции формата
    pub options: FormatOptions,
    
    /// Извлеченные ресурсы
    pub assets: Vec<Asset>,
    
    /// Размеры холста
    pub canvas: Canvas,
}
```

### RenderedOutput

```rust
pub struct RenderedOutput {
    /// Сгенерированные файлы
    pub files: Vec<OutputFile>,
    
    /// Точка входа (главный файл)
    pub main_entry: String,
}

pub struct OutputFile {
    /// Имя файла
    pub name: String,
    
    /// Содержимое
    pub content: String,
    
    /// Относительный путь (например "src/", "components/")
    pub path: String,
}
```

### FormatOptions

```rust
pub struct FormatOptions {
    /// Тип скрипта (module, script)
    pub script_type: ScriptType,
    
    /// CSS препроцессор
    pub css_preprocessor: Option<CssPreprocessor>,
    
    /// Размер по умолчанию
    pub default_size: Option<u32>,
    
    /// Цвет по умолчанию
    pub default_color: Option<String>,
    
    /// Дополнительные опции
    pub extra: HashMap<String, serde_json::Value>,
}
```