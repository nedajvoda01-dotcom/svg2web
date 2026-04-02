use svg2web_cache::CacheKey;

#[test]
fn test_key_deterministic() {
    let key1 = CacheKey::from_svg("<svg></svg>", "config_v1");
    let key2 = CacheKey::from_svg("<svg></svg>", "config_v1");
    assert_eq!(key1.hash, key2.hash);
    assert_eq!(key1.to_string(), key2.to_string());
}

#[test]
fn test_key_version_invalidates() {
    let key1 = CacheKey::from_svg("<svg></svg>", "config_v1");
    let key2 = CacheKey::from_svg("<svg></svg>", "config_v2");
    assert_ne!(key1.to_string(), key2.to_string());
}

#[test]
fn test_key_different_svg_different_hash() {
    let key1 = CacheKey::from_svg("<svg></svg>", "config_v1");
    let key2 = CacheKey::from_svg("<svg><rect/></svg>", "config_v1");
    assert_ne!(key1.hash, key2.hash);
}
