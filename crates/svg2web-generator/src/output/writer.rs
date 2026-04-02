use std::fs;
use std::path::Path;

use crate::registry::RenderedOutput;
use crate::{Error, Result};

pub struct OutputOptions {
	pub overwrite: bool,
}

impl Default for OutputOptions {
	fn default() -> Self {
		Self { overwrite: true }
	}
}

pub fn write_files(output: &RenderedOutput, base_dir: &Path, opts: &OutputOptions) -> Result<()> {
	for file in &output.files {
		let path = base_dir.join(&file.name);

		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent)
				.map_err(|e| Error::RenderError(format!("failed to create output dir: {e}")))?;
		}

		if !opts.overwrite && path.exists() {
			continue;
		}

		fs::write(&path, &file.content)
			.map_err(|e| Error::RenderError(format!("failed to write file {}: {e}", path.display())))?;
	}

	Ok(())
}
