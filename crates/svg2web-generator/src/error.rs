use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
	RenderError(String),
}

impl Error {
	pub fn render_error(message: &str) -> Self {
		Self::RenderError(message.to_string())
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::RenderError(message) => write!(f, "render error: {message}"),
		}
	}
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
