### The `clusterevents` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/clusterevents.ts:
### change the text there, then run it again.

clusterevents-nav = События
clusterevents-title = События
clusterevents-summary = { $count ->
        [one] { NUM($count) } событие
        [few] { NUM($count) } события
        [many] { NUM($count) } событий
       *[other] { NUM($count) } события
    }
clusterevents-search = Поиск по причине, объекту или сообщению
clusterevents-empty = Событий нет. Kubernetes хранит их около часа.
clusterevents-filter = Тип
clusterevents-all = Все
clusterevents-warnings = Предупреждения
clusterevents-normal = Обычные
clusterevents-column-type = Тип
clusterevents-column-reason = Причина
clusterevents-column-object = Объект
clusterevents-column-namespace = Пространство имён
clusterevents-column-message = Сообщение
clusterevents-column-count = Количество
clusterevents-column-lastSeen = Последнее
