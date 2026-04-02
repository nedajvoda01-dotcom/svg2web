use svg2web_cache::{CacheKey, MemoryCache};

#[test]
fn test_memory_cache_hit() {
    let cache = MemoryCache::new(100);
    let key = CacheKey::from_svg("test", "v1");
    let data = vec![1, 2, 3];

    cache
        .set(key.clone(), data.clone())
        .expect("set should not fail");
    let result = cache.get(&key);

    assert_eq!(result, Some(data));
}

#[test]
fn test_memory_cache_miss() {
    let cache = MemoryCache::new(100);
    let key = CacheKey::from_svg("nonexistent", "v1");

    assert_eq!(cache.get(&key), None);
}

#[test]
fn test_memory_cache_no_panic_on_empty_key() {
    let cache = MemoryCache::new(100);
    let key = CacheKey::from_svg("", "");

    let _ = cache.get(&key);
    let _ = cache.set(key, vec![]);
}
