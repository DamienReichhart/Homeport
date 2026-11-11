### The `network` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/network.ts:
### change the text there, then run it again.

network-nav = Rede
network-title = Rede
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
network-searchServices = Pesquisar Services
network-searchIngresses = Pesquisar Ingresses
network-column-name = Nome
network-column-namespace = Namespace
network-column-type = Tipo
network-column-clusterIp = IP do cluster
network-column-external = Externo
network-column-ports = Portas
network-column-class = Classe
network-column-hosts = Hosts
network-column-addresses = Endereços
network-column-age = Idade
network-tab-overview = Visão geral
network-tab-pods = Pods
network-tab-rules = Regras
network-tab-events = Eventos
network-tab-source = YAML
network-detail-namespace = Namespace
network-detail-type = Tipo
network-detail-clusterIp = IP do cluster
network-detail-external = Endereços externos
network-detail-selector = Envia para Pods com
network-detail-noSelector = Ele não seleciona nenhum Pod; seus endpoints são gerenciados manualmente.
network-detail-affinity = Afinidade de sessão
network-detail-class = Classe
network-detail-addresses = Endereços
network-detail-tls = TLS
network-detail-age = Idade
network-detail-labels = Rótulos
network-detail-annotations = Anotações
network-port-name = Nome
network-port-port = Porta
network-port-target = Porta de destino
network-port-node = Porta do Node
network-rule-host = Host
network-rule-path = Caminho
network-rule-type = Correspondência
network-rule-backend = Backend
