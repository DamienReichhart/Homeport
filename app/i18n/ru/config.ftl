### The `config` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/config.ts:
### change the text there, then run it again.

config-nav = Конфигурация
config-title = Конфигурация
config-configMaps = ConfigMap
config-secrets = Secret
config-configMap = ConfigMap
config-secret = Secret
config-configMapsSummary = { $count ->
        [one] { NUM($count) } ConfigMap
        [few] { NUM($count) } ConfigMap
        [many] { NUM($count) } ConfigMap
       *[other] { NUM($count) } ConfigMap
    }
config-secretsSummary = { $count ->
        [one] { NUM($count) } Secret
        [few] { NUM($count) } Secret
        [many] { NUM($count) } Secret
       *[other] { NUM($count) } Secret
    }
config-searchConfigMaps = Поиск ConfigMap
config-searchSecrets = Поиск Secret
config-column-name = Имя
config-column-namespace = Пространство имён
config-column-type = Тип
config-column-keys = Ключи
config-column-age = Возраст
config-tab-data = Данные
config-tab-overview = Обзор
config-tab-events = События
config-tab-source = YAML
config-detail-namespace = Пространство имён
config-detail-type = Тип
config-detail-labels = Метки
config-detail-annotations = Аннотации
config-noKeys = Ключей нет.
config-binary = (двоичные данные)
config-base64 = base64
config-reveal = Показать
config-revealOne = Показать { $key }
config-hide = Скрыть
config-copy = Скопировать { $key }
config-reveal-failed = Homeport не удалось показать { $key }
