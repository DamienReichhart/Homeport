### The `board` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/board.ts:
### change the text there, then run it again.

board-nav = Доска
board-title = Доска
board-summary = Работает { $running } из { $count ->
        [one] { NUM($count) } контейнера
        [few] { NUM($count) } контейнеров
        [many] { NUM($count) } контейнеров
       *[other] { NUM($count) } контейнера
    } на { $hosts ->
        [one] { NUM($hosts) } хосте
        [few] { NUM($hosts) } хостах
        [many] { NUM($hosts) } хостах
       *[other] { NUM($hosts) } хоста
    }
board-column-host = Хост
board-column-stack = Стек
board-sortBy = Сортировать по столбцу «{ $column }»
board-filter-hosts = Хосты
board-filter-allHosts = Все хосты
board-filter-someHosts = { $count ->
        [one] { NUM($count) } хост
        [few] { NUM($count) } хоста
        [many] { NUM($count) } хостов
       *[other] { NUM($count) } хоста
    }
board-filter-state = Состояние
board-filter-stack = Стек
board-filter-anyStack = Любой стек
board-filter-clear = Сбросить фильтры
board-state-all = Любое состояние
board-state-running = Работает
board-state-stopped = Остановлен
board-state-unhealthy = Нездоров
board-search = Поиск на всех хостах
board-skipped = { $name } ({ $status }) не показан.
board-openHost = Открыть хост
board-listFailed = Не удалось получить список контейнеров { $name }.
board-noHosts-title = Нет подключённых хостов
board-noHosts-description = На доске перечислены контейнеры всех подключённых хостов.
board-empty-title = Контейнеров пока нет
board-empty-description = Ни на одном из подключённых хостов не работает ни один контейнер.
board-noMatch = Нет контейнеров, подходящих под эти фильтры.
board-selection = Выбранные контейнеры
board-selectAll = Выбрать все показанные контейнеры
board-selectOne = Выбрать { $name } на { $host }
board-remove-title = Удалить { $count ->
        [one] { NUM($count) } контейнер
        [few] { NUM($count) } контейнера
        [many] { NUM($count) } контейнеров
       *[other] { NUM($count) } контейнера
    }?
board-remove-description = Они удаляются вместе со своими анонимными томами; именованные тома сохраняются. Работающие сначала останавливаются.
board-remove-onHost = На { $host }: { $names }
