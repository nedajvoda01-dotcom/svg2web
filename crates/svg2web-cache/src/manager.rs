use crate::memory::CacheError;
use crate::{CacheKey, MemoryCache};

#[cfg(feature = "disk")]
use crate::disk::{DiskCache, DiskCacheError};

#[derive(Debug)]
pub enum CacheManagerError {
	Memory(CacheError),
	#[cfg(feature = "disk")]
	Disk(DiskCacheError),
}

pub struct CacheManager {
	#[cfg(feature = "disk")]
	disk: Option<DiskCache>,
	memory: MemoryCache,
	version: String,
}

impl CacheManager {
	#[cfg(feature = "disk")]
	pub fn new(disk: Option<DiskCache>, memory_size: usize) -> Self {
		Self {
			disk,
			memory: MemoryCache::new(memory_size),
			version: env!("CARGO_PKG_VERSION").to_string(),
		}
	}

	#[cfg(not(feature = "disk"))]
	pub fn new(memory_size: usize) -> Self {
		Self {
			memory: MemoryCache::new(memory_size),
			version: env!("CARGO_PKG_VERSION").to_string(),
		}
	}

	pub fn get(&self, key: &CacheKey) -> Option<Vec<u8>> {
		if key.version != self.version {
			return None;
		}

		if let Some(value) = self.memory.get(key) {
			return Some(value);
		}

		#[cfg(feature = "disk")]
		{
			if let Some(value) = self.disk.as_ref().and_then(|disk| disk.get(key)) {
				let _ = self.memory.set(key.clone(), value.clone());
				return Some(value);
			}
		}

		None
	}

	pub fn set(&self, key: CacheKey, value: Vec<u8>) -> Result<(), CacheManagerError> {
		if key.version != self.version {
			return Ok(());
		}

		self
			.memory
			.set(key.clone(), value.clone())
			.map_err(CacheManagerError::Memory)?;

		#[cfg(feature = "disk")]
		if let Some(disk) = &self.disk {
			disk
				.set(key, value)
				.map_err(CacheManagerError::Disk)?;
		}

		Ok(())
	}

	pub fn invalidate_old_versions(&self) {
		let _ = &self.version;
	}

	pub fn invalidate(&self, key: &CacheKey) -> Result<(), CacheManagerError> {
		self
			.memory
			.invalidate(key)
			.map_err(CacheManagerError::Memory)?;

		#[cfg(feature = "disk")]
		if let Some(disk) = &self.disk {
			disk
				.invalidate(key)
				.map_err(CacheManagerError::Disk)?;
		}

		Ok(())
	}

	pub fn clear(&self) -> Result<(), CacheManagerError> {
		self.memory.clear().map_err(CacheManagerError::Memory)?;

		#[cfg(feature = "disk")]
		if let Some(disk) = &self.disk {
			disk.clear().map_err(CacheManagerError::Disk)?;
		}

		Ok(())
	}
}
