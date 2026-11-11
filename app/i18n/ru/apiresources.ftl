### The `apiresources` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/apiresources.ts:
### change the text there, then run it again.

apiresources-nav = Ресурсы API
apiresources-title = Ресурсы API
apiresources-summary = Этот кластер обслуживает { $count ->
        [one] { NUM($count) } тип
        [few] { NUM($count) } типа
        [many] { NUM($count) } типов
       *[other] { NUM($count) } типа
    }
apiresources-refresh = Обновить
apiresources-types = Типы ресурсов
apiresources-search = Kind или краткое имя (pvc)
apiresources-searchObjects = Поиск
apiresources-noMatch = Подходящих типов нет.
apiresources-coreGroup = core
apiresources-pick = Выберите тип, чтобы увидеть его объекты: встроенные и добавленные вашими CRD.
apiresources-typeGone = Этот кластер не обслуживает (больше) этот тип.
apiresources-cantList = Список недоступен
apiresources-notAllowed = Ваши учётные данные не позволяют получить список
apiresources-cantListWhy = Список объектов { $kind } получить нельзя: их API позволяет только создавать или читать их по одному.
apiresources-empty = Здесь ничего нет.
apiresources-column-name = Имя
apiresources-column-namespace = Пространство имён
apiresources-column-age = Возраст
apiresources-tab-overview = Обзор
apiresources-tab-conditions = Условия
apiresources-tab-events = События
apiresources-tab-source = YAML
apiresources-detail-namespace = Пространство имён
apiresources-detail-owners = Владельцы
apiresources-detail-age = Возраст
apiresources-detail-labels = Метки
apiresources-detail-annotations = Аннотации
apiresources-edit = Изменить
apiresources-copyYaml = Скопировать YAML
