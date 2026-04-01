```markdown
# svg2web-cache

Крейт для кэширования промежуточных результатов парсинга и генерации SVG. Предоставляет двухуровневое кэширование (диск + память) для CLI и серверных сценариев, а также in-memory кэш для WASM.

## Типы

### `Cache`

Главная структура кэша, объединяющая дисковое и памятное хранилища.

```rust
pub struct Cache {
    disk: Option<DiskCache>,
    memory: MemoryCache,
}
```

**Методы:**

- `get(&self, key: &CacheKey) -> Option<CacheEntry>` — получение записи из кэша. Сначала проверяет память, затем диск.
- `set(&self, key: CacheKey, entry: CacheEntry) -> Result<()>` — сохранение записи в оба уровня кэша.
- `invalidate(&self, key: &CacheKey) -> Result<()>` — удаление конкретной записи.
- `clear(&self) -> Result<()>` — полная очистка кэша (диск и память).

### `CacheKey`

Идентификатор записи кэша, обеспечивающий уникальность и версионирование.

```rust
pub struct CacheKey {
    pub hash: String,        // SHA256 содержимого SVG
    pub version: String,     // Версия генератора (CARGO_PKG_VERSION)
    pub config_hash: String, // SHA256 сериализованного конфига
}
```

**Методы:**

- `from_svg(svg: &str, config: &Config) -> Self` — создание ключа из SVG-строки и конфигурации. Вычисляет хеш содержимого, берет версию крейта из окружения компиляции и хеширует конфигурацию.

### `CacheConfig`

Конфигурация кэширования.

```rust
pub struct CacheConfig {
    pub disk_path: Option<PathBuf>, // Путь к директории для Sled (None = только память)
    pub memory_size: usize,         // Размер LRU кэша в памяти (количество записей)
    pub ttl: Option<Duration>,     // Время жизни записи на диске (None = бессрочно)
}
```

**Значения по умолчанию:**
- `memory_size`: 1000 записей
- `disk_path`: `~/.cache/svg2web/` (только для CLI)
- `ttl`: None (записи не удаляются по времени, только по версии)

### `DiskCache`

Внутренняя структура дискового кэша на базе Sled (embedded key-value store).

```rust
pub struct DiskCache {
    db: sled::Db,
    path: PathBuf,
}
```

Особенности:
- ACID транзакции (атомарность записи)
- Компрессия данных на лету
- Персистентность между перезапусками приложения
- Версионирование: автоматическая инвалидация при изменении `version` в ключе

### `MemoryCache`

In-memory LRU кэш для горячих данных.

```rust
pub struct MemoryCache {
    cache: LruCache<CacheKey, CacheEntry>,
    max_size: usize,
}
```

Использует `lru` crate с `Arc<Mutex<_>>` для thread-safety.

## Когда использовать

### CLI (инкрементальные сборки)

```rust
use svg2web_cache::{Cache, CacheConfig, CacheKey};
use std::path::PathBuf;

let config = CacheConfig {
    disk_path: Some(PathBuf::from(".svg2web-cache")),
    memory_size: 100,
    ttl: None,
};
let cache = Cache::new(config).await?;

// При повторной сборке того же файла с тем же конфигом — мгновенный результат
let key = CacheKey::from_svg(&svg_content, &parse_config);
if let Some(entry) = cache.get(&key).await? {
    return Ok(entry.output);
}
```

### Сервер (кэширование повторных запросов)

```rust
// Для API-сервисов: кэширование результатов парсинга часто запрашиваемых SVG
let cache = Cache::new(CacheConfig {
    disk_path: Some(PathBuf::from("/var/cache/svg2web")),
    memory_size: 10000,
    ttl: Some(Duration::from_secs(3600)), // 1 час TTL
}).await?;

// LRU в памяти обрабатывает горячие запросы без I/O
```

### WASM (только память)

```rust
// В браузере нет доступа к файловой системе, используем только MemoryCache
use svg2web_cache::MemoryCache;

let cache = MemoryCache::new(50); // 50 записей в памяти
let key = CacheKey::from_svg(svg_input, &config);

if let Some(result) = cache.get(&key) {
    return Ok(result);
}
// Иначе — парсим и сохраняем
cache.set(key, entry);
```

## Особенности реализации

**Ключ кэша (CacheKey):**
Детерминированный SHA256 от конкатенации:
1. SHA256(SVG содержимого)
2. Версия крейта (меняется при обновлении)
3. SHA256(сериализованного конфига в JSON)

Это гарантирует инвалидацию при любом изменении входных данных, версии кода или настроек.

**Двухуровневая стратегия:**
- `get` сначала проверяет `MemoryCache` (O(1), нет I/O)
- При промахе — читает `DiskCache` и обновляет память
- `set` пишет в оба уровня параллельно (или последовательно при ошибке диска)

**Версионирование:**
Если `key.version != cache.version` (например, обновился svg2web с новыми фичами), дисковый кэш автоматически игнорирует старые записи через метод `invalidate_old_versions()` при старте приложения.

**Ограничения WASM:**
- `DiskCache` недоступен (feature `disk` отключена по умолчанию для wasm target)
- `MemoryCache` использует `lru` без зависимости от файловой системы
- Размер кэша ограничен доступной памятью JS-heap (обычно 50-100 записей для больших SVG)
```