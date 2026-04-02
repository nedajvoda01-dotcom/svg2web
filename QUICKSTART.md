# QUICK START для разработки

> Скопируй эту шпаргалку в избранное. Используй её каждый день.

---

## 📍 ТЫ ЗДЕСЬ? (Чеклист ориентации)

- [ ] Не знаешь куда начать → `cat .ai/README.md` (главная инструкция)
- [ ] Не знаешь какой модуль реализовывать → `cat STATUS.md` (что готово, что нет)
- [ ] Не знаешь какие запреты → `cat .ai/context.md` (жёсткие правила)
- [ ] Запускаешь сеанс разработки → `cat .ai/session-checklist.md` (процесс)
- [ ] Готов к первому запуску → `cat .ai/LAUNCH.md` (готовый промпт)

---

## 🔨 ТРИ КОМАНДЫ КОТОРЫЕ НУЖНЫ 100%

```bash
# 1. Проверка синтаксиса (запускай после каждого сеанса)
cargo check --workspace && cargo test -p svg2web-core && cargo clippy

# 2. Проверка примеров (убедись что ничего не сломал)
bash ./scripts/verify-examples.sh

# 3. Обнови инвентарь (ИИ должен это делать, но добавь в вовремя)
cat STATUS.md | grep -E "^\s*- \[x\]" | wc -l  # сколько реализовано
```

---

## 📋 ПОРЯДОК РЕАЛИЗАЦИИ (не менять!)

```
1. svg2web-core/src/model/     ← НА ЭТО НАЧНИ
   ├─ element.rs
   ├─ geometry.rs
   ├─ style.rs
   ├─ asset.rs
   └─ analysis.rs

2. svg2web-core/src/parser/    ← зависит от model
   ├─ svg.rs (usvg wrapper)
   ├─ geometry.rs (rect/circle → PathData)
   ├─ styles.rs (CSS)
   └─ text.rs (fonts)

3. svg2web-core/src/{optimizer,extractor,serializer,validator}/

4. svg2web-cache/              ← можно отложить

5. svg2web-generator/

6. svg2web-cli/

7. svg2web-wasm/

8. web/src/

9. plugins/ (Svelte, Solid)
```

---

## 🎯 КОГДА ДАЁШЬ ЗАДАЧУ ИИ

**Копируй-вставь этот шаблон:**

```
Задача: Реализуй [НАЗВАНИЕ МОДУЛЯ].

Файлы: [ПЕРЕЧИСЛИ КАК В STATUS.md]
Требования: [КОП И-ВСТАВЬ ИЗ docs/]
Проверка: cargo check -p [CRATE] && ./scripts/verify-examples.sh
Обнови STATUS.md: отметь [x] готовые чекбоксы

НЕ делай: [КОПИЯ ИЗ .cursorrules]
```

---

## 🔍ЧТО СМОТРЕТЬ В СЛУЧАЕ ПРОБЛЕМЫ

| Ошибка | Где искать решение |
|--------|------------------|
| `unwrap()` запрещен | `.ai/context.md` → Жёсткие запреты |
| `std::fs` в core | `.ai/context.md` → Запрещено в svg2web-core |
| Не знаю как запустить тест | `.ai/session-checklist.md` → Проверка |
| Какой модуль дальше? | `STATUS.md` → посмотри какие `[ ]` блокированы |
| Где пример ожидаемого output? | `examples/basic/expected/` → это golden truth |
| Как правильно структурировать код? | `docs/api-reference/rust/core.md` → сигнатуры |

---

## 📊МЕТРИКИ УСПЕХА

**После каждого сеанса проверь:**

```bash
# Компилируется ли?
cargo check --workspace

# Тесты проходят?
cargo test -p svg2web-core

# Стиль норм?
cargo clippy -- -D warnings

# Примеры работают?
./scripts/verify-examples.sh

# STATUS.md обновлён?
git diff STATUS.md
```

**Если все зелёные и STATUS обновлён → готов к merge/next session**

---

## ⚡БЫСТРЫЕ КОМАНДЫ

```bash
# Узнать сколько модулей реализовано
grep -c "^\s*- \[x\]" STATUS.md

# Узнать сколько осталось
grep -c "^\s*- \[ \]" STATUS.md

# Проверить что ИИ не использовал unwrap в production
grep -r "\.unwrap()" crates/svg2web-core/src --exclude-dir=tests

# Проверить что нет println в lib коде
grep -r "println\!" crates/svg2web-core/src

# Запустить только smoke checks (без бинарника)
./scripts/verify-examples.sh 2>&1 | grep -E "PASS|SKIP|FAIL"
```

---

## 🚀ПЕРВЫЙ ЗАПУСК

1. Прочитай `.ai/LAUNCH.md`
2. Скопируй промпт из этого файла
3. Дай его ИИ-ассистенту
4. Жди результата (~15 минут)
5. Проверь: `cargo check -p svg2web-core`
6. Обнови `STATUS.md` вместе с ИИ
7. Переходи к итерации 2

---

## 🔗ПОЛЕЗНЫЕ ССЫЛКИ

- **Главный гайд:** `.ai/README.md`
- **Правила проекта:** `.ai/context.md`
- **Процесс сеанса:** `.ai/session-checklist.md`
- **Первый запуск:** `.ai/LAUNCH.md`
- **Статус реализации:** `STATUS.md`
- **API контракты:** `docs/api-reference/`
- **Архитектура:** `docs/internals/architecture.md`
- **Примеры:** `examples/basic/expected/`

---

**Помни:** Лучше 10 маленьких проверяемых итераций, чем одна огромная неконтролируемая.

**Good luck!** ☀️
