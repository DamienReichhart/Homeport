### The `pods` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/pods.ts:
### change the text there, then run it again.

pods-nav = Pods
pods-title = Pods
pods-summary = { $count ->
        [one] { NUM($count) } Pod
       *[other] { NUM($count) } Pods
    }
pods-search = Pods durchsuchen
pods-empty = Hier sind keine Pods.
pods-column-name = Name
pods-column-namespace = Namespace
pods-column-status = Status
pods-column-ready = Bereit
pods-column-restarts = Neustarts
pods-column-node = Node
pods-column-age = Alter
pods-tab-overview = Übersicht
pods-tab-conditions = Bedingungen
pods-tab-events = Events
pods-tab-source = YAML
pods-detail-readyOf = { $ready } von { $total } Containern bereit
pods-detail-namespace = Namespace
pods-detail-node = Node
pods-detail-ip = Pod-IP
pods-detail-hostIp = Node-IP
pods-detail-qos = QoS-Klasse
pods-detail-serviceAccount = Service-Account
pods-detail-owner = Gesteuert von
pods-detail-age = Alter
pods-detail-labels = Labels
pods-detail-annotations = Annotationen
pods-container-name = Container
pods-container-init = init
pods-container-image = Image
pods-container-state = Zustand
pods-container-ready = Bereit
pods-container-isReady = Ja
pods-container-notReady = Nein
pods-container-restarts = Neustarts
pods-container-restartsWhy = { $count } (zuletzt: { $reason })
pods-container-started = Gestartet
pods-container-ports = Ports
pods-state-running = Läuft
pods-state-waiting = Wartet
pods-state-terminated = Beendet
pods-tab-logs = Logs
pods-logs-container = Container
pods-logs-initContainer = { $name } (init)
pods-logs-previous = Vorherige Instanz
pods-logs-noRestart = Er wurde nicht neu gestartet, daher gibt es keine vorherige Instanz.
pods-logs-pod = Pod
pods-logs-allPods = { $count ->
        [one] Der Pod
       *[other] Alle { NUM($count) } Pods
    }
pods-logs-noPods = Er führt gerade keinen Pod aus.
pods-tab-terminal = Terminal
pods-terminal-shell = Shell
pods-terminal-command = Befehl
pods-terminal-connect = Verbinden
pods-terminal-reconnect = Neu verbinden
pods-terminal-notRunning = Dieser Container läuft nicht.
pods-terminal-exited = Der Prozess wurde beendet.
pods-terminal-exitedWithCode = Der Prozess wurde mit Code { $code } beendet.
pods-column-cpu = CPU
pods-column-memory = Arbeitsspeicher
pods-tab-metrics = Metriken
pods-metrics-noSample = metrics-server hat noch keinen Messwert dieses Pods: Er wurde vielleicht gerade gestartet oder läuft nicht.
