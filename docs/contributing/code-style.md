```markdown
# Стиль кода

## Форматирование (cargo fmt)

Конфигурация в `rustfmt.toml`:

```toml
line_width = 100
tab_spaces = 4
edition = "2021"
use_small_heuristics = "Default"
reorder_imports = true
reorder_modules = true
group_imports = "StdExternalCrate"
```

**Правила:**
- Максимальная ширина строки: 100 символов
- Отступы: 4 пробела
- Импорты: сначала стандартная библиотека, затем внешние крейты, затем внутренние модули
- Модули сортируются алфавитно

**Проверка:**
```bash
cargo fmt --check
cargo fmt  # автоисправление
```

---

## Линтинг (clippy)

Конфигурация в `clippy.toml`:

```toml
deny = ["unwrap_used", "expect_used"]
allow = ["too_many_arguments"]
cognitive_complexity_threshold = 30
type_complexity_threshold = 300
too_many_arguments_threshold = 7
```

**Запрещенные lints:**
- `unwrap_used` — запрет на `.unwrap()` в production коде
- `expect_used` — запрет на `.expect()` в production коде

**Разрешенные:**
- `too_many_arguments` — до 7 аргументов

**Проверка:**
```bash
cargo clippy -- -D warnings
```

---

## Обработка ошибок

### В библиотечных крейтах (core, generator, cache)

Использовать `thiserror` для типизированных ошибок:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse error at line {line}: {message}")]
    ParseError {
        line: usize,
        message: String,
    },
    
    #[error("optimize error: {0}")]
    OptimizeError(String),
    
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
```

**Требования:**
- Каждая ошибка должна иметь контекст (строка, элемент, причина)
- Использовать `#[from]` для автоматической конвертации
- Не использовать `anyhow` в библиотеках

### В CLI

Разрешено использовать `anyhow` для удобства:

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = std::env::args().nth(1)
        .context("missing input file")?;
    
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    
    Ok(())
}
```

---

## Документация

### Rustdoc для публичных элементов

Каждый публичный элемент должен иметь документацию:

```rust
/// Парсит SVG строку в промежуточное представление.
///
/// # Arguments
/// * `svg` - Содержимое SVG файла
/// * `options` - Опции парсинга (нормализация, комментарии)
///
/// # Returns
/// `Result<ParseOutput, Error>` - структура с разобранным SVG
///
/// # Examples
/// ```
/// use svg2web_core::{parse, ParseOptions};
///
/// let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100"/></svg>"#;
/// let result = parse(svg, ParseOptions::default())?;
/// assert_eq!(result.meta.canvas.width, 100);
/// ```
pub fn parse(svg: &str, options: ParseOptions) -> Result<ParseOutput> {
    // ...
}
```

**Требования:**
- `# Examples` — runnable код, тестируется через `cargo test`
- `# Panics` — если функция может запаниковать
- `# Errors` — описание возможных ошибок
- `# Safety` — для unsafe функций

### README для каждого крейта

Каждый крейт должен содержать `README.md` с:
- Названием и описанием
- Примером использования
- Ссылкой на документацию
- Лицензией

---

## Запрещенные практики

### `unwrap()` и `expect()` в production

❌ **Запрещено:**
```rust
let value = map.get("key").unwrap();  // может запаниковать
let config = serde_json::from_str(json).expect("invalid json");
```

✅ **Правильно:**
```rust
let value = map.get("key").ok_or(Error::MissingKey)?;
let config = serde_json::from_str(json)
    .with_context(|| format!("invalid json: {}", json))?;
```

### `println!` в библиотеках

❌ **Запрещено в библиотечных крейтах:**
```rust
println!("Parsing SVG...");  // выводит в stdout пользователя
```

✅ **Использовать tracing:**
```rust
use tracing::{info, debug, warn};

info!("parsing SVG from file: {}", path);
debug!("found {} elements", elements.len());
warn!("unsupported feature: {}", feature);
```

### Хардкод путей

❌ **Запрещено:**
```rust
let cache_dir = "/tmp/svg2web";
```

✅ **Использовать стандартные пути:**
```rust
use directories::ProjectDirs;

let dirs = ProjectDirs::from("com", "svg2web", "svg2web")
    .context("failed to get cache directory")?;
let cache_dir = dirs.cache_dir();
```

---

## Именование

| Тип | Стиль | Пример |
|-----|-------|--------|
| Крейты | snake_case | `svg2web_core` |
| Модули | snake_case | `parser`, `optimizer` |
| Структуры | PascalCase | `ParseOptions`, `CacheKey` |
| Перечисления | PascalCase | `Error`, `Framework` |
| Варианты | PascalCase | `Error::ParseError` |
| Функции | snake_case | `parse_svg`, `optimize` |
| Методы | snake_case | `cache.get()`, `cache.set()` |
| Переменные | snake_case | `svg_content`, `result` |
| Константы | SCREAMING_SNAKE_CASE | `MAX_CACHE_SIZE` |
| Типы-примитивы | PascalCase | `Result<T>`, `Option<T>` |

---

## Структура модуля

```rust
//! Документация модуля

// Импорты
use std::path::Path;
use std::fs;

use thiserror::Error;
use tracing::info;

use crate::model::SVGElement;

// Константы
const DEFAULT_TOLERANCE: f64 = 0.5;

// Структуры
pub struct Optimizer {
    config: Config,
}

// Реализации
impl Optimizer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    
    pub fn optimize(&self, element: SVGElement) -> Result<SVGElement> {
        // ...
    }
}

// Функции
pub fn optimize(element: SVGElement) -> Result<SVGElement> {
    Optimizer::new(Config::default()).optimize(element)
}

// Приватные функции
fn simplify_path(path: &Path) -> Path {
    // ...
}

// Тесты
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_optimize() {
        // ...
    }
}
```

---

## Обработка ошибок в CLI

```rust
use anyhow::{Context, Result};

pub async fn run(args: BuildArgs) -> Result<()> {
    // Чтение с контекстом
    let svg = std::fs::read_to_string(&args.input)
        .with_context(|| format!("failed to read {}", args.input.display()))?;
    
    // Парсинг с преобразованием ошибки
    let parsed = svg2web_core::parse(&svg, ParseOptions::default())
        .map_err(|e| anyhow::anyhow!("parse error: {}", e))?;
    
    // Прогресс с логированием
    info!("parsed {} elements", parsed.structure.children.len());
    
    // Запись с контекстом
    std::fs::write(&args.output, result)
        .with_context(|| format!("failed to write {}", args.output.display()))?;
    
    Ok(())
}
```

---

## Асинхронный код

```rust
// Использовать tokio для async
use tokio::fs;

pub async fn read_file(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)
        .await
        .with_context(|| format!("failed to read {}", path.display()))?;
    Ok(content)
}

// Комбинировать sync и async
pub fn process_sync(input: &str) -> Result<String> {
    // синхронная обработка
}

pub async fn process_async(input: &str) -> Result<String> {
    tokio::task::spawn_blocking(move || process_sync(input))
        .await
        .context("blocking task failed")?
}
```

---

## Тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_parse_simple() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100"/></svg>"#;
        let result = parse(svg, ParseOptions::default()).unwrap();
        
        assert_eq!(result.meta.canvas.width, 100);
        assert_eq!(result.meta.canvas.height, 100);
    }
    
    #[test]
    fn test_with_temp_file() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("test.svg");
        
        std::fs::write(&path, "<svg/>")?;
        let result = parse_file(&path)?;
        
        assert!(result.meta.source.ends_with("test.svg"));
        Ok(())
    }
    
    #[tokio::test]
    async fn test_async_operation() {
        let result = process_async("test").await.unwrap();
        assert_eq!(result, "processed");
    }
}
```
```