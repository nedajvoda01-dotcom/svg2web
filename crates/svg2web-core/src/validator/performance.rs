//! Processing time budgets for SVG conversion pipeline.
//!
//! Enforces latency constraints based on SVG complexity (layer count):
//! - Simple SVGs (< 50 layers): max 100 ms
//! - Complex SVGs (≥ 50 layers): max 1000 ms

use std::time::{Duration, Instant};

/// Per-tier timing budgets for the SVG conversion pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingBudget {
	/// Maximum allowed duration for SVGs with fewer than 50 layers.
	pub simple: Duration,
	/// Maximum allowed duration for SVGs with 50 or more layers.
	pub complex: Duration,
}

impl Default for TimingBudget {
	fn default() -> Self {
		Self {
			simple: Duration::from_millis(100),
			complex: Duration::from_millis(1000),
		}
	}
}

/// Returned when a processing step exceeds its timing budget.
#[derive(Debug)]
pub struct TimingViolation {
	/// Actual elapsed time.
	pub elapsed: Duration,
	/// The budget that was exceeded.
	pub budget: Duration,
	/// Number of layers in the processed SVG.
	pub layers: usize,
}

/// Check whether `elapsed` is within the appropriate budget for `layers`.
///
/// - `layers < 50` → `budget.simple`
/// - `layers >= 50` → `budget.complex`
pub fn check_timing(
	elapsed: Duration,
	layers: usize,
	budget: &TimingBudget,
) -> Result<(), TimingViolation> {
	let limit = if layers < 50 { budget.simple } else { budget.complex };
	if elapsed > limit {
		Err(TimingViolation { elapsed, budget: limit, layers })
	} else {
		Ok(())
	}
}

/// Execute a closure and return both its result and elapsed wall-clock time.
pub fn measure<F, R>(f: F) -> (R, Duration)
where
	F: FnOnce() -> R,
{
	let start = Instant::now();
	let result = f();
	(result, start.elapsed())
}
