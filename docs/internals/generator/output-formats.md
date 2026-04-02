<!-- [Doc] | Реализация генерации по форматам: HTML/CSS, React, Vue и общие оптимизации выходного кода. -->

# Output Formats

## HTML/CSS

- Используются семантические теги там, где возможно (section/article вместо избыточных div).
- CSS переменные генерируются из styles.json.
- Адаптивность строится по mobile-first через min-width media queries.

## React

- Генерируются TypeScript interfaces из ожидаемых props.
- Основа: functional components (React.FC или typed function).
- Для статичных компонентов применяется React.memo.
- Стратегия стилей выбирается через config: CSS Modules или Styled Components.

## Vue

- Генерируется SFC структура: template/script/style.
- Scoped CSS реализуется через data-v-xxx атрибуты.
- Для code splitting поддерживается defineAsyncComponent.

## Оптимизации

- Code splitting для React через dynamic imports.
- Совместимость с tree-shaking через ES module exports и избегание side effects.
