```markdown
# content.json

Файл содержит структурированный текстовый контент, извлеченный из SVG: заголовки, навигация, контактная информация, hero-секции, карточки и локализация. Предназначен для i18n, CMS-интеграции и редактирования контента без изменения SVG-структуры.

## Структура

```json
{
  "logo": "SVG2Web",
  "tagline": "Convert SVG to Web Components",
  "navigation": {
    "main": [
      {
        "label": "Features",
        "href": "#features",
        "active": false,
        "icon": "icon-star"
      },
      {
        "label": "Pricing",
        "href": "#pricing",
        "active": true
      }
    ],
    "secondary": [
      {
        "label": "Documentation",
        "href": "/docs"
      }
    ]
  },
  "contact": {
    "phone": "+1-555-123-4567",
    "phone_formatted": "+1 (555) 123-4567",
    "email": "hello@svg2web.dev",
    "address": "123 Web Street, San Francisco, CA 94102",
    "work_hours": "Mon-Fri: 9:00 - 18:00"
  },
  "hero": {
    "title": "Transform Your SVGs",
    "subtitle": "Into Production-Ready Components",
    "description": "Automated conversion from Figma to React, Vue, or Vanilla JS with optimization and asset extraction.",
    "cta": "Get Started",
    "cta_link": "/app",
    "secondary_cta": "View Demo",
    "image_alt": "Dashboard interface preview"
  },
  "cards": [
    {
      "id": "card-001",
      "title": "Parsing",
      "description": "Full SVG spec support with usvg",
      "icon": "icon-code",
      "cta": "Learn more",
      "badge": "Core",
      "stats": {
        "metric": "Speed",
        "value": "10ms"
      }
    }
  ],
  "footer": {
    "copyright": "© 2024 SVG2Web Contributors. MIT License.",
    "social": [
      {
        "platform": "github",
        "url": "https://github.com/org/svg2web"
      },
      {
        "platform": "twitter",
        "url": "https://twitter.com/svg2web"
      }
    ],
    "legal": [
      {
        "label": "Privacy Policy",
        "href": "/privacy"
      },
      {
        "label": "Terms of Service",
        "href": "/terms"
      }
    ]
  },
  "localization": {
    "locale": "en-US",
    "currency": "USD",
    "currency_symbol": "$",
    "date_format": "MM/DD/YYYY"
  }
}
```

## Поля

### `logo`
- **Тип**: `string | null`
- **Описание**: Текстовое содержимое логотипа, извлеченное из текстового элемента или атрибута `aria-label`

### `tagline`
- **Тип**: `string | null`  
- **Описание**: Слоган/подзаголовок под логотипом или в hero-секции

### `navigation`
Объект навигационных структур интерфейса.

**main** и **secondary** (Array):
- `label`: string — текст ссылки
- `href`: string — URL или anchor (#features)
- `active`: boolean (опционально) — текущий активный пункт
- `icon`: string (опционально) — ID иконки из assets.json/icons

### `contact`
Контактная информация, извлеченная из текстовых блоков и ссылок mailto:/tel:.

- `phone`: string — номер для href (E.164 формат)
- `phone_formatted`: string — отформатированный для отображения
- `email`: string — email адрес
- `address`: string — физический адрес офиса
- `work_hours`: string — часы работы

### `hero`
Содержимое главной hero-секции лендинга.

- `title`: string — главный заголовок (обычно H1)
- `subtitle`: string — подзаголовок (H2)
- `description`: string — описание/параграф
- `cta`: string — текст primary кнопки
- `cta_link`: string — URL для primary кнопки
- `secondary_cta`: string (опционально) — текст secondary кнопки
- `image_alt`: string — alt текст для изображения hero-секции

### `cards`
Массив карточек/фич-секций (Array).

**Card Object**:
- `id`: string — уникальный идентификатор карточки (из ID элемента в SVG или сгенерированный)
- `title`: string — заголовок карточки
- `description`: string — описание
- `icon`: string — ID иконки из assets.json/icons или имя SVG-иконки
- `cta`: string (опционально) — текст кнопки/ссылки
- `badge`: string (опционально) — метка (New, Core, Beta)
- `stats`: object (опционально) — метрики/статистика
  - `metric`: string — название метрики
  - `value`: string — значение метрики

### `footer`
Содержимое футера.

- `copyright`: string — текст копирайта
- `social`: Array — социальные сети
  - `platform`: string — название (github, twitter, linkedin)
  - `url`: string — полный URL
- `legal`: Array — юридические ссылки
  - `label`: string — текст ссылки
  - `href`: string — относительный или абсолютный URL

### `localization`
Настройки локали для форматирования.

- `locale`: string — BCP 47 код локали (en-US, ru-RU)
- `currency`: string — ISO 4217 код валюты (USD, EUR, RUB)
- `currency_symbol`: string — символ валюты ($, €, ₽)
- `date_format`: string — шаблон формата даты (MM/DD/YYYY, DD.MM.YYYY)

## Примечания

- **Извлечение**: Текст извлекается из SVG text-элементов с учетом группировки (tspan) и стилей. Автоматически определяются паттерны (телефоны, email) по регулярным выражениям.
- **CMS интеграция**: Этот файл может быть использован как источник правды для headless CMS (Contentful, Sanity, Strapi) — генератор может подставлять контент из CMS вместо извлеченного при совпадении ключей.
- **i18n**: Для многоязычности создаются отдельные файлы `content.en.json`, `content.ru.json` с теми же ключами. Генератор подключает нужный файл по параметру `locale`.
- **Перезапись**: При конфликтах приоритет имеет явный конфиг (svg2web.toml `content.override`) над извлеченным автоматически.
```