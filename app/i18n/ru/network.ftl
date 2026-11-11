### The `network` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/network.ts:
### change the text there, then run it again.

network-nav = Сеть
network-title = Сеть
network-services = Service
network-ingresses = Ingress
network-service = Service
network-ingress = Ingress
network-servicesSummary = { $count ->
        [one] { NUM($count) } Service
        [few] { NUM($count) } Service
        [many] { NUM($count) } Service
       *[other] { NUM($count) } Service
    }
network-ingressesSummary = { $count ->
        [one] { NUM($count) } Ingress
        [few] { NUM($count) } Ingress
        [many] { NUM($count) } Ingress
       *[other] { NUM($count) } Ingress
    }
network-searchServices = Поиск Service
network-searchIngresses = Поиск Ingress
network-column-name = Имя
network-column-namespace = Пространство имён
network-column-type = Тип
network-column-clusterIp = IP кластера
network-column-external = Внешние
network-column-ports = Порты
network-column-class = Класс
network-column-hosts = Хосты
network-column-addresses = Адреса
network-column-age = Возраст
network-tab-overview = Обзор
network-tab-pods = Pod
network-tab-rules = Правила
network-tab-events = События
network-tab-source = YAML
network-detail-namespace = Пространство имён
network-detail-type = Тип
network-detail-clusterIp = IP кластера
network-detail-external = Внешние адреса
network-detail-selector = Направляет на Pod с метками
network-detail-noSelector = Он не выбирает ни одного Pod; его конечные точки задаются вручную.
network-detail-affinity = Привязка сеансов
network-detail-class = Класс
network-detail-addresses = Адреса
network-detail-tls = TLS
network-detail-age = Возраст
network-detail-labels = Метки
network-detail-annotations = Аннотации
network-port-name = Имя
network-port-port = Порт
network-port-target = Целевой порт
network-port-node = Порт Node
network-rule-host = Хост
network-rule-path = Путь
network-rule-type = Сопоставление
network-rule-backend = Бэкенд
