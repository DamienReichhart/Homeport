### The `network` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/network.ts:
### change the text there, then run it again.

network-nav = Réseau
network-title = Réseau
network-services = Services
network-ingresses = Ingress
network-service = Service
network-ingress = Ingress
network-servicesSummary = { $count ->
        [one] { NUM($count) } Service
       *[other] { NUM($count) } Services
    }
network-ingressesSummary = { $count ->
        [one] { NUM($count) } Ingress
       *[other] { NUM($count) } Ingress
    }
network-searchServices = Rechercher des Services
network-searchIngresses = Rechercher des Ingress
network-column-name = Nom
network-column-namespace = Namespace
network-column-type = Type
network-column-clusterIp = IP du cluster
network-column-external = Externe
network-column-ports = Ports
network-column-class = Classe
network-column-hosts = Noms d’hôte
network-column-addresses = Adresses
network-column-age = Âge
network-tab-overview = Vue d’ensemble
network-tab-pods = Pods
network-tab-rules = Règles
network-tab-events = Événements
network-tab-source = YAML
network-detail-namespace = Namespace
network-detail-type = Type
network-detail-clusterIp = IP du cluster
network-detail-external = Adresses externes
network-detail-selector = Envoie aux Pods ayant
network-detail-noSelector = Il ne sélectionne aucun Pod ; ses endpoints sont gérés à la main.
network-detail-affinity = Affinité de session
network-detail-class = Classe
network-detail-addresses = Adresses
network-detail-tls = TLS
network-detail-age = Âge
network-detail-labels = Labels
network-detail-annotations = Annotations
network-port-name = Nom
network-port-port = Port
network-port-target = Port cible
network-port-node = Port du Node
network-rule-host = Nom d’hôte
network-rule-path = Chemin
network-rule-type = Correspondance
network-rule-backend = Backend
