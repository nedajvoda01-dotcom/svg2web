<!-- [Doc] | Контракт Vue SFC-генерации: .vue формат, script setup, scoped стили, интеграция и ограничения. -->

# Vue генерация

## Формат выхода

Генерируется .vue файл с тремя блоками:

- <template>
- <script setup>
- <style scoped>

Пример:

```vue
<template>
  <svg class="logo" @click="onClick" />
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{ size?: number | string; color?: string }>(), {
  size: '100%',
  color: 'currentColor'
});

const emit = defineEmits<{ (e: 'click', event: MouseEvent): void }>();

function onClick(event: MouseEvent) {
  emit('click', event);
}
</script>

<style scoped>
.logo { display: block; }
</style>
```

## Script Setup

- defineProps<{}> используется с дефолтами через withDefaults.
- defineEmits<{}> для событий, например click и hover.
- при необходимости типы выносятся в отдельный файл и импортируются.

## Стили

- scoped включен по умолчанию для изоляции.
- Vue реализует scoped CSS через атрибуты вида data-v-xxxx на сгенерированных узлах и селекторах.
- CSS-переменные можно пробрасывать через :global().
- поддерживаются CSS preprocessors, включая SASS, если это указано в конфиге проекта.

Пример :global():

```vue
<style scoped>
:global(:root) {
  --logo-primary: #2563eb;
}
.logo {
  color: var(--logo-primary);
}
</style>
```

## Интеграция

- Nuxt: можно использовать авто-импорт из components/.
- Vue CLI/Vite: ручной импорт компонента.
- регистрация доступна как глобально, так и локально.

Пример локального импорта:

```vue
<script setup lang="ts">
import Logo from './output/components/Logo.vue';
</script>
```

## Когда использовать

- для Vue-проектов, где нужен SFC-формат из коробки;
- в простых кейсах обычно меньше boilerplate, чем в React;
- требуется Vue 3 с Composition API.

## См. также

- [CLI](../cli.md)
- [Конфигурация](../config.md)
