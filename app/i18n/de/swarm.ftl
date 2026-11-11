### The `swarm` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/swarm.ts:
### change the text there, then run it again.

swarm-nav = Swarms
swarm-badge = Swarm
swarm-reachable = Ein Manager ist verbunden
swarm-unreachable = Kein Manager verbunden
swarm-hostCounts = { $managers ->
        [one] { NUM($managers) } Manager-Host
       *[other] { NUM($managers) } Manager-Hosts
    }, { $workers ->
        [one] { NUM($workers) } Worker-Host
       *[other] { NUM($workers) } Worker-Hosts
    }
swarm-host-managerOf = Manager von { $swarm }
swarm-host-workerOf = Worker von { $swarm }
swarm-role-manager = Manager
swarm-role-worker = Worker
swarm-tab-summary = Zusammenfassung
swarm-tab-nodes = Nodes
swarm-tab-services = Services
swarm-tab-tasks = Tasks
swarm-tab-stacks = Stacks
swarm-tab-secrets = Secrets
swarm-tab-configs = Configs
swarm-summary-title = Dieser Swarm
swarm-summary-managerHosts = Manager-Hosts
swarm-summary-workerHosts = Worker-Hosts
swarm-summary-nodes = Nodes
swarm-summary-nodeCounts = { $nodes ->
        [one] { NUM($nodes) } Node
       *[other] { NUM($nodes) } Nodes
    }, { $managers ->
        [one] { NUM($managers) } Manager
       *[other] { NUM($managers) } Manager
    }
swarm-summary-services = Services
swarm-summary-created = Erstellt
swarm-summary-id = Swarm-ID
swarm-nodes-search = Nodes durchsuchen
swarm-nodes-column-hostname = Node
swarm-nodes-column-role = Rolle
swarm-nodes-column-availability = Verfügbarkeit
swarm-nodes-column-state = Zustand
swarm-nodes-column-engine = Engine
swarm-nodes-column-address = Adresse
swarm-nodes-column-labels = Labels
swarm-nodes-isHost = Host { $name }
swarm-nodes-availability-active = Aktiv
swarm-nodes-availability-pause = Pausiert
swarm-nodes-availability-drain = Geleert
swarm-nodes-manager-leader = Leader
swarm-nodes-manager-reachable = erreichbar
swarm-nodes-manager-unreachable = nicht erreichbar
swarm-nodes-actionsFor = Aktionen für { $name }
swarm-nodes-action-activate = Aktivieren
swarm-nodes-action-pause = Pausieren
swarm-nodes-action-drain = Leeren
swarm-nodes-action-promote = Zum Manager befördern
swarm-nodes-action-demote = Zum Worker herabstufen
swarm-nodes-action-labels = Labels bearbeiten
swarm-nodes-action-tasks = Seine Tasks anzeigen
swarm-nodes-updated = { $name } aktualisiert
swarm-nodes-removed = { $name } aus dem Swarm entfernt
swarm-nodes-labels-title = Labels von { $name }
swarm-nodes-labels-description = Platzierungsbedingungen wie node.labels.zone==a wählen Nodes anhand dieser Labels aus.
swarm-nodes-labels-key = Schlüssel
swarm-nodes-labels-value = Wert
swarm-nodes-labels-add = Label hinzufügen
swarm-nodes-labels-removeOne = Label { $key } entfernen
swarm-nodes-remove-title = { $name } aus dem Swarm entfernen?
swarm-nodes-remove-ready = Dieser Node läuft noch. Wird er zwangsweise entfernt, glaubt seine Engine, sie sei im Swarm, bis sie ihn von selbst verlässt; seine Tasks werden woanders neu eingeplant.
swarm-nodes-remove-down = Dieser Node ist ausgefallen. Seine Tasks werden auf den anderen Nodes neu eingeplant.
swarm-nodes-remove-force = Erzwingen: entfernen, obwohl er läuft
swarm-services-search = Services durchsuchen
swarm-services-empty = Auf diesem Swarm laufen keine Services
swarm-services-column-name = Name
swarm-services-column-image = Image
swarm-services-column-mode = Modus
swarm-services-column-replicas = Laufend
swarm-services-column-ports = Ports
swarm-services-column-stack = Stack
swarm-services-column-update = Update
swarm-services-mode-replicated = Repliziert
swarm-services-mode-replicatedCount = Repliziert ({ $count })
swarm-services-mode-global = Global
swarm-services-mode-replicatedJob = Job
swarm-services-mode-globalJob = Globaler Job
swarm-services-actionsFor = Aktionen für { $name }
swarm-services-action-logs = Logs
swarm-services-action-scale = Skalieren
swarm-services-action-rollback = Zurücksetzen
swarm-services-scaled = { $name } auf { $count ->
        [one] { NUM($count) } Replikat
       *[other] { NUM($count) } Replikate
    } skaliert
swarm-services-rolledBack = { $name } wird zurückgesetzt
swarm-services-removed = { $name } entfernt
swarm-services-scale-title = { $name } skalieren
swarm-services-scale-description = Der Swarm startet oder stoppt Tasks, bis so viele laufen. 0 stoppt den Service, ohne ihn zu entfernen.
swarm-services-scale-replicas = Replikate
swarm-services-scale-invalid = Geben Sie eine ganze Zahl von 0 bis 10000 ein.
swarm-services-rollback-title = { $name } zurücksetzen?
swarm-services-rollback-description = Seine vorherigen Einstellungen (Image, Umgebung, Replikate…) werden wiederhergestellt, und seine Tasks gehen wie bei einem Update darauf über.
swarm-services-remove-title = Service { $name } entfernen?
swarm-services-remove-description = { $count ->
        [0] Er führt derzeit keinen Task aus.
        [one] Sein laufender Task wird gestoppt und entfernt.
       *[other] Seine { NUM($count) } laufenden Tasks werden gestoppt und entfernt.
    } Seine Volumes bleiben erhalten.
swarm-service-tab-tasks = Tasks
swarm-service-tab-logs = Logs
swarm-service-tab-inspect = Untersuchen
swarm-service-gone = Dieser Service existiert nicht mehr.
swarm-service-backToList = Zurück zu den Services
swarm-tasks-column-task = Task
swarm-tasks-column-node = Node
swarm-tasks-column-state = Zustand
swarm-tasks-column-error = Meldung
swarm-tasks-column-image = Image
swarm-tasks-desired = sollte { $state } sein
swarm-tasks-openContainer = Container öffnen
swarm-tasks-history = Verlauf anzeigen
swarm-tasks-onNode = Tasks auf { $name }
swarm-tasks-allNodes = Alle Nodes anzeigen
swarm-tasks-empty = Keine Tasks
swarm-objects-searchSecrets = Secrets durchsuchen
swarm-objects-searchConfigs = Configs durchsuchen
swarm-objects-createSecret = Secret erstellen
swarm-objects-createConfig = Config erstellen
swarm-objects-secretNote = Der Wert eines Secrets geht einmal an den Swarm und kann nicht zurückgelesen werden, weder von Homeport noch von sonst jemandem: Nur die Services, denen es übergeben wird, sehen ihn.
swarm-objects-noSecrets = Keine Secrets
swarm-objects-noConfigs = Keine Configs
swarm-objects-column-name = Name
swarm-objects-column-services = Verwendet von
swarm-objects-column-labels = Labels
swarm-objects-column-created = Erstellt
swarm-objects-column-updated = Aktualisiert
swarm-objects-unused = Unbenutzt
swarm-objects-viewOne = { $name } anzeigen
swarm-objects-removeOne = { $name } entfernen
swarm-objects-removed = { $name } entfernt
swarm-objects-remove-title = { $name } entfernen?
swarm-objects-remove-used = Der Swarm lehnt ab, solange Services es verwenden: { $services }. Entfernen Sie es zuerst aus ihnen.
swarm-objects-remove-unused = Kein Service verwendet es. Dies lässt sich nicht rückgängig machen.
swarm-objects-binary = Binärer Inhalt, { $size ->
        [one] { NUM($size) } Byte
       *[other] { NUM($size) } Bytes
    }.
swarm-objects-create-secretTitle = Secret erstellen
swarm-objects-create-configTitle = Config erstellen
swarm-objects-create-secretDescription = Der Wert wird an den Swarm gesendet und nie von Homeport aufbewahrt. Services lesen ihn als Datei unter /run/secrets/.
swarm-objects-create-configDescription = Services lesen sie als Datei, wo immer sie sie einhängen. Configs lassen sich später erneut anzeigen.
swarm-objects-create-name = Name
swarm-objects-create-secretPlaceholder = db_password
swarm-objects-create-configPlaceholder = nginx_conf
swarm-objects-create-value = Wert
swarm-objects-create-content = Inhalt
swarm-objects-create-show = Anzeigen
swarm-objects-create-hide = Verbergen
swarm-objects-create-loadFile = Aus Datei laden…
swarm-objects-create-pickFile = Zu sendende Datei wählen
swarm-objects-create-clearFile = Stattdessen den Wert eingeben
swarm-objects-create-fileNote = Die Datei wird beim Erstellen gelesen, an den Swarm gesendet und nicht aufbewahrt.
swarm-services-created = { $name } erstellt
swarm-services-updating = { $name } geht auf seine neuen Einstellungen über
swarm-service-update = Update { $state }
swarm-create-open = Service erstellen
swarm-create-title = Service erstellen
swarm-create-description = Der Swarm pullt das Image auf den Nodes, die er wählt, und hält so viele Tasks am Laufen. Platzierung, Ressourcenlimits und Healthchecks lassen sich mit der Docker-CLI setzen und werden unter „Untersuchen“ angezeigt.
swarm-create-image = Image
swarm-create-imagePlaceholder = nginx:1.27
swarm-create-name = Name
swarm-create-mode = Modus
swarm-create-restart = Neustart
swarm-create-restart-default = Immer (Standard)
swarm-create-restart-onFailure = Bei Fehler
swarm-create-restart-none = Nie
swarm-create-command = Befehl
swarm-create-commandPlaceholder = Der eigene Befehl des Images
swarm-create-env = Umgebung
swarm-create-hostMode = Nur auf dem Node veröffentlichen, auf dem der jeweilige Task läuft, statt auf jedem Node
swarm-create-networks = Overlay-Netzwerke
swarm-create-secrets = Secrets
swarm-create-configs = Configs
swarm-create-mountedAt = Wo { $name } eingehängt wird
swarm-create-secretTarget = /run/secrets/{ $name }
swarm-create-configTarget = /{ $name }
swarm-create-submit = Erstellen
swarm-edit-open = Bearbeiten
swarm-edit-title = { $name } bearbeiten
swarm-edit-description = Nur was Sie hier ändern, wird gesendet; der Rest des Service bleibt erhalten. Der Swarm ersetzt seine Tasks nach und nach durch solche mit den neuen Einstellungen.
swarm-edit-redeploy = Alle Tasks neu bereitstellen und das Image erneut pullen, auch wenn sein Tag gleich ist
swarm-edit-submit = Aktualisieren
swarm-membership-card-title = Swarm
swarm-membership-card-none = Dieser Host ist in keinem Swarm.
swarm-membership-card-state = Sein Node ist { $state }.
swarm-membership-card-ports = Starten Sie hier einen Swarm oder treten Sie einem bei. Die Nodes eines Swarms müssen sich gegenseitig über 2377/tcp, 7946/tcp und udp sowie 4789/udp erreichen; Homeport kann diese Ports nicht für Sie öffnen.
swarm-membership-created = Swarm erstellt
swarm-membership-joined = Dem Swarm beigetreten
swarm-membership-left = Swarm verlassen
swarm-membership-advertise = Advertise-Adresse
swarm-membership-advertisePlaceholder = Von der Engine gewählt
swarm-membership-advertiseHelp = Die Adresse, unter der andere Nodes diesen Host erreichen. Lassen Sie sie leer, damit die Engine sie wählt; das funktioniert, wenn der Rechner nur eine Adresse hat.
swarm-membership-listen = Listen-Adresse
swarm-membership-init-open = Swarm initialisieren
swarm-membership-init-title = Swarm auf { $name } starten
swarm-membership-init-description = Dieser Host wird der erste Manager eines neuen Swarms. Andere Hosts können ihm dann als Worker oder Manager beitreten.
swarm-membership-init-submit = Swarm starten
swarm-membership-join-open = Swarm beitreten
swarm-membership-join-title = { $name } einem Swarm beitreten lassen
swarm-membership-join-description = Wählen Sie einen Swarm, den Homeport kennt, und Homeport holt sein Join-Token von einem verbundenen Manager. Fügen Sie für einen anderen Swarm das Token und die Adresse eines Managers aus docker swarm join-token ein.
swarm-membership-join-swarm = Swarm
swarm-membership-join-paste = Ein anderer Swarm, über sein Token
swarm-membership-join-token = Join-Token
swarm-membership-join-address = Manager-Adresse
swarm-membership-join-addressPlaceholder = 192.168.1.10:2377
swarm-membership-join-as = Beitreten als
swarm-membership-join-submit = Beitreten
swarm-membership-leave-open = Swarm verlassen
swarm-membership-leave-title = { $name } seinen Swarm verlassen lassen?
swarm-membership-leave-worker = Seine Tasks stoppen hier und werden auf den anderen Nodes neu eingeplant. Sein Node bleibt als ausgefallen gelistet, bis ein Manager ihn entfernt.
swarm-membership-leave-manager = Er ist dann weder Manager noch Node. Die anderen Manager halten den Swarm am Laufen; ihn vorher herabzustufen ist schonender.
swarm-membership-leave-last = Dies ist der letzte Manager des Swarms: Der Swarm verschwindet mit ihm, samt allen Services, Secrets und Configs. Andere Nodes bleiben in einem Swarm ohne Manager zurück.
swarm-membership-leave-force = Erzwingen: verlassen, obwohl er ein Manager ist
swarm-membership-leave-typeName = Geben Sie zur Bestätigung { $name } ein
swarm-membership-leave-submit = Verlassen
swarm-join-title = Beitrittsbefehle
swarm-join-description = Für Rechner, die Homeport nicht verwaltet: Führen Sie dort einen dieser Befehle aus. Von Homeport verwaltete Hosts treten stattdessen über ihre Seite bei, ohne etwas zu kopieren.
swarm-join-reveal = Beitrittsbefehle anzeigen
swarm-join-hide = Ausblenden
swarm-join-worker = Als Worker
swarm-join-manager = Als Manager
swarm-stacks-search = Stacks durchsuchen
swarm-stacks-new = Neuer Stack
swarm-stacks-deploy = Bereitstellen
swarm-stacks-redeploy = Neu bereitstellen
swarm-stacks-adopt = Übernehmen
swarm-stacks-link = Mit einem Ordner verknüpfen…
swarm-stacks-linkTitle = Ordner mit den Compose-Dateien von { $name } wählen
swarm-stacks-linked = { $name } ist jetzt ein verknüpfter Stack
swarm-stacks-empty = Keine Stacks auf diesem Swarm
swarm-stacks-emptyHint = Mit docker stack deploy bereitgestellte Stacks erscheinen hier, ebenso die, die Sie aus Homeport in diesem Swarm bereitstellen.
swarm-stacks-backToList = Zurück zu den Stacks
swarm-stacks-services = Services
swarm-stacks-noServices = Es ist noch nichts bereitgestellt.
swarm-stacks-parts = Netzwerke, Secrets und Configs
swarm-stacks-networks = Netzwerke
swarm-stacks-noStartStop = Ein Swarm-Stack lässt sich nicht als Ganzes stoppen oder starten: Skalieren Sie stattdessen seine Services auf ihren Seiten auf 0.
swarm-stacks-removed = { $name } entfernt
swarm-stacks-removedPartly = { $name } entfernt, außer:
swarm-stacks-remove-title = Stack { $name } entfernen?
swarm-stacks-remove-description = Seine Services werden entfernt, dann seine Netzwerke, Secrets und Configs, wie bei docker stack rm. Seine Volumes bleiben auf jedem Node erhalten.
