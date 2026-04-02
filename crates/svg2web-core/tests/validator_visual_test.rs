//! RED → GREEN tests for validator/{visual,performance,strict}
//!
//! Run with: cargo test -p svg2web-core --test validator_visual_test
#![allow(clippy::unwrap_used, clippy::expect_used)]

use svg2web_core::validator::{
	performance::{check_timing, measure, TimingBudget},
	strict::{check_strict, StrictConfig, PSNR_THRESHOLD},
	visual::{calculate_psnr, render_svg_to_rgba, visual_diff},
};
use std::time::Duration;

// ─── Documentation presence ────────────────────────────────────────────────

fn read_validation_doc() -> String {
	std::fs::read_to_string("docs/internals/core/validation.md").unwrap_or_default()
}

#[test]
fn validation_doc_exists_t080() {
	let d = read_validation_doc();
	assert!(
		d.contains("PSNR"),
		"validation.md must document PSNR calculation"
	);
}

#[test]
fn validation_doc_mentions_resvg_t080() {
	let d = read_validation_doc();
	assert!(
		d.contains("resvg"),
		"validation.md must name resvg as the headless renderer"
	);
}

#[test]
fn validation_doc_psnr_threshold_t080() {
	let d = read_validation_doc();
	assert!(
		d.contains("50 dB") || d.contains("50.0"),
		"validation.md must state PSNR threshold of 50 dB"
	);
}

#[test]
fn validation_doc_timing_budgets_t081() {
	let d = read_validation_doc();
	assert!(
		d.contains("100 ms") || d.contains("100ms"),
		"validation.md must document 100 ms budget for simple SVGs"
	);
	assert!(
		d.contains("1000 ms") || d.contains("1000ms") || d.contains("1 s"),
		"validation.md must document 1000 ms budget for complex SVGs"
	);
}

#[test]
fn validation_doc_strict_mode_t082() {
	let d = read_validation_doc();
	assert!(
		d.contains("strict") || d.contains("--strict"),
		"validation.md must document strict mode"
	);
}

// ─── visual.rs — render_svg_to_rgba ────────────────────────────────────────

const SIMPLE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <rect width="100" height="100" fill="red"/>
</svg>"#;

const BLUE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <rect width="100" height="100" fill="blue"/>
</svg>"#;

#[test]
fn render_produces_correct_buffer_size_t083() {
	let result = render_svg_to_rgba(SIMPLE_SVG, 100, 100);
	assert!(result.is_some(), "render_svg_to_rgba must return Some for valid SVG");
	let r = result.unwrap();
	// RGBA = 4 bytes per pixel
	assert_eq!(r.data.len(), 100 * 100 * 4, "buffer must be width*height*4 bytes");
	assert_eq!(r.width, 100);
	assert_eq!(r.height, 100);
}

#[test]
fn render_invalid_svg_returns_none_t083() {
	let result = render_svg_to_rgba("not svg at all", 100, 100);
	assert!(result.is_none(), "invalid SVG must return None");
}

#[test]
fn render_zero_dimension_returns_none_t083() {
	let result = render_svg_to_rgba(SIMPLE_SVG, 0, 0);
	assert!(result.is_none(), "zero-size pixmap must return None");
}

#[test]
fn render_red_svg_has_red_pixels_t083() {
	let r = render_svg_to_rgba(SIMPLE_SVG, 10, 10).unwrap();
	// Top-left pixel should be RGBA = (255, 0, 0, 255)
	assert_eq!(r.data[0], 255, "red channel");
	assert_eq!(r.data[1], 0, "green channel");
	assert_eq!(r.data[2], 0, "blue channel");
}

// ─── visual.rs — calculate_psnr ────────────────────────────────────────────

#[test]
fn psnr_identical_buffers_is_infinity_t084() {
	let data = vec![128u8; 400];
	let psnr = calculate_psnr(&data, &data);
	assert!(
		psnr.is_infinite() && psnr > 0.0,
		"PSNR of identical buffers must be +infinity, got {psnr}"
	);
}

#[test]
fn psnr_max_diff_is_zero_t084() {
	let black = vec![0u8; 400];
	let white = vec![255u8; 400];
	let psnr = calculate_psnr(&black, &white);
	// MSE = 255^2, PSNR = 10 * log10(255^2 / 255^2) = 0
	assert!(
		(psnr - 0.0_f32).abs() < 0.01,
		"PSNR of black vs white must be ~0 dB, got {psnr}"
	);
}

#[test]
fn psnr_mismatched_lengths_is_zero_t084() {
	let a = vec![0u8; 100];
	let b = vec![0u8; 200];
	let psnr = calculate_psnr(&a, &b);
	assert_eq!(psnr, 0.0, "mismatched buffer lengths must return 0.0");
}

#[test]
fn psnr_ranges_from_0_to_infinity_t084() {
	// Slightly different buffers should produce a finite PSNR > 0
	let mut a = vec![128u8; 400];
	let b = vec![128u8; 400];
	a[0] = 129; // single bit difference
	let psnr = calculate_psnr(&a, &b);
	assert!(psnr > 0.0 && psnr.is_finite(), "single-pixel diff must give finite > 0 PSNR");
	assert!(psnr > 50.0, "single-pixel diff in 400 bytes must give high PSNR (got {psnr})");
}

// ─── visual.rs — visual_diff ───────────────────────────────────────────────

#[test]
fn visual_diff_identical_svg_exceeds_threshold_t085() {
	let psnr = visual_diff(SIMPLE_SVG, SIMPLE_SVG, 100, 100);
	assert!(psnr.is_some());
	let psnr = psnr.unwrap();
	assert!(
		psnr > PSNR_THRESHOLD,
		"identical SVG visual_diff must exceed {PSNR_THRESHOLD} dB, got {psnr}"
	);
}

#[test]
fn visual_diff_different_svgs_computes_t085() {
	let psnr = visual_diff(SIMPLE_SVG, BLUE_SVG, 100, 100);
	assert!(psnr.is_some(), "visual_diff must work for different valid SVGs");
	let psnr = psnr.unwrap();
	// Red vs blue: should be markedly lower than identical
	assert!(psnr < PSNR_THRESHOLD, "red vs blue SVG PSNR should be < {PSNR_THRESHOLD} dB, got {psnr}");
}

#[test]
fn visual_diff_invalid_svg_returns_none_t085() {
	let result = visual_diff("bad", SIMPLE_SVG, 100, 100);
	assert!(result.is_none(), "invalid original SVG must return None");
}

// ─── performance.rs ────────────────────────────────────────────────────────

#[test]
fn timing_budget_default_values_t086() {
	let budget = TimingBudget::default();
	assert_eq!(budget.simple, Duration::from_millis(100));
	assert_eq!(budget.complex, Duration::from_millis(1000));
}

#[test]
fn check_timing_passes_within_simple_budget_t086() {
	let budget = TimingBudget::default();
	assert!(
		check_timing(Duration::from_millis(50), 10, &budget).is_ok(),
		"50ms < 100ms budget for 10 layers must pass"
	);
}

#[test]
fn check_timing_passes_within_complex_budget_t086() {
	let budget = TimingBudget::default();
	assert!(
		check_timing(Duration::from_millis(500), 100, &budget).is_ok(),
		"500ms < 1000ms budget for 100 layers must pass"
	);
}

#[test]
fn check_timing_fails_over_simple_budget_t086() {
	let budget = TimingBudget::default();
	assert!(
		check_timing(Duration::from_millis(200), 10, &budget).is_err(),
		"200ms > 100ms budget for simple SVG must fail"
	);
}

#[test]
fn check_timing_fails_over_complex_budget_t086() {
	let budget = TimingBudget::default();
	assert!(
		check_timing(Duration::from_millis(2000), 200, &budget).is_err(),
		"2000ms > 1000ms budget for complex SVG must fail"
	);
}

#[test]
fn check_timing_boundary_at_49_layers_uses_simple_budget_t086() {
	let budget = TimingBudget::default();
	// 49 layers → simple budget (100ms)
	assert!(check_timing(Duration::from_millis(99), 49, &budget).is_ok());
	assert!(check_timing(Duration::from_millis(101), 49, &budget).is_err());
}

#[test]
fn check_timing_boundary_at_50_layers_uses_complex_budget_t086() {
	let budget = TimingBudget::default();
	// 50 layers → complex budget (1000ms)
	assert!(check_timing(Duration::from_millis(999), 50, &budget).is_ok());
	assert!(check_timing(Duration::from_millis(1001), 50, &budget).is_err());
}

#[test]
fn measure_wraps_closure_and_returns_duration_t086() {
	let (result, elapsed) = measure(|| 42_u32);
	assert_eq!(result, 42);
	assert!(elapsed.as_millis() < 100, "trivial closure must complete < 100ms");
}

// ─── strict.rs ─────────────────────────────────────────────────────────────

#[test]
fn strict_disabled_always_passes_t087() {
	let config = StrictConfig { enabled: false, ..StrictConfig::default() };
	// Even with terrible timing and quality, disabled strict never blocks
	let result = check_strict(SIMPLE_SVG, SIMPLE_SVG, Duration::from_secs(999), 9999, &config);
	assert!(result.is_ok(), "disabled strict must always pass");
}

#[test]
fn strict_enabled_identical_svg_passes_t087() {
	let config = StrictConfig { enabled: true, ..StrictConfig::default() };
	let result = check_strict(SIMPLE_SVG, SIMPLE_SVG, Duration::from_millis(10), 5, &config);
	assert!(result.is_ok(), "identical SVG in strict mode must pass: {result:?}");
}

#[test]
fn strict_enabled_timing_violation_fails_t087() {
	let config = StrictConfig { enabled: true, ..StrictConfig::default() };
	// Identical SVG (PSNR = inf) but timing exceeded
	let result = check_strict(SIMPLE_SVG, SIMPLE_SVG, Duration::from_millis(200), 5, &config);
	assert!(result.is_err(), "timing violation must fail strict mode");
}

#[test]
fn strict_psnr_threshold_is_50_t087() {
	assert_eq!(PSNR_THRESHOLD, 50.0_f32);
}

#[test]
fn strict_default_config_is_disabled_t087() {
	let config = StrictConfig::default();
	assert!(!config.enabled, "strict mode must be disabled by default");
	assert_eq!(config.psnr_threshold, 50.0);
}
