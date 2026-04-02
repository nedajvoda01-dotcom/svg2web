use crate::model::SVGElement;

/// Builds a deterministic string signature for an SVG subtree.
///
/// Used for structural deduplication and component detection.
/// The signature is order-independent for attributes (sorted) and captures
/// the full recursive shape of the subtree.
pub fn subtree_signature(element: &SVGElement) -> String {
    let mut pairs: Vec<(&str, &str)> = element
        .attributes
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    pairs.sort_unstable_by(|a, b| a.0.cmp(b.0).then(a.1.cmp(b.1)));

    let attrs = pairs
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(";");

    let children = element
        .children
        .iter()
        .map(subtree_signature)
        .collect::<Vec<_>>()
        .join("|");

    format!(
        "id={:?};tag={};etype={:?};attrs={};text={:?};children=[{}]",
        element.id, element.tag, element.element_type, attrs, element.text_content, children
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use crate::model::element::{ElementType, SVGElement};

    fn make_element(tag: &str, attrs: &[(&str, &str)]) -> SVGElement {
        SVGElement {
            id: None,
            tag: tag.to_string(),
            element_type: ElementType::Unknown,
            attributes: attrs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<HashMap<_, _>>(),
            children: vec![],
            text_content: None,
        }
    }

    #[test]
    fn different_elements_have_different_signatures() {
        let e1 = make_element("circle", &[("cx", "10"), ("cy", "10"), ("r", "5")]);
        let e2 = make_element("circle", &[("cx", "20"), ("cy", "20"), ("r", "5")]);
        let e3 = make_element("rect", &[("x", "0"), ("y", "0"), ("width", "10")]);
        let e4 = make_element("path", &[("d", "M0,0 L10,10")]);

        let sigs: HashSet<String> = [&e1, &e2, &e3, &e4]
            .iter()
            .map(|e| subtree_signature(e))
            .collect();

        assert_eq!(sigs.len(), 4, "Duplicate signature detected among distinct elements");
    }

    #[test]
    fn same_element_produces_stable_signature() {
        let e = make_element("circle", &[("cx", "10"), ("r", "5")]);
        let s1 = subtree_signature(&e);
        let s2 = subtree_signature(&e);
        assert_eq!(s1, s2);
    }

    #[test]
    fn attribute_order_does_not_affect_signature() {
        let e1 = make_element("rect", &[("x", "0"), ("y", "5"), ("width", "10")]);
        let e2 = make_element("rect", &[("width", "10"), ("x", "0"), ("y", "5")]);
        assert_eq!(subtree_signature(&e1), subtree_signature(&e2));
    }
}
