//! # Структурный анализ SVG (не семантический!)
//!
//! Этот модуль находит **структурные** дубли (одинаковые поддеревья по SHA256),
//! но НЕ пытается ответить "что это за компонент семантически" (кнопка/инпут/карточка).
//!
//! Пример того, что мы находим:
//! - "Эти два поддерева идентичны по структуре — можно объединить в один компонент"
//!
//! Пример того, что мы НЕ делаем:
//! - "Это синий прямоугольник с текстом → значит это кнопка"
//!
//! См. STATUS.md § Архитектурные принципы v1.0.

pub mod complexity;
pub mod components;
pub mod hierarchy;

use crate::model::analysis::{ComplexityMetrics, Component, HierarchyInfo};
use crate::model::SVGElement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
	pub components: Vec<Component>,
	pub complexity: ComplexityMetrics,
	pub hierarchy: HierarchyInfo,
}

pub fn analyze(element: &SVGElement) -> AnalysisResult {
	let components = components::detect_components(element);
	let hierarchy = hierarchy::analyze_hierarchy(element);
	let complexity = complexity::compute_complexity(element);

	AnalysisResult {
		components,
		complexity,
		hierarchy,
	}
}
