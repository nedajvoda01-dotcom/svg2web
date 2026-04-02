# Руководство по контрибьютингу

## Fork и клонирование

1. Создайте fork репозитория на GitHub.
2. Клонируйте именно свой fork, не оригинал:

```bash
git clone https://github.com/<your-user>/svg2web.git
cd svg2web
```

3. Добавьте upstream:

```bash
git remote add upstream https://github.com/nedajvoda01-dotcom/svg2web.git
git remote -v
```

## Именование веток

- Новые фичи: feature/description
- Багфиксы: fix/issue-123
- Документация: docs/typo-fix

Regex для CI / pre-push hook:

```text
^(feature|fix|docs)\/.+
```

Репозиторий использует helper script: `scripts/ci/check-branch-name.sh`.

Примеры:

```bash
git checkout -b feature/react-props-cleanup
git checkout -b fix/issue-123
git checkout -b docs/typo-fix
```

## Commit messages

Используется Conventional Commits.

Разрешенные типы:

- feat
- fix
- docs
- test
- refactor
- chore

Regex для CI / git hook:

```text
^(feat|fix|docs|test|refactor|chore)(\(.+\))?: .+
```

Репозиторий использует helper script: `scripts/ci/check-commit-message.sh`.

Формат:

```text
type(scope): subject
```

Требования:

- subject <= 50 символов
- body описывает что и почему
- не описывайте как (это видно из diff)

Пример:

```text
feat(generator): add vue props defaults

Adds default prop values to generated Vue components
for consistent behavior between formats.
```

## PR процесс

PR должен использовать стандартный шаблон с разделами:

- Description
- Linked issue
- Checklist

Обязательный checklist:

- тесты проходят
- документация обновлена
- clippy чистый

Связь с задачей:

```text
Closes #123
```

## Code Review

Условия принятия:

- минимум 1 approve от мейнтейнера
- конфликты с main решены (через rebase)
- CI полностью зеленый

Рекомендуемый поток:

```bash
git fetch upstream
git rebase upstream/main
git push --force-with-lease
```

## Стратегия merge

- Squash and merge: для feature PR (чистая история)
- Rebase and merge: для hotfix PR (сохранение коммитов)
