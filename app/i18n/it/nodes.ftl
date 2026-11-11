### The `nodes` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/nodes.ts:
### change the text there, then run it again.

nodes-nav = Node
nodes-title = Node
nodes-summary = { $count ->
        [one] { NUM($count) } Node
       *[other] { NUM($count) } Node
    }
nodes-search = Cerca Node
nodes-ready = Pronto
nodes-notReady = Non pronto
nodes-cordoned = pianificazione disattivata
nodes-column-name = Nome
nodes-column-status = Stato
nodes-column-roles = Ruoli
nodes-column-version = Versione
nodes-column-ip = IP interno
nodes-column-platform = Piattaforma
nodes-column-cpu = CPU
nodes-column-memory = Memoria
nodes-column-age = Età
nodes-tab-overview = Panoramica
nodes-tab-pods = Pod
nodes-tab-conditions = Condizioni
nodes-tab-events = Eventi
nodes-tab-source = YAML
nodes-detail-roles = Ruoli
nodes-detail-version = Kubelet
nodes-detail-os = Immagine del sistema operativo
nodes-detail-kernel = Kernel
nodes-detail-runtime = Runtime dei container
nodes-detail-addresses = Indirizzi
nodes-detail-taints = Taint
nodes-detail-noTaints = Nessuno
nodes-detail-age = Età
nodes-detail-labels = Etichette
nodes-detail-resources = Risorse
nodes-resource-name = Risorsa
nodes-resource-capacity = Capacità
nodes-resource-allocatable = Allocabile
nodes-column-cpuUsed = CPU usata
nodes-column-memoryUsed = Memoria usata
nodes-tab-metrics = Metriche
