//! Strict mode: combined PSNR + timing gate for `--strict` CLI flag.
//!
//! When enabled, the conversion pipeline is **blocked** (returns an error) if:
//! - Visual quality drops below `psnr_threshold` dB (default: 50.0 dB)
//! - Processing time exceeds the budget for the SVG's complexity tier
//!
//! # Example (CLI behaviour)
//! ```text
//! svg2web build input.svg --format react --strict
//! error: PSNR 42.3 dB is below threshold 50.0 dB — visual quality degraded
//! ```

use std::time::Duration;
use thiserror::Error;

use super::performance::{check_timing, TimingBudget, TimingViolation};
use super::visual::visual_diff;

/// PSNR threshold in dB below which visual degradation is considered unacceptable.
pub const PSNR_THRESHOLD: f32 = 50.0;

/// Reason why strict mode blocked the pipeline.
#[derive(Debug, Error)]
pub enum StrictViolation {
	#[error("PSNR {psnr:.1} dB is below threshold {threshold:.1} dB — visual quality degraded")]
	PsnrViolation {
		/// Measured PSNR score (dB).
		psnr: f32,
		/// Configured threshold (default: 50.0 dB).
		threshold: f32,
	},
	#[error(
		"Processing took {elapsed:?}, budget was {budget:?} for {layers} layers"
	)]
	TimingViolation {
		/// Actual elapsed processing time.
		elapsed: Duration,
		/// The allowed budget for the complexity tier.
		budget: Duration,
		/// Layer count used to select the budget tier.
		layers: usize,
	},
}

impl From<TimingViolation> for StrictViolation {
	fn from(v: TimingViolation) -> Self {
		StrictViolation::TimingViolation { elapsed: v.elapsed, budget: v.budget, layers: v.layers }
	}
}

/// Configuration for strict mode validation.
#[derive(Debug, Clone)]
pub struct StrictConfig {
	/// Whether strict mode is active. Default: `false`.
	pub enabled: bool,
	/// Minimum acceptable PSNR in dB. Default: [`PSNR_THRESHOLD`] (50.0).
	pub psnr_threshold: f32,
	/// Per-tier timing budgets. Default: 100 ms (simple) / 1000 ms (complex).
	pub timing_budget: TimingBudget,
}

impl Default for StrictConfig {
	fn default() -> Self {
		Self {
			enabled: false,
			psnr_threshold: PSNR_THRESHOLD,
			timing_budget: TimingBudget::default(),
		}
	}
}

/// Run strict-mode checks for a completed conversion step.
///
/// - If `config.enabled` is `false`, always returns `Ok(())`.
/// - Renders both SVGs at 100×100 px and checks PSNR.
/// - Checks `elapsed` against the timing budget.
///
/// PSNR is checked first; a timing violation is only reported if PSNR passes.
pub fn check_strict(
	original_svg: &str,
	result_svg: &str,
	elapsed: Duration,
	layers: usize,
	config: &StrictConfig,
) -> Result<(), StrictViolation> {
	if !config.enabled {
		return Ok(());
	}

	if let Some(psnr) = visual_diff(original_svg, result_svg, 100, 100) {
		if psnr < config.psnr_threshold {
			return Err(StrictViolation::PsnrViolation {
				psnr,
				threshold: config.psnr_threshold,
			});
		}
	}

	check_timing(elapsed, layers, &config.timing_budget).map_err(StrictViolation::from)
}
