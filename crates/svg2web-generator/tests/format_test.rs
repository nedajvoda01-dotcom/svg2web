fn workspace_root() -> std::path::PathBuf {
    let mut dir = std::env::current_dir().expect("current dir must be readable");
    loop {
        if dir.join("docs").is_dir() && dir.join("crates").is_dir() {
            return dir;
        }
        assert!(dir.pop(), "workspace root not found");
    }
}

fn read_doc(path: &str) -> String {
    std::fs::read_to_string(workspace_root().join(path)).unwrap_or_default()
}

#[test]
fn html_output_files_structure_t018() {
    let d = read_doc("docs/usage/frameworks/html-css.md");
    assert!(d.contains("index.html"));
    assert!(d.contains("styles.css"));
    assert!(d.contains("script.js"));
}

#[test]
fn css_variables_generation_t019() {
    let d = read_doc("docs/usage/frameworks/html-css.md");
    assert!(d.contains(":root"));
    for var in ["--color-primary", "--color-secondary", "--spacing-unit", "--radius-base", "--font-size-base"] {
        assert!(d.contains(var), "missing CSS variable in docs: {var}");
    }
}

#[test]
fn media_queries_existence_t020() {
    let d = read_doc("docs/usage/frameworks/html-css.md");
    assert!(d.contains("@media (min-width:"), "docs must define mobile-first media queries");
    assert!(d.contains("768px") && d.contains("1024px"));
}

#[test]
fn tsx_extension_contract_t021() {
    let d = read_doc("docs/usage/frameworks/react.md");
    assert!(d.contains(".tsx"), "React docs must require .tsx output");
    assert!(!d.contains(".jsx"), "React docs must not advertise .jsx output");
}

#[test]
fn functional_component_pattern_t022() {
    let d = read_doc("docs/usage/frameworks/react.md");
    assert!(d.contains("functional components") || d.contains("functional component"));
    assert!(d.contains("const Logo =") || d.contains("memo(function Logo"));
    assert!(!d.contains("class ComponentName extends React.Component"));
}

#[test]
fn props_interface_completeness_t023() {
    let d = read_doc("docs/usage/frameworks/react.md");
    for field in ["size?", "color?", "className?", "style?", "onClick?"] {
        assert!(d.contains(field), "missing React prop in docs: {field}");
    }
}

#[test]
fn hooks_usage_for_interactive_t024() {
    let d = read_doc("docs/usage/frameworks/react.md");
    assert!(d.contains("useState"), "interactive React output must mention useState");
    assert!(d.contains("useEffect"), "interactive React output must mention useEffect");
}

#[test]
fn sfc_format_validation_t025() {
    let d = read_doc("docs/usage/frameworks/vue.md");
    assert!(d.contains(".vue"));
    assert!(d.contains("<template>"));
    assert!(d.contains("<script setup"));
    assert!(d.contains("<style scoped>"));
}

#[test]
fn scoped_styles_isolation_t026() {
    let d = read_doc("docs/usage/frameworks/vue.md");
    assert!(d.contains("scoped"), "Vue docs must require scoped styles");
    assert!(d.contains(":global(") || d.contains("data-v-"), "Vue docs must describe scoped isolation mechanics");
}

#[test]
fn define_props_with_defaults_t027() {
    let d = read_doc("docs/usage/frameworks/vue.md");
    assert!(d.contains("withDefaults(defineProps"), "Vue docs must use defineProps with defaults");
    assert!(!d.contains("export default { props:"), "Vue docs must not use options API props pattern");
}

#[test]
fn es_modules_format_t028() {
    let d = read_doc("docs/usage/frameworks/vanilla.md");
    assert!(d.contains("ES Module") || d.contains("ESM"));
    assert!(d.contains("export/") || d.contains("export/import") || d.contains("import"));
    assert!(!d.contains("module.exports"), "Vanilla docs must not advertise CommonJS output");
}

#[test]
fn optional_web_components_t029() {
    let d = read_doc("docs/usage/frameworks/vanilla.md");
    assert!(d.contains("Web Component"));
    assert!(d.contains("customElements.define"), "Vanilla docs must document customElements.define for WC mode");
}

#[test]
fn minimal_runtime_size_t030() {
    let d = read_doc("docs/usage/frameworks/vanilla.md");
    assert!(d.contains("0-4 KB gzip") || d.contains("< 5KB") || d.contains("5KB"));
}

// T-077: Структура директорий шаблонов
#[test]
fn template_directories_documented_t077() {
    let d = read_doc("docs/internals/generator/templates.md");
    for dir in ["templates/base/", "templates/react/", "templates/vue/"] {
        assert!(
            d.contains(dir),
            "templates.md must document template directory: {dir}"
        );
    }
}

#[test]
fn tera_templating_engine_documented_t077() {
    let d = read_doc("docs/internals/generator/templates.md");
    assert!(
        d.contains("Tera") || d.contains("tera"),
        "templates.md must document Tera as templating engine"
    );
    assert!(
        d.contains("LRU") || d.contains("lru"),
        "templates.md must document LRU cache for compiled templates"
    );
}

// T-078: Кастомные фильтры
#[test]
fn custom_filters_documented_t078() {
    let d = read_doc("docs/internals/generator/templates.md");
    assert!(
        d.contains("to_camel_case"),
        "templates.md must document to_camel_case filter"
    );
    assert!(
        d.contains("css_value"),
        "templates.md must document css_value filter (Color -> CSS string)"
    );
    assert!(
        d.contains("px"),
        "templates.md must document px filter (f64 -> '10px')"
    );
}

// T-079: Наследование шаблонов
#[test]
fn template_inheritance_blocks_documented_t079() {
    let d = read_doc("docs/internals/generator/templates.md");
    for block in ["head", "body", "imports"] {
        assert!(
            d.contains(block),
            "templates.md must document inherited block: {block}"
        );
    }
}

#[test]
fn template_lru_cache_documented_t079() {
    let d = read_doc("docs/internals/generator/templates.md");
    assert!(
        d.contains("LRU") && d.contains("кэш") || d.contains("LRU-кэш"),
        "templates.md must document LRU cache for compiled templates"
    );
}

// T-081: Оптимизации под фреймворки
#[test]
fn react_memo_and_ts_interfaces_t081() {
    let d = read_doc("docs/internals/generator/output-formats.md");
    assert!(
        d.contains("React.memo"),
        "output-formats.md must document React.memo for static components"
    );
    assert!(
        d.contains("TypeScript interfaces"),
        "output-formats.md must document TypeScript interfaces generation from props"
    );
}

#[test]
fn vue_define_async_component_t081() {
    let d = read_doc("docs/internals/generator/output-formats.md");
    assert!(
        d.contains("defineAsyncComponent"),
        "output-formats.md must document defineAsyncComponent for Vue code splitting"
    );
}

#[test]
fn html_tree_shaking_es_modules_t081() {
    let d = read_doc("docs/internals/generator/output-formats.md");
    assert!(
        d.contains("tree-shaking"),
        "output-formats.md must document tree-shaking compatibility"
    );
    assert!(
        d.contains("ES module") || d.contains("ES Module"),
        "output-formats.md must document ES module exports for tree-shaking"
    );
}
