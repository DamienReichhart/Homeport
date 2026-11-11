### The `volumes` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/volumes.ts:
### change the text there, then run it again.

volumes-nav = Тома
volumes-title = Тома
volumes-summary = { $count ->
        [one] { NUM($count) } том
        [few] { NUM($count) } тома
        [many] { NUM($count) } томов
       *[other] { NUM($count) } тома
    } · не используются: { $unused }
volumes-cleanUp = Очистить
volumes-cleanUp-anonymous = Неиспользуемые анонимные тома
volumes-cleanUp-all = Все неиспользуемые тома
volumes-create = Создать том
volumes-search = Поиск томов
volumes-empty-title = Томов нет
volumes-empty-description = Тома сохраняют данные контейнеров между перезапусками и пересозданиями.
volumes-column-name = Имя
volumes-column-stack = Стек
volumes-column-driver = Драйвер
volumes-column-size = Размер
volumes-size-label = Размер:
volumes-size-measuring = Измерение
volumes-size-unknown = Драйвер этого тома не сообщает его размер.
volumes-size-refresh = Обновить размеры
volumes-column-usedBy = Используется
volumes-column-created = Создан
volumes-unused = Не используется
volumes-inspectOne = Просмотреть { $name }
volumes-deleteOne = Удалить { $name }
volumes-browseOne = Просмотреть файлы { $name }
volumes-browse-title = Файлы { $name }
volumes-browse-description = Только чтение: ничто здесь не может изменить том, даже пока его используют контейнеры.
volumes-browse-saveOne = Сохранить { $name }
volumes-browse-saved = { $name } сохранён
volumes-browse-savedArchive = { $name } сохранён как архив .tar
volumes-helper-needed = Homeport открывает тома с помощью кратковременного вспомогательного контейнера, которому нужен образ { $image } (около { $size }). На этом хосте его нет.
volumes-helper-removed = Homeport скачает его, только если вы согласитесь, и удалит после завершения, если его не использует что-то ещё.
volumes-helper-pull = Скачать { $image }
volumes-helper-pulling = Скачивание { $image }…
volumes-helper-progress = Ход скачивания
volumes-moreActions = Другие действия для { $name }
volumes-backupMenu = Создать резервную копию в файл…
volumes-restoreMenu = Восстановить из файла…
volumes-cloneMenu = Клонировать…
volumes-clone-title = Клонировать { $volume }
volumes-clone-description = Копирует его файлы с владельцами и правами в новый том на этом хосте. Копия не принадлежит ни одному стеку.
volumes-clone-name = Имя копии
volumes-clone-start = Клонировать
volumes-clone-into = В { $target }
volumes-clone-copying = Копирование…
volumes-clone-done = { $source } клонирован в { $target }
volumes-restoreFromFile = Восстановить из файла
volumes-backup-title = Резервная копия { $volume }
volumes-backup-saveAs = Сохранить резервную копию { $volume } в
volumes-backup-to = В { $path }
volumes-backup-done = Резервная копия { $volume } сохранена в { $path }
volumes-restore-pick = Восстановить том из
volumes-restore-title = Восстановить том
volumes-restore-titleInto = Восстановить { $volume }
volumes-restore-from = Из { $path }
volumes-restore-into = Восстановить в
volumes-restore-newVolume = Новый том
volumes-restore-existingVolume = Существующий том
volumes-restore-volume = Том
volumes-restore-chooseVolume = Выберите том
volumes-restore-nameTaken = Том с именем { $volume } уже существует.
volumes-restore-start = Восстановить
volumes-restore-done = { $volume } восстановлен
volumes-restore-cancelled = Восстановление { $volume } отменено; том может быть записан частично.
volumes-transfer-written = Записано
volumes-transfer-sent = Отправлено
volumes-transfer-archives = Архивы tar
volumes-flow-running-backup = { $containers } { $count ->
        [1] работает и использует
        [one] работают и используют
        [few] работают и используют
        [many] работают и используют
       *[other] работают и используют
    } { $volume }. Резервная копия, снятая во время записи в том, может оказаться несогласованной — например, база данных посреди записи.
volumes-flow-running-restore = { $containers } { $count ->
        [1] работает и использует
        [one] работают и используют
        [few] работают и используют
        [many] работают и используют
       *[other] работают и используют
    } { $volume }. Homeport { $count ->
        [1] остановит его
        [one] остановит их
        [few] остановит их
        [many] остановит их
       *[other] остановит их
    } на время восстановления и затем { $count ->
        [1] снова запустит его
        [one] снова запустит их
        [few] снова запустит их
        [many] снова запустит их
       *[other] снова запустит их
    }.
volumes-flow-running-clone = { $containers } { $count ->
        [1] работает и использует
        [one] работают и используют
        [few] работают и используют
        [many] работают и используют
       *[other] работают и используют
    } { $volume }. Копия, снятая во время записи в том, может оказаться несогласованной — например, база данных посреди записи.
volumes-flow-anyway-backup = Всё равно создать копию
volumes-flow-anyway-clone = Всё равно копировать
volumes-flow-stopDuring-backup = { $count ->
        [1] Остановить его
        [one] Остановить их
        [few] Остановить их
        [many] Остановить их
       *[other] Остановить их
    } на время резервного копирования
volumes-flow-stopDuring-restore = Остановить и восстановить
volumes-flow-stopDuring-clone = { $count ->
        [1] Остановить его
        [one] Остановить их
        [few] Остановить их
        [many] Остановить их
       *[other] Остановить их
    } на время копирования
volumes-flow-replace = В { $volume } уже есть файлы. Восстановление удалит их и поместит на их место файлы из резервной копии.
volumes-flow-replaceConfirm = Заменить содержимое
volumes-usedBy = Используется: { $containers }.
volumes-notUsed = Ни один контейнер не использует этот том.
volumes-createDialog-title = Создать том
volumes-createDialog-name = Имя
volumes-createDialog-driver = Драйвер
volumes-deleted = { $name } удалён
volumes-pruned = Удалено: { $count ->
        [one] { NUM($count) } том
        [few] { NUM($count) } тома
        [many] { NUM($count) } томов
       *[other] { NUM($count) } тома
    }, освобождено { $space }
volumes-remove-title = Удалить том { $name }?
volumes-remove-warning = <strong>Все данные в этом томе будут удалены безвозвратно.</strong>
volumes-remove-mounted = {" "}Он смонтирован в { $containers }; сначала удалите эти контейнеры.
volumes-remove-confirm = Удалить том и данные
volumes-prune-allTitle = Удалить все неиспользуемые тома?
volumes-prune-anonymousTitle = Удалить неиспользуемые анонимные тома?
volumes-prune-allWarning = Данные во всех томах, которые не использует ни один контейнер, будут удалены безвозвратно, включая именованные тома.
volumes-prune-anonymousWarning = Данные в безымянных томах, которые не использует ни один контейнер, будут удалены безвозвратно.
volumes-prune-confirm = Удалить тома и данные
