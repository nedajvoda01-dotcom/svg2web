<!-- [Doc] | Кэширование в svg2web: derivation key, disk/memory backend, invalidation и режимы по окружениям. -->

# Cache

## Key Derivation

Алгоритм ключа:

```text
SHA256(concat(sha256(svg_content), core_version, sha256(config_json)))
```

Свойства:

- Детерминированность: одинаковый вход дает одинаковый key.
- Версионирование: изменение версии core/crate автоматически инвалидирует прошлый кэш.

## Sled (Disk)

- Embedded KV store без отдельного сервера.
- ACID transactions.
- Сжатие zstd.
- Путь хранения: ~/.cache/svg2web/
- TTL для протухания записей.

## LRU (Memory)

- Реализация на crate lru.
- Configurable size limit (default 1000 entries).
- Thread-safe доступ через Arc<Mutex<...>>.

## Инвалидация

- Явная: команда svg2web cache clean.
- Автоматическая: при смене версии crate.
- TTL expiry: удаление устаревших записей.

## Выбор бэкенда

- CLI: Disk + Memory (двухуровневый кэш).
- WASM: Memory only (в текущей реализации без IndexedDB).
- Server: Disk с TTL для многопользовательского режима.
