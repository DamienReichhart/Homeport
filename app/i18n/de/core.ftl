### The `core` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/core.ts:
### change the text there, then run it again.

core-host-engineVersion = Engine { $version }
core-host-podmanVersion = Podman { $version }
core-host-containerdVersion = containerd { $version }
core-host-incusVersion = Incus { $version }
core-engine-docker = Docker-kompatibel
core-engine-containerd = containerd
core-engine-incus = Incus
core-unsupported-title = { $name } kann diese Seite nicht anzeigen
core-unsupported-docker = Diese Seite benötigt eine Docker-kompatible Engine, und auf { $name } läuft { $engine }.
core-unsupported-podman = Diese Seite ist für Podman, und auf { $name } läuft eine andere Docker-kompatible Engine. Wählen Sie im Umschalter einen Podman-Host, um sie zu verwenden.
core-unsupported-containerd = Diese Seite ist für containerd, und auf { $name } läuft { $engine }. Wählen Sie im Umschalter einen containerd-Host, um sie zu verwenden.
core-unsupported-incus = Diese Seite ist für Incus, und auf { $name } läuft { $engine }. Wählen Sie im Umschalter einen Incus-Host, um sie zu verwenden.
core-unsupported-overview = Zur Übersicht
core-cluster-kind-kubernetes = Kubernetes
core-cluster-kind-nomad = Nomad
core-cluster-unsupported-title = { $name } kann diese Seite nicht anzeigen
core-cluster-unsupported-description = Diese Seite ist für { $kind }-Cluster, und { $name } ist ein { $current }-Cluster. Wählen Sie im Umschalter einen { $kind }-Cluster, um sie zu verwenden.
core-containerd-namespace = Namespace
core-containerd-holds = { $containers ->
        [one] { NUM($containers) } Container
       *[other] { NUM($containers) } Container
    }, { $images ->
        [one] { NUM($images) } Image
       *[other] { NUM($images) } Images
    }
core-containerd-manage = Namespaces verwalten
core-containerd-rememberFailed = Homeport konnte sich den gewählten Namespace nicht merken
core-incus-project = Projekt
core-incus-holds = { $instances ->
        [one] { NUM($instances) } Instanz
       *[other] { NUM($instances) } Instanzen
    }, { $images ->
        [one] { NUM($images) } Image
       *[other] { NUM($images) } Images
    }
core-incus-rememberFailed = Homeport konnte sich das gewählte Projekt nicht merken
core-selection-rememberFailed = Homeport konnte sich Ihre Auswahl nicht merken
core-connection-local = Lokal
core-status-connected = Verbunden
core-status-connecting = Wird verbunden
core-status-disabled = Deaktiviert
core-status-unreachable = Nicht erreichbar
core-status-unreachableDetail = { $message } — neuer Versuch in { $secs } s
core-status-unauthorized = Zugriff verweigert
core-status-hostKeyUntrusted = Serverschlüssel nicht vertrauenswürdig
core-status-hostKeyUntrustedDetail = Prüfen Sie den Fingerabdruck { $fingerprint } und vertrauen Sie ihm, um sich zu verbinden.
core-status-hostKeyMismatch = Serverschlüssel geändert
core-status-hostKeyMismatchDetail = Erwartet: { $expected }, erhalten: { $actual }.
core-status-credentialsLost = Zugangsdaten verloren
core-status-credentialsLostDetail = Die gespeicherten Zugangsdaten lassen sich nicht entschlüsseln. Geben Sie sie erneut ein oder erzeugen Sie den Schlüssel neu.
core-status-keychainUnavailable = Schlüsselbund nicht verfügbar
core-status-invalid = Falsch konfiguriert
core-swarm-label = Swarm von { $host }
core-swarm-labelUnknown = Swarm { $id }
core-swarm-gone-title = Dieser Swarm ist nicht mehr bekannt
core-swarm-gone-description = Keiner der Hosts, mit denen Homeport sich verbindet, gehört jetzt zu ihm. Fügen Sie einen seiner Manager als Host hinzu, um ihn zu verwalten.
core-cluster-serverVersion = Kubernetes { $version }
core-cluster-nomadVersion = Nomad { $version }, Region { $region }
core-cluster-nomadRegion = Nomad, Region { $region }
core-cluster-credentialsLostDetail = Die gespeicherten Zugangsdaten lassen sich nicht entschlüsseln. Aktualisieren Sie den Cluster aus seiner kubeconfig.
core-cluster-reconnectFailed = Homeport konnte sich nicht erneut mit dem Cluster verbinden
core-cluster-gate-none = Kein Cluster ausgewählt
core-cluster-gate-noneDescription = Wählen Sie in der Seitenleiste einen Cluster oder fügen Sie einen aus Ihrer kubeconfig hinzu.
core-cluster-gate-add = Cluster hinzufügen
core-cluster-gate-openSettings = Cluster-Einstellungen öffnen
core-localClusters-title = Cluster auf diesem Rechner
core-localClusters-description = Lokal laufendes Kubernetes, so wie Ihre kubeconfig es erreicht.
core-localClusters-add = Hinzufügen
core-localClusters-added = Hinzugefügt
core-localClusters-notRunning = Läuft nicht
core-localClusters-addFailed = Homeport konnte den Cluster nicht hinzufügen
core-localClusters-missing = Der { $tool }-Cluster { $name } läuft, aber Ihre kubeconfig erreicht ihn nicht. Fügen Sie ihn so hinzu:
core-localClusters-tool-dockerDesktop = Docker Desktop
core-localClusters-tool-orbstack = OrbStack
core-localClusters-tool-rancherDesktop = Rancher Desktop
core-localClusters-tool-kind = kind
core-localClusters-tool-k3d = k3d
core-localClusters-tool-minikube = minikube
core-namespace-label = Namespace
core-namespace-all = Alle Namespaces
core-namespace-type = Namespace
core-namespace-cantList = Ihre Zugangsdaten dürfen keine Namespaces auflisten. Geben Sie den ein, den Sie verwenden dürfen.
core-namespace-rememberFailed = Homeport konnte sich den Namespace nicht merken
core-health-ready = Bereit
core-health-progressing = In Arbeit
core-health-degraded = Beeinträchtigt
core-health-failed = Fehlgeschlagen
core-health-paused = Pausiert
core-events-type = Typ
core-events-reason = Grund
core-events-message = Meldung
core-events-count = Anzahl
core-events-lastSeen = Zuletzt gesehen
core-events-none = Keine Events. Kubernetes bewahrt sie etwa eine Stunde lang auf.
core-conditions-type = Bedingung
core-conditions-status = Status
core-conditions-reason = Grund
core-conditions-message = Meldung
core-conditions-since = Seit
core-conditions-none = Keine Bedingungen gemeldet.
core-labels-none = Keine
core-yaml-label = YAML
core-list-empty = Hier ist nichts.
core-yaml-copy = YAML kopieren
core-gate-welcome-title = Willkommen bei Homeport
core-gate-welcome-description = Fügen Sie zum Start eine Docker Engine hinzu — die dieses Rechners oder eine entfernte über SSH oder TLS — oder einen Kubernetes-Cluster aus Ihrer kubeconfig.
core-gate-addAnother = Weiteren Host hinzufügen
core-gate-addCluster = Cluster hinzufügen
core-gate-connecting = Verbindung mit { $name } wird hergestellt…
core-gate-statusTitle = { $name }: { $status }
core-gate-retryingOnItsOwn = Homeport versucht es in { $secs } s von selbst erneut.
core-gate-retryNow = Jetzt erneut versuchen
core-gate-openSettings = Host-Einstellungen öffnen
core-localEngines-title = Auf diesem Rechner gefunden
core-localEngines-description = Engines, die über lokale Sockets antworten.
core-localEngines-add = Hinzufügen
core-freshness-check = Nach neueren Images suchen
core-freshness-checking = Wird geprüft…
core-freshness-allUpToDate = Alle Images sind aktuell
core-freshness-newerCount = { $count ->
        [one] Für { NUM($count) } Image gibt es
       *[other] Für { NUM($count) } Images gibt es
    } eine neuere Version
core-freshness-newerOne = Ein neueres { $reference } wurde veröffentlicht
core-freshness-newerMany = Neuere Images wurden veröffentlicht: { $references }
core-delete-title = { $kind } { $name } löschen?
core-delete-titleMany = { $count } { $kind }-Objekte löschen?
core-delete-titleObjects = { $count } Objekte löschen?
core-delete-one = { $kind } { $name } wird aus { $cluster } gelöscht.
core-delete-oneIn = { $kind } { $name } wird aus dem Namespace { $namespace } auf { $cluster } gelöscht.
core-delete-many = Diese werden aus { $cluster } gelöscht: { $names }.
core-delete-manyIn = Diese werden aus dem Namespace { $namespace } auf { $cluster } gelöscht: { $names }.
core-delete-namespace = Alles im Namespace wird mitgelöscht: seine Workloads, Pods, Services, ConfigMaps und Secrets.
core-delete-recreated = { $kind } { $name } steuert ihn und startet daher an seiner Stelle einen neuen Pod.
core-delete-pods = Seine Pods werden mitgelöscht.
core-delete-cronJob = Er startet keine Jobs mehr; die von ihm gestarteten werden mitgelöscht.
core-delete-node = Homeport leert ihn vorher nicht: Seine Pods bleiben, bis der Node sich wieder registriert oder sie neu eingeplant werden.
core-delete-confirm = Löschen
core-delete-failed = Homeport konnte { $name } nicht löschen
core-delete-done = { $count ->
        [one] { $name } gelöscht
       *[other] { NUM($count) } Objekte gelöscht
    }
core-delete-action = Löschen
core-delete-selected = { $count } löschen
core-apply-preview = Änderungen anzeigen
core-apply-apply = Anwenden
core-apply-force = Trotzdem anwenden
core-apply-stale = Das YAML hat sich seit dieser Vorschau geändert.
core-apply-noChanges = Keine Änderungen: Der Cluster enthält dies bereits.
core-apply-confirmTitle = { $count ->
        [one] { NUM($count) } Änderung
       *[other] { NUM($count) } Änderungen
    } auf { $cluster } anwenden?
core-apply-confirmDescription = Homeport wendet es per Server-side Apply als Field Manager homeport an, wie die Vorschau zeigt.
core-apply-forceTitle = Die Felder mit Konflikt übernehmen?
core-apply-forceDescription = Die Tools, die sie auf { $cluster } verwalten, setzen sie womöglich zurück oder verwalten sie nicht mehr. Homeport wird ihr Manager.
core-apply-done = Auf { $cluster } angewendet
core-apply-diffOf = Änderungen an { $kind } { $name }
core-apply-action-created = Erstellt
core-apply-action-changed = Geändert
core-apply-action-unchanged = Keine Änderungen
core-apply-action-failed = Nicht anwendbar
core-yaml-edit = Bearbeiten
core-yaml-editTitle = { $kind } { $name } bearbeiten
core-yaml-editDescription = Auf { $cluster }. Ändern Sie das YAML, sehen Sie sich die Änderungen an und wenden Sie es dann an.
core-metrics-cpu = CPU
core-metrics-memory = Arbeitsspeicher
core-metrics-used = Belegt
core-metrics-request = Anforderung
core-metrics-limit = Limit
core-metrics-allocatable = Zuweisbar
core-metrics-ofLimit = { $percent } des Limits
core-metrics-ofRequest = { $percent } der Anforderung
core-metrics-ofAllocatable = { $percent } des Zuweisbaren
core-metrics-notInstalled-title = metrics-server ist nicht installiert
core-metrics-notInstalled-body = Dieser Cluster stellt keine Metrics API bereit, aus der CPU und Arbeitsspeicher stammen. Homeport installiert nichts auf einem Cluster: Sobald metrics-server installiert ist, erscheinen sie hier.
core-metrics-notInstalled-link = So installieren Sie metrics-server
core-metrics-notResponding = Die Metrics API antwortet nicht: metrics-server startet vielleicht noch oder läuft nicht. Homeport versucht es alle 15 Sekunden erneut.
core-metrics-waiting = Warten auf den ersten Messwert von metrics-server…
core-metrics-kept = Ein Messwert alle 15 Sekunden, aufbewahrt, solange diese Seite offen ist, höchstens 15 Minuten lang.
