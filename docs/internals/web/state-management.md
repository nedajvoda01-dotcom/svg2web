<!-- [Doc] | React state-management web-клиента: useConverter/useAnalyzer/useWorker и глобальный ConversionContext. -->

# State Management

## useConverter hook

Основные состояния:

- idle -> parsing -> analyzing -> optimizing -> generating -> done

Допустимые переходы состояний:

- Обратный переход из analyzing в idle невозможен напрямую: только через error state.
- Из любого активного состояния можно перейти в error, из error — только в idle.

Прогресс по этапам:

- idle: 0
- parsing: 25
- analyzing: 50
- optimizing: 75
- generating: 100

Значения прогресса только возрастают (0→25→50→75→100), откат назад невозможен.

Дополнительно:

- Progress 0-100 рассчитывается по фазам конвейера и весам шагов.
- Error handling включает retry logic для временных сбоев.
- Result хранит GeneratedCode и метаданные генерации.

## useAnalyzer hook

- Выполняет анализ без генерации.
- Возвращает метрики сложности: количество nodes, глубина дерева, размер входа.
- Используется для preview и ранней оценки качества SVG.

## useWorker hook

- Абстракция над postMessage API.
- Promise-based интерфейс для запрос/ответ.
- Очередь запросов, если воркер занят.

## ConversionContext

Глобальное состояние:

- выбранный файл
- настройки генерации
- история запусков

Оптимизации:

- useMemo для тяжелых вычислений.
- useCallback для стабильных handlers.

Persistence:

- настройки сохраняются в localStorage и восстанавливаются при следующем запуске.
