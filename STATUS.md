# STATUS.md — Текущее состояние реализации

> Обновляй этот файл при каждом переводе модуля из стаб-комментария в реальный код.
> Формат: `- [x]` = реализовано и проходит тесты, `- [ ]` = стаб (однострочный комментарий-чертёж).

---

## Архитектурные принципы v1.0 (Non-Negotiable)

### 1. JSON-First Pipeline (No Semantic Analysis)
**Принцип:** Мы НЕ пытаемся "понять" дизайн. Мы фиксируем его 1:1 в JSON, затем генерируем HTML/CSS.

- ✅ **Делаем:** Figma SVG → usvg → JSON (координаты, цвета, слои) → HTML/CSS (position: absolute с точными x/y)
- ❌ **Не делаем:** Распознавание "кнопок", "карточек", восстановление текста из path, Auto Layout → Flexbox
- ✅ **Оптимизация:** Только структурная (SHA256 дедупликация идентичных слоёв, minify IDs)

### 2. Pixel-Perfect via Coordinates
Гарантия точности: сохраняем оригинальные x/y/width/height из SVG. Не пересчитываем, не "улучшаем" layout.
Валидация: PSNR > 50 dB между rendered original SVG и generated HTML.

### 3. Radical Simplification Fallback
Если слой слишком сложен для парсинга в HTML/CSS (сложные фильтры, маски, градиенты) — **растрируем** его в PNG/WebP/AVIF и вставляем как `<img>`.
Не пытаемся воспроизвести сложный SVG в CSS.

**Код-гарант:** `crates/svg2web-core/src/architecture_constraints.rs` — тесты, ломающиеся при нарушении принципов.

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
- Поддержка в `parser/text.rs`: `font-family`, `font-size`, `font-weight`, `letter-spacing`, `text-anchor`
- [x] `parser/mod.rs` — `parse(&[u8])`, `parse_file(&Path)`, fallback roxmltree

### analyzer/
- [x] `analyzer/mod.rs` — `analyze(element)`, `AnalysisResult`
- [x] `analyzer/components.rs` — `detect_components` (поддеревья/структурные сигнатуры)
- [x] `analyzer/hierarchy.rs` — `analyze_hierarchy` (depth/width/paths)
- [x] `analyzer/complexity.rs` — `compute_complexity` (score 0-100)

### optimizer/
- [x] `optimizer/path_simplifier.rs` — Douglas-Peucker `simplify_path`
- [x] `optimizer/deduplication.rs` — SHA256 хеширование + structural similarity 95%
- [x] `optimizer/minifier.rs` — `minify_ids` (a,b,c...), удаление дефолтных атрибутов
- [x] `optimizer/mod.rs` — `optimize(ParseOutput, &OptimizerConfig)`
- [ ] `optimizer/sprites.rs` — CSS sprites для >5 мелких иконок (объединение в одно изображение + `background-position`)

### extractor/
- [x] `extractor/images.rs` — base64 decode, magic bytes, WebP конвертация 85%, 1x/2x
- [ ] AVIF encoding (наряду с WebP 2x/3x), цепочка fallback: AVIF → WebP → PNG
- [x] `extractor/fonts.rs` — `detect_font_families`, Google Fonts API, fallback stack
- [ ] Font subsetting через Python CLI `fonttools` (`pyftsubset`) для лицензированных шрифтов
- [ ] Base64 WOFF2 inline опция
- [x] `extractor/external.rs` — `fetch_external_resource` (reqwest, 30s timeout, 3 retry)
- [x] `extractor/mod.rs` — `extract_assets(SVGElement)`

### serializer/
- [x] `serializer/json.rs` — `JsonSerializer::serialize/deserialize` (serde; `Cow` применим в сценариях borrow-friendly десериализации)
- [x] `serializer/binary.rs` — bincode, zstd compression
- [x] `serializer/mod.rs` — `serialize_to_json`, `serialize_to_file`, streaming 10MB+

### validator/
- [x] `validator/visual.rs` — `render_svg_to_rgba` (resvg::Tree::from_usvg), `calculate_psnr` (MSE formula), `visual_diff` → PSNR (dB)
- [x] `validator/performance.rs` — `TimingBudget { simple: 100ms, complex: 1000ms }`, `check_timing`, `measure<F, R>`
- [x] `validator/strict.rs` — `StrictConfig`, `PSNR_THRESHOLD = 50.0`, `check_strict` (gates pipeline on PSNR + timing)
- [x] `validator/mod.rs` — well-formedness, xmlns check, cycle detection (moved from validator.rs)

### ⚠️ Текущее ограничение (Iteration 34)
- **Strict mode:** Проверяет только identity (original vs original, PSNR = ∞)
- **PSNR optimized:** Заблокирован отсутствием SVG-сериализатора (SVGElement → SVG string)
- **Workaround:** Структурная проверка optimize (детерминизм + сохранение дерева)
- **План:** Полная strict mode с PSNR после SVG-сериализатора → Iteration 40+ (Polish Phase)

### other
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
- [x] `builders/css.rs` — генерация CSS из `Style` и базовых CSS custom properties
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
- [x] `output/structure.rs` — builder структуры папок (`css/variables.css`, `css/base.css`, `assets/vectors/`, `js/components/`, `src/components/`)

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
- [x] `tests/node.rs` — 8 wasm-bindgen-test (parse/analyze/optimize/StringPool), `wasm-pack test --node` ✅
- ⚠️ WASM bundle: 789 KB release (wasm-opt disabled), JS bindings 12 KB
- [ ] `renderer.rs` — WASM-обёртка для рендеринга в Canvas/OffscreenCanvas (для preview)
- [ ] `validator.rs` — WASM-версия visual diff (сравнение ArrayBuffer, PSNR)
- [ ] `cache.rs` — MemoryCache интеграция
- [ ] `analyzer.rs` — WASM-специфичные обертки

---

## web/src (TypeScript/React)

- [x] `workers/convert.worker.ts` — Web Worker, инициализация WASM, PARSE/ANALYZE/OPTIMIZE с корреляционными ID
- [x] `hooks/useWorker.ts` — Promise-based postMessage абстракция, lifecycle management
- [x] `hooks/useConverter.ts` — state machine idle→parsing→analyzing→optimizing→done, progress tracking
- [ ] `hooks/useAnalyzer.ts` — анализ без генерации (stub `export {}`)
- [ ] `lib/svg2web.ts` — публичный TS API (stub `export {}`)
- [ ] `components/Uploader.tsx` (stub `export {}`)
- [ ] `components/Preview.tsx` — интеграция с WASM renderer, отображение PSNR score (stub `export {}`)
- [ ] `components/QualityGate.tsx` — UI strict mode (блокировка скачивания при PSNR < 50 dB) (stub `export {}`)
- [ ] `components/FormatSelector.tsx` (stub `export {}`)
- [ ] `components/Settings.tsx` (stub `export {}`)
- [ ] `components/Progress.tsx` (stub `export {}`)
- [ ] `components/Download.tsx` (stub `export {}`)
- [x] `App.tsx` — textarea input, convert button, analysis display, optimized SVG preview
- [ ] `lib/benchmark.ts` — измерение времени конвертации в браузере (<1 s budget)
- [x] `types/index.ts` — ConversionOptions, AnalysisResult, ComplexityMetrics, HierarchyInfo, ConversionResult, ConversionStatus
- [x] `main.tsx` — React entry point (createRoot, StrictMode)
- [x] `styles/globals.css` — базовые стили
- ✅ **`npm run build` (tsc + vite) проходит** — dist: index.html + JS 205KB + WASM 807KB + Worker 4.6KB

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
- [x] 100+ synthetic fixtures (`tests/fixtures/synthetic/`, seed-based generator)
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
| Fixtures | 100+ synthetic (simple/complex/stress) | `tests/fixtures/synthetic/` + `tests/support/fixture_generator.rs` |

---

## Тестовая инфраструктура

- [x] `tests/support/fixture_generator.rs` — deterministic synthetic SVG generation (seed-based, 3 categories)
- [x] `tests/fixtures/synthetic/` — 100 fixtures (simple 1-20, complex 21-70, stress 71-100)
- [x] `tests/validator_on_fixtures_test.rs` — validator integration (parse/render/optimize 100/100/100, identity PSNR)
- [ ] `tests/fixtures/` — 100+ реальных Figma SVG (simple / medium / complex)
- [ ] `tests/benchmarks/` — `criterion` benches для регрессий производительности
- [ ] `tests/visual/` — snapshot тесты с PSNR сравнением
- [ ] `scripts/verify-examples.sh` — проверка всех `examples/` без diff

---

## Итог (обновляй счётчики вручную)

| Слой | Реализовано | Всего | % |
|------|-------------|-------|---|
| svg2web-core | 31 | 35 | 89% |
| svg2web-cache | 5 | 5 | 100% |
| svg2web-generator | 17 | 24 | 71% |
| svg2web-cli | 8 | 11 | 73% |
| svg2web-wasm | 3 | 7 | 43% |
| web/src | 7 | 18 | 39% |
| plugins | 0 | 5 | 0% |
| tests | 3 | 7 | 43% |
| **ИТОГО** | **74** | **112** | **66%** |
