use std::io::{Seek, Write};

use zip::write::FileOptions;
use zip::ZipWriter;

use crate::registry::RenderedOutput;
use crate::{Error, Result};

pub fn write_zip<W: Write + Seek>(output: &RenderedOutput, writer: W) -> Result<()> {
	let mut zip = ZipWriter::new(writer);
	let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

	for file in &output.files {
		zip.start_file(&file.name, options)
			.map_err(|e| Error::RenderError(format!("failed to start zip file {}: {e}", file.name)))?;
		zip.write_all(file.content.as_bytes())
			.map_err(|e| Error::RenderError(format!("failed to write zip file {}: {e}", file.name)))?;
	}

	zip.finish()
		.map_err(|e| Error::RenderError(format!("failed to finalize zip: {e}")))?;

	Ok(())
}
