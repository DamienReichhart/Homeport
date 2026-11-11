### The `pods` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/pods.ts:
### change the text there, then run it again.

pods-nav = Pod
pods-title = Pod
pods-summary = { $count ->
        [one] { NUM($count) } Pod
       *[other] { NUM($count) } Pod
    }
pods-search = Cerca Pod
pods-empty = Nessun Pod qui.
pods-column-name = Nome
pods-column-namespace = Namespace
pods-column-status = Stato
pods-column-ready = Pronti
pods-column-restarts = Riavvii
pods-column-node = Node
pods-column-age = Età
pods-tab-overview = Panoramica
pods-tab-conditions = Condizioni
pods-tab-events = Eventi
pods-tab-source = YAML
pods-detail-readyOf = { $ready } di { $total } container pronti
pods-detail-namespace = Namespace
pods-detail-node = Node
pods-detail-ip = IP del Pod
pods-detail-hostIp = IP del Node
pods-detail-qos = Classe QoS
pods-detail-serviceAccount = Service account
pods-detail-owner = Controllato da
pods-detail-age = Età
pods-detail-labels = Etichette
pods-detail-annotations = Annotazioni
pods-container-name = Container
pods-container-init = init
pods-container-image = Immagine
pods-container-state = Stato
pods-container-ready = Pronto
pods-container-isReady = Sì
pods-container-notReady = No
pods-container-restarts = Riavvii
pods-container-restartsWhy = { $count } (ultimo: { $reason })
pods-container-started = Avviato
pods-container-ports = Porte
pods-state-running = In esecuzione
pods-state-waiting = In attesa
pods-state-terminated = Terminato
pods-tab-logs = Log
pods-logs-container = Container
pods-logs-initContainer = { $name } (init)
pods-logs-previous = Istanza precedente
pods-logs-noRestart = Non si è riavviato, quindi non c’è un’istanza precedente.
pods-logs-pod = Pod
pods-logs-allPods = { $count ->
        [one] Il Pod
       *[other] Tutti i { NUM($count) } Pod
    }
pods-logs-noPods = Al momento non esegue alcun Pod.
pods-tab-terminal = Terminale
pods-terminal-shell = Shell
pods-terminal-command = Comando
pods-terminal-connect = Connetti
pods-terminal-reconnect = Riconnetti
pods-terminal-notRunning = Questo container non è in esecuzione.
pods-terminal-exited = Il processo è terminato.
pods-terminal-exitedWithCode = Il processo è terminato con codice { $code }.
pods-column-cpu = CPU
pods-column-memory = Memoria
pods-tab-metrics = Metriche
pods-metrics-noSample = metrics-server non ha ancora un campione di questo Pod: potrebbe essere appena partito, o non essere in esecuzione.
