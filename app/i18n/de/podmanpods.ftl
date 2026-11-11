### The `podmanpods` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/podmanpods.ts:
### change the text there, then run it again.

podmanpods-nav = Pods
podmanpods-title = Pods
podmanpods-summary = { $count ->
        [one] { NUM($count) } Pod
       *[other] { NUM($count) } Pods
    }
podmanpods-search = Pods durchsuchen
podmanpods-empty = Keine Pods
podmanpods-emptyDescription = Ein Pod gruppiert Container, die sich ein Netzwerk teilen und gemeinsam starten und stoppen. Erstellen Sie einen und führen Sie dann Container darin aus.
podmanpods-new = Neuer Pod
podmanpods-column-name = Name
podmanpods-column-status = Status
podmanpods-column-containers = Container
podmanpods-column-networks = Netzwerke
podmanpods-column-created = Erstellt
podmanpods-running = { $running } von { $count } laufen
podmanpods-status-running = Läuft
podmanpods-status-degraded = Läuft teilweise
podmanpods-status-paused = Pausiert
podmanpods-status-created = Erstellt
podmanpods-status-stopped = Gestoppt
podmanpods-status-failed = Fehlgeschlagen
podmanpods-action-start = Starten
podmanpods-action-stop = Stoppen
podmanpods-action-restart = Neu starten
podmanpods-action-pause = Pausieren
podmanpods-action-unpause = Fortsetzen
podmanpods-actionOne-start = { $name } starten
podmanpods-actionOne-stop = { $name } stoppen
podmanpods-actionOne-restart = { $name } neu starten
podmanpods-actionOne-pause = { $name } pausieren
podmanpods-actionOne-unpause = { $name } fortsetzen
podmanpods-removeOne = { $name } entfernen
podmanpods-done-start = { $name } gestartet
podmanpods-done-stop = { $name } gestoppt
podmanpods-done-restart = { $name } neu gestartet
podmanpods-done-pause = { $name } pausiert
podmanpods-done-unpause = { $name } fortgesetzt
podmanpods-removed = { $name } entfernt, { $count ->
        [0] ohne Container
        [one] mit { NUM($count) } Container
       *[other] mit { NUM($count) } Containern
    }
podmanpods-created = { $name } erstellt
podmanpods-remove-title = { $name } entfernen?
podmanpods-remove-stopped = { $count ->
        [0] Der Pod und sein Infra-Container werden entfernt.
        [one] Der Pod, sein Infra-Container und sein { NUM($count) } Container werden entfernt.
       *[other] Der Pod, sein Infra-Container und seine { NUM($count) } Container werden entfernt.
    } Ihre Volumes bleiben erhalten.
podmanpods-remove-running = { $count ->
        [0] Der Pod läuft: Er wird gestoppt und dann mit seinem Infra-Container entfernt.
        [one] Der Pod läuft: Sein { NUM($count) } Container wird gestoppt und dann mit dem Pod und dessen Infra-Container entfernt.
       *[other] Der Pod läuft: Seine { NUM($count) } Container werden gestoppt und dann mit dem Pod und dessen Infra-Container entfernt.
    } Ihre Volumes bleiben erhalten.
podmanpods-remove-stopAndRemove = Stoppen und entfernen
podmanpods-detail-containers = { $count ->
        [0] Container
        [one] { NUM($count) } Container
       *[other] { NUM($count) } Container
    }
podmanpods-detail-noContainers = In diesem Pod läuft außer seinem Infra-Container noch nichts.
podmanpods-detail-infra = Infra
podmanpods-detail-settings = Einstellungen
podmanpods-detail-ports = Veröffentlichte Ports
podmanpods-detail-noPorts = Keine
podmanpods-detail-shared = Geteilte Namespaces
podmanpods-detail-networks = Netzwerke
podmanpods-detail-hostname = Hostname
podmanpods-detail-created = Erstellt
podmanpods-detail-labels = Labels
podmanpods-detail-command = Erstellt mit
podmanpods-detail-runContainer = Container ausführen
podmanpods-detail-runContainerIn = Container in { $name } ausführen
podmanpods-newDialog-title = Neuer Pod
podmanpods-newDialog-description = Seine Container teilen sich ein Netzwerk, daher werden Ports am Pod veröffentlicht, für alle gemeinsam.
podmanpods-newDialog-name = Name
podmanpods-newDialog-portsHint = Lassen Sie den Host-Port leer, damit Podman einen freien wählt. Im Pod ausgeführte Container können keine eigenen Ports veröffentlichen.
podmanpods-newDialog-labels = Labels
podmanpods-newDialog-create = Pod erstellen
podmanpods-form-nameInvalid = Beginnen Sie mit einem Buchstaben oder einer Ziffer und verwenden Sie dann Buchstaben, Ziffern, _, . oder -.
podmanpods-form-portInvalid = Ports sind Zahlen von 1 bis 65535.
podmanpods-form-labelInvalid = Schreiben Sie ein Label key=value pro Zeile.
