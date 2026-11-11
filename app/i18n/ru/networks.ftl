### The `networks` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/networks.ts:
### change the text there, then run it again.

networks-nav = Сети
networks-title = Сети
networks-summary = { $count ->
        [one] { NUM($count) } сеть
        [few] { NUM($count) } сети
        [many] { NUM($count) } сетей
       *[other] { NUM($count) } сети
    }
networks-cleanUp = Очистить
networks-create = Создать сеть
networks-search = Поиск сетей
networks-empty = Сетей нет
networks-column-name = Имя
networks-column-stack = Стек
networks-column-driver = Драйвер
networks-column-subnet = Подсеть
networks-column-containers = Контейнеры
networks-builtIn = встроенная
networks-internal = внутренняя
networks-inspectOne = Просмотреть { $name }
networks-removeOne = Удалить { $name }
networks-remove-title = Удалить сеть { $name }?
networks-remove-attached = К ней всё ещё { $count ->
        [one] подключён { NUM($count) } контейнер
        [few] подключены { NUM($count) } контейнера
        [many] подключено { NUM($count) } контейнеров
       *[other] подключено { NUM($count) } контейнера
    }; сначала отключите их.
networks-remove-unused = Ни один контейнер её не использует. Контейнеры, которые ссылаются на неё по имени, не смогут запуститься.
networks-prune-title = Удалить неиспользуемые сети?
networks-prune-description = Удаляются все пользовательские сети, к которым не подключён ни один контейнер.
networks-connected = Подключено
networks-disconnected = Отключено
networks-removed = { $name } удалена
networks-pruned = Удалено: { $count ->
        [one] { NUM($count) } сеть
        [few] { NUM($count) } сети
        [many] { NUM($count) } сетей
       *[other] { NUM($count) } сети
    }
networks-createDialog-title = Создать сеть
networks-createDialog-name = Имя
networks-createDialog-driver = Драйвер
networks-createDialog-subnet = Подсеть
networks-createDialog-gateway = Шлюз
networks-createDialog-automatic = автоматически
networks-createDialog-internal = Внутренняя — без доступа к внешним сетям
networks-createDialog-attachable = Подключаемая
networks-dialog-containers = Контейнеры
networks-dialog-raw = Исходный JSON
networks-dialog-noContainer = Ни один контейнер не подключён.
networks-dialog-disconnect = Отключить
networks-dialog-containerToConnect = Контейнер для подключения
networks-dialog-connectPlaceholder = Подключить контейнер…
networks-dialog-connect = Подключить
