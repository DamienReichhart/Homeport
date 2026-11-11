### The `events` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/events.ts:
### change the text there, then run it again.

events-nav = События
events-title = События
events-description = То, о чём сообщил движок; сохраняется между перезапусками.
events-descriptionKept = То, о чём сообщил движок; хранится { $days ->
        [one] { NUM($days) } день
        [few] { NUM($days) } дня
        [many] { NUM($days) } дней
       *[other] { NUM($days) } дня
    }, не более 10 000 событий.
events-clear = Очистить историю
events-clear-title = Очистить историю событий?
events-clear-description = Все события, сохранённые для этого хоста, будут забыты. Новые продолжат появляться; собственные события движка это не затронет.
events-clear-confirm = Очистить историю
events-cleared = История событий очищена
events-loadOlder = Загрузить более ранние события
events-search = Поиск событий
events-type = Тип
events-allTypes = Все типы
events-empty-title = Событий пока нет
events-empty-description = Запустите или остановите контейнер, и событие появится здесь.
events-column-time = Время
events-column-type = Тип
events-column-action = Действие
events-column-object = Объект
events-column-stack = Стек
