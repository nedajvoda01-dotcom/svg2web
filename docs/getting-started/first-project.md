```markdown
# Первый проект: от SVG до веб-компонента

Это руководство проведет вас через полный процесс конвертации SVG-иконки в готовый веб-компонент.

---

## Шаг 1: Подготовка SVG

### Экспорт из Figma

1. Выберите иконку или компонент
2. Нажмите **Export** → формат **SVG**
3. Настройки экспорта:
   - **Include**: "Frame" (только содержимое)
   - **Optimize**: включите
   - **Remove unused styles**: включите

**До очистки:**
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
  <rect x="0.5" y="0.5" width="23" height="23" stroke="black" stroke-opacity="0.1" fill="white"/>
  <path d="M12 4L12 20M4 12L20 12" stroke="currentColor" stroke-width="2"/>
</svg>
```

**После очистки (ручное удаление):**
```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
  <path d="M12 4L12 20M4 12L20 12" stroke="currentColor" stroke-width="2"/>
</svg>
```

### Оптимизация

```bash
# Используем svg2web для оптимизации
svg2web optimize input.svg --output optimized.svg --remove-comments --simplify-paths
```

---

## Шаг 2: Выбор фреймворка

| Фреймворк | Лучший выбор | Особенности |
|-----------|--------------|-------------|
| **React** | Интерактивные дашборды, SPA | TypeScript, хуки, пропсы |
| **Vue** | Простота, быстрая интеграция | SFC, scoped стили, реактивность |
| **Vanilla** | Статические сайты, лендинги | Нет зависимостей, минимальный размер |

**Для этого примера:** выберем **React + TypeScript**

---

## Шаг 3: Конфигурация

### Создание `svg2web.toml`

```toml
# svg2web.toml

[parse]
normalize = true
keep_comments = false

[optimizer]
simplify_paths = true
deduplicate = true
minify_ids = true
precision = 3

[generate]
framework = "react"
styling = "scoped"
responsive = true

[generate.components]
detect = true
min_size = 3
naming = "pascal"

[generate.output]
pretty = true
typescript = true
separate_css = true

[extractor]
extract_images = true
convert_to_webp = true
webp_quality = 85
extract_fonts = true
```

### Альтернатива: аргументы CLI

```bash
svg2web build input.svg \
  --output ./output \
  --format react \
  --typescript \
  --scoped-css \
  --optimize \
  --responsive
```

---

## Шаг 4: Запуск сборки

### Установка

```bash
# Через cargo
cargo install svg2web-cli

# Или через Docker
docker pull ghcr.io/org/svg2web:latest
```

### Запуск

```bash
# Базовый запуск
svg2web build icon.svg --output ./my-icon --format react

# С кастомным конфигом
svg2web build icon.svg --config svg2web.toml --output ./my-icon

# С подробным выводом
svg2web build icon.svg --output ./my-icon --verbose
```

### Результат

```
my-icon/
├── src/
│   └── Icon.tsx          # React компонент
├── Icon.module.css        # Scoped стили
├── assets/
│   └── icon.webp         # Оптимизированное изображение (если было)
└── index.html            # Демо страница
```

---

## Шаг 5: Интеграция в проект

### Сгенерированный компонент

```tsx
// Icon.tsx
import React from 'react';
import styles from './Icon.module.css';

interface IconProps {
  size?: number;
  color?: string;
  className?: string;
  onClick?: () => void;
}

export const Icon: React.FC<IconProps> = ({
  size = 24,
  color = 'currentColor',
  className,
  onClick
}) => {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      className={`${styles.icon} ${className || ''}`}
      onClick={onClick}
    >
      <path
        d="M12 4L12 20M4 12L20 12"
        stroke={color}
        strokeWidth="2"
        strokeLinecap="round"
      />
    </svg>
  );
};

export default Icon;
```

```css
/* Icon.module.css */
.icon {
  display: inline-block;
  flex-shrink: 0;
  transition: transform 0.2s ease;
}

.icon:hover {
  transform: scale(1.1);
}
```

### Использование в приложении

```tsx
// App.tsx
import { Icon } from './components/Icon';

function App() {
  const [isActive, setIsActive] = useState(false);

  return (
    <div>
      <Icon 
        size={32} 
        color={isActive ? '#3b82f6' : '#6b7280'}
        onClick={() => setIsActive(!isActive)}
      />
      <p>Кликни на иконку</p>
    </div>
  );
}
```

### Интеграция с Tailwind CSS

```tsx
// С Tailwind вместо CSS модулей
export const Icon: React.FC<IconProps> = ({ size = 24, color = 'currentColor', className }) => {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      className={`inline-block transition-transform hover:scale-110 ${className}`}
    >
      <path
        d="M12 4L12 20M4 12L20 12"
        stroke={color}
        strokeWidth="2"
        strokeLinecap="round"
      />
    </svg>
  );
};
```

---

## Шаг 6: Деплой

### Vercel (React)

```bash
# Создаем проект
vercel init react
cd my-app
cp -r ../my-icon/src/components ./src/

# Настраиваем
npm run dev

# Деплой
vercel --prod
```

### GitHub Pages

```yaml
# .github/workflows/deploy.yml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install svg2web
        run: cargo install svg2web-cli
      
      - name: Generate icons
        run: |
          svg2web build icon.svg --output ./public/icons --format react
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./public
```

### Docker

```dockerfile
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

---

## Результат

### До конвертации

```
input.svg (2.3 KB)
├── Неоптимизированный код
├── Лишние атрибуты
├── Фиксированные размеры
└── Нет интерактивности
```

### После конвертации

```
my-icon/
├── Icon.tsx (1.2 KB)        # Типизированный компонент
├── Icon.module.css (0.3 KB) # Изолированные стили
└── Демо (работает в браузере)
```

### Преимущества

- ✅ **Типизация**: TypeScript интерфейсы
- ✅ **Адаптивность**: Динамические размеры
- ✅ **Интерактивность**: Обработка событий
- ✅ **Производительность**: Оптимизированный код
- ✅ **Переиспользование**: Компонентный подход

---

## Полный пример кода

```bash
# 1. Подготовка
curl -o icon.svg https://raw.githubusercontent.com/org/svg2web/main/examples/basic/input.svg

# 2. Конфигурация
cat > svg2web.toml << EOF
[generate]
framework = "react"
typescript = true
scoped_css = true
EOF

# 3. Генерация
svg2web build icon.svg --output ./Icon --config svg2web.toml

# 4. Просмотр
cd Icon
npm init -y
npm install react react-dom typescript
npm run dev
```

---

## Следующие шаги

- [Интеграция с Tailwind CSS](/usage/frameworks/react.md#tailwind-css)
- [Создание анимированных иконок](/usage/animations.md)
- [Оптимизация для production](/usage/optimization.md)
- [Кастомные плагины](/contributing/plugin-development.md)
```