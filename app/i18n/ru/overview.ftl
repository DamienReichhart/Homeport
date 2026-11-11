### The `overview` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/overview.ts:
### change the text there, then run it again.

overview-nav = Обзор
overview-title = Обзор
overview-addHost = Добавить хост
overview-addCluster = Добавить кластер
overview-cluster-nodes = Готово Node: { $ready } из { $total }
overview-cluster-nodesNotAllowed = Node: нет доступа
overview-cluster-pods = Работает Pod: { $running } из { $total }
overview-cluster-podsNotAllowed = Pod: нет доступа
overview-cluster-notAllowed = Нет доступа.
overview-cluster-versionTitle = Кластер
overview-cluster-version = Версия
overview-cluster-platform = Платформа
overview-cluster-server = Сервер API
overview-nomad-region = Регион
overview-nomad-address = Адрес
overview-cluster-namespace = Пространство имён
overview-cluster-allNamespaces = Все пространства имён
overview-cluster-nodesTitle = Node
overview-cluster-nodesReady = готовы
overview-cluster-allNodes = Все Node
overview-cluster-podsTitle = Pod
overview-cluster-running = Работают: { $count }
overview-cluster-pending = Ожидают: { $count }
overview-cluster-crashing = Сбоят: { $count }
overview-cluster-done = Завершены или другое: { $count }
overview-cluster-allPods = Все Pod
overview-cluster-unreadyTitle = Неготовые рабочие нагрузки
overview-cluster-allReady = У каждого Deployment, StatefulSet и DaemonSet все Pod готовы.
overview-cluster-warningsTitle = Недавние предупреждения
overview-cluster-noWarnings = Событий-предупреждений нет.
overview-cluster-allEvents = Все события
overview-host-running = <strong>{ $running }</strong> / { $total } работают
overview-host-images = { $count ->
        [one] { NUM($count) } образ
        [few] { NUM($count) } образа
        [many] { NUM($count) } образов
       *[other] { NUM($count) } образа
    }
overview-host-containerdContainers = { $count ->
        [one] { NUM($count) } контейнер
        [few] { NUM($count) } контейнера
        [many] { NUM($count) } контейнеров
       *[other] { NUM($count) } контейнера
    } в { $namespaces ->
        [one] { NUM($namespaces) } пространстве имён
        [few] { NUM($namespaces) } пространствах имён
        [many] { NUM($namespaces) } пространствах имён
       *[other] { NUM($namespaces) } пространства имён
    }
overview-host-incusInstances = { $count ->
        [one] { NUM($count) } экземпляр
        [few] { NUM($count) } экземпляра
        [many] { NUM($count) } экземпляров
       *[other] { NUM($count) } экземпляра
    } в { $projects ->
        [one] { NUM($projects) } проекте
        [few] { NUM($projects) } проектах
        [many] { NUM($projects) } проектах
       *[other] { NUM($projects) } проекта
    }
overview-incus-title = Incus
overview-incus-version = Версия
overview-incus-os = Система
overview-incus-arch = Архитектура
overview-incus-projects = Проекты
overview-incus-shown = Показан
overview-incus-holds = { $instances ->
        [one] { NUM($instances) } экземпляр
        [few] { NUM($instances) } экземпляра
        [many] { NUM($instances) } экземпляров
       *[other] { NUM($instances) } экземпляра
    }, { $images ->
        [one] { NUM($images) } образ
        [few] { NUM($images) } образа
        [many] { NUM($images) } образов
       *[other] { NUM($images) } образа
    }
overview-incus-instances = Экземпляры
overview-incus-images = Образы
overview-incus-storage = Хранилище
overview-containerd-title = containerd
overview-containerd-version = Версия
overview-containerd-revision = Ревизия
overview-containerd-platform = Платформа
overview-containerd-namespaces = Пространства имён
overview-containerd-shown = Показано
overview-containerd-holds = { $containers ->
        [one] { NUM($containers) } контейнер
        [few] { NUM($containers) } контейнера
        [many] { NUM($containers) } контейнеров
       *[other] { NUM($containers) } контейнера
    }, { $images ->
        [one] { NUM($images) } образ
        [few] { NUM($images) } образа
        [many] { NUM($images) } образов
       *[other] { NUM($images) } образа
    }
overview-containerd-containers = Контейнеры
overview-containerd-images = Образы
overview-containerd-snapshots = Снимки
overview-host-cpu = { $percent } от { $cpus ->
        [one] { NUM($cpus) } ЦП
        [few] { NUM($cpus) } ЦП
        [many] { NUM($cpus) } ЦП
       *[other] { NUM($cpus) } ЦП
    }
overview-host-cpuTitle = Загрузка ЦП контейнерами, 100% на один ЦП
overview-host-memoryTitle = Память, используемая контейнерами
overview-engine-title = Движок
overview-engine-name = Имя
overview-engine-version = Версия
overview-engine-os = ОС
overview-engine-kernel = Ядро
overview-engine-architecture = Архитектура
overview-engine-resources = Ресурсы
overview-engine-resourcesValue = { $cpus ->
        [one] { NUM($cpus) } ЦП
        [few] { NUM($cpus) } ЦП
        [many] { NUM($cpus) } ЦП
       *[other] { NUM($cpus) } ЦП
    } · { $memory }
overview-engine-storageDriver = Драйвер хранилища
overview-engine-rootDir = Корневой каталог
overview-disk-title = Использование диска
overview-disk-cleanUp = Очистить…
overview-disk-column-type = Тип
overview-disk-column-total = Всего
overview-disk-column-active = Используется
overview-disk-column-size = Размер
overview-disk-column-reclaimable = Можно освободить
overview-disk-images = Образы
overview-disk-containers = Контейнеры
overview-disk-volumes = Тома
overview-disk-buildCache = Кеш сборки
overview-disk-partial = Этот Docker Engine старше версии 29 и сообщает меньше: размеры образов оценочные, а размеры томов и кеша сборки недоступны.
overview-events-title = Недавние события
overview-events-all = Все события
overview-events-empty = С момента подключения Homeport ничего не произошло.
overview-prune-title = Очистка
overview-prune-description = Выберите, что удалить с этого хоста. Это нельзя отменить.
overview-prune-confirm = Очистить
overview-prune-confirmWithVolumes = Удалить, включая данные томов
overview-prune-containers = Остановленные контейнеры
overview-prune-containers-detail = Удаляет все контейнеры, которые не работают.
overview-prune-images = Висячие образы
overview-prune-images-detail = Слои без тегов, оставшиеся после сборок и скачиваний.
overview-prune-allImages = Все неиспользуемые образы
overview-prune-allImages-detail = Также удаляет образы с тегами, которые не использует ни один контейнер.
overview-prune-networks = Неиспользуемые сети
overview-prune-networks-detail = Пользовательские сети, к которым не подключён ни один контейнер.
overview-prune-buildCache = Кеш сборки
overview-prune-buildCache-detail = Кешированные слои сборки.
overview-prune-volumes = Неиспользуемые тома
overview-prune-volumes-detail = Удаляет данные во всех томах, которые не использует ни один контейнер, включая именованные тома.
overview-pruned = Удалено: { $containers ->
        [one] { NUM($containers) } контейнер
        [few] { NUM($containers) } контейнера
        [many] { NUM($containers) } контейнеров
       *[other] { NUM($containers) } контейнера
    }, { $images ->
        [one] { NUM($images) } образ
        [few] { NUM($images) } образа
        [many] { NUM($images) } образов
       *[other] { NUM($images) } образа
    }, { $networks ->
        [one] { NUM($networks) } сеть
        [few] { NUM($networks) } сети
        [many] { NUM($networks) } сетей
       *[other] { NUM($networks) } сети
    } и { $volumes ->
        [one] { NUM($volumes) } том
        [few] { NUM($volumes) } тома
        [many] { NUM($volumes) } томов
       *[other] { NUM($volumes) } тома
    } — освобождено { $space }.
overview-palette-prune = Очистить выбранный хост
overview-host-onBoard = Его контейнеры на доске
