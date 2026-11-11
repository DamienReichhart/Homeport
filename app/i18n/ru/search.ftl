### The `search` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/search.ts:
### change the text there, then run it again.

search-kind-container = Контейнеры
search-kind-stack = Стеки
search-kind-image = Образы
search-kind-volume = Тома
search-kind-network = Сети
search-onHostState = { $host } · { $state ->
        [running] работает
       *[other] остановлен
    }
search-showAll = Показать все ({ $count })
search-skipped = Поиск не выполнен на { $count ->
        [one] { NUM($count) } хосте
        [few] { NUM($count) } хостах
        [many] { NUM($count) } хостах
       *[other] { NUM($count) } хоста
    } (нет подключения)
