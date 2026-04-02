pub mod deduplication;
pub mod minifier;
pub mod path_simplifier;

use crate::model::SVGElement;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct OptimizationConfig {
	pub simplify_paths: bool,
	pub path_tolerance: f64,
	pub deduplicate: bool,
	pub similarity_threshold: f64,
	pub minify_ids: bool,
	pub remove_comments: bool,
}

impl Default for OptimizationConfig {
	fn default() -> Self {
		Self {
			simplify_paths: true,
			path_tolerance: 1.0,
			deduplicate: true,
			similarity_threshold: 0.95,
			minify_ids: true,
			remove_comments: true,
		}
	}
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("path tolerance must be >= 0.0, got {0}")]
	InvalidPathTolerance(f64),
	#[error("similarity threshold must be in [0.0, 1.0], got {0}")]
	InvalidSimilarityThreshold(f64),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn optimize(element: &mut SVGElement, config: &OptimizationConfig) -> Result<()> {
	validate_config(config)?;

	if config.remove_comments {
		remove_comment_nodes(element);
	}

	if config.simplify_paths && config.path_tolerance > 0.0 {
		simplify_paths_in_tree(element, config.path_tolerance);
	}

	if config.deduplicate {
		deduplication::deduplicate_elements(element);
	}

	if config.minify_ids {
		minifier::minify_ids(element);
	}

	Ok(())
}

fn validate_config(config: &OptimizationConfig) -> Result<()> {
	if config.path_tolerance < 0.0 {
		return Err(Error::InvalidPathTolerance(config.path_tolerance));
	}

	if !(0.0..=1.0).contains(&config.similarity_threshold) {
		return Err(Error::InvalidSimilarityThreshold(config.similarity_threshold));
	}

	Ok(())
}

fn remove_comment_nodes(root: &mut SVGElement) {
	root.children
		.retain(|child| child.tag != "#comment" && child.tag != "!--");

	for child in &mut root.children {
		remove_comment_nodes(child);
	}
}

fn simplify_paths_in_tree(root: &mut SVGElement, tolerance: f64) {
	if root.tag == "path" {
		let _ = tolerance;
	}

	for child in &mut root.children {
		simplify_paths_in_tree(child, tolerance);
	}
}
