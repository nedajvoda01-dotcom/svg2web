//! Архитектурные ограничения v1.0 — JSON-First, No Semantic Analysis.
//!
//! Эти тесты ломаются при нарушении принципов проекта.
//! Если тест упал — обсуди изменение с командой ПЕРЕД тем как исправлять тест.

#[cfg(test)]
mod tests {
    use crate::model::analysis::Component;

    /// Component содержит только структурную информацию (id, template, occurrences).
    /// Если кто-то добавит поле `semantic_type`, `ui_role`, `component_class` —
    /// размер структуры изменится и тест сломается.
    ///
    /// Текущие поля:
    /// - id: String (SHA256 хеш структуры)
    /// - template: SVGElement (поддерево-шаблон)
    /// - occurrences: Vec<Occurrence> (места вхождения)
    #[test]
    fn no_semantic_classification_in_model() {
        // Component = String (24) + SVGElement (~128) + Vec<Occurrence> (24) + padding
        // Текущий измеренный размер: 200 байт на 64-bit Linux.
        //
        // Если тест упал — кто-то добавил поле в Component.
        // При ЛЕГИТИМНОМ расширении: обнови константу + добавь обоснование в PR.
        // Запрещены поля: ui_role, semantic_type, component_class, is_button, etc.
        const EXPECTED_SIZE: usize = 200;
        assert_eq!(
            std::mem::size_of::<Component>(),
            EXPECTED_SIZE,
            "Component struct size changed ({} bytes, expected {EXPECTED_SIZE}). \
             Did you add semantic classification fields? \
             We do NOT classify UI components. See STATUS.md § Архитектурные принципы v1.0.",
            std::mem::size_of::<Component>(),
        );
    }

    /// Исходники model/ не должны содержать семантических классификаций.
    #[test]
    fn no_semantic_keywords_in_model_source() {
        let model_source = include_str!("model/analysis.rs");
        let element_source = include_str!("model/element.rs");
        let combined = format!("{model_source}\n{element_source}");

        let forbidden = [
            "semantic_type",
            "component_class",
            "ui_role",
            "is_button",
            "is_card",
            "is_input",
            "is_navbar",
            "is_modal",
        ];

        for keyword in forbidden {
            assert!(
                !combined.contains(keyword),
                "Forbidden semantic keyword '{}' found in model source. \
                 We do NOT classify UI components. See STATUS.md § Архитектурные принципы.",
                keyword,
            );
        }
    }

    /// Cargo.toml не должен содержать ML/AI зависимостей.
    /// Мы не используем машинное обучение для распознавания компонентов.
    #[test]
    fn no_ml_dependencies_in_cargo_toml() {
        let cargo_toml = include_str!("../Cargo.toml");

        let forbidden = [
            "tch",
            "onnx",
            "tract",
            "candle",
            "tokenizers",
            "tensorflow",
            "pytorch",
        ];

        for dep in forbidden {
            assert!(
                !cargo_toml.contains(dep),
                "Forbidden ML dependency '{}' found in Cargo.toml. \
                 We do NOT do semantic analysis. See STATUS.md § Архитектурные принципы.",
                dep,
            );
        }
    }
}
