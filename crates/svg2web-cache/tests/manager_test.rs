use std::path::PathBuf;

use svg2web_cache::{CacheKey, CacheManager, DiskCache};

fn temp_cache_dir(test_name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "svg2web-manager-{}-{}-{}",
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
fn test_manager_set_get_roundtrip() {
    let dir = temp_cache_dir("roundtrip");
    let disk = DiskCache::new(&dir).expect("disk cache must initialize");
    let manager = CacheManager::new(Some(disk), 64);

    let key = CacheKey::from_svg("<svg/>", "v1");
    let data = vec![7_u8, 8, 9];

    manager
        .set(key.clone(), data.clone())
        .expect("manager set must succeed");
    assert_eq!(manager.get(&key), Some(data));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_manager_version_mismatch_returns_none() {
    let dir = temp_cache_dir("version");
    let disk = DiskCache::new(&dir).expect("disk cache must initialize");
    let manager = CacheManager::new(Some(disk), 64);

    let key = CacheKey::from_svg("<svg/>", "v1");
    manager
        .set(key.clone(), vec![1_u8])
        .expect("manager set must succeed");

    let mut mismatched = key.clone();
    mismatched.version = "0.0.0-mismatch".to_string();
    assert_eq!(manager.get(&mismatched), None);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_manager_get_missing_returns_none() {
    let dir = temp_cache_dir("missing");
    let disk = DiskCache::new(&dir).expect("disk cache must initialize");
    let manager = CacheManager::new(Some(disk), 64);

    let missing_key = CacheKey::from_svg("missing", "v1");
    assert_eq!(manager.get(&missing_key), None);

    let _ = std::fs::remove_dir_all(&dir);
}