# Validation in svg2web-core

## Overview

The `validator` module provides three layers of SVG validation:

1. **Structural validation** (`validate_svg`) — XML well-formedness, required attributes (`xmlns`, `viewBox`)
2. **Visual validation** (`visual.rs`) — pixel-perfect PSNR comparison using headless rendering
3. **Performance validation** (`performance.rs`) — timing budgets enforced per complexity tier
4. **Strict mode** (`strict.rs`) — combines visual + performance, blocks pipeline on violation

---

## visual.rs — PSNR / Headless Rendering

### Approach

Uses `resvg` (wraps `usvg` + `tiny-skia`) for pure-Rust headless rendering. No headless browser
required — works in WASM too (for Web Preview component).

```
original SVG → resvg::render → RGBA Pixmap
result SVG   → resvg::render → RGBA Pixmap
                               ↓
                         calculate_psnr(original, result) → f32 (dB)
```

### Functions

- `render_svg_to_rgba(svg: &str, width: u32, height: u32) -> Option<RenderResult>`  
  Renders SVG string to a raw RGBA byte buffer via `resvg::render`.

- `calculate_psnr(original: &[u8], processed: &[u8]) -> f32`  
  Standard PSNR formula: `10 * log10(255² / MSE)`. Returns `f32::INFINITY` for identical images.

- `visual_diff(original_svg: &str, result_svg: &str, width: u32, height: u32) -> Option<f32>`  
  Convenience wrapper: renders both SVGs and returns PSNR score in dB.

### PSNR Formula

$$\mathrm{PSNR} = 10 \cdot \log_{10} \left( \frac{255^2}{\mathrm{MSE}} \right)$$

$$\mathrm{MSE} = \frac{1}{N} \sum_{i=0}^{N-1} (a_i - b_i)^2$$

Threshold: **50 dB** — below this, visual degradation is perceptible.

| PSNR (dB) | Visual Quality |
|-----------|----------------|
| ∞         | Identical       |
| > 50      | Imperceptible difference |
| 40–50     | Slight difference visible on close inspection |
| 30–40     | Noticeable degradation |
| < 30      | Severe quality loss |

### Dependencies

- `resvg = "0.35"` — re-exports `tiny_skia` and `usvg`
- `usvg::TreeParsing` trait — needed for `Tree::from_str`

---

## performance.rs — Timing Budgets

Enforces processing time constraints based on SVG complexity (layer count).

### Budgets

| Tier | Threshold | Budget |
|------|-----------|--------|
| Simple | < 50 layers | 100 ms |
| Complex | ≥ 50 layers | 1000 ms |

### Functions

- `TimingBudget` — configurable `simple: Duration` + `complex: Duration`
- `check_timing(elapsed, layers, budget) -> Result<(), TimingViolation>`
- `measure<F, R>(f: F) -> (R, Duration)` — wraps a closure with timing

---

## strict.rs — Strict Mode

The `--strict` CLI flag enables strict mode. Pipeline is **blocked** (returns error) if:

- PSNR < 50 dB (configurable via `StrictConfig::psnr_threshold`)
- Processing time exceeds budget for complexity tier

### Config

```rust
StrictConfig {
    enabled: bool,
    psnr_threshold: f32,    // default: 50.0 dB
    timing_budget: TimingBudget,
}
```

### StrictViolation enum

- `PsnrViolation { psnr, threshold }` — visual quality degraded
- `TimingViolation { elapsed, budget, layers }` — performance budget exceeded

### Integration with CLI

```
svg2web build input.svg --format react --strict
```

When `--strict` is active and PSNR < 50 dB, the CLI exits with code 1 and prints:

```
error: PSNR 42.3 dB is below threshold 50.0 dB — visual quality degraded
```

---

## validate_svg (top-level)

Located in `validator/mod.rs`. Performs three checks:

1. Parser-level: `usvg::Tree::from_str` — catches malformed XML and invalid SVG
2. Root element: `<svg>` with `xmlns="http://www.w3.org/2000/svg"` attribute required
3. Dimensions: `viewBox` OR (`width` AND `height`) required

### Cycle Detection

usvg automatically resolves and validates `<use>` elements, detecting circular references.

---

## Testing

Tests are in `crates/svg2web-core/tests/validator_visual_test.rs`.

Key test cases:

- PSNR returns `f32::INFINITY` for identical RGBA buffers
- PSNR returns `0.0` for maximally different buffers (black vs white)
- `visual_diff(svg, svg, ...)` returns `> 50.0` dB
- `check_timing` passes within budget, fails over budget
- Strict mode disabled: always passes regardless of degradation
- Strict mode enabled: blocks on PSNR < threshold
