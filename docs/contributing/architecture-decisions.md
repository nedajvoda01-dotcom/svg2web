# Architecture Decision Records (ADR)

## ADR-1: usvg vs roxmltree

Решение:

- основной парсер: usvg
- roxmltree не используется как прямой production parser API

Почему usvg:

- полная поддержка SVG spec
- нормализация структуры

Почему не roxmltree напрямую:

- слишком много edge cases нужно реализовывать вручную
- CI проверяет, что код опирается на usvg, а не на прямой roxmltree parser flow

## ADR-2: 6 JSON файлов vs 1

Решение: split-output по секциям.

Контракт секций:

- meta.json
- structure.json
- geometry.json
- styles.json
- assets.json
- content.json

Причины:

- разделение ответственности
- частичное чтение (generator не всегда нужна вся геометрия)
- кэширование секций и частичная инвалидация

## ADR-3: Tera vs Handlebars/Askama

Решение: Tera.

Контракт:

- используется Tera
- Handlebars не используется в generator runtime

Причины:

- runtime-гибкость (шаблоны без перекомпиляции)
- наследование шаблонов через blocks
- производительность приемлемая при кэшировании compiled templates

## ADR-4: 5 крейтов vs монолит

Решение: модульная архитектура из core/cache/generator/cli/wasm.

Причины:

- разделение ответственности
- меньший WASM размер (без CLI-кода)
- независимое тестирование и versioning

## ADR-5: Sled vs SQLite/Redis

Решение: Sled для дискового кэша.

Причины:

- embedded и zero-config
- высокая производительность на small keys
- не нужен отдельный процесс, как у Redis

## ADR-6: Web Workers vs Main Thread

Решение: тяжелая обработка в Web Workers.

Причины:

- неблокирующий UI
- задел на параллелизм (несколько воркеров в будущем)
