use std::fs;
use std::path::{Path, PathBuf};

use crate::registry::RenderedOutput;
use crate::{Error, Result};

use super::OutputOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
	Auto,
	Vanilla,
	React,
	Vue,
}

pub struct OutputStructure<'a> {
	base_path: &'a Path,
	format: OutputFormat,
}

impl<'a> OutputStructure<'a> {
	pub fn new(base_path: &'a Path) -> Self {
		Self {
			base_path,
			format: OutputFormat::Auto,
		}
	}

	pub fn with_format(mut self, format: OutputFormat) -> Self {
		self.format = format;
		self
	}

	pub fn write(&self, output: &RenderedOutput, opts: &OutputOptions) -> Result<()> {
		let format = self.detect_format(output);
		match format {
			OutputFormat::Vanilla => self.write_vanilla_structure(output, opts),
			OutputFormat::React => self.write_react_structure(output, opts),
			OutputFormat::Vue => self.write_vue_structure(output, opts),
			OutputFormat::Auto => self.write_vanilla_structure(output, opts),
		}
	}

	fn detect_format(&self, output: &RenderedOutput) -> OutputFormat {
		if self.format != OutputFormat::Auto {
			return self.format;
		}

		let has_vue = output.files.iter().any(|f| f.name.ends_with(".vue"));
		if has_vue {
			return OutputFormat::Vue;
		}

		let has_react = output.files.iter().any(|f| f.name.ends_with(".tsx") || f.name.ends_with(".ts"));
		if has_react {
			return OutputFormat::React;
		}

		OutputFormat::Vanilla
	}

	fn write_vanilla_structure(&self, output: &RenderedOutput, opts: &OutputOptions) -> Result<()> {
		fs::create_dir_all(self.base_path.join("css"))
			.map_err(|e| Error::render_error(&format!("failed to create css dir: {e}")))?;
		fs::create_dir_all(self.base_path.join("assets/vectors"))
			.map_err(|e| Error::render_error(&format!("failed to create assets dir: {e}")))?;
		fs::create_dir_all(self.base_path.join("js/components"))
			.map_err(|e| Error::render_error(&format!("failed to create js/components dir: {e}")))?;

		for file in &output.files {
			let path = match file.name.as_str() {
				"variables.css" => self.base_path.join("css/variables.css"),
				"base.css" => self.base_path.join("css/base.css"),
				"styles.css" => self.base_path.join("css/styles.css"),
				"script.js" | "component.js" => self.base_path.join("js/components/component.js"),
				"index.html" => self.base_path.join("index.html"),
				_ => self.base_path.join(&file.name),
			};
			self.write_one(&path, &file.content, opts)?;
		}

		self.ensure_file(self.base_path.join("css/variables.css"), "/* design tokens */\n", opts)?;
		self.ensure_file(self.base_path.join("css/base.css"), "/* base styles */\n", opts)?;
		self.ensure_file(
			self.base_path.join("figma.json"),
			"{\n  \"version\": \"1.0.0\",\n  \"generator\": \"svg2web\",\n  \"structure\": \"vanilla\"\n}\n",
			opts,
		)?;

		Ok(())
	}

	fn write_react_structure(&self, output: &RenderedOutput, opts: &OutputOptions) -> Result<()> {
		fs::create_dir_all(self.base_path.join("src/components"))
			.map_err(|e| Error::render_error(&format!("failed to create src/components dir: {e}")))?;
		fs::create_dir_all(self.base_path.join("public"))
			.map_err(|e| Error::render_error(&format!("failed to create public dir: {e}")))?;

		for file in &output.files {
			let path = if file.name.ends_with(".tsx") || file.name.ends_with(".ts") {
				self.base_path.join("src/components").join(&file.name)
			} else if file.name.ends_with(".html") {
				self.base_path.join("public/index.html")
			} else {
				self.base_path.join(&file.name)
			};
			self.write_one(&path, &file.content, opts)?;
		}

		self.ensure_file(
			self.base_path.join("package.json"),
			"{\n  \"name\": \"svg2web-react-output\",\n  \"version\": \"0.1.0\"\n}\n",
			opts,
		)?;

		Ok(())
	}

	fn write_vue_structure(&self, output: &RenderedOutput, opts: &OutputOptions) -> Result<()> {
		fs::create_dir_all(self.base_path.join("src/components"))
			.map_err(|e| Error::render_error(&format!("failed to create src/components dir: {e}")))?;

		for file in &output.files {
			let path = if file.name.ends_with(".vue") {
				self.base_path.join("src/components").join(&file.name)
			} else {
				self.base_path.join(&file.name)
			};
			self.write_one(&path, &file.content, opts)?;
		}

		Ok(())
	}

	fn ensure_file(&self, path: PathBuf, content: &str, opts: &OutputOptions) -> Result<()> {
		if path.exists() {
			return Ok(());
		}
		self.write_one(&path, content, opts)
	}

	fn write_one(&self, path: &Path, content: &str, opts: &OutputOptions) -> Result<()> {
		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent)
				.map_err(|e| Error::render_error(&format!("failed to create output dir: {e}")))?;
		}

		if !opts.overwrite && path.exists() {
			return Ok(());
		}

		fs::write(path, content)
			.map_err(|e| Error::render_error(&format!("failed to write {}: {e}", path.display())))
	}
}
