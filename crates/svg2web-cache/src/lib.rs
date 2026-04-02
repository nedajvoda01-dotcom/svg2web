#[cfg(feature = "disk")]
pub mod disk;
pub mod key;
pub mod manager;
pub mod memory;

#[cfg(feature = "disk")]
pub use disk::{DiskCache, DiskCacheError};
pub use key::CacheKey;
pub use manager::{CacheManager, CacheManagerError};
pub use memory::{CacheError, MemoryCache};

#[derive(Debug)]
pub enum CacheFacadeError {
	Manager(CacheManagerError),
	#[cfg(feature = "disk")]
	Disk(DiskCacheError),
}

pub struct Cache {
	manager: CacheManager,
}

impl Cache {
	pub fn new_with_memory(memory_size: usize) -> Self {
		#[cfg(feature = "disk")]
		{
			Self {
				manager: CacheManager::new(None, memory_size),
			}
		}

		#[cfg(not(feature = "disk"))]
		{
			Self {
				manager: CacheManager::new(memory_size),
			}
		}
	}

	#[cfg(feature = "disk")]
	pub fn new_with_disk(path: &std::path::Path, memory_size: usize) -> Result<Self, CacheFacadeError> {
		let disk = DiskCache::new(path).map_err(CacheFacadeError::Disk)?;
		Ok(Self {
			manager: CacheManager::new(Some(disk), memory_size),
		})
	}

	pub fn new_temp() -> Result<Self, CacheFacadeError> {
		Ok(Self::new_with_memory(256))
	}

	pub fn get(&self, key: &CacheKey) -> Option<Vec<u8>> {
		self.manager.get(key)
	}

	pub fn set(&self, key: &CacheKey, value: Vec<u8>) -> Result<(), CacheFacadeError> {
		self
			.manager
			.set(key.clone(), value)
			.map_err(CacheFacadeError::Manager)
	}

	pub fn invalidate(&self, key: &CacheKey) -> Result<(), CacheFacadeError> {
		self
			.manager
			.invalidate(key)
			.map_err(CacheFacadeError::Manager)
	}

	pub fn clear(&self) -> Result<(), CacheFacadeError> {
		self
			.manager
			.clear()
			.map_err(CacheFacadeError::Manager)
	}
}
