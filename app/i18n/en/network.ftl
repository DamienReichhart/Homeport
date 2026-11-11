### The `network` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/network.ts:
### change the text there, then run it again.

network-nav = Network
network-title = Network
network-services = Services
network-ingresses = Ingresses
network-service = Service
network-ingress = Ingress
network-servicesSummary = { $count ->
        [one] { NUM($count) } service
       *[other] { NUM($count) } services
    }
network-ingressesSummary = { $count ->
        [one] { NUM($count) } ingress
       *[other] { NUM($count) } ingresses
    }
network-searchServices = Search services
network-searchIngresses = Search ingresses
network-column-name = Name
network-column-namespace = Namespace
network-column-type = Type
network-column-clusterIp = Cluster IP
network-column-external = External
network-column-ports = Ports
network-column-class = Class
network-column-hosts = Hosts
network-column-addresses = Addresses
network-column-age = Age
network-tab-overview = Overview
network-tab-pods = Pods
network-tab-rules = Rules
network-tab-events = Events
network-tab-source = YAML
network-detail-namespace = Namespace
network-detail-type = Type
network-detail-clusterIp = Cluster IP
network-detail-external = External addresses
network-detail-selector = Sends to Pods with
network-detail-noSelector = It selects no Pods; its endpoints are managed by hand.
network-detail-affinity = Session affinity
network-detail-class = Class
network-detail-addresses = Addresses
network-detail-tls = TLS
network-detail-age = Age
network-detail-labels = Labels
network-detail-annotations = Annotations
network-port-name = Name
network-port-port = Port
network-port-target = Target port
network-port-node = Node port
network-rule-host = Host
network-rule-path = Path
network-rule-type = Match
network-rule-backend = Backend
