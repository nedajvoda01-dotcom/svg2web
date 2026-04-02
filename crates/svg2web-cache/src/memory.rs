use crate::CacheKey;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Mutex;

#[derive(Debug)]
pub enum CacheError {
	LockPoisoned,
}

pub struct MemoryCache {
	cache: Mutex<LruCache<String, Vec<u8>>>,
}

impl MemoryCache {
	pub fn new(max_size: usize) -> Self {
		let size = if let Some(v) = NonZeroUsize::new(max_size.max(1)) {
			v
		} else {
			NonZeroUsize::MIN
		};
		Self {
			cache: Mutex::new(LruCache::new(size)),
		}
	}

	pub fn get(&self, key: &CacheKey) -> Option<Vec<u8>> {
		let mut cache = self.cache.lock().ok()?;
		cache.get(&key.to_string()).cloned()
	}

	pub fn set(&self, key: CacheKey, value: Vec<u8>) -> Result<(), CacheError> {
		let mut cache = self.cache.lock().map_err(|_| CacheError::LockPoisoned)?;
		cache.put(key.to_string(), value);
		Ok(())
	}

	pub fn invalidate(&self, key: &CacheKey) -> Result<(), CacheError> {
		let mut cache = self.cache.lock().map_err(|_| CacheError::LockPoisoned)?;
		let _ = cache.pop(&key.to_string());
		Ok(())
	}

	pub fn clear(&self) -> Result<(), CacheError> {
		let mut cache = self.cache.lock().map_err(|_| CacheError::LockPoisoned)?;
		cache.clear();
		Ok(())
	}
}
