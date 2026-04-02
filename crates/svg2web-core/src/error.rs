use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("parse error: {0}")]
	Parse(#[from] crate::parser::ParseError),
	#[error("validation error: {0}")]
	Validation(#[from] crate::validator::ValidationError),
	#[error("optimize error: {0}")]
	Optimize(#[from] crate::optimizer::Error),
	#[error("extract error: {0}")]
	Extract(#[from] crate::extractor::Error),
	#[error("serialize error: {0}")]
	Serialize(#[from] crate::serializer::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
