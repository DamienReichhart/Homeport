### The `incus` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/incus.ts:
### change the text there, then run it again.

incus-delete = Удалить
incus-status-running = Работает
incus-status-stopped = Остановлен
incus-status-frozen = Заморожен
incus-status-starting = Запускается
incus-status-stopping = Останавливается
incus-status-freezing = Замораживается
incus-status-error = Ошибка
incus-status-unknown = Неизвестно
incus-type-container = Контейнер
incus-type-virtualMachine = Виртуальная машина
incus-action-start = Запустить
incus-action-stop = Остановить
incus-action-restart = Перезапустить
incus-action-freeze = Заморозить
incus-action-unfreeze = Разморозить
incus-action-forceStop = Остановить принудительно
incus-actionOne-start = Запустить { $name }
incus-actionOne-stop = Остановить { $name }
incus-actionOne-restart = Перезапустить { $name }
incus-actionOne-freeze = Заморозить { $name }
incus-actionOne-unfreeze = Разморозить { $name }
incus-instances-nav = Экземпляры
incus-instances-title = Экземпляры
incus-instances-summary = { $count ->
        [one] { NUM($count) } экземпляр
        [few] { NUM($count) } экземпляра
        [many] { NUM($count) } экземпляров
       *[other] { NUM($count) } экземпляра
    }, работает: { $running }, в проекте { $project }.
incus-instances-search = Фильтр по имени или описанию
incus-instances-filter-all = Все
incus-instances-empty = В { $project } нет экземпляров
incus-instances-emptyDescription = Здесь показываются системные контейнеры и виртуальные машины этого проекта. Чтобы увидеть другие, выберите другой проект под переключателем хостов.
incus-instances-more = Другие действия для { $name }
incus-instances-column-name = Имя
incus-instances-column-type = Тип
incus-instances-column-status = Состояние
incus-instances-column-image = Образ
incus-instances-column-addresses = Адреса
incus-instances-column-memory = Память
incus-instances-column-created = Создан
incus-instances-done-start = { $name } запущен
incus-instances-done-stop = { $name } остановлен
incus-instances-done-restart = { $name } перезапущен
incus-instances-done-freeze = { $name } заморожен
incus-instances-done-unfreeze = { $name } разморожен
incus-instances-deleted = { $name } удалён
incus-instances-delete-title = Удалить { $name }?
incus-instances-delete-description = Экземпляр и его диск будут удалены из проекта { $project }. Это нельзя отменить.
incus-instances-delete-snapshots = { $count ->
        [1] Его снимок удаляется
        [one] Его { NUM($count) } снимок удаляется
        [few] Его { NUM($count) } снимка удаляются
        [many] Его { NUM($count) } снимков удаляются
       *[other] Его { NUM($count) } снимка удаляются
    } вместе с ним.
incus-instances-delete-snapshotsNamed = { $count ->
        [1] Его снимок { $names } удаляется
        [one] Его снимки { $names } удаляются
        [few] Его снимки { $names } удаляются
        [many] Его снимки { $names } удаляются
       *[other] Его снимки { $names } удаляются
    } вместе с ним.
incus-instances-delete-stopFirst = Сначала остановите его: он работает
incus-instances-delete-stopAndDelete = Остановить и удалить
incus-instances-backToList = Назад к экземплярам
incus-instances-tab-overview = Обзор
incus-instances-tab-console = Консоль
incus-instances-tab-exec = Exec
incus-instances-tab-snapshots = Снимки
incus-instances-tab-config = Конфигурация
incus-instances-tab-devices = Устройства
incus-instances-tab-raw = Исходные данные
incus-instances-field-project = Проект
incus-instances-field-architecture = Архитектура
incus-instances-field-addresses = Адреса
incus-instances-field-cpu = Время ЦП
incus-instances-field-memory = Память
incus-instances-field-processes = Процессы
incus-instances-field-created = Создан
incus-instances-field-lastUsed = Последнее использование
incus-instances-field-snapshots = Снимки
incus-instances-field-location = Член кластера
incus-instances-field-description = Описание
incus-instances-cpuSeconds = { $seconds } с
incus-instances-profiles = Профили
incus-instances-noConfig = Конфигурации нет.
incus-instances-noDevices = Устройств нет.
incus-instances-config-key = Ключ
incus-instances-config-value = Значение
incus-instances-config-fromProfile = Из профиля
incus-progress-waiting = Ожидание Incus…
incus-remote-server = Сервер образов
incus-remote-search = Фильтр по системе, выпуску или псевдониму
incus-remote-list = Образы на сервере
incus-remote-none = Подходящих образов нет.
incus-remote-alias = Псевдоним
incus-images-nav = Образы
incus-images-title = Образы
incus-images-summary = { $count ->
        [one] { NUM($count) } образ
        [few] { NUM($count) } образа
        [many] { NUM($count) } образов
       *[other] { NUM($count) } образа
    } в проекте { $project }.
incus-images-search = Фильтр по псевдониму, отпечатку или описанию
incus-images-empty = В { $project } нет образов
incus-images-emptyDescription = Образы скачиваются, когда из них создаётся экземпляр, или заранее через «Получить образ».
incus-images-get = Получить образ
incus-images-get-title = Получить образ
incus-images-get-description = Incus скачает его с сервера образов в проект { $project }, чтобы экземпляры быстро из него запускались.
incus-images-get-autoUpdate = Обновлять его с сервера
incus-images-get-keepAlias = Дать ему здесь тот же псевдоним
incus-images-get-confirm = Получить образ
incus-images-get-done = Образ готов.
incus-images-copied = { $alias } скопирован в { $project }
incus-images-column-aliases = Псевдонимы
incus-images-column-description = Описание
incus-images-column-type = Тип
incus-images-column-arch = Архитектура
incus-images-column-size = Размер
incus-images-column-uploaded = Добавлен
incus-images-cached = в кеше
incus-images-cachedHint = Скачан для создания экземпляра; Incus удалит его, когда его какое-то время ничто не будет использовать.
incus-images-aliasAdd = Добавить псевдоним для { $name }
incus-images-aliasRemove = Удалить псевдоним { $name }
incus-images-aliasRemoved = Псевдоним { $name } удалён
incus-images-alias-title = Добавить псевдоним для { $name }
incus-images-alias-name = Псевдоним
incus-images-alias-add = Добавить псевдоним
incus-images-refreshOne = Обновить { $name } с его сервера
incus-images-refreshed = { $name } обновлён
incus-images-deleteOne = Удалить { $name }
incus-images-deleted = { $name } удалён
incus-images-delete-title = Удалить образ { $name }?
incus-images-delete-description = Созданные из него экземпляры продолжат работать: у каждого своя копия. Для новых его придётся скачать снова.
incus-profiles-nav = Профили
incus-profiles-title = Профили
incus-profiles-summary = { $count ->
        [one] { NUM($count) } профиль
        [few] { NUM($count) } профиля
        [many] { NUM($count) } профилей
       *[other] { NUM($count) } профиля
    } в проекте { $project }.
incus-profiles-search = Фильтр по имени или описанию
incus-profiles-empty = В { $project } нет профилей
incus-profiles-new = Новый профиль
incus-profiles-new-title = Новый профиль
incus-profiles-new-description = Пустой профиль в проекте { $project }. Добавьте конфигурацию и устройства после его создания.
incus-profiles-new-name = Имя
incus-profiles-new-descriptionField = Описание
incus-profiles-created = Профиль { $name } создан
incus-profiles-column-name = Имя
incus-profiles-column-description = Описание
incus-profiles-column-settings = Параметры
incus-profiles-column-usedBy = Используется
incus-profiles-settings = { $config ->
        [one] { NUM($config) } ключ
        [few] { NUM($config) } ключа
        [many] { NUM($config) } ключей
       *[other] { NUM($config) } ключа
    }, { $devices ->
        [one] { NUM($devices) } устройство
        [few] { NUM($devices) } устройства
        [many] { NUM($devices) } устройств
       *[other] { NUM($devices) } устройства
    }
incus-profiles-defaultKept = Профиль default Incus сохраняет всегда.
incus-profiles-inUse = Используется в { $names }: сначала уберите его оттуда.
incus-profiles-deleteOne = Удалить { $name }
incus-profiles-deleted = Профиль { $name } удалён
incus-profiles-delete-title = Удалить профиль { $name }?
incus-profiles-delete-description = Он будет удалён из проекта { $project }. Ни один экземпляр его не использует.
incus-profiles-backToList = Назад к профилям
incus-profiles-noDescription = Без описания
incus-profiles-reload = Перезагрузить
incus-profiles-saved = Профиль { $name } сохранён
incus-profiles-usedBy = Используется
incus-profiles-editHint = Его описание, конфигурация и устройства в том виде, в каком их показывает incus profile edit. При сохранении они применяются к использующим его экземплярам, как это делает Incus; если кто-то изменил его после загрузки, Homeport откажется сохранять, а не перезапишет.
incus-profiles-yamlOf = YAML { $name }
incus-new-action = Новый экземпляр
incus-new-title = Новый экземпляр
incus-new-description = Системный контейнер или виртуальная машина в проекте { $project }, из образа.
incus-new-name = Имя
incus-new-image = Образ
incus-new-origin-remote = С сервера образов
incus-new-origin-local = Из образов этого хоста
incus-new-localImage = Образ на этом хосте
incus-new-localImagePlaceholder = Выберите образ
incus-new-profiles = Профили
incus-new-cpu = ЦП
incus-new-memory = Память
incus-new-disk = Корневой диск
incus-new-limitsHint = Необязательно: если оставить пустым, решают профили. Размеры записываются как 512MiB или 20GiB.
incus-new-start = Запустить после создания
incus-new-confirm = Создать
incus-new-open = Открыть
incus-new-made = { $name } создан
incus-new-done = { $name } готов.
incus-new-problem-name = Используйте в имени буквы, цифры и дефисы, начните с буквы, не более 63 символов.
incus-new-problem-image = Выберите образ или введите его псевдоним.
incus-new-problem-cpu = Число ЦП — целое число от 1.
incus-new-problem-memory = Память записывается как 512MiB или 2GiB.
incus-new-problem-disk = Корневой диск записывается как 20GiB.
incus-exec-startFirst = Запустите экземпляр, чтобы выполнять в нём команды.
incus-exec-command = Команда
incus-exec-commandPlaceholder = Оболочка (bash, иначе sh)
incus-exec-user = ID пользователя
incus-exec-cwd = Папка
incus-exec-open = Открыть
incus-exec-exited = Команда завершилась.
incus-exec-exitedWithCode = Команда завершилась с кодом { $code }.
incus-console-startFirst = Запустите экземпляр, чтобы подключиться к его консоли.
incus-console-hint = Его текстовая консоль, как её показывает incus console: приглашение входа или загрузка в реальном времени. Уход с вкладки отключает консоль, ничего не останавливая.
incus-console-reattach = Подключиться снова
incus-console-detached = Отключено от консоли.
incus-snapshots-take = Сделать снимок
incus-snapshots-take-title = Сделать снимок { $name }
incus-snapshots-take-description = Копия его диска в текущем виде, чтобы потом к ней вернуться: перед обновлением или рискованным изменением.
incus-snapshots-take-name = Имя
incus-snapshots-take-namePlaceholder = snap0, snap1…, если оставить пустым
incus-snapshots-take-expiry = Хранить
incus-snapshots-take-stateful = Включить состояние выполнения (память)
incus-snapshots-expiry-never = До удаления
incus-snapshots-expiry-1d = Один день
incus-snapshots-expiry-7d = Одну неделю
incus-snapshots-expiry-30d = 30 дней
incus-snapshots-taken = Снимок { $name } сделан
incus-snapshots-empty = Снимков нет
incus-snapshots-emptyDescription = Сделайте снимок, прежде чем менять то, что, возможно, захотите отменить.
incus-snapshots-column-name = Имя
incus-snapshots-column-created = Сделан
incus-snapshots-column-expires = Истекает
incus-snapshots-column-size = Размер
incus-snapshots-stateful = с состоянием
incus-snapshots-restore = Восстановить
incus-snapshots-restore-title = Восстановить { $name } из { $snapshot }?
incus-snapshots-restore-description = Всё, что изменилось на его диске с { $time }, когда был сделан { $snapshot }, будет потеряно. Работающий экземпляр перезапустится.
incus-snapshots-restored = { $name } восстановлен из { $snapshot }
incus-snapshots-deleteOne = Удалить { $snapshot }
incus-snapshots-delete-title = Удалить снимок { $snapshot }?
incus-snapshots-delete-description = Экземпляр остаётся как есть; восстановиться из этого снимка больше будет нельзя.
incus-snapshots-deleted = Снимок { $snapshot } удалён
incus-storage-nav = Хранилище
incus-storage-title = Хранилище
incus-storage-summary = { $count ->
        [one] { NUM($count) } пул хранения
        [few] { NUM($count) } пула хранения
        [many] { NUM($count) } пулов хранения
       *[other] { NUM($count) } пула хранения
    }; тома проекта { $project }.
incus-storage-empty = Пулов хранения нет
incus-storage-emptyDescription = Создайте пул командой incus storage create или incus admin init.
incus-storage-usage = Занятое место в { $pool }
incus-storage-space = Использовано { $used } из { $total } ({ $percent })
incus-storage-spaceUnknown = Его драйвер не сообщает, насколько он заполнен.
incus-storage-volumesOf = Тома в { $pool }
incus-storage-noVolumes = В пуле нет томов этого проекта.
incus-storage-new = Новый том
incus-storage-new-title = Новый том в { $pool }
incus-storage-new-description = Пользовательский том проекта { $project }, который подключается к экземплярам для данных, переживающих их.
incus-storage-new-name = Имя
incus-storage-new-size = Размер
incus-storage-new-content = Содержимое
incus-storage-content-filesystem = Файловая система
incus-storage-content-block = Блочное устройство
incus-storage-created = Том { $name } создан
incus-storage-column-name = Имя
incus-storage-column-type = Тип
incus-storage-column-content = Содержимое
incus-storage-column-size = Размер
incus-storage-column-usedBy = Используется
incus-storage-inUse = Используется в { $names }: сначала отключите его.
incus-storage-deleteOne = Удалить { $name }
incus-storage-delete-title = Удалить том { $name }?
incus-storage-delete-description = Его данные будут удалены из { $pool }. Ничто его не использует.
incus-storage-deleted = Том { $name } удалён
incus-networks-nav = Сети
incus-networks-title = Сети
incus-networks-summary = Проект { $project } видит { $count ->
        [one] { NUM($count) } сеть
        [few] { NUM($count) } сети
        [many] { NUM($count) } сетей
       *[other] { NUM($count) } сети
    }.
incus-networks-search = Фильтр по имени или типу
incus-networks-empty = Сетей нет
incus-networks-new = Новый мост
incus-networks-new-title = Новый мост
incus-networks-new-description = Управляемый мост: Incus даёт ему адреса, DHCP и DNS, а экземпляры подключаются к нему через профиль или устройство.
incus-networks-new-name = Имя
incus-networks-new-ipv4 = IPv4
incus-networks-new-ipv6 = IPv6
incus-networks-new-addressHint = auto выбирает свободную подсеть, none отключает; либо укажите адрес моста с префиксом, например 10.0.5.1/24.
incus-networks-new-nat = Маскировать исходящий трафик IPv4 (NAT)
incus-networks-created = Мост { $name } создан
incus-networks-column-name = Имя
incus-networks-column-type = Тип
incus-networks-column-ipv4 = IPv4
incus-networks-column-ipv6 = IPv6
incus-networks-column-usedBy = Используется
incus-networks-managed = Управляется Incus
incus-networks-unmanaged = Этого компьютера
incus-networks-inUse = Используется в { $names }: сначала уберите её оттуда.
incus-networks-deleteOne = Удалить { $name }
incus-networks-delete-title = Удалить сеть { $name }?
incus-networks-delete-description = Incus удалит мост вместе с его DHCP и DNS. Ничто его не использует.
incus-networks-deleted = Сеть { $name } удалена
incus-networks-gone = Сети { $name } больше нет.
incus-networks-backToList = Назад к сетям
incus-networks-field-managed = Создана
incus-networks-field-state = Состояние
incus-networks-field-addresses = Адреса на хосте
incus-networks-field-ipv4 = Параметр IPv4
incus-networks-field-ipv6 = Параметр IPv6
incus-networks-field-nat = NAT
incus-networks-field-mtu = MTU
incus-networks-field-hwaddr = MAC-адрес
incus-networks-yes = Да
incus-networks-no = Нет
incus-networks-usedBy = Используется
incus-networks-leases = Аренды DHCP
incus-networks-noLeases = Аренд нет.
incus-networks-lease-hostname = Имя хоста
incus-networks-lease-address = Адрес
incus-networks-lease-hwaddr = MAC-адрес
incus-networks-lease-type = Тип
