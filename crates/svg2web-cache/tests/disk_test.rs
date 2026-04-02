use std::path::PathBuf;

use svg2web_cache::{CacheKey, DiskCache};

fn temp_cache_dir(test_name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "svg2web-cache-{}-{}-{}",
        test_name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    path
}

#[test]
fn test_disk_cache_set_get_roundtrip() {
    let dir = temp_cache_dir("roundtrip");
    let cache = DiskCache::new(&dir).expect("disk cache must initialize");

    let key = CacheKey::from_svg("<svg/>", "v1");
    let data = vec![1_u8, 2, 3, 4];

    cache
        .set(key.clone(), data.clone())
        .expect("disk cache set must succeed");
    let loaded = cache.get(&key).expect("disk cache get must return value");

    assert_eq!(loaded, data);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_disk_cache_miss_returns_none() {
    let dir = temp_cache_dir("miss");
    let cache = DiskCache::new(&dir).expect("disk cache must initialize");

    let key = CacheKey::from_svg("missing", "v1");
    assert_eq!(cache.get(&key), None);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_disk_cache_overwrite_existing_value() {
    let dir = temp_cache_dir("overwrite");
    let cache = DiskCache::new(&dir).expect("disk cache must initialize");

    let key = CacheKey::from_svg("<svg/>", "v1");
    cache
        .set(key.clone(), vec![1_u8])
        .expect("first set must succeed");
    cache
        .set(key.clone(), vec![9_u8, 9])
        .expect("second set must succeed");

    assert_eq!(cache.get(&key), Some(vec![9_u8, 9]));
    let _ = std::fs::remove_dir_all(&dir);
}