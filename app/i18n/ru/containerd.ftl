### The `containerd` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/containerd.ts:
### change the text there, then run it again.

containerd-state-none = Нет задачи
containerd-state-created = Создан
containerd-state-running = Работает
containerd-state-paused = Приостановлен
containerd-state-pausing = Приостанавливается
containerd-state-stopped = Остановлен
containerd-state-unknown = Неизвестно
containerd-state-exited = Завершён ({ $code })
containerd-createdBy-kubernetes = Kubernetes
containerd-createdBy-nerdctl = nerdctl
containerd-createdBy-docker = Docker
containerd-createdBy-other = ctr или другой
containerd-containers-nav = Контейнеры
containerd-containers-title = Контейнеры
containerd-containers-summary = { $count ->
        [one] { NUM($count) } контейнер
        [few] { NUM($count) } контейнера
        [many] { NUM($count) } контейнеров
       *[other] { NUM($count) } контейнера
    }, работает: { $running }, в пространстве имён { $namespace }.
containerd-containers-search = Фильтр по имени, ID или образу
containerd-containers-showSandboxes = Показывать контейнеры pause для Pod
containerd-containers-sandbox = pause
containerd-containers-empty = В { $namespace } нет контейнеров
containerd-containers-emptyDescription = Здесь показываются контейнеры, созданные через nerdctl, ctr или Kubernetes. Чтобы увидеть другие, выберите другое пространство имён под переключателем хостов.
containerd-containers-column-name = Имя
containerd-containers-column-image = Образ
containerd-containers-column-state = Задача
containerd-containers-column-createdBy = Создан через
containerd-containers-column-created = Создан
containerd-containers-action-start = Запустить
containerd-containers-action-stop = Остановить
containerd-containers-action-pause = Приостановить
containerd-containers-action-resume = Возобновить
containerd-containers-action-kill = Отправить сигнал
containerd-containers-actionOne-start = Запустить { $name }
containerd-containers-actionOne-stop = Остановить { $name }
containerd-containers-actionOne-pause = Приостановить { $name }
containerd-containers-actionOne-resume = Возобновить { $name }
containerd-containers-removeOne = Удалить { $name }
containerd-containers-done-start = { $name } запущен
containerd-containers-done-stop = { $name } остановлен
containerd-containers-done-pause = { $name } приостановлен
containerd-containers-done-resume = { $name } возобновлён
containerd-containers-done-kill = Сигнал { $signal } отправлен в { $name }
containerd-containers-removed = { $name } удалён
containerd-containers-kubelet = Этим контейнером управляет Kubernetes: kubelet запускает его снова после остановки и создаёт заново после удаления. Вместо этого измените его Pod в кластере.
containerd-containers-remove-title = Удалить { $name }?
containerd-containers-remove-running = Его задача работает: Homeport принудительно завершит её, затем удалит задачу и контейнер.
containerd-containers-remove-stopped = Homeport удалит его задачу и контейнер.
containerd-containers-remove-snapshot = Также удалить его снимок ({ $key }) вместе с изменёнными в нём файлами
containerd-containers-remove-killAndRemove = Завершить и удалить
containerd-containers-kill-title = Отправить сигнал в { $name }
containerd-containers-kill-description = Главному процессу его задачи, как это делает ctr task kill -s. Имя вроде SIGHUP или номер.
containerd-containers-kill-signal = Сигнал
containerd-containers-kill-send = Отправить
containerd-containers-backToList = Назад к контейнерам
containerd-containers-tab-overview = Обзор
containerd-containers-tab-logs = Журналы
containerd-containers-tab-exec = Exec
containerd-containers-tab-spec = Spec
containerd-logs-from = Из { $path }
containerd-logs-none-title = Вывод этого контейнера не записывается
containerd-logs-none-description = containerd не хранит журналы: куда пойдёт вывод задачи, решает инструмент, который её запускает. Контейнеры, созданные через ctr и большинство других клиентов, никуда его не отправляют; nerdctl и Kubernetes пишут его в файл, который читает Homeport.
containerd-logs-driver-title = nerdctl отправляет вывод этого контейнера в { $driver }
containerd-logs-driver-description = Homeport читает только журналы json-file, которые nerdctl использует по умолчанию. Читайте эти журналы там, где их хранит { $driver }, или запустите контейнер заново с драйвером json-file.
containerd-exec-startFirst = Запустите контейнер, чтобы открыть в нём терминал.
containerd-exec-command = Команда
containerd-exec-commandPlaceholder = Оболочка, если вы не укажете команду
containerd-exec-user = ID пользователя
containerd-exec-workingDir = Папка
containerd-exec-open = Открыть снова
containerd-exec-exited = Процесс завершился.
containerd-exec-exitedWithCode = Процесс завершился с кодом { $code }.
containerd-containers-field-id = ID
containerd-containers-field-namespace = Пространство имён
containerd-containers-field-image = Образ
containerd-containers-field-runtime = Среда выполнения
containerd-containers-field-snapshot = Снимок
containerd-containers-field-pid = PID
containerd-containers-field-exit = Последнее завершение
containerd-containers-field-pod = Pod Kubernetes
containerd-containers-field-created = Создан
containerd-containers-field-updated = Обновлён
containerd-containers-field-extensions = Расширения
containerd-containers-labels = Метки
containerd-containers-noSpec = У containerd нет спецификации среды выполнения для этого контейнера.
containerd-images-nav = Образы
containerd-images-title = Образы
containerd-images-summary = { $count ->
        [one] { NUM($count) } образ
        [few] { NUM($count) } образа
        [many] { NUM($count) } образов
       *[other] { NUM($count) } образа
    } в пространстве имён { $namespace }.
containerd-images-pull = Скачать образ
containerd-images-pulled = { $reference } скачан в { $namespace }
containerd-images-removed = { $name } удалён
containerd-images-search = Фильтр по имени
containerd-images-empty = В { $namespace } нет образов
containerd-images-emptyDescription = Скачайте образ: containerd получит его, а затем распакует для платформы этого хоста, чтобы его контейнеры могли работать.
containerd-images-column-name = Имя
containerd-images-column-platforms = Платформы
containerd-images-column-size = Размер
containerd-images-column-containers = Контейнеры
containerd-images-column-created = Создан
containerd-images-sizeUnknown = Его содержимое для платформы этого хоста есть здесь не полностью.
containerd-images-removeOne = Удалить { $name }
containerd-images-inUse = Из него созданы контейнеры этого пространства имён. Сначала удалите их.
containerd-images-remove-title = Удалить { $name }?
containerd-images-remove-description = containerd удалит имя и освободит его содержимое и снимки, как только их больше ничто не будет удерживать.
containerd-snapshots-nav = Снимки
containerd-snapshots-title = Снимки
containerd-snapshots-summary = В { $namespace }: { $count ->
        [one] { NUM($count) } снимок
        [few] { NUM($count) } снимка
        [many] { NUM($count) } снимков
       *[other] { NUM($count) } снимка
    }, всего { $total }; { $unused ->
        [0] неиспользуемых нет
        [one] не используется { NUM($unused) }
        [few] не используются { NUM($unused) }
        [many] не используются { NUM($unused) }
       *[other] не используется { NUM($unused) }
    } ({ $unusedSize }).
containerd-snapshots-snapshotter = Snapshotter
containerd-snapshots-unusedOnly = Только неиспользуемые
containerd-snapshots-unused = Не используется
containerd-snapshots-usedByContainer = контейнер { $id }
containerd-snapshots-usedByImage = образ { $name }
containerd-snapshots-empty = В { $snapshotter } нет снимков
containerd-snapshots-emptyDescription = Снимки — это слои распакованных образов и записываемые слои контейнеров.
containerd-snapshots-column-key = Ключ
containerd-snapshots-column-kind = Вид
containerd-snapshots-column-size = Размер
containerd-snapshots-column-usedBy = Используется
containerd-snapshots-kind-committed = Слой
containerd-snapshots-kind-active = Записываемый
containerd-snapshots-kind-view = Представление
containerd-snapshots-kind-unknown = Неизвестно
containerd-snapshots-removeOne = Удалить { $key }
containerd-snapshots-removed = Снимок { $key } удалён
containerd-snapshots-inUse = Его что-то использует или построено на нём. Сначала удалите контейнер или образ.
containerd-snapshots-remove-title = Удалить снимок { $key }?
containerd-snapshots-remove-description = Его ничто не использует. Его файлы ({ $size }) будут удалены с хоста.
containerd-namespaces-nav = Пространства имён
containerd-namespaces-title = Пространства имён
containerd-namespaces-summary = { $count ->
        [one] { NUM($count) } пространство имён
        [few] { NUM($count) } пространства имён
        [many] { NUM($count) } пространств имён
       *[other] { NUM($count) } пространства имён
    }. Каждый объект containerd находится в одном из них: nerdctl использует default, Kubernetes — k8s.io, Docker — moby.
containerd-namespaces-new = Новое пространство имён
containerd-namespaces-created = Пространство имён { $name } создано
containerd-namespaces-removed = Пространство имён { $name } удалено
containerd-namespaces-shown = Показано
containerd-namespaces-show = Показать
containerd-namespaces-removeOne = Удалить { $name }
containerd-namespaces-notEmpty = Удалить можно только пустые пространства имён. Сначала удалите его контейнеры и образы.
containerd-namespaces-remove-title = Удалить пространство имён { $name }?
containerd-namespaces-remove-description = containerd откажет, если в нём ещё есть снимки или содержимое, которых не видно в счётчиках.
containerd-namespaces-column-name = Имя
containerd-namespaces-column-containers = Контейнеры
containerd-namespaces-column-images = Образы
containerd-namespaces-column-labels = Метки
containerd-namespaces-empty = Пространств имён нет
containerd-namespaces-emptyDescription = containerd создаёт default, когда в нём впервые что-то создаётся.
containerd-namespaces-newDialog-title = Новое пространство имён
containerd-namespaces-newDialog-description = Пустое пространство имён, чтобы держать контейнеры и образы отдельно от остальных.
containerd-namespaces-newDialog-name = Имя
containerd-namespaces-newDialog-invalid = Используйте буквы и цифры, разделённые одиночными точками, дефисами или подчёркиваниями.
containerd-namespaces-newDialog-create = Создать
