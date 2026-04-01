```markdown
# Тестирование

## Стратегия тестирования

| Уровень | Инструменты | Покрытие | Что тестируем |
|---------|------------|----------|---------------|
| Юнит-тесты | `#[test]` | ≥80% | Структуры данных, сериализация, изолированные функции |
| Интеграционные | `tests/` | Функциональность | Парсинг, оптимизация, полный pipeline |
| Снапшоты | `insta` | Выходные форматы | React/Vue/Vanilla генерация |
| E2E | `assert_cmd`, Playwright | CLI, Web | Полные сценарии использования |
| Бенчмарки | `criterion` | Производительность | Парсинг, оптимизация, генерация |

---

## Юнит-тесты

### Структура

```rust
// crates/svg2web-core/src/model/element.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_svg_element_new() {
        let element = SVGElement::new("rect");
        assert_eq!(element.tag, "rect");
        assert!(element.children.is_empty());
    }
    
    #[test]
    fn test_svg_element_with_attributes() {
        let mut element = SVGElement::new("rect");
        element.attributes.insert("width".to_string(), "100".to_string());
        element.attributes.insert("height".to_string(), "100".to_string());
        
        assert_eq!(element.attributes.get("width"), Some(&"100".to_string()));
    }
    
    #[test]
    fn test_serialization_roundtrip() {
        let element = SVGElement {
            tag: "circle".to_string(),
            attributes: [("cx".to_string(), "50".to_string())].into(),
            children: vec![],
            id: Some("circle1".to_string()),
            class: vec!["shape".to_string()],
            text_content: None,
        };
        
        let json = serde_json::to_string(&element).unwrap();
        let deserialized: SVGElement = serde_json::from_str(&json).unwrap();
        
        assert_eq!(element, deserialized);
    }
}
```

### Запуск

```bash
# Все юнит-тесты
cargo test --workspace --lib

# Конкретный модуль
cargo test -p svg2web-core model::element

# С выводом
cargo test -- --nocapture
```

---

## Интеграционные тесты

### Fixtures

```
tests/fixtures/
├── simple.svg
├── text.svg
├── gradients.svg
├── complex.svg
├── invalid/
│   ├── unclosed-tag.svg
│   └── missing-xmlns.svg
└── golden/
    ├── simple.expected.json
    └── complex.expected.json
```

### parser_test.rs

```rust
use svg2web_core::{parse, ParseOptions};

#[test]
fn test_parse_simple() {
    let svg = include_str!("fixtures/simple.svg");
    let result = parse(svg, ParseOptions::default()).unwrap();
    
    assert_eq!(result.meta.canvas.width, 100);
    assert_eq!(result.meta.canvas.height, 100);
    assert_eq!(result.structure.children.len(), 2);
}

#[test]
fn test_parse_with_options() {
    let svg = include_str!("fixtures/simple.svg");
    
    let result = parse(svg, ParseOptions {
        normalize: true,
        keep_comments: false,
    }).unwrap();
    
    assert!(result.meta.title.is_none());
}

#[test]
fn test_parse_invalid_svg() {
    let svg = include_str!("fixtures/invalid/unclosed-tag.svg");
    let result = parse(svg, ParseOptions::default());
    
    assert!(result.is_err());
    match result {
        Err(Error::ParseError { line, .. }) => {
            assert_eq!(line, Some(3));
        }
        _ => panic!("Expected ParseError"),
    }
}
```

### optimizer_test.rs

```rust
use svg2web_core::{parse, optimize, OptimizationConfig, ParseOptions};
use resvg::tiny_skia::Pixmap;

fn render_svg(svg: &str) -> Pixmap {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).unwrap();
    let mut pixmap = Pixmap::new(tree.size.width() as u32, tree.size.height() as u32).unwrap();
    resvg::render(&tree, &resvg::Options::default(), &mut pixmap.as_mut());
    pixmap
}

fn images_equal(p1: &Pixmap, p2: &Pixmap, tolerance: f64) -> bool {
    let data1 = p1.data();
    let data2 = p2.data();
    
    if data1.len() != data2.len() {
        return false;
    }
    
    let diff_count = data1.iter()
        .zip(data2.iter())
        .filter(|(a, b)| (*a as i32 - *b as i32).abs() as f64 > tolerance)
        .count();
    
    diff_count as f64 / data1.len() as f64 < 0.01 // <1% разницы
}

#[test]
fn test_optimize_preserves_visuals() {
    let svg = include_str!("fixtures/complex.svg");
    let original_pixmap = render_svg(svg);
    
    let parsed = parse(svg, ParseOptions::default()).unwrap();
    let optimized = optimize(parsed, OptimizationConfig::default());
    
    let optimized_svg = serialize_to_svg(&optimized);
    let optimized_pixmap = render_svg(&optimized_svg);
    
    assert!(images_equal(&original_pixmap, &optimized_pixmap, 2.0));
}

#[test]
fn test_deduplication() {
    let svg = include_str!("fixtures/duplicates.svg");
    let parsed = parse(svg, ParseOptions::default()).unwrap();
    let original_count = count_elements(&parsed.structure);
    
    let optimized = optimize(parsed, OptimizationConfig {
        deduplicate: true,
        ..Default::default()
    });
    
    let optimized_count = count_elements(&optimized.structure);
    assert!(optimized_count < original_count);
}
```

---

## Golden файлы

### Создание golden файлов

```rust
#[test]
fn test_parse_complex_golden() {
    let svg = include_str!("fixtures/complex.svg");
    let result = parse(svg, ParseOptions::default()).unwrap();
    let json = serde_json::to_string_pretty(&result).unwrap();
    
    // Первый запуск создает файл
    insta::assert_snapshot!("complex-parse", json);
}
```

### Обновление снапшотов

```bash
# Просмотр различий
cargo insta review

# Принять все изменения
cargo insta accept

# Отклонить все
cargo insta reject
```

---

## Снапшоты генератора

```rust
// crates/svg2web-generator/tests/format_test.rs

use insta::assert_snapshot;
use svg2web_generator::{generate, GenerateOptions, Framework, Styling};

fn create_test_context() -> (ParseOutput, GenerateOptions) {
    let svg = include_str!("../../svg2web-core/tests/fixtures/icon.svg");
    let parsed = parse(svg, ParseOptions::default()).unwrap();
    
    let options = GenerateOptions {
        framework: Framework::React,
        styling: Styling::ScopedCss,
        output: OutputOptions {
            pretty: true,
            typescript: true,
            ..Default::default()
        },
        ..Default::default()
    };
    
    (parsed, options)
}

#[test]
fn test_react_snapshot() {
    let (parsed, options) = create_test_context();
    let output = generate(&parsed, options).unwrap();
    
    assert_snapshot!("react-component", output.html);
    assert_snapshot!("react-styles", output.css.unwrap());
}

#[test]
fn test_vue_snapshot() {
    let (parsed, options) = create_test_context();
    let options = GenerateOptions {
        framework: Framework::Vue,
        ..options
    };
    
    let output = generate(&parsed, options).unwrap();
    assert_snapshot!("vue-component", output.html);
}

#[test]
fn test_vanilla_snapshot() {
    let (parsed, options) = create_test_context();
    let options = GenerateOptions {
        framework: Framework::Vanilla,
        ..options
    };
    
    let output = generate(&parsed, options).unwrap();
    assert_snapshot!("vanilla-html", output.html);
    assert_snapshot!("vanilla-css", output.css.unwrap());
}
```

---

## E2E тесты CLI

### assert_cmd

```rust
// tests/cli_test.rs

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_cli_parse_command() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.svg");
    let output = dir.path().join("output.json");
    
    std::fs::write(&input, r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100"/></svg>"#).unwrap();
    
    Command::cargo_bin("svg2web")
        .unwrap()
        .arg("parse")
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
    
    assert!(output.exists());
    let content = std::fs::read_to_string(&output).unwrap();
    assert!(content.contains("canvas"));
}

#[test]
fn test_cli_build_command() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.svg");
    let output = dir.path().join("output");
    
    std::fs::write(&input, r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="50" cy="50" r="40"/></svg>"#).unwrap();
    
    Command::cargo_bin("svg2web")
        .unwrap()
        .arg("build")
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .arg("--format")
        .arg("react")
        .assert()
        .success();
    
    assert!(output.join("index.html").exists());
    assert!(output.join("Icon.tsx").exists());
}

#[test]
fn test_cli_error_on_invalid_input() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("invalid.svg");
    
    std::fs::write(&input, "<invalid>").unwrap();
    
    Command::cargo_bin("svg2web")
        .unwrap()
        .arg("parse")
        .arg("--input")
        .arg(&input)
        .assert()
        .failure()
        .stderr(predicate::str::contains("parse error"));
}
```

---

## E2E тесты Web (Playwright)

```typescript
// web/tests/converter.spec.ts

import { test, expect } from '@playwright/test';

test.describe('SVG Converter', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:5173');
  });

  test('should upload and parse SVG', async ({ page }) => {
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles({
      name: 'test.svg',
      mimeType: 'image/svg+xml',
      buffer: Buffer.from('<svg><rect width="100" height="100"/></svg>')
    });

    await expect(page.locator('.preview')).toBeVisible();
    await expect(page.locator('.complexity-score')).toHaveText(/[0-9]+/);
  });

  test('should generate React component', async ({ page }) => {
    await page.setInputFiles('input[type="file"]', {
      name: 'test.svg',
      mimeType: 'image/svg+xml',
      buffer: Buffer.from('<svg><circle cx="50" cy="50" r="40"/></svg>')
    });

    await page.selectOption('select', 'react');
    await page.click('button:has-text("Generate")');

    const code = await page.locator('.code-preview').textContent();
    expect(code).toContain('export function Icon');
    expect(code).toContain('interface IconProps');
  });

  test('should download ZIP', async ({ page }) => {
    await page.setInputFiles('input[type="file"]', {
      name: 'test.svg',
      mimeType: 'image/svg+xml',
      buffer: Buffer.from('<svg><rect width="100" height="100"/></svg>')
    });

    const downloadPromise = page.waitForEvent('download');
    await page.click('button:has-text("Download")');
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toContain('.zip');
  });
});
```

---

## Покрытие кода

### cargo-tarpaulin

```bash
# Установка
cargo install cargo-tarpaulin

# Запуск
cargo tarpaulin --workspace --out Html --output-dir coverage

# Проверка порога
cargo tarpaulin --workspace --fail-under 80

# Игнорирование тестов
cargo tarpaulin --workspace --exclude-files "tests/*"
```

### CI проверка

```yaml
- name: Code coverage
  run: |
    cargo tarpaulin --workspace --fail-under 80 --out Xml
  env:
    CARGO_TARPAULIN_IGNORE_TESTS: true
```

### .gitignore для покрытия

```
coverage/
*.profraw
```

---

## Тестовые утилиты

```rust
// crates/svg2web-core/tests/common/mod.rs

use tempfile::TempDir;

pub fn create_temp_svg(content: &str) -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.svg");
    std::fs::write(&path, content).unwrap();
    (dir, path)
}

pub fn assert_svg_equal(svg1: &str, svg2: &str) {
    let parsed1 = parse(svg1, ParseOptions::default()).unwrap();
    let parsed2 = parse(svg2, ParseOptions::default()).unwrap();
    
    assert_eq!(parsed1, parsed2);
}

pub fn count_elements(element: &SVGElement) -> usize {
    1 + element.children.iter().map(count_elements).sum::<usize>()
}
```

---

## CI интеграция

```yaml
test:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Run unit tests
      run: cargo test --workspace --lib
    
    - name: Run integration tests
      run: cargo test --workspace --test '*'
    
    - name: Run snapshot tests
      run: cargo test --workspace -- --ignored snapshot
    
    - name: Code coverage
      run: cargo tarpaulin --workspace --fail-under 80
    
    - name: WASM tests
      run: wasm-pack test crates/svg2web-wasm --headless --chrome
    
    - name: Web E2E tests
      run: |
        cd web
        npm ci
        npm run build
        npm run test:e2e
```