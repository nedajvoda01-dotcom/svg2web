```markdown
# Бенчмарки

## Criterion benchmarks

### Структура бенчмарков

```
benches/
├── parse_bench.rs      # Парсинг SVG разных размеров
├── optimize_bench.rs   # Оптимизация сложных SVG
├── generate_bench.rs   # Генерация кода для компонентов
├── analyze_bench.rs    # Анализ структуры
└── cache_bench.rs      # Операции с кэшем
```

### parse_bench.rs

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svg2web_core::{parse, ParseOptions};

fn parse_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    
    // 10KB SVG
    let svg_10kb = include_str!("../tests/fixtures/10kb.svg");
    group.bench_function("10kb", |b| {
        b.iter(|| parse(black_box(svg_10kb), ParseOptions::default()))
    });
    
    // 100KB SVG
    let svg_100kb = include_str!("../tests/fixtures/100kb.svg");
    group.bench_function("100kb", |b| {
        b.iter(|| parse(black_box(svg_100kb), ParseOptions::default()))
    });
    
    // 1MB SVG
    let svg_1mb = include_str!("../tests/fixtures/1mb.svg");
    group.bench_function("1mb", |b| {
        b.iter(|| parse(black_box(svg_1mb), ParseOptions::default()))
    });
    
    // 10MB SVG (генерируется динамически)
    let svg_10mb = generate_large_svg(10 * 1024 * 1024);
    group.bench_function("10mb", |b| {
        b.iter(|| parse(black_box(&svg_10mb), ParseOptions::default()))
    });
    
    group.finish();
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
```

### optimize_bench.rs

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svg2web_core::{parse, optimize, OptimizationConfig, ParseOptions};

fn optimize_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimize");
    
    // Сложные пути
    let svg_paths = include_str!("../tests/fixtures/complex-paths.svg");
    let parsed = parse(svg_paths, ParseOptions::default()).unwrap();
    
    group.bench_function("complex-paths", |b| {
        b.iter(|| optimize(black_box(parsed.clone()), OptimizationConfig::default()))
    });
    
    // Много элементов (1000+)
    let svg_many = include_str!("../tests/fixtures/1000-elements.svg");
    let parsed = parse(svg_many, ParseOptions::default()).unwrap();
    
    group.bench_function("1000-elements", |b| {
        b.iter(|| optimize(black_box(parsed.clone()), OptimizationConfig::default()))
    });
    
    // С дубликатами
    let svg_duplicates = include_str!("../tests/fixtures/duplicates.svg");
    let parsed = parse(svg_duplicates, ParseOptions::default()).unwrap();
    
    group.bench_function("deduplicate", |b| {
        b.iter(|| {
            optimize(black_box(parsed.clone()), OptimizationConfig {
                deduplicate: true,
                ..Default::default()
            })
        })
    });
    
    group.finish();
}

criterion_group!(benches, optimize_benchmark);
criterion_main!(benches);
```

### generate_bench.rs

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svg2web_core::{parse, ParseOptions};
use svg2web_generator::{generate, GenerateOptions, Framework, Styling};

fn generate_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate");
    
    let svg = include_str!("../tests/fixtures/complex.svg");
    let parsed = parse(svg, ParseOptions::default()).unwrap();
    
    // React генерация
    group.bench_function("react", |b| {
        b.iter(|| {
            generate(black_box(&parsed), GenerateOptions {
                framework: Framework::React,
                styling: Styling::ScopedCss,
                ..Default::default()
            })
        })
    });
    
    // Vue генерация
    group.bench_function("vue", |b| {
        b.iter(|| {
            generate(black_box(&parsed), GenerateOptions {
                framework: Framework::Vue,
                styling: Styling::ScopedCss,
                ..Default::default()
            })
        })
    });
    
    // С детекцией компонентов
    group.bench_function("with-component-detection", |b| {
        b.iter(|| {
            generate(black_box(&parsed), GenerateOptions {
                framework: Framework::React,
                components: ComponentOptions {
                    detect: true,
                    min_size: 2,
                    ..Default::default()
                },
                ..Default::default()
            })
        })
    });
    
    group.finish();
}
```

### analyze_bench.rs

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svg2web_core::{parse, analyze, ParseOptions};

fn analyze_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("analyze");
    
    let svg = include_str!("../tests/fixtures/complex.svg");
    let parsed = parse(svg, ParseOptions::default()).unwrap();
    
    group.bench_function("complex-svg", |b| {
        b.iter(|| analyze(black_box(&parsed)))
    });
    
    let svg_deep = include_str!("../tests/fixtures/deep-nesting.svg");
    let parsed = parse(svg_deep, ParseOptions::default()).unwrap();
    
    group.bench_function("deep-nesting", |b| {
        b.iter(|| analyze(black_box(&parsed)))
    });
    
    group.finish();
}
```

### cache_bench.rs

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svg2web_cache::{Cache, CacheConfig, CacheKey};
use tempfile::tempdir;

fn cache_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache");
    
    let dir = tempdir().unwrap();
    let cache = Cache::new(CacheConfig {
        disk_path: Some(dir.path().to_path_buf()),
        memory_size: 1000,
        ..Default::default()
    });
    
    let svg = "test svg content";
    let key = CacheKey::from_svg(svg, &Default::default());
    let value = vec![1, 2, 3, 4, 5];
    
    group.bench_function("set", |b| {
        b.iter(|| cache.set(black_box(key.clone()), black_box(value.clone())))
    });
    
    group.bench_function("get", |b| {
        b.iter(|| cache.get(black_box(&key)))
    });
    
    group.finish();
}
```

## Запуск бенчмарков

```bash
# Все бенчмарки
cargo bench --workspace

# Конкретный бенчмарк
cargo bench --package svg2web-core --bench parse_bench

# С фильтром
cargo bench -- parse 10kb

# Сохранение baseline
cargo bench -- --save-baseline main

# Сравнение с baseline
cargo bench -- --baseline main
```

## Целевые метрики

| Операция | Размер | Целевое время | Текущее |
|----------|--------|---------------|---------|
| parse | 10KB | < 10ms | 5ms |
| parse | 100KB | < 50ms | 35ms |
| parse | 1MB | < 100ms | 85ms |
| parse | 10MB | < 500ms | 420ms |
| analyze | любой | < 100ms | 45ms |
| optimize | сложные пути | < 200ms | 150ms |
| generate | React | < 100ms | 60ms |
| generate | Vue | < 100ms | 55ms |
| generate | с детекцией | < 200ms | 120ms |

## Профилирование

### CPU профилирование (flamegraph)

```bash
# Установка
cargo install flamegraph

# Профилирование парсера
cargo flamegraph --bench parse_bench -- --profile-time 10

# С конкретной функцией
cargo flamegraph --bench parse_bench -- --profile-time 10 -- --filter parse

# Результат: flamegraph.svg
```

**Анализ узких мест:**
- `usvg::Tree::from_str` — основной парсер
- `serde_json::to_string` — сериализация
- `regex` операции в оптимизаторе

### Память (heaptrack)

```bash
# Установка
sudo apt install heaptrack

# Профилирование
heaptrack cargo bench --bench parse_bench -- --profile-time 5

# Анализ
heaptrack_gui heaptrack.cargo.12345.gz
```

**Что проверять:**
- Утечки памяти в WASM
- Дублирование данных при сериализации
- Кэширование больших объектов

### perf (Linux)

```bash
# Запись
perf record --call-graph dwarf cargo bench --bench parse_bench -- --profile-time 5

# Анализ
perf report

# Сборка ядра
perf record -e cycles,instructions,cache-misses cargo bench
```

## CI интеграция

### .github/workflows/benchmark.yml

```yaml
name: Benchmarks

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('Cargo.lock') }}
      
      - name: Run benchmarks
        run: cargo bench --workspace -- --output-format bencher > output.txt
      
      - name: Compare with baseline
        run: |
          # Проверка регрессии
          python scripts/compare_benchmarks.py baseline.json output.txt
      
      - name: Upload benchmark results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results
          path: target/criterion
```

### compare_benchmarks.py

```python
#!/usr/bin/env python3
import json
import sys

def compare(baseline, current, threshold=0.1):
    failures = []
    
    for bench, current_time in current.items():
        if bench not in baseline:
            continue
            
        baseline_time = baseline[bench]
        degradation = (current_time - baseline_time) / baseline_time
        
        if degradation > threshold:
            failures.append({
                'bench': bench,
                'baseline': baseline_time,
                'current': current_time,
                'degradation': degradation
            })
    
    if failures:
        print("Benchmark regressions detected:")
        for f in failures:
            print(f"  {f['bench']}: {f['degradation']:.1%} slower")
        sys.exit(1)
    
    print("All benchmarks within threshold")

if __name__ == "__main__":
    with open('baseline.json') as f:
        baseline = json.load(f)
    
    with open(sys.argv[1]) as f:
        current = json.load(f)
    
    compare(baseline, current)
```

## Сравнение с baseline

```bash
# Сохранение baseline (после релиза)
cargo bench -- --save-baseline v0.1.0

# Проверка регрессии в CI
cargo bench -- --baseline v0.1.0 --output-format bencher | \
  python scripts/check_regression.py --threshold 0.1
```

## Отчеты

```bash
# Генерация HTML отчета
cargo bench -- --output-format html > benchmark.html

# Сравнение двух версий
cargo bench -- --baseline v0.1.0 --output-format html > comparison.html
```

## Оптимизация производительности

### Парсинг

- Использовать `&str` вместо `String` где возможно
- Кэшировать результаты `usvg::Tree`
- Параллельная обработка больших файлов

### Оптимизация

- Алгоритм Douglas-Peucker с пороговым отсечением
- Хеширование поддеревьев через `blake3` вместо `sha2`
- Пропуск оптимизации если изменений мало

### Генерация

- Кэширование скомпилированных шаблонов Tera
- Буферизованная запись
- Асинхронная запись файлов

### Память

- StringPool для больших SVG в WASM
- Arc для общих данных между компонентами
- Streaming сериализация для больших файлов
```