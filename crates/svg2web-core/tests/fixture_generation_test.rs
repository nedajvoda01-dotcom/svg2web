mod support;

use support::fixture_generator::FixtureGenerator;
use std::fs;

/// Генерирует 100 фикстур по категориям: simple (1-20), complex (21-70), stress (71-100).
#[test]
fn test_generates_100_fixtures() {
    let out_dir = "tests/fixtures/synthetic/count";
    let _ = fs::remove_dir_all(out_dir);
    fs::create_dir_all(out_dir).unwrap();

    // simple_001..020: 2-40 элементов, без градиентов/фильтров
    for i in 1..=20 {
        let svg = FixtureGenerator::new(i as u64)
            .with_layers(i * 2)
            .generate();
        fs::write(format!("{out_dir}/simple_{i:03}.svg"), &svg).unwrap();
    }

    // complex_021..070: 50-500 элементов, градиенты, nested groups
    for i in 21..=70 {
        let layers = 10 + (i - 21) * 10; // 10..500
        let svg = FixtureGenerator::new(i as u64)
            .with_layers(layers)
            .with_gradients(true)
            .generate();
        fs::write(format!("{out_dir}/complex_{i:03}.svg"), &svg).unwrap();
    }

    // stress_071..100: 500-1000 элементов, фильтры, text
    for i in 71..=100 {
        let layers = 100 + (i - 71) * 30; // 100..970
        let svg = FixtureGenerator::new(i as u64)
            .with_layers(layers)
            .with_gradients(true)
            .with_filters(true)
            .with_text(true)
            .generate();
        fs::write(format!("{out_dir}/stress_{i:03}.svg"), &svg).unwrap();
    }

    let count = fs::read_dir(out_dir).unwrap().count();
    assert_eq!(count, 100);
}

/// Проверяет что все 100 сгенерированных SVG парсятся через svg2web_core::parse.
#[test]
fn test_all_fixtures_are_valid_svg() {
    let out_dir = "tests/fixtures/synthetic/valid";
    let _ = fs::remove_dir_all(out_dir);
    fs::create_dir_all(out_dir).unwrap();

    // Генерируем все 100 (тест автономен)
    for i in 1..=20 {
        let svg = FixtureGenerator::new(i as u64)
            .with_layers(i * 2)
            .generate();
        fs::write(format!("{out_dir}/simple_{i:03}.svg"), &svg).unwrap();
    }
    for i in 21..=70 {
        let svg = FixtureGenerator::new(i as u64)
            .with_layers(10 + (i - 21) * 10)
            .with_gradients(true)
            .generate();
        fs::write(format!("{out_dir}/complex_{i:03}.svg"), &svg).unwrap();
    }
    for i in 71..=100 {
        let svg = FixtureGenerator::new(i as u64)
            .with_layers(100 + (i - 71) * 30)
            .with_gradients(true)
            .with_filters(true)
            .with_text(true)
            .generate();
        fs::write(format!("{out_dir}/stress_{i:03}.svg"), &svg).unwrap();
    }

    for entry in fs::read_dir(out_dir).unwrap() {
        let path = entry.unwrap().path();
        let content = fs::read_to_string(&path).unwrap();
        assert!(
            svg2web_core::parse(&content).is_ok(),
            "Fixture {} should be valid SVG",
            path.display()
        );
    }
}

/// Одинаковый seed и параметры = одинаковый SVG.
#[test]
fn test_deterministic_output() {
    let gen1 = FixtureGenerator::new(42).with_layers(10).generate();
    let gen2 = FixtureGenerator::new(42).with_layers(10).generate();
    assert_eq!(gen1, gen2);

    // Разные категории тоже детерминированы
    let c1 = FixtureGenerator::new(50)
        .with_layers(200)
        .with_gradients(true)
        .generate();
    let c2 = FixtureGenerator::new(50)
        .with_layers(200)
        .with_gradients(true)
        .generate();
    assert_eq!(c1, c2);
}
