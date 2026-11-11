### The `network` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/network.ts:
### change the text there, then run it again.

network-nav = Netzwerk
network-title = Netzwerk
network-services = Services
network-ingresses = Ingresses
network-service = Service
network-ingress = Ingress
network-servicesSummary = { $count ->
        [one] { NUM($count) } Service
       *[other] { NUM($count) } Services
    }
network-ingressesSummary = { $count ->
        [one] { NUM($count) } Ingress
       *[other] { NUM($count) } Ingresses
    }
network-searchServices = Services durchsuchen
network-searchIngresses = Ingresses durchsuchen
network-column-name = Name
network-column-namespace = Namespace
network-column-type = Typ
network-column-clusterIp = Cluster-IP
network-column-external = Extern
network-column-ports = Ports
network-column-class = Klasse
network-column-hosts = Hosts
network-column-addresses = Adressen
network-column-age = Alter
network-tab-overview = Übersicht
network-tab-pods = Pods
network-tab-rules = Regeln
network-tab-events = Events
network-tab-source = YAML
network-detail-namespace = Namespace
network-detail-type = Typ
network-detail-clusterIp = Cluster-IP
network-detail-external = Externe Adressen
network-detail-selector = Sendet an Pods mit
network-detail-noSelector = Er wählt keine Pods aus; seine Endpoints werden manuell verwaltet.
network-detail-affinity = Session-Affinität
network-detail-class = Klasse
network-detail-addresses = Adressen
network-detail-tls = TLS
network-detail-age = Alter
network-detail-labels = Labels
network-detail-annotations = Annotationen
network-port-name = Name
network-port-port = Port
network-port-target = Zielport
network-port-node = Node-Port
network-rule-host = Host
network-rule-path = Pfad
network-rule-type = Abgleich
network-rule-backend = Backend
