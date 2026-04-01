```markdown
# Правила контрибуции

## Fork и клонирование

1. **Fork репозитория** на GitHub
2. **Клонируйте свой fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/svg2web.git
   cd svg2web
   ```
3. **Добавьте upstream:**
   ```bash
   git remote add upstream https://github.com/org/svg2web.git
   ```
4. **Проверьте настройки:**
   ```bash
   git remote -v
   # origin    https://github.com/YOUR_USERNAME/svg2web.git (fetch)
   # origin    https://github.com/YOUR_USERNAME/svg2web.git (push)
   # upstream  https://github.com/org/svg2web.git (fetch)
   # upstream  https://github.com/org/svg2web.git (push)
   ```

---

## Ветки

### Создание ветки

Все изменения делаются в отдельной ветке:

```bash
# Для новой функциональности
git checkout -b feature/my-feature

# Для исправления бага
git checkout -b fix/issue-123

# Для документации
git checkout -b docs/update-readme
```

### Именование веток

| Тип | Формат | Пример |
|-----|--------|--------|
| Новая функциональность | `feature/краткое-описание` | `feature/react-18-support` |
| Исправление бага | `fix/issue-номер` | `fix/issue-42` |
| Документация | `docs/краткое-описание` | `docs/api-reference` |
| Рефакторинг | `refactor/краткое-описание` | `refactor/parser-module` |
| Тесты | `test/краткое-описание` | `test/coverage-improvement` |

---

## Commit messages

### Формат

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Типы (type)

| Тип | Описание |
|-----|----------|
| `feat` | Новая функциональность |
| `fix` | Исправление бага |
| `docs` | Изменения в документации |
| `test` | Добавление или исправление тестов |
| `refactor` | Рефакторинг кода |
| `perf` | Оптимизация производительности |
| `chore` | Обслуживание (зависимости, конфиги) |
| `ci` | Изменения в CI/CD |
| `style` | Форматирование, линтеры |

### Примеры

```bash
# Функциональность
git commit -m "feat(core): add support for SVG masks

- Implement mask parsing in parser module
- Add mask extraction to extractor
- Update model with Mask struct
- Add tests for masks"

# Исправление
git commit -m "fix(generator): fix React component export

Fixes #123

Export was missing default export for functional components"

# Документация
git commit -m "docs(wasm): add Web Worker usage examples

- Add worker.ts example
- Document message passing patterns
- Add error handling section"

# Рефакторинг
git commit -m "refactor(cache): extract cache key generation to separate module

Move key generation logic from manager.rs to key.rs for better separation of concerns"
```

### Правила

- Subject не более 50 символов
- Subject с маленькой буквы
- Subject без точки в конце
- Body с большой буквы, разбит на строки по 72 символа
- Body объясняет **что** и **почему**, а не **как**
- Ссылки на issues в footer: `Fixes #123`, `Closes #456`

---

## Процесс PR

### Подготовка PR

1. **Обновите ветку:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Проверьте CI локально:**
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test --workspace
   cargo deny check
   ```

3. **Запушьте изменения:**
   ```bash
   git push origin feature/my-feature
   ```

### Шаблон PR

```markdown
## Описание
Краткое описание изменений. Что делает этот PR и зачем.

## Связанные issues
Closes #123
Fixes #456

## Изменения
- [ ] Новая функциональность
- [ ] Исправление бага
- [ ] Документация
- [ ] Рефакторинг
- [ ] Тесты

## Checklist
- [ ] Код отформатирован (`cargo fmt`)
- [ ] Clippy не выдает ошибок (`cargo clippy -- -D warnings`)
- [ ] Все тесты проходят (`cargo test --workspace`)
- [ ] Добавлены тесты для новой функциональности
- [ ] Обновлена документация (rustdoc, README)
- [ ] Изменения совместимы с WASM
- [ ] Нет `unwrap()`/`expect()` в production коде
- [ ] Добавлены примеры использования

## Тестирование
Опишите как тестировали изменения:
- Ручное тестирование: ...
- Автоматические тесты: ...

## Скриншоты (если применимо)
До и после для UI изменений.

## Дополнительно
Любая дополнительная информация.
```

---

## Code Review

### Ожидания от автора PR

- Отвечать на комментарии в течение 2-3 дней
- Запрашивать ревью у мейнтейнеров
- Не мержить PR без аппрува
- Разрешать конфликты с main до мержа

### Ожидания от ревьюера

- Проверить код на соответствие стилю
- Убедиться в наличии тестов
- Проверить документацию
- Указать на потенциальные проблемы производительности
- Быть конструктивным и вежливым

### Критерии принятия PR

- [ ] Минимум 1 аппрув от мейнтейнера
- [ ] Все проверки CI зеленые
- [ ] Нет конфликтов с `main`
- [ ] Документация обновлена
- [ ] Добавлены тесты для новых фич

---

## Merge стратегии

### Feature ветки (squash merge)

Для большинства PR используется **squash merge**:
- Все коммиты объединяются в один
- Сообщение коммита формируется из заголовка PR
- История main остается чистой

```bash
# Результат
feat(core): add SVG mask support (#125)
```

### Hotfix ветки (rebase merge)

Для критических исправлений используется **rebase merge**:
- Коммиты сохраняются индивидуально
- Используется только для срочных фиксов

```bash
# Результат
fix(cli): fix panic when input file not found
fix(cli): add better error message for missing file
```

---

## CI проверки

PR автоматически проверяет:

| Проверка | Команда |
|----------|---------|
| Форматирование | `cargo fmt --check` |
| Линтинг | `cargo clippy -- -D warnings` |
| Тесты | `cargo test --workspace` |
| Документация | `cargo doc --no-deps` |
| Зависимости | `cargo deny check` |
| WASM сборка | `wasm-pack build --target web` |
| Web тесты | `npm test` |

Все проверки должны быть зелеными.

---

## Первый контрибьют

1. Найдите issue с меткой `good-first-issue`
2. Напишите комментарий, что хотите взять задачу
3. Дождитесь назначения
4. Следуйте процессу выше

---

## Вопросы и помощь

- **Discord**: `#contributing` канал
- **GitHub Discussions**: для архитектурных вопросов
- **Issues**: для багов и фич

---

## Code of Conduct

- Будьте вежливы и конструктивны
- Уважайте разные точки зрения
- Критикуйте код, а не автора
- Помогайте новым контрибьюторам
```