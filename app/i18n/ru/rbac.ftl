### The `rbac` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/rbac.ts:
### change the text there, then run it again.

rbac-nav = RBAC
rbac-title = RBAC
rbac-summary = { $roles ->
        [one] { NUM($roles) } роль
        [few] { NUM($roles) } роли
        [many] { NUM($roles) } ролей
       *[other] { NUM($roles) } роли
    }, { $bindings ->
        [one] { NUM($bindings) } привязка
        [few] { NUM($bindings) } привязки
        [many] { NUM($bindings) } привязок
       *[other] { NUM($bindings) } привязки
    }
rbac-refresh = Обновить
rbac-tab-roles = Roles
rbac-tab-bindings = Bindings
rbac-tab-subjects = Субъекты
rbac-tab-access = Что мне здесь можно?
rbac-search-roles = Поиск ролей
rbac-search-bindings = Поиск привязок
rbac-search-subjects = Поиск субъектов
rbac-column-kind = Kind
rbac-column-name = Имя
rbac-column-namespace = Пространство имён
rbac-column-rules = Правила
rbac-column-role = Роль
rbac-column-subjects = Субъекты
rbac-column-bindings = Привязки
rbac-column-how = Как
rbac-column-age = Возраст
rbac-clusterWide = Весь кластер
rbac-aggregated = агрегированная
rbac-roleMissing = не найдена
rbac-empty = Здесь ничего нет.
rbac-refused = Ваши учётные данные не позволяют получить список некоторых объектов RBAC, поэтому показанное их не учитывает:
rbac-forbidden = Ваши учётные данные не позволяют получить список объектов RBAC этого кластера, поэтому показано только то, что они разрешают вам, по данным API-сервера.
rbac-role-kind = Kind
rbac-role-namespace = Пространство имён
rbac-role-aggregation = Объединяет ClusterRole, подходящие под
rbac-role-aggregatedFrom = Содержит правила из
rbac-role-labels = Метки
rbac-role-age = Возраст
rbac-role-rules = Правила
rbac-role-noRules = Правил нет.
rbac-role-bindings = Привязки, которые её используют
rbac-role-noBindings = Ни одна из показанных здесь привязок её не использует.
rbac-role-gone = Этой роли нет (больше) среди полученных.
rbac-rule-resources = Ресурсы
rbac-rule-apiGroups = Группы API
rbac-rule-resourceNames = Имена ресурсов
rbac-rule-urls = URL вне ресурсов
rbac-rule-verbs = Глаголы
rbac-coreGroup = core
rbac-subject-bestEffort = Вычислено по привязкам, которые Homeport может прочитать: приблизительная картина. Она может не учитывать то, что разрешают авторизаторы webhook или ABAC, и группы, которые аутентификатор даёт пользователю.
rbac-subject-groups = Привязки к этим группам тоже учитываются:
rbac-subject-onlyNamespace = Учитываются только RoleBinding в { $namespace }. Выберите все пространства имён, чтобы увидеть остальные.
rbac-subject-cluster = Весь кластер
rbac-subject-clusterHint = Во всех пространствах имён, на ресурсах уровня кластера и на URL вне ресурсов.
rbac-subject-inNamespace = В { $namespace }
rbac-subject-none = Ни одна из показанных здесь привязок ничего ему не даёт.
rbac-subject-bindings = Привязки
rbac-subject-direct = Называет его
rbac-subject-through = Через группу { $group }
rbac-subject-via = Выдано через { $bindings }
rbac-matrix-resource = Ресурс
rbac-matrix-names = Только
rbac-matrix-yes = Разрешено
rbac-matrix-all = Разрешено: любой глагол (*)
rbac-matrix-url = URL вне ресурсов
rbac-matrix-verbs = Глаголы
rbac-matrix-empty = Ничего.
rbac-access-about = Что учётные данные, которые Homeport использует для { $name }, могут делать в { $namespace }, по ответу API-сервера.
rbac-access-user = Вход выполнен как
rbac-access-groups = Группы
rbac-access-myBindings = Привязки, которые называют меня
rbac-access-incomplete = API-сервер сообщает, что этот список может быть неполным: авторизатор, который не умеет перечислять свои правила, например webhook или ABAC, может разрешать больше. Чтобы знать наверняка, спросите «Можно ли мне…?» ниже.
rbac-canI-title = Можно ли мне…?
rbac-canI-verb = Глагол
rbac-canI-resource = Ресурс
rbac-canI-resourcePlaceholder = secrets, deploy, deployments.apps
rbac-canI-namespace = Пространство имён
rbac-canI-allNamespaces = Все пространства имён
rbac-canI-group = Группа API
rbac-canI-subresource = Подресурс
rbac-canI-name = Имя
rbac-canI-optional = Необязательно
rbac-canI-ask = Спросить
rbac-canI-yes = Да
rbac-canI-no = Нет
rbac-canI-denied = Явно запрещено
rbac-canI-checked = для { $resource }
rbac-canI-unknown = Этот кластер не обслуживает ресурс с именем { $resource }, поэтому вопрос задан как введено.
