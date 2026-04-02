# Разработка плагинов

## Trait FormatRenderer

```rust
pub trait FormatRenderer {
    fn name(&self) -> &str;
    fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput, RenderError>;
}
```

Trait должен быть доступен для внешней имплементации из plugin crate.

## Реализация

1. Создайте отдельный crate, например my-formatter.
2. Реализуйте trait для struct MyRenderer.
3. Используйте RenderContext для доступа к ParseOutput и настройкам.

Пример:

```rust
pub struct MyRenderer;

impl FormatRenderer for MyRenderer {
    fn name(&self) -> &str {
        "myformat"
    }

    fn render(&self, ctx: &RenderContext) -> Result<RenderedOutput, RenderError> {
        // generate files from ctx
        todo!()
    }
}
```

## Сборка

В Cargo.toml:

```toml
[lib]
crate-type = ["cdylib"]
```

Сборка:

```bash
cargo build --release
```

Результат: .so/.dll/.dylib в target/release.

## Динамическая загрузка

- Используйте libloading (dlopen)
- Найдите символ entry point
- Зарегистрируйте рендерер в FormatRegistry во время runtime

## Примеры форматов

- Svelte: генерация .svelte с props
- Solid: генерация .jsx с signals

## Тестирование плагинов

- интеграционный тест с generator (trait контракт)
- интеграционный тест динамической загрузки через libloading (dlopen)
- snapshot-тесты выхода через insta
