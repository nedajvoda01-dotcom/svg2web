use svg2web_core::optimizer::deduplication::deduplicate_elements;
use svg2web_core::{parse, SVGElement};

#[test]
fn test_deduplication_removes_duplicates() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10">
        <rect x="0" y="0" width="10" height="10" fill="red"/>
        <rect x="0" y="0" width="10" height="10" fill="red"/>
    </svg>"#;

    let mut element = parse(svg).expect("parse should succeed");
    let original_count = count_nodes(&element);

    let removed = deduplicate_elements(&mut element);
    let new_count = count_nodes(&element);

    assert!(removed > 0, "deduplication should remove at least one duplicate");
    assert!(new_count < original_count, "node count should decrease after deduplication");
}

fn count_nodes(node: &SVGElement) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}
