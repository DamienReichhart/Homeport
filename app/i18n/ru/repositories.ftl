### The `repositories` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/repositories.ts:
### change the text there, then run it again.

repositories-title = Репозитории
repositories-description = Git-репозитории, из которых развёртываются синхронизируемые стеки, на любом хосте. Homeport их только читает.
repositories-add = Добавить репозиторий
repositories-empty = Репозиториев пока нет. Добавьте репозиторий, чтобы создать синхронизируемый стек.
repositories-manageOne = Управлять доступом к { $repository }
repositories-removeOne = Удалить { $repository }
repositories-remove-title = Удалить { $repository }?
repositories-remove-description = Homeport забудет его ключ развёртывания или токен и удалит его клон на этом компьютере. Сам репозиторий не затрагивается. Удалите ключ развёртывания и из него на своём git-хостинге.
repositories-noKeychain = Ключи развёртывания и токены шифруются ключом, который хранится в связке ключей ОС, а она недоступна, поэтому можно добавлять только публичные репозитории.
repositories-public = Публичный репозиторий: Homeport читает его без учётных данных.
repositories-added = { $repository } добавлен
repositories-removed = { $repository } удалён
repositories-keyRotated = Создан новый ключ развёртывания. Добавьте его в репозиторий вместо старого.
repositories-tokenReplaced = Токен заменён
repositories-hostKeyReset = Homeport доверится ключу, который сервер предъявит в следующий раз.
repositories-auth-withKey = С ключом развёртывания, на { $host }
repositories-auth-withToken = Как { $username } с токеном, на { $host }
repositories-auth-public = Публичный, на { $host }
repositories-dialog-addTitle = Добавить репозиторий
repositories-dialog-description = Homeport клонирует его на этот компьютер, чтобы развёртывать из него синхронизируемые стеки. Он никогда не делает коммиты и не отправляет изменения.
repositories-dialog-url = URL репозитория
repositories-dialog-auth = Доступ
repositories-dialog-authKey = Ключ развёртывания
repositories-dialog-authToken = Токен
repositories-dialog-authNone = Публичный
repositories-dialog-authKeyHelp = Homeport создаёт ключ только для этого репозитория. Вы добавляете его в репозиторий с доступом на чтение; ваши собственные SSH-ключи никогда не используются.
repositories-dialog-authTokenHelp = Токен доступа, которому разрешено читать содержимое репозитория; передаётся по HTTPS. Шифруется на этом компьютере и больше не показывается.
repositories-dialog-authNoneHelp = Для публичного репозитория по HTTPS. Ничего не сохраняется.
repositories-dialog-username = Имя пользователя
repositories-dialog-token = Токен доступа
repositories-dialog-add = Добавить
repositories-dialog-addWithKey = Добавить и создать ключ развёртывания
repositories-missing-url = Введите URL репозитория.
repositories-missing-validUrl = Введите URL SSH или HTTPS, например git@github.com:owner/repo.git или https://gitlab.com/owner/repo.git.
repositories-missing-auth = Выберите, как Homeport будет обращаться к этому репозиторию.
repositories-missing-username = Введите имя пользователя, которому принадлежит токен.
repositories-missing-token = Введите токен доступа.
repositories-deployKey-title = Ключ развёртывания
repositories-deployKey-github = GitHub: в репозитории <strong>Settings → Deploy keys → Add deploy key</strong>.
repositories-deployKey-gitlab = GitLab: в проекте <strong>Settings → Repository → Deploy keys</strong>.
repositories-deployKey-gitea = Gitea и Forgejo: в репозитории <strong>Settings → Deploy keys → Add deploy key</strong>.
repositories-deployKey-readOnly = Не включайте доступ на запись: Homeport только читает. Затем проверьте доступ ниже.
repositories-deployKey-fingerprint = Отпечаток: <mono>{ $fingerprint }</mono>
repositories-rotate-button = Новый ключ
repositories-rotate-title = Создать новый ключ развёртывания?
repositories-rotate-description = Текущий ключ перестанет работать для Homeport. Добавьте новый ключ в репозиторий, а старый удалите оттуда.
repositories-rotate-confirm = Создать новый ключ
repositories-token-replace = Заменить токен
repositories-token-save = Сохранить токен
repositories-hostKey-trusted = Доверенный ключ { $host }: <mono>{ $fingerprint }</mono>
repositories-hostKey-reset = Сбросить
repositories-hostKey-resetTitle = Сбросить доверенный ключ { $host }?
repositories-hostKey-resetDescription = Делайте это, только если { $host } сменил свой SSH-ключ и вы проверили новый отпечаток на его сайте. Homeport доверится любому ключу, который тот предъявит следующим.
repositories-test-button = Проверить доступ
repositories-test-testing = Проверка…
repositories-test-success = Homeport может его читать: { $branches ->
        [one] { NUM($branches) } ветка
        [few] { NUM($branches) } ветки
        [many] { NUM($branches) } веток
       *[other] { NUM($branches) } ветки
    } и { $tags ->
        [one] { NUM($tags) } тег
        [few] { NUM($tags) } тега
        [many] { NUM($tags) } тегов
       *[other] { NUM($tags) } тега
    }.
