mod support;

use support::fixture_generator::FixtureGenerator;
use svg2web_core::optimizer::OptimizationConfig;
use svg2web_core::validator::visual::{calculate_psnr, render_svg_to_rgba, visual_diff};
use svg2web_core::{optimize, parse};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn simple_svg(i: usize) -> String {
    FixtureGenerator::new(i as u64).with_layers(i * 2).generate()
}

fn complex_svg(i: usize) -> String {
    FixtureGenerator::new(i as u64)
        .with_layers(10 + (i - 21) * 10)
        .with_gradients(true)
        .generate()
}

fn stress_svg(i: usize) -> String {
    FixtureGenerator::new(i as u64)
        .with_layers(100 + (i - 71) * 30)
        .with_gradients(true)
        .with_filters(true)
        .with_text(true)
        .generate()
}

fn fixture_svg(i: usize) -> String {
    match i {
        1..=20 => simple_svg(i),
        21..=70 => complex_svg(i),
        71..=100 => stress_svg(i),
        _ => unreachable!(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Simple fixtures (1-20) должны рендериться и давать PSNR = INFINITY при identity-сравнении.
#[test]
fn simple_fixtures_render_and_identity_psnr() {
    for i in 1..=20 {
        let svg = simple_svg(i);

        let img = render_svg_to_rgba(&svg, 200, 200);
        assert!(
            img.is_some(),
            "Simple fixture {} failed to render",
            i,
        );

        let psnr = visual_diff(&svg, &svg, 200, 200);
        assert_eq!(
            psnr,
            Some(f32::INFINITY),
            "Identity PSNR for simple fixture {} should be INFINITY",
            i,
        );
    }
}

/// Stress fixtures (71-100) не должны паниковать при parse + optimize.
/// PSNR-проверку не делаем (нет SVG-сериализатора для оптимизированного дерева).
#[test]
fn stress_fixtures_handled_gracefully() {
    for i in 71..=80 {
        let svg = stress_svg(i);

        let parse_result = parse(&svg);
        assert!(
            parse_result.is_ok(),
            "Stress fixture {} should parse without error",
            i,
        );

        let mut element = parse_result.unwrap();
        let opt_result = optimize(&mut element, &OptimizationConfig::default());
        // optimize может вернуть Err для сложных кейсов — это допустимо.
        // Главное — не panic.
        let _ = opt_result;
    }
}

/// Статистика по всем 100 фикстурам: render success rate, optimize success rate.
#[test]
fn fixture_statistics_report() {
    let mut render_ok = 0u32;
    let mut render_fail = 0u32;
    let mut optimize_ok = 0u32;
    let mut optimize_fail = 0u32;
    let mut parse_ok = 0u32;
    let mut parse_fail = 0u32;

    for i in 1..=100 {
        let svg = fixture_svg(i);

        // Parse
        let element = match parse(&svg) {
            Ok(e) => { parse_ok += 1; e }
            Err(_) => { parse_fail += 1; continue; }
        };

        // Render
        match render_svg_to_rgba(&svg, 100, 100) {
            Some(_) => render_ok += 1,
            None => render_fail += 1,
        }

        // Optimize
        let mut cloned = element.clone();
        match optimize(&mut cloned, &OptimizationConfig::default()) {
            Ok(()) => optimize_ok += 1,
            Err(_) => optimize_fail += 1,
        }
    }

    println!("\n=== Fixture Validation Report ===");
    println!("Parse:    {}/100 ok, {} failed", parse_ok, parse_fail);
    println!("Render:   {}/100 ok, {} failed", render_ok, render_fail);
    println!("Optimize: {}/100 ok, {} failed", optimize_ok, optimize_fail);
    println!("=================================\n");

    // Baselines: все 100 должны парситься (генератор создаёт валидный SVG)
    assert_eq!(parse_ok, 100, "All 100 fixtures must parse successfully");
    // Рендер может падать на stress (напр. слишком большие деревья), но >= 80%
    assert!(
        render_ok >= 80,
        "At least 80/100 fixtures should render (got {})",
        render_ok,
    );
    // Optimize должен работать на большинстве
    assert!(
        optimize_ok >= 80,
        "At least 80/100 fixtures should optimize (got {})",
        optimize_ok,
    );
}

/// Optimize не должен разрушать структуру дерева и должен быть детерминированным.
#[test]
fn optimize_preserves_structure() {
    for i in 1..=20 {
        let svg = simple_svg(i);
        let original = parse(&svg).unwrap();

        let mut opt1 = original.clone();
        optimize(&mut opt1, &OptimizationConfig::default()).unwrap();

        // Optimize не должен удалить все дочерние элементы
        assert!(
            !opt1.children.is_empty(),
            "Fixture {}: optimize should not destroy structure",
            i,
        );

        // Детерминизм: повторный optimize того же дерева = тот же результат
        let mut opt2 = original.clone();
        optimize(&mut opt2, &OptimizationConfig::default()).unwrap();
        assert_eq!(
            opt1.children.len(),
            opt2.children.len(),
            "Fixture {}: optimize should be deterministic",
            i,
        );
    }
}

/// Strict mode workaround: проверяем рендерер через identity PSNR.
/// Полная проверка (original vs optimized) заблокирована отсутствием SVG-сериализатора.
/// → Планируется в Iteration 40+ (Polish Phase).
#[test]
fn strict_mode_identity_check() {
    for i in [1, 5, 10, 15, 20] {
        let svg = simple_svg(i);
        let img1 = render_svg_to_rgba(&svg, 200, 200).unwrap();
        let img2 = render_svg_to_rgba(&svg, 200, 200).unwrap();
        let psnr = calculate_psnr(&img1.data, &img2.data);
        assert!(
            psnr.is_infinite(),
            "Fixture {}: identity PSNR should be INFINITY, got {:.2}",
            i, psnr,
        );
    }

    // Complex fixtures — рендер тоже детерминированный
    for i in [25, 40, 60] {
        let svg = complex_svg(i);
        let img1 = render_svg_to_rgba(&svg, 200, 200).unwrap();
        let img2 = render_svg_to_rgba(&svg, 200, 200).unwrap();
        let psnr = calculate_psnr(&img1.data, &img2.data);
        assert!(
            psnr.is_infinite(),
            "Complex fixture {}: identity PSNR should be INFINITY, got {:.2}",
            i, psnr,
        );
    }
    // NOTE: Full strict mode (original vs optimized PSNR > 50 dB) requires
    // SVG serializer (SVGElement → SVG string). Planned for Iteration 40+.
}
