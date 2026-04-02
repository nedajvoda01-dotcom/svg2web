# 🚀 ПЕРВЫЙ ЗАПУСК — Чеклист

Используй этот файл перед запуском первого сеанса разработки. Убедись все ✅ в наличии.

---

## ✅ ПРЕ-ЗАПУСК (Инфраструктура)

- [x] `.cursorrules` создан и содержит полные правила
- [x] `.ai/context.md` содержит жёсткие запреты и архитектуру
- [x] `.ai/session-checklist.md` описывает процесс
- [x] `.ai/README.md` связывает все инструменты
- [x] `STATUS.md` содержит 66 чекбоксов реализации (все `[ ]`)
- [x] `scripts/verify-examples.sh` создан и исполняемый
- [x] `Cargo.toml` workspace корнем содержит `[workspace.lints.clippy] unwrap_used="deny"`
- [x] `examples/basic/expected/` содержит golden files (HTML/CSS/JS)
- [x] Тесты (117 doc-assertion тестов) готовы к запуску

---

## ✅ ПРЕ-ЗАПУСК (Документация)

- [x] `docs/api-reference/rust/core.md` содержит точные сигнатуры (`ParseOutput`, `parse()`, etc)
- [x] `docs/api-reference/rust/cache.md` описывает кэш API
- [x] `docs/internals/architecture.md` описывает границы крейтов
- [x] `docs/internals/` полностью заполнена (12 файлов)
- [x] `docs/usage/` готова для пользователей (не нужна для реализации)

---

## 🎯 ПЕРВЫЙ СЕАНС (Копируй этот промпт дай ИИ)

**Цель:** Foundation layer моделей, итерация 1/10

**Задача:**
```
Реализуй foundation layer моделей в svg2web-core.

ФАЙЛЫ (строго 2 файла):
1. crates/svg2web-core/src/model/element.rs
   Структуры:
   - SVGElement (pub id, tag, attributes, children, text_content)
   - ElementType enum (Group, Path, Rect, Circle, Text, Image, Gradient, etc)
   - Node как обёртка

2. crates/svg2web-core/src/model/geometry.rs
   Структуры:
   - Point { x: f64, y: f64 } + Copy
   - Size { width: f64, height: f64 } + Copy
   - Bounds { x: f64, y: f64, width: f64, height: f64 } + Copy
   - Transform { a,b,c,d,e,f: f64 } + Copy (матрица 3x2)
   - Rect { x, y, width, height: f64 } + Copy
   - PathSegment enum (MoveTo, LineTo, CurveTo, ClosePath)
   - PathData(Vec<PathSegment>)

ТРЕБОВАНИЯ:
- Все структуры: pub поля, #[derive(Debug, Clone, Serialize, Deserialize)]
- Geometry типы добавить #[derive(Copy)] (кроме PathData)
- Никакой impl логики, только структуры и derive
- В model/mod.rs добавить pub use element::*; pub use geometry::*;
- NO unwrap() вне тестов (clippy deny)

ПРОВЕРКА:
1. cargo check -p svg2web-core
   → Должно быть 0 errors (warnings OK, неиспользуемые импорты OK)
2. Обнови STATUS.md: отметь [x] первые 4 чекбокса в разделе Core/model
3. Скопируй output cargo check и покажи мне

НЕ делай сейчас:
- Не трогай другие файлы/крейты
- Не пиши тесты
- Не добавляй impl методы
- Не усложняй структуры
```

---

## 🔍ЧТО ОЖИДАТЬ

**После запуска ИИ:**

### Вариант A: Компилируется (0 errors)
```
✅ SUCCESS
cargo check -p svg2web-core → Finished dev [unoptimized] target(s)
ИИ обновляет STATUS.md → готов к следующей итерации
```

### Вариант B: Ошибки компиляции (ожидаемо!)
```
❌ EXPECTED ERROR (например):
error[E0603]: private module `element`
  --> src/model/mod.rs:1:5
   |
1 | pub use element::*;
   |         ^^^^^^^ private module
```

**Что делать:**
1. Показать ошибку ИИ
2. Попросить исправить (обычно опечатка в пути или модули не созданы)
3. Переделать и заново cargo check

Это **нормально** и учит ИИ исправлять ошибки.

---

## 📊ПО ОКОНЧАНИИ ПЕРВОГО СЕАНСА

После того как `cargo check` пройдёт с 0 errors:

1. **STATUS.md обновлён**
   ```markdown
   ### model/
   - [x] `model/element.rs` — SVGElement, ElementType, Node
   - [x] `model/geometry.rs` — Bounds, Point, Transform, PathData
   ```

2. **Считай это успехом** даже если не все тесты зелёные
   - Тесты требуют полного pipeline (parser/serializer/etc)
   - Для foundation layer достаточно что types компилируются

3. **Следующий запрос:**
   ```
   Реализуй style.rs и asset.rs (see STATUS.md чекбоксы 3-4)
   ```

---

## 🚨ЕСЛИ ЧТО-ТО ПОШЛО НЕ ТАК

| Проблема | Решение |
|----------|---------|
| `cargo check` не работает вообще | Проверь: ли `Cargo.toml` валидный, используй `cargo check` без флагов |
| ИИ добавил logic/методы | Скажи: "Сейчас только структуры, логика будет позже" |
| ИИ начал писать parser | Скажи: "Слишком рано, foundation сначала" |
| ИИ забыл обновить STATUS.md | Скажи: "Не забудь STATUS.md, это твой отчет" |
| Компилируется но verify-examples.sh падает | Нормально! Smoke mode работает, полная проверка будет позже |

---

## ✅ КРИТЕРИЙ ГОТОВНОСТИ К СЛЕДУЮЩЕЙ ИТЕРАЦИИ

**ВСЕ ЭТИ УСЛОВИЯ должны быть TRUE:**

```
✅ cargo check -p svg2web-core → 0 errors
✅ STATUS.md обновлён (отмечены [x])
✅ Все pub поля в структурах
✅ Copy где нужно (geometry types)
✅ Serialize/Deserialize через serde derive
✅ Никакой logic (impl блоки пусты или отсутствуют)
✅ No unwrap в production коде
✅ model/mod.rs реэкспортирует оба модуля
```

**Если всё TRUE → переходи к следующей паре модулей (style.rs + asset.rs)**

---

## 🎯ИТЕРАЦИЯ 2 (когда запланируешь)

```
Реализуй model/style.rs и model/asset.rs

ФАЙЛЫ:
1. model/style.rs
   - Style структура (color, fill, stroke, opacity, etc)
   - Color enum (RGB, Named, Gradient reference)
   - Font структура (family, size, weight, style)
   - Gradient enum (Linear, Radial)

2. model/asset.rs
   - Asset enum (Image, Font, ExternalResource)
   - ImageAsset { id, data, format }
   - FontAsset { family, weights }

ТРЕБОВАНИЯ:
- Все структуры pub
- Сериализуемые через serde
- NO logic, только структуры
- model/mod.rs добавить pub use style::*; asset::*;

ПРОВЕРКА:
- cargo check -p svg2web-core → 0 errors
- Обнови STATUS.md
```

---

**ГОТОВ К БОЕВОМУ ДЕЖУРСТВУ.** ☀️

Когда дашь этот чеклист ИИ-ассистенту вместе с основным промптом — первая итерация займет ~15-20 минут и даст тебе foundation layer, готовый к следующим слоям.

Успеха!
