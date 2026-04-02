# STATUS.md — Текущее состояние реализации

> Обновляй этот файл при каждом переводе модуля из стаб-комментария в реальный код.
> Формат: `- [x]` = реализовано и проходит тесты, `- [ ]` = стаб (однострочный комментарий-чертёж).

---

## svg2web-core

### model/
- [x] `model/element.rs` — `SVGElement`, `Bounds`, `Transform`
- [x] `model/style.rs` — `Style`, `Color`, `Font`, `Gradient`
- [x] `model/geometry.rs` — `PathData`, `Point`, `Rect`
- [x] `model/asset.rs` — `Asset`, `ImageAsset`, `FontAsset`, `ExternalAsset`
- [x] `model/analysis.rs` — `Component`, `ComplexityMetrics`, `HierarchyInfo`

### parser/
- [x] `parser/svg.rs` — `UsvgParser::load`, `UsvgParser::to_model` (usvg wrapper)
- [x] `parser/geometry.rs` — конвертация rect/circle/path → PathData
- [x] `parser/styles.rs` — парсинг CSS-атрибутов → Style
- [x] `parser/text.rs` — `parse_font`, `extract_text_content`
- [x] `font-family` парсинг
- [x] `font-size` парсинг
- [x] `font-weight` парсинг
- [x] `letter-spacing` парсинг
- [x] `text-anchor` парсинг
- [x] `parser/mod.rs` — `parse(&[u8])`, `parse_file(&Path)`, fallback roxmltree

### analyzer/
- [x] `analyzer/mod.rs` — `analyze(element)`, `AnalysisResult`
- [x] `analyzer/components.rs` — `detect_components` (поддеревья/хеши)
- [x] `analyzer/hierarchy.rs` — `analyze_hierarchy` (depth/width/paths)
- [x] `analyzer/complexity.rs` — `compute_complexity` (score 0-100)

### optimizer/
- [x] `optimizer/path_simplifier.rs` — Douglas-Peucker `simplify_path`
- [x] `optimizer/deduplication.rs` — SHA256 хеширование + structural similarity 95%
- [x] `optimizer/minifier.rs` — `minify_ids` (a,b,c...), удаление дефолтных атрибутов
- [x] `optimizer/mod.rs` — `optimize(ParseOutput, &OptimizerConfig)`
- [ ] `optimizer/sprites.rs` — CSS sprites для >5 мелких иконок (объединение в single image + CSS background-position)

### extractor/
- [x] `extractor/images.rs` — base64 decode, magic bytes, WebP конвертация 85%, 1x/2x
- [ ] AVIF encoding (наряду с WebP 2x/3x), fallback цепочка: AVIF → WebP → PNG
- [x] `extractor/fonts.rs` — `detect_font_families`, Google Fonts API, fallback stack
- [ ] Font subsetting через `fonttools` (для licensed шрифтов)
- [ ] Base64 WOFF2 inline опция
- [x] `extractor/external.rs` — `fetch_external_resource` (reqwest, 30s timeout, 3 retry)
- [x] `extractor/mod.rs` — `extract_assets(SVGElement)`

### serializer/
- [x] `serializer/json.rs` — `JsonSerializer::serialize/deserialize` (serde, Cow zero-copy)
- [x] `serializer/binary.rs` — bincode, zstd compression
- [x] `serializer/mod.rs` — `serialize_to_json`, `serialize_to_file`, streaming 10MB+

### validator/
- [x] `validator/visual.rs` — `render_svg_to_rgba` (resvg::Tree::from_usvg), `calculate_psnr` (MSE formula), `visual_diff` → PSNR (dB)
- [x] `validator/performance.rs` — `TimingBudget { simple: 100ms, complex: 1000ms }`, `check_timing`, `measure<F, R>`
- [x] `validator/strict.rs` — `StrictConfig`, `PSNR_THRESHOLD = 50.0`, `check_strict` (gates pipeline on PSNR + timing)

### other
- [x] `validator/mod.rs` — well-formedness, xmlns check, cycle detection (moved from validator.rs)
- [x] `error.rs` — `Error::SvgParse { line, col }`, `Error::InvalidSvg`, `Error::IoError`
- [x] `lib.rs` — публичные экспорты Parser/Analyzer/Optimizer/Extractor/Serializer

---

## svg2web-cache

- [x] `key.rs` — `CacheKey::from_svg(svg, config)` → SHA256(concat(...))
- [x] `disk.rs` — `DiskCache` (sled, ACID, zstd, ~/.cache/svg2web/)
- [x] `memory.rs` — `MemoryCache` (lru, 1000 entries, Arc<Mutex>)
- [x] `manager.rs` — `CacheManager` (disk + memory tier)
- [x] `lib.rs` — `get/set/invalidate/clear` публичные функции

---

## svg2web-generator

### readers/
- [x] `readers/json.rs` — читает `SVGElement` из JSON
- [x] `readers/binary.rs` — feature-gated binary reader (fallback error без feature)
- [x] `readers/mod.rs`

### builders/
- [x] `builders/html.rs` — генерация HTML из SVGElement
- [x] `builders/css.rs` — генерация CSS из Style, CSS variables
- [ ] Генерация `variables.css` (design tokens из Figma: colors, typography, spacing)
- [ ] Разделение на `base.css` (reset) и `components.css` (BEM блоки)
- [x] `builders/js.rs` — генерация JS/TS
- [ ] `builders/responsive.rs` — mobile-first media queries
- [x] `builders/mod.rs`

### components/
- [ ] `components/template.rs` — Component → шаблон
- [ ] `components/mod.rs` — 4-Phase detection algorithm

### formats/
- [x] `formats/builtin/vanilla.rs` — `VanillaRenderer` (HTML/CSS/JS)
- [x] `formats/builtin/react.rs` — `ReactRenderer` (TSX, React.memo, TypeScript interfaces)
- [x] `formats/builtin/vue.rs` — `VueRenderer` (SFC, defineAsyncComponent)
- [ ] `formats/tera_adapter.rs` — `TeraAdapter` (legacy, feature=tera)
- [x] `formats/mod.rs` — `pub trait FormatRenderer`

### registry/
- [x] `registry/context.rs` — `RenderContext` (ParseOutput → template context)
- [x] `registry/mod.rs` — skeleton `FormatRegistry` (new/list)

### output/
- [x] `output/writer.rs` — запись файлов
- [x] `output/zip.rs` — архивирование в ZIP
- [x] `output/mod.rs`
- [ ] `output/meta.rs` — генерация `figma.json` (метаданные: layer IDs, Figma version, component paths, hierarchy mapping)
- [ ] `output/structure.rs` — билдер структуры папок (`css/variables.css`, `css/base.css`, `assets/vectors/` и т.д.)

### facade/
- [x] `generator.rs` — `Generator`, `GeneratorBuilder` high-level API

---

## svg2web-cli

- [x] `commands/parse.rs` — `svg2web parse <file>` → JSON
- [ ] `commands/generate.rs` — `svg2web generate <json> --format react`
- [x] `commands/build.rs` — `svg2web build <svg> --format react` (полный цикл)
- [x] флаг `--strict` (интеграция с `validator::strict`, блокировка при PSNR < 50 dB)
- [ ] `commands/optimize.rs` — `svg2web optimize <file>`
- [x] `commands/analyze.rs` — `svg2web analyze <file>`
- [ ] расширенный отчёт: "что будет растризовано и почему"
- [x] `commands/mod.rs` — dispatch
- [x] `config/mod.rs` — `Config::from_file`, `config.toml` парсинг
- [x] `logger.rs` — tracing subscriber setup
- [x] `main.rs` — entry point, clap dispatch

---

## svg2web-wasm

- [x] `lib.rs` — `parse_svg`, `analyze_svg`, `optimize_svg` биндинги
- [x] `memory.rs` — `StringPool` (pool) для файлов >1 MB: `allocate/read/free/total_size/clear`
- [x] `utils.rs` — panic hooks, logging
- [ ] `renderer.rs` — WASM-обёртка для рендеринга в Canvas/OffscreenCanvas (для preview)
- [ ] `validator.rs` — WASM-версия visual diff (сравнение ArrayBuffer, PSNR)
- [ ] `cache.rs` — MemoryCache интеграция
- [ ] `analyzer.rs` — WASM-специфичные обертки

---

## web/src (TypeScript/React)

- [ ] `workers/convert.worker.ts` — Web Worker, init WASM, PARSE/OPTIMIZE/GENERATE/ERROR
- [ ] `hooks/useWorker.ts` — Promise-based postMessage абстракция
- [ ] `hooks/useConverter.ts` — state machine idle→parsing→analyzing→optimizing→generating→done
- [ ] `hooks/useAnalyzer.ts` — анализ без генерации
- [ ] `lib/svg2web.ts` — публичный TS API
- [ ] `components/Uploader.tsx`
- [ ] `components/Preview.tsx` — интеграция с WASM renderer, отображение PSNR score
- [ ] `components/QualityGate.tsx` — UI strict mode (блокировка скачивания при PSNR < 50 dB)
- [ ] `components/FormatSelector.tsx`
- [ ] `components/Settings.tsx`
- [ ] `components/Progress.tsx`
- [ ] `components/Download.tsx`
- [ ] `App.tsx`
- [ ] `lib/benchmark.ts` — измерение времени конвертации в браузере (<1 s budget)

---

## plugins

- [ ] `plugins/svelte/src/lib.rs` — `SvelteRenderer` implements `FormatRenderer`
- [ ] `plugins/solid/src/lib.rs` — `SolidRenderer` implements `FormatRenderer`

### figma-plugin/
- [ ] `figma-plugin/iframe/` — iframe-версия плагина (использует WASM биндинги)
- [ ] `figma-plugin/native/` — native API версия (отложено → v1.1/v2.0)

### vscode-extension/
- [ ] `vscode-extension/` — базовая структура (preview, convert on save)

---

## examples (ожидаемый output)

- [ ] `examples/basic` — Vanilla HTML/CSS из простого SVG
- [ ] `examples/react-component` — React TSX компонент
- [ ] `examples/responsive` — адаптивный HTML
- [ ] `examples/advanced/custom-fonts` — SVG с кастомными шрифтами
- [ ] `examples/advanced/external-images` — SVG с внешними изображениями
- [ ] `examples/advanced/complex-animation-free` — SVG с анимацией

> Отмечай пример как `[x]` когда `scripts/verify-examples.sh` проходит без diff.

---

## Roadmap: v1.0 Full Stack (6–7 недель)

### Phase 1: Closure (Week 1)
- [x] `validator/visual.rs` — PSNR > 50 dB гарантия (`resvg::Tree::from_usvg`, MSE formula, 29 тестов ✅)
- [x] `validator/performance.rs` — TimingBudget 100ms/1000ms, `check_timing`, `measure`
- [x] `validator/strict.rs` — `check_strict`, `StrictConfig`, `PSNR_THRESHOLD = 50.0`
- [ ] `commands/optimize.rs` — полная реализация
- [ ] `examples/` — все 6 штук + `scripts/verify-examples.sh` с pixel-perfect проверкой

### Phase 2: Ecosystem (Weeks 2–4)
- [ ] `svg2web-wasm/renderer.rs` — Canvas/OffscreenCanvas рендеринг
- [ ] `web/src/` — полный UI с Web Workers
- [ ] `plugins/svelte` & `solid` (опционально, если ресурсы)

### Phase 3: Polish (Weeks 5–6)
- [ ] AVIF encoding (`extractor/images.rs`)
- [ ] Font subsetting (`extractor/fonts.rs`)
- [ ] CSS sprites (`optimizer/sprites.rs`)
- [ ] Performance regression tests (`criterion`)
- [ ] CSS design tokens (`builders/css.rs`)

### Phase 4: Release (Week 7)
- [ ] 100+ fixtures тестов (`tests/fixtures/`)
- [ ] 0 критических багов
- [ ] Deploy Web UI
- [ ] figma-plugin / vscode-extension

---

## Гарантии качества

| Метрика | Порог | Инструмент |
|---------|-------|------------|
| Pixel-perfect | PSNR > 50 dB или explicit warning | `validator/visual.rs` |
| Performance (simple, <50 слоёв) | < 100 ms | `validator/performance.rs` |
| Performance (complex, <1000 слоёв) | < 1 s | `validator/performance.rs` |
| Output size | < 150% от оптимизированного SVG | `optimizer/mod.rs` |
| Test coverage (core) | 100% | `cargo tarpaulin` |
| Fixtures | 100+ реальных Figma SVG | `tests/fixtures/` |

---

## Тестовая инфраструктура

- [ ] `tests/fixtures/` — 100+ реальных Figma SVG (simple / medium / complex)
- [ ] `tests/benchmarks/` — `criterion` benches для регрессий производительности
- [ ] `tests/visual/` — snapshot тесты с PSNR сравнением
- [ ] `scripts/verify-examples.sh` — проверка всех `examples/` без diff

---

## Итог (обновляй счётчики вручную)

| Слой | Реализовано | Всего | % |
|------|-------------|-------|---|
| svg2web-core | 31 | 32 | 97% |
| svg2web-cache | 5 | 5 | 100% |
| svg2web-generator | 17 | 23 | 74% |
| svg2web-cli | 9 | 9 | 100% |
| svg2web-wasm | 3 | 7 | 43% |
| web/src | 0 | 14 | 0% |
| plugins | 0 | 4 | 0% |
| tests | 0 | 4 | 0% |
| **ИТОГО** | **64** | **98** | **65%** |
