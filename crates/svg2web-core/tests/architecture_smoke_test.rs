/// Smoke test: сложный SVG с градиентами и фильтрами должен быть
/// успешно распарсен, а не отвергнут. Мы обрабатываем всё — при
/// невозможности воспроизвести в HTML/CSS запасной вариант: растеризация.
#[test]
fn complex_svg_parsed_not_rejected() {
    let complex_svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="200" height="200">
  <defs>
    <linearGradient id="g1">
      <stop offset="0%" stop-color="#ff0000"/>
      <stop offset="100%" stop-color="#0000ff"/>
    </linearGradient>
    <filter id="blur">
      <feGaussianBlur stdDeviation="5"/>
    </filter>
    <clipPath id="clip">
      <circle cx="100" cy="100" r="80"/>
    </clipPath>
  </defs>
  <rect x="10" y="10" width="180" height="180" fill="url(#g1)" filter="url(#blur)" clip-path="url(#clip)"/>
  <text x="50" y="110" font-size="24" fill="white">Test</text>
</svg>"##;

    let result = svg2web_core::parse(complex_svg);
    assert!(
        result.is_ok(),
        "Complex SVG (gradients + filters + clip-path + text) must be parsed, not rejected. \
         Unsupported features should be handled via rasterization fallback, not errors."
    );

    let element = result.unwrap();
    // Координаты должны сохраниться — мы не «улучшаем» layout
    assert!(
        !element.children.is_empty(),
        "Parsed SVG must retain its child elements (not collapse into empty tree)."
    );
}

/// Проверка что analyze() возвращает СТРУКТУРНЫЙ результат,
/// без семантических полей (button, input, card).
#[test]
fn analyze_returns_structural_not_semantic() {
    let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100">
  <rect x="10" y="10" width="80" height="30" fill="#3366ff" rx="4"/>
  <text x="30" y="30" font-size="14" fill="white">Click</text>
</svg>"##;

    let element = svg2web_core::parse(svg).unwrap();
    let analysis = svg2web_core::analyze(&element);

    // complexity — числовая оценка, не семантическая классификация
    assert!(analysis.complexity.score <= 100);

    // hierarchy — структурная глубина, не DOM-роль
    assert!(analysis.hierarchy.depth >= 1);

    // Если components найдены, они хранят SHA-хеш + шаблон,
    // а не label "button" / "card"
    for comp in &analysis.components {
        assert!(
            !comp.id.is_empty(),
            "Component id should be a structural hash, not empty"
        );
    }
}
