use std::path::Path;

use crate::CacheKey;

#[derive(Debug)]
pub enum DiskCacheError {
	Open(sled::Error),
	Write(sled::Error),
	Remove(sled::Error),
	Clear(sled::Error),
	Flush(sled::Error),
}

pub struct DiskCache {
	db: sled::Db,
}

impl DiskCache {
	pub fn new(path: &Path) -> Result<Self, DiskCacheError> {
		let db = sled::open(path).map_err(DiskCacheError::Open)?;
		Ok(Self { db })
	}

	pub fn get(&self, key: &CacheKey) -> Option<Vec<u8>> {
		self.db
			.get(key.to_string().as_bytes())
			.ok()
			.flatten()
			.map(|bytes| bytes.to_vec())
	}

	pub fn set(&self, key: CacheKey, entry: Vec<u8>) -> Result<(), DiskCacheError> {
		self.db
			.insert(key.to_string().as_bytes(), entry)
			.map_err(DiskCacheError::Write)?;
		self.db.flush().map_err(DiskCacheError::Flush)?;
		Ok(())
	}

	pub fn invalidate(&self, key: &CacheKey) -> Result<(), DiskCacheError> {
		self.db
			.remove(key.to_string().as_bytes())
			.map_err(DiskCacheError::Remove)?;
		self.db.flush().map_err(DiskCacheError::Flush)?;
		Ok(())
	}

	pub fn clear(&self) -> Result<(), DiskCacheError> {
		self.db.clear().map_err(DiskCacheError::Clear)?;
		self.db.flush().map_err(DiskCacheError::Flush)?;
		Ok(())
	}
}
