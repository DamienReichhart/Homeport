### The `core` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/core.ts:
### change the text there, then run it again.

core-host-engineVersion = Движок { $version }
core-host-podmanVersion = Podman { $version }
core-host-containerdVersion = containerd { $version }
core-host-incusVersion = Incus { $version }
core-engine-docker = Совместимый с Docker
core-engine-containerd = containerd
core-engine-incus = Incus
core-unsupported-title = { $name } не может показать эту страницу
core-unsupported-docker = Этой странице нужен движок, совместимый с Docker, а на { $name } работает { $engine }.
core-unsupported-podman = Эта страница для Podman, а на { $name } работает другой движок, совместимый с Docker. Чтобы открыть её, выберите хост Podman в переключателе.
core-unsupported-containerd = Эта страница для containerd, а на { $name } работает { $engine }. Чтобы открыть её, выберите хост containerd в переключателе.
core-unsupported-incus = Эта страница для Incus, а на { $name } работает { $engine }. Чтобы открыть её, выберите хост Incus в переключателе.
core-unsupported-overview = Перейти к обзору
core-cluster-kind-kubernetes = Kubernetes
core-cluster-kind-nomad = Nomad
core-cluster-unsupported-title = { $name } не может показать эту страницу
core-cluster-unsupported-description = Эта страница для кластеров { $kind }, а { $name } — кластер { $current }. Чтобы открыть её, выберите кластер { $kind } в переключателе.
core-containerd-namespace = Пространство имён
core-containerd-holds = { $containers ->
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
core-containerd-manage = Управление пространствами имён
core-containerd-rememberFailed = Homeport не удалось запомнить выбранное пространство имён
core-incus-project = Проект
core-incus-holds = { $instances ->
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
core-incus-rememberFailed = Homeport не удалось запомнить выбранный проект
core-selection-rememberFailed = Homeport не удалось запомнить ваш выбор
core-connection-local = Локальный
core-status-connected = Подключён
core-status-connecting = Подключение
core-status-disabled = Отключён
core-status-unreachable = Недоступен
core-status-unreachableDetail = { $message } — повтор через { $secs } с
core-status-unauthorized = Доступ запрещён
core-status-hostKeyUntrusted = Ключ сервера не доверенный
core-status-hostKeyUntrustedDetail = Проверьте отпечаток { $fingerprint } и доверьтесь ему, чтобы подключиться.
core-status-hostKeyMismatch = Ключ сервера изменился
core-status-hostKeyMismatchDetail = Ожидался { $expected }, получен { $actual }.
core-status-credentialsLost = Учётные данные утеряны
core-status-credentialsLostDetail = Сохранённые учётные данные не удаётся расшифровать. Введите их заново или создайте ключ заново.
core-status-keychainUnavailable = Связка ключей недоступна
core-status-invalid = Ошибка настройки
core-swarm-label = Swarm хоста { $host }
core-swarm-labelUnknown = Swarm { $id }
core-swarm-gone-title = Этот Swarm больше не известен
core-swarm-gone-description = Ни один из хостов, к которым подключается Homeport, теперь в него не входит. Чтобы управлять им, добавьте один из его управляющих узлов как хост.
core-cluster-serverVersion = Kubernetes { $version }
core-cluster-nomadVersion = Nomad { $version }, регион { $region }
core-cluster-nomadRegion = Nomad, регион { $region }
core-cluster-credentialsLostDetail = Сохранённые учётные данные не удаётся расшифровать. Обновите кластер из его kubeconfig.
core-cluster-reconnectFailed = Homeport не удалось заново подключиться к кластеру
core-cluster-gate-none = Кластер не выбран
core-cluster-gate-noneDescription = Выберите кластер на боковой панели или добавьте его из своего kubeconfig.
core-cluster-gate-add = Добавить кластер
core-cluster-gate-openSettings = Открыть настройки кластера
core-localClusters-title = Кластеры на этом компьютере
core-localClusters-description = Kubernetes, работающий локально, — так, как до него доходит ваш kubeconfig.
core-localClusters-add = Добавить
core-localClusters-added = Добавлен
core-localClusters-notRunning = Не запущен
core-localClusters-addFailed = Homeport не удалось добавить кластер
core-localClusters-missing = Кластер { $tool } { $name } работает, но ваш kubeconfig до него не доходит. Добавьте его командой:
core-localClusters-tool-dockerDesktop = Docker Desktop
core-localClusters-tool-orbstack = OrbStack
core-localClusters-tool-rancherDesktop = Rancher Desktop
core-localClusters-tool-kind = kind
core-localClusters-tool-k3d = k3d
core-localClusters-tool-minikube = minikube
core-namespace-label = Пространство имён
core-namespace-all = Все пространства имён
core-namespace-type = Пространство имён
core-namespace-cantList = Ваши учётные данные не позволяют получить список пространств имён. Введите то, которым вам разрешено пользоваться.
core-namespace-rememberFailed = Homeport не удалось запомнить пространство имён
core-health-ready = Готов
core-health-progressing = Выполняется
core-health-degraded = Неполная работа
core-health-failed = Сбой
core-health-paused = Приостановлен
core-events-type = Тип
core-events-reason = Причина
core-events-message = Сообщение
core-events-count = Количество
core-events-lastSeen = Последнее
core-events-none = Событий нет. Kubernetes хранит их около часа.
core-conditions-type = Условие
core-conditions-status = Состояние
core-conditions-reason = Причина
core-conditions-message = Сообщение
core-conditions-since = С
core-conditions-none = Условия не сообщаются.
core-labels-none = Нет
core-yaml-label = YAML
core-list-empty = Здесь ничего нет.
core-yaml-copy = Скопировать YAML
core-gate-welcome-title = Добро пожаловать в Homeport
core-gate-welcome-description = Для начала добавьте Docker Engine — этого компьютера или удалённый по SSH или TLS — или кластер Kubernetes из своего kubeconfig.
core-gate-addAnother = Добавить ещё хост
core-gate-addCluster = Добавить кластер
core-gate-connecting = Подключение к { $name }…
core-gate-statusTitle = { $name }: { $status }
core-gate-retryingOnItsOwn = Homeport сам повторит попытку через { $secs } с.
core-gate-retryNow = Повторить сейчас
core-gate-openSettings = Открыть настройки хоста
core-localEngines-title = Найдено на этом компьютере
core-localEngines-description = Движки, отвечающие на локальных сокетах.
core-localEngines-add = Добавить
core-freshness-check = Проверить новые версии образов
core-freshness-checking = Проверка…
core-freshness-allUpToDate = Все образы актуальны
core-freshness-newerCount = Новая версия есть у { $count ->
        [one] { NUM($count) } образа
        [few] { NUM($count) } образов
        [many] { NUM($count) } образов
       *[other] { NUM($count) } образа
    }
core-freshness-newerOne = Опубликована новая версия { $reference }
core-freshness-newerMany = Опубликованы новые версии образов: { $references }
core-delete-title = Удалить { $kind } { $name }?
core-delete-titleMany = Удалить объекты { $kind } ({ $count })?
core-delete-titleObjects = Удалить объекты ({ $count })?
core-delete-one = { $kind } { $name } будет удалён из { $cluster }.
core-delete-oneIn = { $kind } { $name } будет удалён из пространства имён { $namespace } в { $cluster }.
core-delete-many = Из { $cluster } будут удалены: { $names }.
core-delete-manyIn = Из пространства имён { $namespace } в { $cluster } будут удалены: { $names }.
core-delete-namespace = Вместе с ним удаляется всё, что в нём есть: рабочие нагрузки, Pod, Service, ConfigMap и Secret.
core-delete-recreated = Им управляет { $kind } { $name }, поэтому на его месте будет запущен новый Pod.
core-delete-pods = Его Pod удаляются вместе с ним.
core-delete-cronJob = Он больше не запускает Job; запущенные им Job удаляются вместе с ним.
core-delete-node = Homeport не освобождает его заранее: его Pod остаются, пока Node не зарегистрируется снова или они не будут перераспределены.
core-delete-confirm = Удалить
core-delete-failed = Homeport не удалось удалить { $name }
core-delete-done = { $count ->
        [1] Удалено: { $name }
        [one] Удалён { NUM($count) } объект
        [few] Удалено { NUM($count) } объекта
        [many] Удалено { NUM($count) } объектов
       *[other] Удалено { NUM($count) } объекта
    }
core-delete-action = Удалить
core-delete-selected = Удалить ({ $count })
core-apply-preview = Просмотреть изменения
core-apply-apply = Применить
core-apply-force = Всё равно применить
core-apply-stale = YAML изменился после этого просмотра.
core-apply-noChanges = Изменений нет: в кластере уже это состояние.
core-apply-confirmTitle = Применить { $count ->
        [one] { NUM($count) } изменение
        [few] { NUM($count) } изменения
        [many] { NUM($count) } изменений
       *[other] { NUM($count) } изменения
    } к { $cluster }?
core-apply-confirmDescription = Homeport применяет его через server-side apply как менеджер полей homeport, как показано в просмотре.
core-apply-forceTitle = Взять на себя конфликтующие поля?
core-apply-forceDescription = Инструменты, которые управляют ими в { $cluster }, могут вернуть их обратно или перестать ими управлять. Их менеджером станет Homeport.
core-apply-done = Применено к { $cluster }
core-apply-diffOf = Изменения в { $kind } { $name }
core-apply-action-created = Создан
core-apply-action-changed = Изменён
core-apply-action-unchanged = Без изменений
core-apply-action-failed = Не удаётся применить
core-yaml-edit = Изменить
core-yaml-editTitle = Изменить { $kind } { $name }
core-yaml-editDescription = В { $cluster }. Измените YAML, просмотрите, что он изменит, и примените его.
core-metrics-cpu = ЦП
core-metrics-memory = Память
core-metrics-used = Использовано
core-metrics-request = Запрос
core-metrics-limit = Лимит
core-metrics-allocatable = Доступно для размещения
core-metrics-ofLimit = { $percent } лимита
core-metrics-ofRequest = { $percent } запроса
core-metrics-ofAllocatable = { $percent } доступного для размещения
core-metrics-notInstalled-title = metrics-server не установлен
core-metrics-notInstalled-body = Этот кластер не предоставляет Metrics API, из которого берутся данные о ЦП и памяти. Homeport ничего не устанавливает в кластер: когда metrics-server будет установлен, они появятся здесь.
core-metrics-notInstalled-link = Как установить metrics-server
core-metrics-notResponding = Metrics API не отвечает: metrics-server, возможно, запускается или не работает. Homeport повторяет попытку каждые 15 секунд.
core-metrics-waiting = Ожидание первого замера от metrics-server…
core-metrics-kept = Замер каждые 15 секунд; замеры хранятся, пока эта страница открыта, не дольше 15 минут.
