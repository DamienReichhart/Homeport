### The `settings` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/settings.ts:
### change the text there, then run it again.

settings-nav = Настройки
settings-title = Настройки
settings-appearance-title = Внешний вид
settings-appearance-theme = Тема
settings-theme-system = Как в системе
settings-theme-light = Светлая
settings-theme-dark = Тёмная
settings-appearance-language = Язык
settings-language-system = Как в системе ({ $language })
settings-desktop-title = Рабочий стол
settings-desktop-description = Как Homeport работает на этом компьютере.
settings-desktop-closeToTray = Сворачивать в системный трей при закрытии
settings-desktop-closeToTrayHint = При закрытии окна Homeport и его подключения к хостам продолжают работать в трее. Выйти можно через значок в трее. В окружениях, которые скрывают значки трея, например GNOME без расширения, значок может не отображаться.
settings-desktop-noTray = В этом окружении нет системного трея, который Homeport может использовать, поэтому закрытие окна завершает работу приложения.
settings-desktop-startAtLogin = Запускать при входе в систему
settings-desktop-startAtLoginHint = Homeport запускается, когда вы входите в систему на этом компьютере, и сразу следит за вашими хостами.
settings-desktop-startHidden = Запускать скрытым в трее
settings-desktop-startHiddenHint = При запуске во время входа в систему Homeport остаётся в системном трее, не открывая окно.
settings-desktop-startHiddenNoTray = Нужен системный трей, из которого можно открыть окно, а в этом окружении его нет.
settings-notifications-title = Уведомления
settings-notifications-description = Homeport сообщает вам, даже когда его окно свёрнуто в трей, если:
settings-notifications-containerDied = Контейнер остановился с ошибкой или ему не хватило памяти
settings-notifications-containerUnhealthy = Контейнер стал нездоровым
settings-notifications-hostUnreachable = Подключённый хост стал недоступен
settings-notifications-hostBack = Недоступный хост снова на связи
settings-machine-title = Этот компьютер
settings-machine-description = Что Homeport может здесь использовать.
settings-machine-keychain = Связка ключей ОС
settings-machine-keychainAvailable = Доступна — SSH-ключи, сертификаты, окружения стеков и входы в реестры хранятся в зашифрованном виде.
settings-machine-keychainUnavailable = Недоступна — SSH-ключи, TLS-сертификаты, окружения стеков и входы в реестры сохранить нельзя.
settings-machine-keychainEntryMissing = Запись отсутствует — в связке ключей нет записи для ключа, которым запечатаны секреты Homeport, поэтому их нельзя ни открыть, ни сохранить.
settings-machine-startOverDescription = Если Homeport использует не ту связку ключей, что раньше, вернитесь к прежней и перезапустите Homeport. Если запись пропала навсегда, начните заново с новым ключом.
settings-machine-startOver = Начать заново с новым ключом
settings-machine-startOverTitle = Начать заново с новым ключом?
settings-machine-startOverConfirmDescription = Новый ключ не откроет то, что запечатано отсутствующим: придётся заново ввести учётные данные каждого хоста, кластера, реестра и репозитория и окружение каждого стека или восстановить резервную копию.
settings-machine-compose = Compose CLI
settings-machine-composeMissing = Стеки можно запускать и останавливать, но не развёртывать.
settings-machine-stackDeploy = Docker CLI, для Swarm
settings-machine-stackDeployMissing = Развёртывать стеки в Swarm нельзя; всё остальное для Swarm работает.
settings-machine-git = git
settings-machine-helm = Helm CLI
settings-sync-title = Синхронизируемые стеки
settings-sync-interval = Проверять репозитории каждые
settings-sync-minutes = мин
settings-data-title = Данные
settings-data-description = Что Homeport хранит на этом компьютере.
settings-data-backup = Создать резервную копию…
settings-data-restore = Восстановить…
settings-data-backupHint = Сохраните все хосты, стеки, входы в реестры, репозитории и настройки вместе с их секретами в один файл, защищённый парольной фразой, и восстановите его на этом или другом компьютере.
settings-backup-title = Резервная копия данных Homeport
settings-backup-description = Файл содержит все хосты, кластеры, стеки, входы в реестры, репозитории и настройки, а также их ключи, сертификаты, токены, пароли и окружения. Любой, у кого есть файл и его парольная фраза, может ими воспользоваться, поэтому храните и то и другое надёжно. События движков и история развёртываний не включаются.
settings-backup-passphrase = Парольная фраза
settings-backup-passphraseHint = Не менее { $min } символов. Восстановить её нельзя.
settings-backup-confirmation = Парольная фраза ещё раз
settings-backup-problem-short = Используйте не менее { $min } символов.
settings-backup-problem-mismatch = Парольные фразы не совпадают.
settings-backup-start = Создать резервную копию…
settings-backup-saveTitle = Сохранить резервную копию как
settings-backup-fileType = Резервная копия Homeport
settings-backup-done = В резервную копию сохранено: { $hosts ->
        [one] { NUM($hosts) } хост
        [few] { NUM($hosts) } хоста
        [many] { NUM($hosts) } хостов
       *[other] { NUM($hosts) } хоста
    } и { $stacks ->
        [one] { NUM($stacks) } стек
        [few] { NUM($stacks) } стека
        [many] { NUM($stacks) } стеков
       *[other] { NUM($stacks) } стека
    }
settings-backup-withoutSecrets = Эти объекты сохранены без своих секретов, которые не удалось открыть:
settings-restore-pick = Восстановить из
settings-restore-title = Восстановление данных Homeport
settings-restore-from = Из { $path }
settings-restore-open = Открыть
settings-restore-madeBy = Резервная копия создана Homeport { $version } { $date }. В ней:
settings-restore-holds = Что содержит резервная копия
settings-restore-hosts = { $count ->
        [one] { NUM($count) } хост
        [few] { NUM($count) } хоста
        [many] { NUM($count) } хостов
       *[other] { NUM($count) } хоста
    }
settings-restore-clusters = { $count ->
        [one] { NUM($count) } кластер
        [few] { NUM($count) } кластера
        [many] { NUM($count) } кластеров
       *[other] { NUM($count) } кластера
    }
settings-restore-stacks = { $count ->
        [one] { NUM($count) } стек
        [few] { NUM($count) } стека
        [many] { NUM($count) } стеков
       *[other] { NUM($count) } стека
    }
settings-restore-registries = { $count ->
        [one] { NUM($count) } вход в реестр
        [few] { NUM($count) } входа в реестры
        [many] { NUM($count) } входов в реестры
       *[other] { NUM($count) } входа в реестры
    }
settings-restore-repositories = { $count ->
        [one] { NUM($count) } репозиторий
        [few] { NUM($count) } репозитория
        [many] { NUM($count) } репозиториев
       *[other] { NUM($count) } репозитория
    }
settings-restore-settings = Ваши настройки
settings-restore-withoutSecrets = Эти элементы сохранены без секретов, которые тогда не удалось открыть. Они вернутся без них, заменив секреты, которые есть у них здесь, а входы в реестры среди них не вернутся вовсе:
settings-restore-withoutSecretsList = Сохранены без секретов
settings-restore-replaces = Восстановление заменяет все хосты, стеки, входы в реестры, репозитории и настройки, которые у вас есть сейчас, и забывает события движков и историю развёртываний. Сначала копия текущих данных сохраняется в папке данных Homeport.
settings-restore-confirm = Заменить всё
settings-restore-done = Восстановлено: { $hosts ->
        [one] { NUM($hosts) } хост
        [few] { NUM($hosts) } хоста
        [many] { NUM($hosts) } хостов
       *[other] { NUM($hosts) } хоста
    } и { $stacks ->
        [one] { NUM($stacks) } стек
        [few] { NUM($stacks) } стека
        [many] { NUM($stacks) } стеков
       *[other] { NUM($stacks) } стека
    }
settings-data-keepEvents = Хранить события движка
settings-data-days = дн.
settings-data-keepEventsHint = Каждый хост хранит не более 10 000 событий для страницы «События». Более старые удаляются каждый час.
settings-machine-gitMissing = Он нужен синхронизируемым стекам; все остальные виды стеков работают без него.
settings-machine-helmMissing = Релизы Helm нельзя просматривать и устанавливать; всё остальное для кластеров работает.
settings-about-tagline = Все Docker Engine на одной доске.
settings-about-openLogFolder = Открыть папку журналов
settings-about-licenses = Лицензии
settings-about-exportDiagnostics = Экспортировать диагностику…
settings-diagnostics-title = Экспорт диагностики
settings-diagnostics-description = Zip-файл для отчёта об ошибке: сводка версий и типов хостов и кластеров, которые вы используете, и журналы Homeport без секретов. Ничего никуда не отправляется: делиться ли файлом, решаете вы.
settings-diagnostics-includeNames = Включить имена и адреса
settings-diagnostics-includeNamesHelp = Если выключено, хосты и кластеры обозначаются в сводке и журналах как host-1, cluster-1…
settings-diagnostics-files = Файлы в архиве
settings-diagnostics-save = Сохранить…
settings-diagnostics-saveTitle = Сохранить диагностику
settings-diagnostics-fileType = Zip-архив
settings-diagnostics-done = Диагностика сохранена в { $path }.
settings-licenses-title = Лицензии
settings-licenses-description = Homeport — свободное программное обеспечение под лицензией GNU Affero General Public License версии 3, построенное на перечисленных ниже пакетах с открытым исходным кодом.
settings-licenses-source = Исходный код этой версии опубликован под той же лицензией.
settings-licenses-sourceLink = Исходный код v{ $version }
settings-licenses-thirdParty = Сторонние программы
settings-licenses-missing-title = Не включено в эту сборку
settings-licenses-missing-description = В релизные сборки они входят; для сборки разработки сгенерируйте их командой <code>{ $command }</code>.
settings-palette-group = Настройки
settings-palette-themeLight = Тема: светлая
settings-palette-themeDark = Тема: тёмная
settings-palette-themeSystem = Тема: как в системе
