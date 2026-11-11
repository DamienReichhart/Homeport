### The `podmanpods` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/podmanpods.ts:
### change the text there, then run it again.

podmanpods-nav = Поды
podmanpods-title = Поды
podmanpods-summary = { $count ->
        [one] { NUM($count) } под
        [few] { NUM($count) } пода
        [many] { NUM($count) } подов
       *[other] { NUM($count) } пода
    }
podmanpods-search = Поиск подов
podmanpods-empty = Подов нет
podmanpods-emptyDescription = Под объединяет контейнеры, которые используют общую сеть и запускаются и останавливаются вместе. Создайте под, затем запускайте в нём контейнеры.
podmanpods-new = Новый под
podmanpods-column-name = Имя
podmanpods-column-status = Состояние
podmanpods-column-containers = Контейнеры
podmanpods-column-networks = Сети
podmanpods-column-created = Создан
podmanpods-running = Работает { $running } из { $count }
podmanpods-status-running = Работает
podmanpods-status-degraded = Работает частично
podmanpods-status-paused = Приостановлен
podmanpods-status-created = Создан
podmanpods-status-stopped = Остановлен
podmanpods-status-failed = Сбой
podmanpods-action-start = Запустить
podmanpods-action-stop = Остановить
podmanpods-action-restart = Перезапустить
podmanpods-action-pause = Приостановить
podmanpods-action-unpause = Возобновить
podmanpods-actionOne-start = Запустить { $name }
podmanpods-actionOne-stop = Остановить { $name }
podmanpods-actionOne-restart = Перезапустить { $name }
podmanpods-actionOne-pause = Приостановить { $name }
podmanpods-actionOne-unpause = Возобновить { $name }
podmanpods-removeOne = Удалить { $name }
podmanpods-done-start = { $name } запущен
podmanpods-done-stop = { $name } остановлен
podmanpods-done-restart = { $name } перезапущен
podmanpods-done-pause = { $name } приостановлен
podmanpods-done-unpause = { $name } возобновлён
podmanpods-removed = { $name } удалён { $count ->
        [0] без контейнеров
        [one] вместе с { NUM($count) } контейнером
        [few] вместе с { NUM($count) } контейнерами
        [many] вместе с { NUM($count) } контейнерами
       *[other] вместе с { NUM($count) } контейнера
    }
podmanpods-created = { $name } создан
podmanpods-remove-title = Удалить { $name }?
podmanpods-remove-stopped = { $count ->
        [0] Под и его инфраструктурный контейнер будут удалены.
        [one] Под, его инфраструктурный контейнер и его { NUM($count) } контейнер будут удалены.
        [few] Под, его инфраструктурный контейнер и его { NUM($count) } контейнера будут удалены.
        [many] Под, его инфраструктурный контейнер и его { NUM($count) } контейнеров будут удалены.
       *[other] Под, его инфраструктурный контейнер и его { NUM($count) } контейнера будут удалены.
    } Их тома сохраняются.
podmanpods-remove-running = { $count ->
        [0] Под работает: он будет остановлен, затем удалён вместе с инфраструктурным контейнером.
        [one] Под работает: его { NUM($count) } контейнер будет остановлен, затем удалён вместе с подом и его инфраструктурным контейнером.
        [few] Под работает: его { NUM($count) } контейнера будут остановлены, затем удалены вместе с подом и его инфраструктурным контейнером.
        [many] Под работает: его { NUM($count) } контейнеров будут остановлены, затем удалены вместе с подом и его инфраструктурным контейнером.
       *[other] Под работает: его { NUM($count) } контейнера будут остановлены, затем удалены вместе с подом и его инфраструктурным контейнером.
    } Их тома сохраняются.
podmanpods-remove-stopAndRemove = Остановить и удалить
podmanpods-detail-containers = { $count ->
        [0] Контейнеры
        [one] { NUM($count) } контейнер
        [few] { NUM($count) } контейнера
        [many] { NUM($count) } контейнеров
       *[other] { NUM($count) } контейнера
    }
podmanpods-detail-noContainers = Кроме инфраструктурного контейнера, в этом поде пока ничего не работает.
podmanpods-detail-infra = Инфраструктурный
podmanpods-detail-settings = Параметры
podmanpods-detail-ports = Опубликованные порты
podmanpods-detail-noPorts = Нет
podmanpods-detail-shared = Общие пространства имён
podmanpods-detail-networks = Сети
podmanpods-detail-hostname = Имя хоста
podmanpods-detail-created = Создан
podmanpods-detail-labels = Метки
podmanpods-detail-command = Создан командой
podmanpods-detail-runContainer = Запустить контейнер
podmanpods-detail-runContainerIn = Запустить контейнер в { $name }
podmanpods-newDialog-title = Новый под
podmanpods-newDialog-description = Его контейнеры используют одну сеть, поэтому порты публикуются на поде, сразу для всех.
podmanpods-newDialog-name = Имя
podmanpods-newDialog-portsHint = Оставьте порт хоста пустым, и Podman выберет свободный. Контейнеры, запущенные в поде, не могут публиковать собственные порты.
podmanpods-newDialog-labels = Метки
podmanpods-newDialog-create = Создать под
podmanpods-form-nameInvalid = Начните с буквы или цифры, затем используйте буквы, цифры, _, . или -.
podmanpods-form-portInvalid = Порт — число от 1 до 65535.
podmanpods-form-labelInvalid = Пишите по одной метке key=value на строку.
