### The `network` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/network.ts:
### change the text there, then run it again.

network-nav = Rete
network-title = Rete
network-services = Service
network-ingresses = Ingress
network-service = Service
network-ingress = Ingress
network-servicesSummary = { $count ->
        [one] { NUM($count) } Service
       *[other] { NUM($count) } Service
    }
network-ingressesSummary = { $count ->
        [one] { NUM($count) } Ingress
       *[other] { NUM($count) } Ingress
    }
network-searchServices = Cerca Service
network-searchIngresses = Cerca Ingress
network-column-name = Nome
network-column-namespace = Namespace
network-column-type = Tipo
network-column-clusterIp = IP del cluster
network-column-external = Esterni
network-column-ports = Porte
network-column-class = Classe
network-column-hosts = Host
network-column-addresses = Indirizzi
network-column-age = Età
network-tab-overview = Panoramica
network-tab-pods = Pod
network-tab-rules = Regole
network-tab-events = Eventi
network-tab-source = YAML
network-detail-namespace = Namespace
network-detail-type = Tipo
network-detail-clusterIp = IP del cluster
network-detail-external = Indirizzi esterni
network-detail-selector = Invia ai Pod con
network-detail-noSelector = Non seleziona alcun Pod; i suoi endpoint sono gestiti manualmente.
network-detail-affinity = Affinità di sessione
network-detail-class = Classe
network-detail-addresses = Indirizzi
network-detail-tls = TLS
network-detail-age = Età
network-detail-labels = Etichette
network-detail-annotations = Annotazioni
network-port-name = Nome
network-port-port = Porta
network-port-target = Porta di destinazione
network-port-node = Porta del Node
network-rule-host = Host
network-rule-path = Percorso
network-rule-type = Corrispondenza
network-rule-backend = Backend
