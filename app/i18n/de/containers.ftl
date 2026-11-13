### The `containers` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/containers.ts:
### change the text there, then run it again.

containers-nav = Container
containers-title = Container
containers-summary = { $running } von { $count } laufen
containers-runContainer = Container ausführen
containers-search = Container durchsuchen
containers-onlyRunning = Nur laufende
containers-empty-title = Keine Container
containers-empty-description = Führen Sie einen aus einem Image aus oder stellen Sie einen Stack bereit.
containers-noMatch = Keine Container passen.
containers-column-name = Name
containers-column-image = Image
containers-column-status = Status
containers-column-ports = Ports
containers-column-cpu = CPU
containers-column-memory = Arbeitsspeicher
containers-column-created = Erstellt
containers-selectOne = { $name } auswählen
containers-selectStack = Stack { $stack } auswählen
containers-stack-running = { $running }/{ $count } laufen
containers-stack-swarm = Swarm
containers-stack-open = Stack öffnen
containers-inPod = Im Podman-Pod { $pod }
containers-fromUnit = Erstellt von der Quadlet-Unit { $unit }
containers-selected = { $count } ausgewählt
containers-action-start = Starten
containers-action-stop = Stoppen
containers-action-restart = Neu starten
containers-action-pause = Pausieren
containers-action-resume = Fortsetzen
containers-action-kill = Beenden erzwingen
containers-action-rename = Umbenennen
containers-action-edit = Bearbeiten
containers-action-duplicate = Duplizieren
containers-action-logs = Logs
containers-action-terminal = Terminal öffnen
containers-action-commit = Als Image committen…
containers-action-export = Dateisystem exportieren…
containers-kill-title = Beenden von { $name } erzwingen?
containers-kill-description = Der Hauptprozess erhält sofort SIGKILL, ohne die Möglichkeit, sauber herunterzufahren.
containers-done-start = { $count ->
        [one] Container
       *[other] { NUM($count) } Container
    } gestartet
containers-done-stop = { $count ->
        [one] Container
       *[other] { NUM($count) } Container
    } gestoppt
containers-done-restart = { $count ->
        [one] Container
       *[other] { NUM($count) } Container
    } neu gestartet
containers-done-pause = { $count ->
        [one] Container
       *[other] { NUM($count) } Container
    } pausiert
containers-done-unpause = { $count ->
        [one] Container
       *[other] { NUM($count) } Container
    } fortgesetzt
containers-done-kill = { $count ->
        [one] Container
       *[other] { NUM($count) } Container
    } zwangsweise beendet
containers-done-remove = { $count ->
        [one] Container
       *[other] { NUM($count) } Container
    } entfernt
containers-remove-title = { $name } entfernen?
containers-remove-titleMany = { $count ->
        [one] { NUM($count) } Container
       *[other] { NUM($count) } Container
    } entfernen?
containers-remove-deleted = { $count ->
        [one] Der Container wird mit seiner beschreibbaren Schicht und seinen anonymen Volumes gelöscht.
       *[other] Die Container werden mit ihrer beschreibbaren Schicht und ihren anonymen Volumes gelöscht.
    }
containers-remove-running = { $count } laufende werden zuerst gestoppt.
containers-remove-namedKept = Benannte Volumes bleiben erhalten.
containers-detail-gone = Dieser Container existiert nicht mehr.
containers-detail-backToList = Zurück zu den Containern
containers-tab-logs = Logs
containers-tab-inspect = Untersuchen
containers-tab-health = Zustand
containers-tab-exec = Exec
containers-tab-files = Dateien
containers-tab-stats = Statistik
containers-tab-processes = Prozesse
containers-notRunning = Der Container läuft nicht.
containers-logs-tail-100 = Letzte 100
containers-logs-tail-1000 = Letzte 1 000
containers-logs-tail-10000 = Letzte 10 000
containers-logs-tail-all = Alles
containers-logs-since-all = Beliebiger Zeitraum
containers-logs-since-300 = Letzte 5 Min.
containers-logs-since-3600 = Letzte Stunde
containers-logs-since-86400 = Letzte 24 Std.
containers-logs-history = Verlauf
containers-logs-sinceLabel = Seit
containers-logs-timestamps = Zeitstempel
containers-logs-follow = Mitverfolgen
containers-logs-find = In Logs suchen
containers-logs-previous = Vorheriger Treffer
containers-logs-next = Nächster Treffer
containers-logs-clear = Leeren
containers-logs-download = Logs herunterladen
containers-logs-saveTitle = Logs speichern
containers-logs-saved = Logs gespeichert
containers-logs-saving = Logs werden gespeichert… { $percent }
containers-logs-stdout = stdout
containers-logs-stderr = stderr
containers-logs-end = — Ende der Logs —
containers-inspect-raw = Rohes JSON
containers-inspect-container = Container
containers-inspect-health = Zustand
containers-inspect-environment = Umgebung
containers-inspect-mounts = Mounts
containers-inspect-ports = Ports
containers-inspect-networks = Netzwerke
containers-inspect-labels = Labels
containers-inspect-id = ID
containers-inspect-image = Image
containers-inspect-created = Erstellt
containers-inspect-state = Zustand
containers-inspect-started = Gestartet
containers-inspect-entrypoint = Entrypoint
containers-inspect-command = Befehl
containers-inspect-workingDir = Arbeitsverzeichnis
containers-inspect-user = Benutzer
containers-inspect-restartPolicy = Neustart-Richtlinie
containers-inspect-networkMode = Netzwerkmodus
containers-inspect-exited = { $status } (Exit { $code })
containers-inspect-outOfMemory = { $state }, kein Speicher mehr
containers-inspect-restarted = { $policy } · { $count }× neu gestartet
containers-inspect-restartedMax = { $policy } (max. { $max }) · { $count }× neu gestartet
containers-inspect-imageDefault = Standard des Images
containers-inspect-notPublished = nicht veröffentlicht
containers-inspect-readWrite = Lesen/Schreiben
containers-inspect-readOnly = schreibgeschützt
containers-inspect-failingStreak = { $count } Fehlschläge in Folge
containers-inspect-noHealthcheck = Das Image definiert keinen Healthcheck.
containers-inspect-column-started = Gestartet
containers-inspect-column-exit = Exit
containers-inspect-column-output = Ausgabe
containers-inspect-column-variable = Variable
containers-inspect-column-value = Wert
containers-inspect-column-type = Typ
containers-inspect-column-source = Quelle
containers-inspect-column-destination = Ziel
containers-inspect-column-mode = Modus
containers-inspect-column-container = Container
containers-inspect-column-publishedOn = Veröffentlicht auf
containers-inspect-column-network = Netzwerk
containers-inspect-column-ip = IP
containers-inspect-column-gateway = Gateway
containers-inspect-column-aliases = Aliasse
containers-inspect-column-label = Label
containers-health-none = Dieser Container hat keinen Healthcheck.
containers-health-title = Healthcheck
containers-health-command = Befehl
containers-health-timingLabel = Zeitsteuerung
containers-health-timing = alle { $interval }, Timeout { $timeout }, { $retries ->
        [one] { NUM($retries) } Wiederholung
       *[other] { NUM($retries) } Wiederholungen
    }, Startphase { $start }
containers-health-failingStreak = Fehlschläge in Folge
containers-health-lastResults = Letzte Ergebnisse
containers-health-noRuns = Es wurde noch keine Prüfung ausgeführt.
containers-health-noOutput = Keine Ausgabe.
containers-health-showLess = Weniger anzeigen
containers-health-showMore = { $count ->
        [one] { NUM($count) } weitere Zeile
       *[other] { NUM($count) } weitere Zeilen
    } anzeigen
containers-health-exit = Exit { $code }
containers-health-took = · dauerte { $duration }
containers-exec-startFirst = Starten Sie den Container, um ein Terminal zu öffnen.
containers-exec-shell = Shell
containers-exec-shellNumber = Shell { $number }
containers-exec-closeOne = { $title } schließen
containers-exec-new = Neues Terminal
containers-exec-command = Befehl
containers-exec-commandPlaceholder = bash, oder sh, falls nicht vorhanden
containers-exec-user = Benutzer
containers-exec-directory = Verzeichnis
containers-exec-default = Standard
containers-exec-open = Öffnen
containers-exec-none = Kein Terminal geöffnet.
containers-exec-exited = [Prozess beendet]
containers-exec-exitedWithCode = [Prozess mit Code { $code } beendet]
containers-files-startFirst = Starten Sie den Container, um seine Dateien zu durchsuchen.
containers-files-saveOne = { $name } speichern
containers-files-uploadTo = Nach { $path } hochladen
containers-files-uploadHere = Hierher hochladen
containers-files-saved = { $name } gespeichert
containers-files-savedArchive = { $name } als .tar-Archiv gespeichert
containers-files-uploaded = { $count ->
        [one] { NUM($count) } Datei
       *[other] { NUM($count) } Dateien
    } nach { $path } hochgeladen
containers-stats-startFirst = Starten Sie den Container, um seine Ressourcennutzung zu sehen.
containers-stats-cpu = CPU
containers-stats-memory = Arbeitsspeicher
containers-stats-memoryNow = · { $used } von { $limit }
containers-stats-used = Belegt
containers-stats-network = Netzwerk
containers-stats-networkNow = · { $received } ein, { $sent } aus
containers-stats-received = Empfangen
containers-stats-sent = Gesendet
containers-stats-disk = Datenträger-E/A
containers-stats-processes = · { $count ->
        [one] { NUM($count) } Prozess
       *[other] { NUM($count) } Prozesse
    }
containers-stats-read = Gelesen
containers-stats-written = Geschrieben
containers-run-title = Container ausführen
containers-run-description = Das Image wird zuerst gepullt, falls dieser Host es nicht hat.
containers-run-submit = Ausführen
containers-run-starting = Wird gestartet…
containers-run-started = Container gestartet
containers-run-image = Image
containers-run-name = Name
containers-run-namePlaceholder = Wird erzeugt, wenn leer
containers-run-nameKept = Beibehalten. Verwenden Sie „Umbenennen“, um ihn zu ändern.
containers-run-command = Befehl
containers-run-commandPlaceholder = Standard des Images
containers-run-environment = Umgebung
containers-run-network = Netzwerk
containers-run-pod = Pod
containers-run-noPod = Keiner
containers-run-podOwnsNetwork = Container in { $pod } teilen sich dessen Netzwerk: Ports, Netzwerk, Hostname und DNS sind die des Pods.
containers-run-defaultNetwork = Standard (bridge)
containers-run-restart = Neustart
containers-run-restartPolicy = Neustart-Richtlinie
containers-run-restart-no = Nie
containers-run-restart-onFailure = Bei Fehler
containers-run-restart-unlessStopped = Außer wenn gestoppt
containers-run-restart-always = Immer
containers-run-autoRemove = Container beim Beenden entfernen
containers-run-ports = Ports
containers-run-hostPort = Host-Port
containers-run-hostPortPlaceholder = Host-Port (beliebig)
containers-run-containerPort = Container-Port
containers-run-protocol = Protokoll
containers-run-removePort = Port entfernen
containers-run-addPort = Port hinzufügen
containers-run-volumes = Volumes
containers-run-source = Quelle
containers-run-sourcePlaceholder = Volume-Name oder /host/pfad
containers-run-containerPath = Container-Pfad
containers-run-readOnly = schreibgeschützt
containers-run-removeVolume = Volume entfernen
containers-run-addVolume = Volume hinzufügen
containers-run-removeValue = { $value } entfernen
containers-run-error-size = Geben Sie eine Größe wie 512m oder 2g an.
containers-run-error-cpus = Geben Sie eine Anzahl CPUs wie 0.5 oder 2 an.
containers-run-error-cpuShares = Geben Sie eine ganze Zahl wie 512 an.
containers-run-error-memorySwap = Geben Sie eine Größe wie 1g an, oder -1 für unbegrenzten Swap.
containers-run-error-gpuCount = Geben Sie an, wie viele GPUs, etwa 1.
containers-run-error-duration = Geben Sie eine Dauer wie 30s oder 1m30s an.
containers-run-error-retries = Geben Sie eine ganze Zahl wie 3 an.
containers-advanced-title = Erweiterte Optionen
containers-advanced-hasErrors = Enthält Fehler
containers-advanced-resources = Ressourcen
containers-advanced-labels = Labels
containers-advanced-security = Sicherheit
containers-advanced-devices = Geräte
containers-advanced-healthcheck = Healthcheck
containers-advanced-network = Netzwerkidentität
containers-advanced-logging = Protokollierung
containers-advanced-cpus = CPU-Limit
containers-advanced-cpusHint = CPUs, etwa 0.5 oder 2.
containers-advanced-cpusPlaceholder = Alle CPUs
containers-advanced-cpuShares = CPU-Anteile
containers-advanced-cpuSharesHint = Gewichtung gegenüber anderen Containern; 1024 ist der Standard.
containers-advanced-memory = Speicherlimit
containers-advanced-memoryHint = Etwa 512m oder 2g.
containers-advanced-memoryPlaceholder = Kein Limit
containers-advanced-memoryReservation = Speicherreservierung
containers-advanced-memoryReservationHint = Ein weiches Limit, das eingehalten wird, wenn der Host knapp wird.
containers-advanced-memorySwap = Limit für Speicher und Swap
containers-advanced-memorySwapHint = Speicher plus Swap; -1 für unbegrenzten Swap.
containers-advanced-memorySwapPlaceholder = Das Doppelte des Speicherlimits
containers-advanced-none = Keine
containers-advanced-labelsHint = Ein key=value pro Zeile, so wie Reverse Proxys und Backup-Tools sie lesen.
containers-advanced-capAdd = Capabilities hinzufügen
containers-advanced-capAddHint = Etwa NET_ADMIN für ein VPN.
containers-advanced-capDrop = Capabilities entziehen
containers-advanced-capDropHint = ALL entzieht alle.
containers-advanced-privileged = Privilegiert
containers-advanced-privilegedTitle = Der Container erhält vollen Zugriff auf den Host
containers-advanced-privilegedWarning = Er kann jedes Gerät und jede Capability nutzen, und alles, was darin läuft, kann den Host übernehmen. Fügen Sie besser die Capabilities und Geräte hinzu, die er braucht.
containers-advanced-hostDevice = Host-Gerät
containers-advanced-containerDevice = Container-Gerät
containers-advanced-samePath = Gleicher Pfad
containers-advanced-permissions = Berechtigungen
containers-advanced-removeDevice = Gerät entfernen
containers-advanced-addDevice = Gerät hinzufügen
containers-advanced-gpus = GPUs
containers-advanced-gpusHint = Erfordert das auf dem Host installierte Container-Toolkit des GPU-Herstellers.
containers-advanced-gpus-all = Alle GPUs
containers-advanced-gpus-count = Eine Anzahl GPUs
containers-advanced-gpus-ids = Ausgewählte GPUs
containers-advanced-gpuCount = Wie viele
containers-advanced-gpuIds = GPU-Indizes oder UUIDs
containers-advanced-gpuIdsHint = Durch Kommas getrennt.
containers-advanced-health-image = Der des Images, falls vorhanden
containers-advanced-health-custom = Ein Befehl
containers-advanced-health-disabled = Keiner (den des Images abschalten)
containers-advanced-healthCommand = Prüfbefehl
containers-advanced-healthCommandHint = Mit /bin/sh -c ausgeführt; Exit-Code 0 bedeutet gesund.
containers-advanced-interval = Intervall
containers-advanced-timeout = Timeout
containers-advanced-retries = Wiederholungen
containers-advanced-startPeriod = Startphase
containers-advanced-hostname = Hostname
containers-advanced-hostnamePlaceholder = Die Container-ID
containers-advanced-domainname = Domainname
containers-advanced-dns = DNS-Server
containers-advanced-dnsHint = Eine Adresse pro Zeile.
containers-advanced-dnsSearch = DNS-Suchdomains
containers-advanced-dnsSearchHint = Eine pro Zeile.
containers-advanced-extraHosts = Zusätzliche Hosts
containers-advanced-extraHostsHint = Ein name:adresse pro Zeile; host-gateway steht für die Adresse des Hosts.
containers-advanced-logDriver = Log-Treiber
containers-advanced-logDriverHint = Mit none bleibt der Tab „Logs“ leer.
containers-advanced-logDriverPlaceholder = Standard der Engine
containers-advanced-logOptions = Treiberoptionen
containers-advanced-logOptionsHint = Ein key=value pro Zeile; max-size und max-file rotieren json-file-Logs.
containers-rename-title = { $name } umbenennen
containers-rename-newName = Neuer Name
containers-duplicate-title = { $name } duplizieren
containers-duplicate-sharedPorts = { $name } veröffentlicht bereits { $count ->
        [one] den Host-Port
       *[other] die Host-Ports
    } { $ports }.
containers-duplicate-sharedPortsFix = { $count ->
        [one] Die Kopie kann nicht starten, solange beide ihn verwenden
       *[other] Die Kopie kann nicht starten, solange beide sie verwenden
    }: Ändern Sie den Host-Port oder lassen Sie ihn leer, damit die Engine einen wählt.
containers-edit-title = { $name } bearbeiten
containers-edit-description = Beim Anwenden wird der Container mit diesen Einstellungen neu erstellt; Name, Netzwerke und benannte Volumes bleiben erhalten.
containers-edit-apply = Anwenden
containers-edit-recreating = Wird neu erstellt…
containers-edit-recreated = Container neu erstellt
containers-edit-confirmTitle = { $name } neu erstellen?
containers-edit-nothingLost = Sonst geht nichts verloren: Name, Netzwerke und benannte Volumes bleiben erhalten.
containers-edit-keptBut = Name, Netzwerke und benannte Volumes bleiben erhalten, nicht jedoch:
containers-edit-commitFirst = Zuerst als Image committen…
containers-edit-recreate = Neu erstellen
containers-loss-anonymousVolume = Das anonyme Volume unter { $path }: Der neue Container startet mit einem leeren. Das alte bleibt als unbenutztes Volume erhalten.
containers-loss-filesystemChanges = { $count ->
        [one] { NUM($count) } Datei wurde
       *[other] { NUM($count) } Dateien wurden
    } im Container seit seiner Erstellung geändert. Committen Sie ihn zuerst als Image, um sie zu behalten.
containers-loss-downtime = Er stoppt, während die beiden ausgetauscht werden, meist für einige Sekunden.
containers-loss-stackRedeploy = Er gehört zum Stack { $stack }: Die nächste Bereitstellung des Stacks erstellt ihn aus seiner Compose-Datei neu, ohne diese Änderung.
containers-commit-title = { $name } als Image committen
containers-commit-description = Das neue Image enthält die Dateien des Containers im jetzigen Zustand, einschließlich darin vorgenommener Änderungen, und seine Einstellungen. Volumes sind nicht enthalten.
containers-commit-imageName = Image-Name
containers-commit-imageNameHint = Etwa repo:tag.
containers-commit-nameTaken = Ein Image heißt bereits { $name }. Der Name wechselt zum neuen Image; das alte behält seine anderen Namen oder wird ungetaggt.
containers-commit-message = Nachricht
containers-commit-messagePlaceholder = Vor dem Upgrade
containers-commit-author = Autor
containers-commit-changes = Änderungen
containers-commit-changesHint = Dockerfile-Anweisungen, eine pro Zeile: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME oder WORKDIR.
containers-commit-pause = Container während des Commits pausieren
containers-commit-pauseOn = Seine Prozesse werden für die wenigen Sekunden des Commits eingefroren, damit gerade geschriebene Dateien konsistent sind.
containers-commit-pauseOff = Er läuft weiter, daher werden gerade geschriebene Dateien womöglich halb erfasst.
containers-commit-submit = Committen
containers-commit-committing = Wird committet…
containers-commit-done = Als { $reference } committet
containers-commit-openImages = Images öffnen
containers-export-tarArchive = Tar-Archiv
containers-export-pick = Dateisystem von { $name } exportieren
containers-export-title = { $name } wird exportiert
containers-export-to = Nach { $path }
containers-export-written = Geschrieben
containers-export-finished = Fertig. Importieren Sie es auf der Seite „Images“ oder mit docker import.
containers-export-done = Nach { $destination } exportiert
containers-palette-run = Container ausführen
