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

### extractor/
- [x] `extractor/images.rs` — base64 decode, magic bytes, WebP конвертация 85%, 1x/2x
- [x] `extractor/fonts.rs` — `detect_font_families`, Google Fonts API, fallback stack
- [x] `extractor/external.rs` — `fetch_external_resource` (reqwest, 30s timeout, 3 retry)
- [x] `extractor/mod.rs` — `extract_assets(SVGElement)`

### serializer/
- [x] `serializer/json.rs` — `JsonSerializer::serialize/deserialize` (serde, Cow zero-copy)
- [x] `serializer/binary.rs` — bincode, zstd compression
- [x] `serializer/mod.rs` — `serialize_to_json`, `serialize_to_file`, streaming 10MB+

### other
- [x] `validator.rs` — quick-xml well-formedness, xmlns check, cycle detection
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

### facade/
- [x] `generator.rs` — `Generator`, `GeneratorBuilder` high-level API

---

## svg2web-cli

- [ ] `commands/parse.rs` — `svg2web parse <file>` → JSON
- [ ] `commands/generate.rs` — `svg2web generate <json> --format react`
- [ ] `commands/build.rs` — `svg2web build <svg> --format react` (полный цикл)
- [ ] `commands/optimize.rs` — `svg2web optimize <file>`
- [ ] `commands/analyze.rs` — `svg2web analyze <file>`
- [x] `config/mod.rs` — `Config::from_file`, `config.toml` парсинг
- [x] `logger.rs` — tracing subscriber setup
- [x] `main.rs` — `#[tokio::main]`, clap dispatch

---

## svg2web-wasm

- [ ] `lib.rs` — `parse_svg`, `analyze_svg`, `optimize_svg`, `extract_assets`, `serialize_to_json`
- [ ] `memory.rs` — `StringPool` (allocate/read_chunk/free, handles >1MB)
- [ ] `cache.rs` — MemoryCache интеграция
- [ ] `analyzer.rs` — WASM-обёртка над analyzer
- [ ] `utils.rs` — `set_panic_hook`, serde_wasm_bindgen хелперы

---

## web/src (TypeScript/React)

- [ ] `workers/convert.worker.ts` — Web Worker, init WASM, PARSE/OPTIMIZE/GENERATE/ERROR
- [ ] `hooks/useWorker.ts` — Promise-based postMessage абстракция
- [ ] `hooks/useConverter.ts` — state machine idle→parsing→analyzing→optimizing→generating→done
- [ ] `hooks/useAnalyzer.ts` — анализ без генерации
- [ ] `lib/svg2web.ts` — публичный TS API
- [ ] `components/Uploader.tsx`
- [ ] `components/Preview.tsx`
- [ ] `components/FormatSelector.tsx`
- [ ] `components/Settings.tsx`
- [ ] `components/Progress.tsx`
- [ ] `components/Download.tsx`
- [ ] `App.tsx`

---

## plugins

- [ ] `plugins/svelte/src/lib.rs` — `SvelteRenderer` implements `FormatRenderer`
- [ ] `plugins/solid/src/lib.rs` — `SolidRenderer` implements `FormatRenderer`

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

## Итог (обновляй счётчики вручную)

| Слой | Реализовано | Всего | % |
|------|-------------|-------|---|
| svg2web-core | 28 | 28 | 100% |
| svg2web-cache | 5 | 5 | 100% |
| svg2web-generator | 16 | 16 | 100% |
| svg2web-cli | 3 | 8 | 38% |
| svg2web-wasm | 0 | 5 | 0% |
| web/src | 0 | 11 | 0% |
| plugins | 0 | 2 | 0% |
| **ИТОГО** | **53** | **66** | **80%** |
