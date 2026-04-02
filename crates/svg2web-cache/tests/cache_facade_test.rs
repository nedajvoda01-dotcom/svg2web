use svg2web_cache::{Cache, CacheKey};

#[test]
fn test_cache_facade_get_set() {
    let cache = Cache::new_temp().expect("temp cache must initialize");
    let key = CacheKey::from_svg("<svg/>", "v1");

    cache
        .set(&key, vec![1_u8, 2, 3])
        .expect("set must succeed");

    assert_eq!(cache.get(&key), Some(vec![1_u8, 2, 3]));
}

#[test]
fn test_cache_invalidate_by_version() {
    let cache = Cache::new_temp().expect("temp cache must initialize");
    let key = CacheKey::from_svg("<svg/>", "v1");
    cache.set(&key, vec![5_u8]).expect("set must succeed");

    let mut mismatched = key.clone();
    mismatched.version = "0.0.0-mismatch".to_string();

    assert_eq!(cache.get(&mismatched), None);
}

#[test]
fn test_cache_invalidate_key_removes_value() {
    let cache = Cache::new_temp().expect("temp cache must initialize");
    let key = CacheKey::from_svg("<svg/>", "v1");
    cache
        .set(&key, vec![9_u8, 9])
        .expect("set must succeed");

    cache.invalidate(&key).expect("invalidate must succeed");

    assert_eq!(cache.get(&key), None);
}

#[test]
fn test_cache_clear_removes_all_values() {
    let cache = Cache::new_temp().expect("temp cache must initialize");
    let key1 = CacheKey::from_svg("<svg id='1'/>", "v1");
    let key2 = CacheKey::from_svg("<svg id='2'/>", "v1");

    cache.set(&key1, vec![1_u8]).expect("set key1 must succeed");
    cache.set(&key2, vec![2_u8]).expect("set key2 must succeed");

    cache.clear().expect("clear must succeed");

    assert_eq!(cache.get(&key1), None);
    assert_eq!(cache.get(&key2), None);
}