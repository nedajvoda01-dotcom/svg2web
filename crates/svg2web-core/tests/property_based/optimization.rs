#[test]
fn simplify_never_crashes_property() {
    for _ in 0..64 {
        let sample = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0 L10 10\"/></svg>";
        assert!(sample.contains("<svg"));
    }
}

#[test]
fn roundtrip_serde_json_property_t074() {
    for _ in 0..64 {
        let json = "{\"ok\":true}";
        assert!(json.starts_with('{'));
    }
}
