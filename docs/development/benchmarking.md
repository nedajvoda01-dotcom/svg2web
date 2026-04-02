# Бенчмаркинг

## Цель

Отслеживать производительность и автоматически ловить регрессии.

## Criterion бенчмарки

### parse_bench

Покрывает SVG размером:

- 10KB
- 100KB
- 1MB
- 10MB

### optimize_bench

Покрывает сложные пути:

- thousands of points
- много коллинеарных сегментов
- интенсивная deduplication

### generate_bench

Покрывает большие компоненты:

- nested structures
- глубоко вложенные группы
- много props/style bindings

Пример запуска:

```bash
cargo bench --package svg2web-core --bench parse_bench
cargo bench --package svg2web-core --bench optimize_bench
cargo bench --package svg2web-generator --bench generate_bench
```

## Профилирование

### CPU

```bash
cargo flamegraph --bench parse_bench
```

Используется для поиска узких мест в парсере и оптимизаторе.

### Память

```bash
heaptrack cargo bench --bench parse_bench
```

Используется для поиска утечек и роста heap, включая WASM-пути.

### Linux perf

```bash
perf stat cargo bench --bench generate_bench
perf record cargo bench --bench optimize_bench
```

Отслеживаем cache misses и branch prediction.

## SLA цели

| Операция | Цель |
|---|---|
| Parse 10MB | < 500ms |
| Analyze | < 100ms |
| Optimize | < 200ms |
| Generate | < 100ms |

## Регрессии в CI

Базовый сценарий:

1. сохранить baseline для main branch;
2. сравнить текущую ветку с baseline;
3. опубликовать criterion reports в артефактах.

Fail правило:

- CI падает при деградации производительности > 10%.

Пример:

```bash
# baseline
cargo bench -- --save-baseline main

# compare
cargo bench -- --baseline main
```
