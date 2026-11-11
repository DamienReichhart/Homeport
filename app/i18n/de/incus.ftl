### The `incus` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/incus.ts:
### change the text there, then run it again.

incus-delete = Löschen
incus-status-running = Läuft
incus-status-stopped = Gestoppt
incus-status-frozen = Eingefroren
incus-status-starting = Wird gestartet
incus-status-stopping = Wird gestoppt
incus-status-freezing = Wird eingefroren
incus-status-error = Fehler
incus-status-unknown = Unbekannt
incus-type-container = Container
incus-type-virtualMachine = Virtuelle Maschine
incus-action-start = Starten
incus-action-stop = Stoppen
incus-action-restart = Neu starten
incus-action-freeze = Einfrieren
incus-action-unfreeze = Auftauen
incus-action-forceStop = Stopp erzwingen
incus-actionOne-start = { $name } starten
incus-actionOne-stop = { $name } stoppen
incus-actionOne-restart = { $name } neu starten
incus-actionOne-freeze = { $name } einfrieren
incus-actionOne-unfreeze = { $name } auftauen
incus-instances-nav = Instanzen
incus-instances-title = Instanzen
incus-instances-summary = { $count ->
        [one] { NUM($count) } Instanz
       *[other] { NUM($count) } Instanzen
    }, { $running } laufen, im Projekt { $project }.
incus-instances-search = Nach Name oder Beschreibung filtern
incus-instances-filter-all = Alle
incus-instances-empty = Keine Instanzen in { $project }
incus-instances-emptyDescription = Systemcontainer und virtuelle Maschinen dieses Projekts erscheinen hier. Wählen Sie unter dem Host-Umschalter ein anderes Projekt, um andere zu sehen.
incus-instances-more = Weitere Aktionen für { $name }
incus-instances-column-name = Name
incus-instances-column-type = Typ
incus-instances-column-status = Status
incus-instances-column-image = Image
incus-instances-column-addresses = Adressen
incus-instances-column-memory = Arbeitsspeicher
incus-instances-column-created = Erstellt
incus-instances-done-start = { $name } gestartet
incus-instances-done-stop = { $name } gestoppt
incus-instances-done-restart = { $name } neu gestartet
incus-instances-done-freeze = { $name } eingefroren
incus-instances-done-unfreeze = { $name } aufgetaut
incus-instances-deleted = { $name } gelöscht
incus-instances-delete-title = { $name } löschen?
incus-instances-delete-description = Die Instanz und ihre Festplatte werden aus dem Projekt { $project } gelöscht. Dies lässt sich nicht rückgängig machen.
incus-instances-delete-snapshots = { $count ->
        [one] Ihr Snapshot wird
       *[other] Ihre { NUM($count) } Snapshots werden
    } mit ihr gelöscht.
incus-instances-delete-snapshotsNamed = { $count ->
        [one] Ihr Snapshot { $names } wird
       *[other] Ihre Snapshots { $names } werden
    } mit ihr gelöscht.
incus-instances-delete-stopFirst = Zuerst stoppen: Sie läuft
incus-instances-delete-stopAndDelete = Stoppen und löschen
incus-instances-backToList = Zurück zu den Instanzen
incus-instances-tab-overview = Übersicht
incus-instances-tab-console = Konsole
incus-instances-tab-exec = Exec
incus-instances-tab-snapshots = Snapshots
incus-instances-tab-config = Konfiguration
incus-instances-tab-devices = Geräte
incus-instances-tab-raw = Roh
incus-instances-field-project = Projekt
incus-instances-field-architecture = Architektur
incus-instances-field-addresses = Adressen
incus-instances-field-cpu = CPU-Zeit
incus-instances-field-memory = Arbeitsspeicher
incus-instances-field-processes = Prozesse
incus-instances-field-created = Erstellt
incus-instances-field-lastUsed = Zuletzt verwendet
incus-instances-field-snapshots = Snapshots
incus-instances-field-location = Cluster-Mitglied
incus-instances-field-description = Beschreibung
incus-instances-cpuSeconds = { $seconds } s
incus-instances-profiles = Profile
incus-instances-noConfig = Keine Konfiguration.
incus-instances-noDevices = Keine Geräte.
incus-instances-config-key = Schlüssel
incus-instances-config-value = Wert
incus-instances-config-fromProfile = Aus einem Profil
incus-progress-waiting = Warten auf Incus…
incus-remote-server = Image-Server
incus-remote-search = Nach System, Release oder Alias filtern
incus-remote-list = Images auf dem Server
incus-remote-none = Kein Image passt.
incus-remote-alias = Alias
incus-images-nav = Images
incus-images-title = Images
incus-images-summary = { $count ->
        [one] { NUM($count) } Image
       *[other] { NUM($count) } Images
    } im Projekt { $project }.
incus-images-search = Nach Alias, Fingerabdruck oder Beschreibung filtern
incus-images-empty = Keine Images in { $project }
incus-images-emptyDescription = Images werden heruntergeladen, wenn eine Instanz daraus erstellt wird, oder vorab mit „Image holen“.
incus-images-get = Image holen
incus-images-get-title = Image holen
incus-images-get-description = Incus lädt es vom Image-Server in das Projekt { $project } herunter, damit Instanzen schnell daraus starten.
incus-images-get-autoUpdate = Vom Server aktuell halten
incus-images-get-keepAlias = Hier denselben Alias vergeben
incus-images-get-confirm = Image holen
incus-images-get-done = Das Image ist bereit.
incus-images-copied = { $alias } nach { $project } kopiert
incus-images-column-aliases = Aliasse
incus-images-column-description = Beschreibung
incus-images-column-type = Typ
incus-images-column-arch = Architektur
incus-images-column-size = Größe
incus-images-column-uploaded = Hinzugefügt
incus-images-cached = zwischengespeichert
incus-images-cachedHint = Zum Erstellen einer Instanz heruntergeladen; Incus löscht es, sobald es eine Weile nicht verwendet wurde.
incus-images-aliasAdd = Alias zu { $name } hinzufügen
incus-images-aliasRemove = Alias { $name } entfernen
incus-images-aliasRemoved = Alias { $name } entfernt
incus-images-alias-title = Alias zu { $name } hinzufügen
incus-images-alias-name = Alias
incus-images-alias-add = Alias hinzufügen
incus-images-refreshOne = { $name } von seinem Server aktualisieren
incus-images-refreshed = { $name } aktualisiert
incus-images-deleteOne = { $name } löschen
incus-images-deleted = { $name } gelöscht
incus-images-delete-title = Image { $name } löschen?
incus-images-delete-description = Daraus erstellte Instanzen funktionieren weiter: Jede hat ihre eigene Kopie. Für neue muss es erneut heruntergeladen werden.
incus-profiles-nav = Profile
incus-profiles-title = Profile
incus-profiles-summary = { $count ->
        [one] { NUM($count) } Profil
       *[other] { NUM($count) } Profile
    } im Projekt { $project }.
incus-profiles-search = Nach Name oder Beschreibung filtern
incus-profiles-empty = Keine Profile in { $project }
incus-profiles-new = Neues Profil
incus-profiles-new-title = Neues Profil
incus-profiles-new-description = Ein leeres Profil im Projekt { $project }. Fügen Sie seine Konfiguration und Geräte hinzu, sobald es erstellt ist.
incus-profiles-new-name = Name
incus-profiles-new-descriptionField = Beschreibung
incus-profiles-created = Profil { $name } erstellt
incus-profiles-column-name = Name
incus-profiles-column-description = Beschreibung
incus-profiles-column-settings = Einstellungen
incus-profiles-column-usedBy = Verwendet von
incus-profiles-settings = { $config ->
        [one] { NUM($config) } Schlüssel
       *[other] { NUM($config) } Schlüssel
    }, { $devices ->
        [one] { NUM($devices) } Gerät
       *[other] { NUM($devices) } Geräte
    }
incus-profiles-defaultKept = Incus behält das Profil default.
incus-profiles-inUse = Verwendet von { $names }: Entfernen Sie es zuerst dort.
incus-profiles-deleteOne = { $name } löschen
incus-profiles-deleted = Profil { $name } gelöscht
incus-profiles-delete-title = Profil { $name } löschen?
incus-profiles-delete-description = Es wird aus dem Projekt { $project } gelöscht. Keine Instanz verwendet es.
incus-profiles-backToList = Zurück zu den Profilen
incus-profiles-noDescription = Keine Beschreibung
incus-profiles-reload = Neu laden
incus-profiles-saved = Profil { $name } gespeichert
incus-profiles-usedBy = Verwendet von
incus-profiles-editHint = Seine Beschreibung, Konfiguration und Geräte, wie incus profile edit sie zeigt. Beim Speichern werden sie auf die Instanzen angewendet, die es verwenden, wie Incus es tut; wenn jemand es seit dem Laden geändert hat, lehnt Homeport ab, statt es zu überschreiben.
incus-profiles-yamlOf = YAML von { $name }
incus-new-action = Neue Instanz
incus-new-title = Neue Instanz
incus-new-description = Ein Systemcontainer oder eine virtuelle Maschine im Projekt { $project }, aus einem Image.
incus-new-name = Name
incus-new-image = Image
incus-new-origin-remote = Von einem Image-Server
incus-new-origin-local = Aus den Images dieses Hosts
incus-new-localImage = Image auf diesem Host
incus-new-localImagePlaceholder = Image wählen
incus-new-profiles = Profile
incus-new-cpu = CPUs
incus-new-memory = Arbeitsspeicher
incus-new-disk = Root-Festplatte
incus-new-limitsHint = Optional: Leer gelassen entscheiden die Profile. Größen werden wie 512MiB oder 20GiB angegeben.
incus-new-start = Nach dem Erstellen starten
incus-new-confirm = Erstellen
incus-new-open = Öffnen
incus-new-made = { $name } erstellt
incus-new-done = { $name } ist bereit.
incus-new-problem-name = Benennen Sie sie mit Buchstaben, Ziffern und Bindestrichen, beginnend mit einem Buchstaben, bis zu 63 Zeichen.
incus-new-problem-image = Wählen Sie ein Image oder geben Sie seinen Alias ein.
incus-new-problem-cpu = CPUs ist eine ganze Zahl ab 1.
incus-new-problem-memory = Arbeitsspeicher wird wie 512MiB oder 2GiB angegeben.
incus-new-problem-disk = Root-Festplatte wird wie 20GiB angegeben.
incus-exec-startFirst = Starten Sie die Instanz, um darin Befehle auszuführen.
incus-exec-command = Befehl
incus-exec-commandPlaceholder = Eine Shell (bash, sonst sh)
incus-exec-user = Benutzer-ID
incus-exec-cwd = Ordner
incus-exec-open = Öffnen
incus-exec-exited = Der Befehl wurde beendet.
incus-exec-exitedWithCode = Der Befehl wurde mit Code { $code } beendet.
incus-console-startFirst = Starten Sie die Instanz, um sich mit ihrer Konsole zu verbinden.
incus-console-hint = Ihre Textkonsole, wie incus console sie zeigt: die Anmeldeaufforderung oder einen laufenden Bootvorgang. Beim Verlassen des Tabs wird die Verbindung getrennt, ohne etwas zu stoppen.
incus-console-reattach = Erneut verbinden
incus-console-detached = Von der Konsole getrennt.
incus-snapshots-take = Snapshot erstellen
incus-snapshots-take-title = Snapshot von { $name } erstellen
incus-snapshots-take-description = Eine Kopie ihrer Festplatte im jetzigen Zustand, um sie später darauf zurückzusetzen: vor einem Upgrade oder einer riskanten Änderung.
incus-snapshots-take-name = Name
incus-snapshots-take-namePlaceholder = snap0, snap1… wenn leer gelassen
incus-snapshots-take-expiry = Aufbewahren
incus-snapshots-take-stateful = Laufzustand einschließen (Arbeitsspeicher)
incus-snapshots-expiry-never = Bis zum Löschen
incus-snapshots-expiry-1d = Einen Tag
incus-snapshots-expiry-7d = Eine Woche
incus-snapshots-expiry-30d = 30 Tage
incus-snapshots-taken = Snapshot von { $name } erstellt
incus-snapshots-empty = Keine Snapshots
incus-snapshots-emptyDescription = Erstellen Sie einen, bevor Sie etwas ändern, das Sie vielleicht rückgängig machen möchten.
incus-snapshots-column-name = Name
incus-snapshots-column-created = Erstellt
incus-snapshots-column-expires = Läuft ab
incus-snapshots-column-size = Größe
incus-snapshots-stateful = mit Zustand
incus-snapshots-restore = Wiederherstellen
incus-snapshots-restore-title = { $name } auf { $snapshot } zurücksetzen?
incus-snapshots-restore-description = Alles, was sich seit { $time }, als { $snapshot } erstellt wurde, auf ihrer Festplatte geändert hat, geht verloren. Eine laufende Instanz startet neu.
incus-snapshots-restored = { $name } auf { $snapshot } zurückgesetzt
incus-snapshots-deleteOne = { $snapshot } löschen
incus-snapshots-delete-title = Snapshot { $snapshot } löschen?
incus-snapshots-delete-description = Die Instanz bleibt, wie sie ist; dieser Snapshot lässt sich dann nicht mehr wiederherstellen.
incus-snapshots-deleted = Snapshot { $snapshot } gelöscht
incus-storage-nav = Speicher
incus-storage-title = Speicher
incus-storage-summary = { $count ->
        [one] { NUM($count) } Speicherpool
       *[other] { NUM($count) } Speicherpools
    }; Volumes des Projekts { $project }.
incus-storage-empty = Keine Speicherpools
incus-storage-emptyDescription = Erstellen Sie einen mit incus storage create oder incus admin init.
incus-storage-usage = Belegter Platz in { $pool }
incus-storage-space = { $used } von { $total } belegt ({ $percent })
incus-storage-spaceUnknown = Sein Treiber meldet nicht, wie voll er ist.
incus-storage-volumesOf = Volumes in { $pool }
incus-storage-noVolumes = Keine Volumes dieses Projekts im Pool.
incus-storage-new = Neues Volume
incus-storage-new-title = Neues Volume in { $pool }
incus-storage-new-description = Ein benutzerdefiniertes Volume des Projekts { $project }, das an Instanzen angehängt wird, für Daten, die sie überdauern.
incus-storage-new-name = Name
incus-storage-new-size = Größe
incus-storage-new-content = Inhalt
incus-storage-content-filesystem = Dateisystem
incus-storage-content-block = Blockgerät
incus-storage-created = Volume { $name } erstellt
incus-storage-column-name = Name
incus-storage-column-type = Typ
incus-storage-column-content = Inhalt
incus-storage-column-size = Größe
incus-storage-column-usedBy = Verwendet von
incus-storage-inUse = Verwendet von { $names }: Trennen Sie es zuerst.
incus-storage-deleteOne = { $name } löschen
incus-storage-delete-title = Volume { $name } löschen?
incus-storage-delete-description = Seine Daten werden aus { $pool } gelöscht. Nichts verwendet es.
incus-storage-deleted = Volume { $name } gelöscht
incus-networks-nav = Netzwerke
incus-networks-title = Netzwerke
incus-networks-summary = { $count ->
        [one] { NUM($count) } Netzwerk
       *[other] { NUM($count) } Netzwerke
    }, die das Projekt { $project } sieht.
incus-networks-search = Nach Name oder Typ filtern
incus-networks-empty = Keine Netzwerke
incus-networks-new = Neue Bridge
incus-networks-new-title = Neue Bridge
incus-networks-new-description = Eine verwaltete Bridge: Incus gibt ihr Adressen, DHCP und DNS, damit Instanzen ihr über ein Profil oder ein Gerät beitreten.
incus-networks-new-name = Name
incus-networks-new-ipv4 = IPv4
incus-networks-new-ipv6 = IPv6
incus-networks-new-addressHint = auto wählt ein freies Subnetz, none schaltet es aus; oder geben Sie die Adresse der Bridge mit Präfix an, etwa 10.0.5.1/24.
incus-networks-new-nat = Ausgehenden IPv4-Verkehr maskieren (NAT)
incus-networks-created = Bridge { $name } erstellt
incus-networks-column-name = Name
incus-networks-column-type = Typ
incus-networks-column-ipv4 = IPv4
incus-networks-column-ipv6 = IPv6
incus-networks-column-usedBy = Verwendet von
incus-networks-managed = Von Incus verwaltet
incus-networks-unmanaged = Von diesem Rechner
incus-networks-inUse = Verwendet von { $names }: Entfernen Sie es zuerst dort.
incus-networks-deleteOne = { $name } löschen
incus-networks-delete-title = Netzwerk { $name } löschen?
incus-networks-delete-description = Incus entfernt die Bridge und ihr DHCP und DNS. Nichts verwendet sie.
incus-networks-deleted = Netzwerk { $name } gelöscht
incus-networks-gone = Es gibt kein Netzwerk { $name } mehr.
incus-networks-backToList = Zurück zu den Netzwerken
incus-networks-field-managed = Erstellt von
incus-networks-field-state = Zustand
incus-networks-field-addresses = Adressen auf dem Host
incus-networks-field-ipv4 = IPv4-Einstellung
incus-networks-field-ipv6 = IPv6-Einstellung
incus-networks-field-nat = NAT
incus-networks-field-mtu = MTU
incus-networks-field-hwaddr = MAC-Adresse
incus-networks-yes = Ja
incus-networks-no = Nein
incus-networks-usedBy = Verwendet von
incus-networks-leases = DHCP-Leases
incus-networks-noLeases = Keine Leases.
incus-networks-lease-hostname = Hostname
incus-networks-lease-address = Adresse
incus-networks-lease-hwaddr = MAC-Adresse
incus-networks-lease-type = Typ
