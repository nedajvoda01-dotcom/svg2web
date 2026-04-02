# Workspace Test Suite

Этот каталог содержит workspace-level проверки:

- docs_integrity.rs (T-001, T-002)
- docs/changelog_lint.rs (T-004)
- architecture/boundaries.rs (T-061, T-062)
- json_schema/validation.rs (T-054..T-060)
- development/setup_contract.rs (T-093, T-094)
- development/testing_contract.rs (T-095, T-097)
- development/benchmarking_contract.rs (T-098, T-099)
- development/debugging_contract.rs (T-100, T-101)
- development/releasing_contract.rs (T-102, T-105)
- contributing/contracts.rs (T-106, T-111)
- migration/v01_v02.rs (T-114, T-117)
- examples_validation.rs

Internals (T-061..T-092) покрываются по крейтам:

- T-061, T-062: tests/architecture/boundaries.rs
- T-063..T-066: crates/svg2web-core/tests/parser_test.rs
- T-067..T-069: crates/svg2web-core/tests/optimizer_test.rs
- T-070..T-073: crates/svg2web-core/tests/contracts_test.rs
- T-074..T-076: crates/svg2web-core/tests/serializer_test.rs
- T-077..T-079, T-081: crates/svg2web-generator/tests/format_test.rs
- T-080: crates/svg2web-generator/tests/component_detection.rs
- T-082..T-084: crates/svg2web-cache/tests/cache_test.rs
- T-085..T-087: crates/svg2web-wasm/tests/wasm_api.rs
- T-088..T-092: web/src/lib/svg2web.test.ts

Примечания:

- T-096 покрывается через development/testing_contract.rs и корневые каталоги tests/fixtures/, tests/snapshots/
- T-103 покрывается и через development/releasing_contract.rs, и через docs/changelog_lint.rs
- T-112 и T-113 покрываются через plugins/tests/plugin_loading.rs
