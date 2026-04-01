```markdown
# Оптимизация

## Обзор

Модуль `optimizer` реализует три основных алгоритма оптимизации SVG:
- Упрощение путей (Path Simplification)
- Удаление дубликатов (Deduplication)
- Минификация (Minification)

---

## Упрощение путей (Simplify Paths)

### Алгоритм Douglas-Peucker

```rust
use lyon::geom::Line;
use lyon::math::Point;

pub fn simplify_path(points: &[Point], tolerance: f64) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec();
    }
    
    // Найти точку с максимальным расстоянием до линии
    let line = Line::new(points[0], points[points.len() - 1]);
    let (max_dist, max_index) = points.iter()
        .enumerate()
        .skip(1)
        .take(points.len() - 2)
        .map(|(i, p)| (line.distance_to_point(p).abs(), i))
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap_or((0.0, 0));
    
    // Рекурсивное упрощение
    if max_dist > tolerance {
        let mut left = simplify_path(&points[..=max_index], tolerance);
        let right = simplify_path(&points[max_index..], tolerance);
        left.pop(); // Удалить дубликат точки
        left.extend(right);
        left
    } else {
        vec![points[0], points[points.len() - 1]]
    }
}
```

### Конвертация кривых в линии

```rust
use lyon::path::{Path, PathEvent, Builder};
use lyon::algorithms::walk::PathWalker;

pub fn flatten_curves(path: &Path, tolerance: f64) -> Path {
    let mut builder = Builder::new();
    let mut events = Vec::new();
    
    path.for_each(|event| {
        match event {
            PathEvent::Quadratic { from, ctrl, to } => {
                // Конвертировать квадратичную кривую в линии
                let steps = (distance(&from, &to) / tolerance).ceil() as usize;
                for t in 0..=steps {
                    let t = t as f64 / steps as f64;
                    let point = quadratic_bezier(from, ctrl, to, t);
                    builder.line_to(point);
                }
            }
            PathEvent::Cubic { from, ctrl1, ctrl2, to } => {
                // Конвертировать кубическую кривую в линии
                let steps = (distance(&from, &to) / tolerance).ceil() as usize;
                for t in 0..=steps {
                    let t = t as f64 / steps as f64;
                    let point = cubic_bezier(from, ctrl1, ctrl2, to, t);
                    builder.line_to(point);
                }
            }
            _ => {
                builder.add_event(event);
            }
        }
    });
    
    builder.build()
}

fn quadratic_bezier(p0: Point, p1: Point, p2: Point, t: f64) -> Point {
    let t2 = t * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    
    Point::new(
        mt2 * p0.x + 2.0 * mt * t * p1.x + t2 * p2.x,
        mt2 * p0.y + 2.0 * mt * t * p1.y + t2 * p2.y,
    )
}
```

### Удаление избыточных точек

```rust
pub fn remove_redundant_points(points: &[Point], tolerance: f64) -> Vec<Point> {
    if points.len() < 3 {
        return points.to_vec();
    }
    
    let mut result = Vec::new();
    result.push(points[0]);
    
    for i in 1..points.len() - 1 {
        let p0 = result.last().unwrap();
        let p1 = &points[i];
        let p2 = &points[i + 1];
        
        // Проверка, лежит ли точка на прямой
        let area = (p1.x - p0.x) * (p2.y - p0.y) - (p1.y - p0.y) * (p2.x - p0.x);
        if area.abs() > tolerance {
            result.push(*p1);
        }
    }
    
    result.push(*points.last().unwrap());
    result
}
```

---

## Удаление дубликатов (Deduplication)

### Хеширование поддеревьев

```rust
use blake3::Hash;
use std::collections::HashMap;

pub struct TreeHasher;

impl TreeHasher {
    pub fn hash_element(element: &SVGElement) -> Hash {
        let mut hasher = blake3::Hasher::new();
        
        // Хешировать tag
        hasher.update(element.tag.as_bytes());
        
        // Хешировать атрибуты (сортированные)
        let mut attrs: Vec<_> = element.attributes.iter().collect();
        attrs.sort_by_key(|(k, _)| *k);
        for (key, value) in attrs {
            hasher.update(key.as_bytes());
            hasher.update(value.as_bytes());
        }
        
        // Хешировать детей (рекурсивно)
        for child in &element.children {
            let child_hash = Self::hash_element(child);
            hasher.update(child_hash.as_bytes());
        }
        
        hasher.finalize()
    }
}
```

### Поиск одинаковых элементов

```rust
pub struct DuplicateFinder {
    hash_map: HashMap<Hash, Vec<ElementRef>>,
}

impl DuplicateFinder {
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }
    
    pub fn find_duplicates(&mut self, element: &SVGElement) -> Vec<Vec<ElementRef>> {
        self.collect_hashes(element);
        
        let mut duplicates = Vec::new();
        for (_, elements) in &self.hash_map {
            if elements.len() > 1 {
                duplicates.push(elements.clone());
            }
        }
        
        duplicates
    }
    
    fn collect_hashes(&mut self, element: &SVGElement) {
        let hash = TreeHasher::hash_element(element);
        self.hash_map
            .entry(hash)
            .or_insert_with(Vec::new)
            .push(element.into());
        
        for child in &element.children {
            self.collect_hashes(child);
        }
    }
}
```

### Merge стилей

```rust
pub fn merge_styles(original: &mut SVGElement, duplicate: &SVGElement) {
    // Объединение классов
    let mut classes = original.class.clone();
    classes.extend(duplicate.class.clone());
    classes.sort();
    classes.dedup();
    original.class = classes;
    
    // Объединение атрибутов (приоритет оригиналу)
    for (key, value) in &duplicate.attributes {
        if !original.attributes.contains_key(key) {
            original.attributes.insert(key.clone(), value.clone());
        }
    }
    
    // Замена на <use> элемент
    if duplicate.children.is_empty() {
        original.tag = "use".to_string();
        original.attributes.insert(
            "xlink:href".to_string(),
            format!("#{}", duplicate.id.as_ref().unwrap()),
        );
        original.children.clear();
    }
}
```

---

## Минификация (Minification)

### Генерация коротких id

```rust
pub struct IdMinifier {
    counter: usize,
    mapping: HashMap<String, String>,
}

impl IdMinifier {
    pub fn new() -> Self {
        Self {
            counter: 0,
            mapping: HashMap::new(),
        }
    }
    
    pub fn minify_id(&mut self, id: &str) -> String {
        if let Some(minified) = self.mapping.get(id) {
            return minified.clone();
        }
        
        let minified = self.generate_id();
        self.mapping.insert(id.to_string(), minified.clone());
        minified
    }
    
    fn generate_id(&mut self) -> String {
        let mut id = String::new();
        let mut n = self.counter;
        
        // Буквенно-цифровая кодировка (a-z, A-Z, 0-9)
        loop {
            let remainder = n % 62;
            let c = if remainder < 26 {
                (b'a' + remainder as u8) as char
            } else if remainder < 52 {
                (b'A' + (remainder - 26) as u8) as char
            } else {
                (b'0' + (remainder - 52) as u8) as char
            };
            id.insert(0, c);
            
            n /= 62;
            if n == 0 {
                break;
            }
        }
        
        self.counter += 1;
        id
    }
    
    pub fn apply(&mut self, element: &mut SVGElement) {
        if let Some(id) = &element.id {
            let minified = self.minify_id(id);
            element.id = Some(minified);
        }
        
        // Обновить ссылки в атрибутах
        for (key, value) in &mut element.attributes {
            if key == "fill" || key == "stroke" || key == "filter" {
                if let Some(captures) = regex::Regex::new(r"url\(#(.+?)\)").unwrap().captures(value) {
                    let old_id = captures.get(1).unwrap().as_str();
                    if let Some(new_id) = self.mapping.get(old_id) {
                        *value = value.replace(old_id, new_id);
                    }
                }
            }
        }
        
        for child in &mut element.children {
            self.apply(child);
        }
    }
}
```

### Удаление дефолтных атрибутов

```rust
pub fn strip_default_attributes(element: &mut SVGElement) {
    // Дефолтные значения SVG атрибутов
    let defaults = [
        ("fill", "black"),
        ("stroke", "none"),
        ("stroke-width", "1"),
        ("fill-rule", "nonzero"),
        ("stroke-linecap", "butt"),
        ("stroke-linejoin", "miter"),
    ];
    
    for (attr, default) in defaults {
        if let Some(value) = element.attributes.get(attr) {
            if value == default {
                element.attributes.remove(attr);
            }
        }
    }
    
    for child in &mut element.children {
        strip_default_attributes(child);
    }
}
```

### Удаление комментариев

```rust
pub fn strip_comments(element: &mut SVGElement) {
    // Комментарии в XML не представлены в DOM модели
    // Но могут быть в текстовых узлах
    if let Some(text) = &mut element.text_content {
        let re = regex::Regex::new(r"<!--.*?-->").unwrap();
        *text = re.replace_all(text, "").to_string();
        
        if text.is_empty() {
            element.text_content = None;
        }
    }
    
    for child in &mut element.children {
        strip_comments(child);
    }
}
```

---

## Конфигурация

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Включить упрощение путей
    pub simplify_paths: bool,
    
    /// Включить удаление дубликатов
    pub deduplicate: bool,
    
    /// Включить минификацию id
    pub minify_ids: bool,
    
    /// Включить удаление комментариев
    pub remove_comments: bool,
    
    /// Точность координат (количество знаков)
    pub precision: usize,
    
    /// Порог упрощения (0-1)
    pub simplification_tolerance: f64,
    
    /// Порог дубликатов (минимальный размер)
    pub duplicate_threshold: usize,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            simplify_paths: true,
            deduplicate: true,
            minify_ids: true,
            remove_comments: true,
            precision: 3,
            simplification_tolerance: 0.5,
            duplicate_threshold: 2,
        }
    }
}
```

---

## Пайплайн оптимизации

```rust
pub fn optimize(
    mut element: SVGElement,
    config: &OptimizationConfig,
) -> Result<SVGElement> {
    // 1. Удаление комментариев
    if config.remove_comments {
        strip_comments(&mut element);
    }
    
    // 2. Упрощение путей
    if config.simplify_paths {
        simplify_all_paths(&mut element, config.simplification_tolerance)?;
    }
    
    // 3. Удаление дубликатов
    if config.deduplicate {
        deduplicate_elements(&mut element, config.duplicate_threshold);
    }
    
    // 4. Минификация
    if config.minify_ids {
        let mut minifier = IdMinifier::new();
        minifier.apply(&mut element);
    }
    
    // 5. Удаление дефолтных атрибутов
    strip_default_attributes(&mut element);
    
    // 6. Округление координат
    round_coordinates(&mut element, config.precision);
    
    Ok(element)
}
```

---

## Трейдоффы

| Оптимизация | Преимущества | Недостатки |
|-------------|--------------|------------|
| **Simplify Paths** | Меньший размер, быстрее рендеринг | Потеря точности, визуальные артефакты |
| **Deduplication** | Значительное уменьшение размера | Увеличение времени оптимизации |
| **Minification** | Компактные id, меньше атрибутов | Потеря читаемости |
| **Round Coordinates** | Меньший размер | Накопление ошибок при масштабировании |

### Рекомендации

```rust
// Для production (максимальная оптимизация)
let config = OptimizationConfig {
    simplify_paths: true,
    deduplicate: true,
    minify_ids: true,
    precision: 2,
    simplification_tolerance: 0.8,
    ..Default::default()
};

// Для отладки (сохранить читаемость)
let config = OptimizationConfig {
    simplify_paths: false,
    deduplicate: false,
    minify_ids: false,
    precision: 5,
    ..Default::default()
};

// Для иконок (баланс)
let config = OptimizationConfig {
    simplify_paths: true,
    deduplicate: true,
    minify_ids: true,
    precision: 3,
    simplification_tolerance: 0.3,
    duplicate_threshold: 3,
    ..Default::default()
};
```

### Производительность

| Размер SVG | Время оптимизации | Уменьшение размера |
|------------|-------------------|-------------------|
| 10 KB | 5-10 ms | 20-30% |
| 100 KB | 20-50 ms | 30-40% |
| 1 MB | 100-200 ms | 40-50% |
| 10 MB | 500-1000 ms | 50-60% |
```