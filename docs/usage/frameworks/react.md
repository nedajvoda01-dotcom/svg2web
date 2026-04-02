<!-- [Doc] | Контракт React-генерации: формат .tsx, props, стили, интеграция и ограничения SSR/размера. -->

# React генерация

## Формат выхода

- Файлы имеют расширение .tsx.
- Генерируются functional components (не class components).
- Типовая структура файла: imports -> interface Props -> component -> export.

Пример структуры:

```text
output/
  components/
    Logo.tsx
    Logo.module.css
  index.ts
```

## Props interface

```tsx
export interface LogoProps {
  size?: number | string; // default: "100%"
  color?: string; // default: "currentColor" или цвет из SVG
  className?: string; // для Tailwind/CSS Modules
  style?: React.CSSProperties;
  onClick?: (e: React.MouseEvent) => void;
}
```

Поведение по умолчанию:

- size: "100%"
- color: "currentColor" (или значение из исходного SVG)

## Внутреннее состояние

Для интерактивных элементов генератор может использовать:

- useState для hover/click состояния.
- useEffect для side-effects (анимации, подписки).
- memo для сокращения лишних ререндеров.

Пример:

```tsx
const Logo = memo(function Logo({ onClick, ...props }: LogoProps) {
  const [hovered, setHovered] = useState(false);

  useEffect(() => {
    // side-effects только на клиенте
  }, []);

  return <svg onClick={onClick} data-hovered={hovered} {...props} />;
});
```

## Стили

Поддерживаемые варианты:

1. CSS Modules: файл .module.css рядом с компонентом.
2. Styled Components: при выбранной опции генерации.
3. Inline styles: для критических правил.

## Интеграция

Импорт:

```tsx
import { Logo } from './output/components/Logo';
```

Tree-shaking:

- импортируйте только нужные компоненты;
- избегайте wildcard-импорта из больших index-файлов;
- проверяйте bundle analyzer в сборщике.

Next.js dynamic import:

```tsx
import dynamic from 'next/dynamic';

const Logo = dynamic(() => import('./output/components/Logo').then(m => m.Logo));
```

## Ограничения

- SSR без дополнительной настройки может быть ограничен, если код опирается на window.
- Доступ к window должен быть только внутри useEffect.
- Типичный вклад одного компонента в бандл: ~1-6 KB gzip (зависит от SVG и режима стилей).

## См. также

- [CLI](../cli.md)
- [Конфигурация](../config.md)
