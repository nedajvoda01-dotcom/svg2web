use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct CacheKey {
	pub hash: String,
	pub version: String,
	pub config_hash: String,
}

impl CacheKey {
	pub fn from_svg(svg: &str, config: &str) -> Self {
		let mut hasher = Sha256::new();
		hasher.update(svg.as_bytes());
		hasher.update(config.as_bytes());
		let hash = format!("{:x}", hasher.finalize());

		Self {
			hash,
			version: env!("CARGO_PKG_VERSION").to_string(),
			config_hash: format!("{:x}", Sha256::digest(config.as_bytes())),
		}
	}
}

impl std::fmt::Display for CacheKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}-{}-{}", self.hash, self.version, self.config_hash)
	}
}
