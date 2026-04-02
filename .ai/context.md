# AI Context — svg2web

**GPS для ИИ-ассистентов**: этот файл сообщает текущее состояние проекта, запреты и приоритеты.
Читай его перед любыми изменениями кода. Актуальный статус реализации — в [STATUS.md](../STATUS.md).

---

## Текущее состояние реализации

> **Важно**: все `.rs` файлы в `crates/` — **однострочные стабы** (blueprint-комментарии, не код).
> Реальная Rust-реализация ещё не написана. Документация, тесты и архитектура — готовы.
> Web (`web/src/`) — также стабы. Plugins — стабы.

### Что сделано (не требует реализации)
- [x] Архитектура и API-контракты (`docs/`)
- [x] Полный набор contract-тестов (117 тестов, doc-assertion)
- [x] JSON Schema спецификация (6 файлов: meta/structure/geometry/styles/assets/content)
- [x] TypeScript типы и интерфейсы (`web/src/types/index.ts`)
- [x] CI-скрипты (`scripts/ci/`)
- [x] Миграционный скрипт v0.1→v0.2 (`scripts/migrations/`)
- [x] Примеры с expected output (`examples/basic/expected/`)

### Порядок реализации (следуй этому порядку)
1. `svg2web-core` model (структуры данных)
2. `svg2web-core` parser (usvg wrapper)
3. `svg2web-core` serializer (→ JSON)
4. `svg2web-generator` readers + builders (vanilla)
5. `svg2web-generator` formats/react
6. `svg2web-cli` (parse/generate команды)
7. `svg2web-wasm` (WASM экспорты)
8. `web/src/` (React UI)
9. Vue, Solid, Svelte плагины (nice to have)

---

## Жёсткие запреты (HARD RULES)

### Запрещено в `svg2web-core` и `svg2web-generator`
```
❌ use std::fs;          // только в svg2web-cli
❌ use tokio::fs;        // только в svg2web-cli
❌ unwrap()              // везде кроме тестов (workspace deny)
❌ expect("...")         // везде кроме тестов (workspace deny)
❌ extern crate sled;    // только в svg2web-cache
❌ extern crate tokio;   // только в svg2web-cache, svg2web-cli (не в core, не в wasm)
```

### Запрещено в `svg2web-wasm`
```
❌ use std::fs;
❌ sled (disk cache)     // только MemoryCache (feature = "memory")
❌ tokio runtime         // wasm32-unknown-unknown не поддерживает tokio threads
❌ std::thread           // WASM single-threaded
```

### Запрещено в Generator
```
❌ use svg2web_cli::*;   // generator не знает о CLI
❌ use svg2web_wasm::*;  // generator не знает о WASM
```

### Запрещено везде
```
❌ std::process::exit()  // возвращай Result, не выходи
❌ println!/eprintln!    // используй tracing::info!/error!
❌ pub fn без rustdoc    // все публичные функции должны иметь doc-комментарий с примером
```

### В тестах разрешено
```rust
// В #[cfg(test)] блоках и tests/*.rs:
✅ .unwrap()     // паника в тестах — приемлемо
✅ .expect("...") // OK в тестах
✅ std::fs::read_to_string(...)  // OK в тестах
```

---

## Приоритеты принятия решений

### 1. Корректность > Производительность
Пример: парсер должен вернуть правильную ошибку (`SvgParse` с line number),
даже если это медленнее чем fast-fail.

### 2. WASM-совместимость имеет приоритет над CLI-удобством
Если фича требует `tokio::spawn` или `sled` — она **не может** быть в `core`.
Выноси в `cli`-only или `cache` crate.

### 3. Размер WASM бандла < 500KB gzipped
Перед добавлением новой зависимости в `svg2web-core` или `svg2web-wasm`:
- проверь `wasm-pack build && ls -lh pkg/*.wasm`
- тяжёлые крейты (image, font-kit) — только с feature gate и не в core

### 4. React и Vanilla — критичные форматы
Vue, Svelte, Solid — nice to have. Не блокируй React/Vanilla ради Vue.

### 5. Обратная совместимость API
Поля `ParseOutput` (meta/structure/geometry/styles/assets/content) — **заморожены**.
Не переименовывай, не удаляй. Смотри `docs/api-reference/json-schema/`.

---

## Как проверить, что ничего не сломал

```bash
# Полная проверка (обязательна перед коммитом)
cargo test --workspace

# Проверка WASM
wasm-pack test --headless --chrome

# Стиль кода
cargo fmt --check
cargo clippy -- -D warnings

# Проверка примеров против expected output
bash scripts/verify-examples.sh

# Проверка TS
cd web && npm test
```

---

## Структура крейтов и зависимости

```
svg2web-core        ← ни от чего не зависит (no IO, no FS)
    ↓
svg2web-cache       ← зависит от core (добавляет sled + lru)
    ↓
svg2web-generator   ← зависит от core + cache
    ↓
svg2web-cli         ← зависит от core + cache + generator (единственный с std::fs)
svg2web-wasm        ← зависит от core + cache[memory-only]
```

---

## Типичные грабли

### `cargo test` падает сразу?
- Проверь что в `.rs` нет `unwrap()` вне `#[cfg(test)]`
- `workspace.lints.clippy.unwrap_used = "deny"` применяется ко всему workspace

### Хочешь добавить зависимость?
- Новые зависимости — **только в `[workspace.dependencies]`**, не в крейты напрямую
- В крейте ссылайся: `my-crate.workspace = true`
- Исключение: dev-dependencies крейта можно класть напрямую (criterion, tempfile)

### WASM не компилируется?
- `wasm32-unknown-unknown` не имеет `std::fs`, `std::net`, `std::thread`
- Проверяй с `cargo check --target wasm32-unknown-unknown -p svg2web-wasm`

### `cargo clippy` ругается на complexity?
- Порог когнитивной сложности: **30** (`clippy.toml`)
- Разбивай функции, не повышай порог

### Тест читает файл и падает "No such file"?
- Тесты в crates запускаются с CWD = корень workspace
- Используй пути типа `"docs/..."` или `"crates/svg2web-core/tests/fixtures/..."`
- **Не** используй относительные пути без префикса крейта

---

## Что не делать при работе с тестами

Тесты в этом проекте — **doc-assertion тесты**: они читают markdown файлы и проверяют
наличие строк. Не удаляй формулировки из docs просто потому что они "неточные" —
сначала проверь, есть ли тест, который на них завязан.

Перед изменением любого файла в `docs/`:
```bash
grep -r "$(имя_файла)" tests/ crates/*/tests/
```
