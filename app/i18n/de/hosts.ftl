### The `hosts` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/hosts.ts:
### change the text there, then run it again.

hosts-title = Hosts
hosts-description = Die Docker Engines, mit denen Homeport sich verbindet.
hosts-add = Host hinzufügen
hosts-empty = Noch keine Hosts.
hosts-column-name = Name
hosts-column-type = Typ
hosts-column-address = Adresse
hosts-column-status = Status
hosts-statusTitle = { $label }: { $detail }
hosts-saved = Host gespeichert
hosts-notFound = Dieser Host existiert nicht mehr.
hosts-addPage-description = Verbinden Sie Homeport mit einer Docker Engine.
hosts-switcher-label = Host
hosts-switcher-placeholder = Kein Host
hosts-switcher-add = Host hinzufügen…
hosts-switcher-manage = Hosts verwalten
hosts-switcher-hosts = Hosts
hosts-switcher-clusters = Cluster
hosts-switcher-addCluster = Cluster hinzufügen…
hosts-switcher-manageClusters = Cluster verwalten
hosts-switcher-whyUnusable = Warum { $name } nicht verwendet werden kann
hosts-detail-enabled = Aktiviert
hosts-detail-reconnect = Neu verbinden
hosts-connection-title = Verbindung
hosts-remove-title = Host entfernen
hosts-remove-description = Homeport vergisst diesen Host, seine Zugangsdaten und die Definitionen seiner verwalteten Stacks. Nichts, was auf dem Host läuft, wird angetastet.
hosts-remove-confirmTitle = { $name } entfernen?
hosts-remove-confirmDescription = Löscht die gespeicherte Verbindung, den SSH-Schlüssel oder die Zertifikate und die Compose-Definitionen seiner verwalteten Stacks. Container, Images und Volumes auf dem Host bleiben, wie sie sind.
hosts-sshSetup-title = Homeport auf dem Server autorisieren
hosts-sshSetup-description = Führen Sie diese Befehle auf <mono>{ $server }</mono> als SSH-Benutzer aus und verbinden Sie dann neu.
hosts-sshSetup-descriptionBastions = Homeport meldet sich bei jedem dieser Server mit demselben Schlüssel an. Fügen Sie ihn auf jedem als dessen SSH-Benutzer hinzu und verbinden Sie dann neu.
hosts-sshSetup-servers = Hinzufügen auf:
hosts-sshSetup-addKey = 1. Öffentlichen Schlüssel hinzufügen
hosts-sshSetup-dockerGroup = 2. Dem Benutzer Zugriff auf den Socket der Engine geben (falls nötig)
hosts-sshSetup-fingerprint = Schlüssel-Fingerabdruck <mono>{ $fingerprint }</mono>
hosts-regenerate-button = Schlüssel neu erzeugen
hosts-regenerate-title = SSH-Schlüssel neu erzeugen?
hosts-regenerate-description = Der aktuelle Schlüssel funktioniert nicht mehr. Sie müssen den neuen öffentlichen Schlüssel auf dem Server hinzufügen.
hosts-regenerate-confirm = Neu erzeugen
hosts-trust-untrusted-title = Ist das der richtige Server?
hosts-trust-untrusted-description = Erste Verbindung. Der Server weist sich mit dem Schlüssel-Fingerabdruck <mono>{ $fingerprint }</mono> aus. Vergleichen Sie ihn mit <mono>{ $command }</mono> auf dem Server.
hosts-trust-untrusted-button = Diesem Schlüssel vertrauen
hosts-trust-untrusted-bastionTitle = Ist das die richtige Bastion?
hosts-trust-untrusted-bastionDescription = Erste Verbindung zur Bastion <mono>{ $bastion }</mono>. Sie weist sich mit dem Schlüssel-Fingerabdruck <mono>{ $fingerprint }</mono> aus. Vergleichen Sie ihn mit <mono>{ $command }</mono> auf der Bastion.
hosts-trust-mismatch-bastionTitle = Der Schlüssel der Bastion { $bastion } hat sich geändert
hosts-trust-mismatch-title = Der Schlüssel des Servers hat sich geändert
hosts-trust-mismatch-description = Erwartet wurde <mono>{ $expected }</mono>, aber der Server hat <mono>{ $fingerprint }</mono> vorgelegt. Das passiert nach einer Neuinstallation des Servers, kann aber auch bedeuten, dass jemand die Verbindung abfängt.
hosts-trust-mismatch-button = Dem neuen Schlüssel vertrauen
hosts-trust-mismatch-confirmTitle = Dem neuen Serverschlüssel vertrauen?
hosts-trust-mismatch-confirmDescription = Tun Sie dies nur, wenn Sie wissen, warum sich der Schlüssel geändert hat. Neuer Fingerabdruck: { $fingerprint }.
hosts-trust-mismatch-confirm = Neuem Schlüssel vertrauen
hosts-kind-ssh = SSH
hosts-kind-tls = TLS
hosts-kind-tcp = TCP
hosts-kind-local = Lokaler Socket
hosts-form-keepHint = Leer lassen, um den gespeicherten Wert beizubehalten.
hosts-form-name = Name
hosts-form-engine = Engine
hosts-form-engineHint = Docker-kompatibel umfasst Docker und Podman; Homeport unterscheidet sie beim Verbinden. containerd wird über seinen Socket erreicht, hier oder über SSH; Incus auch über HTTPS (TLS).
hosts-form-socketOnly = { $engine } antwortet nur auf seinem Socket: Verbinden Sie lokal oder über SSH.
hosts-form-noPlainTcp = { $engine } antwortet nicht über unverschlüsseltes TCP: Verwenden Sie TLS, SSH oder seinen Socket.
hosts-form-socketPath = Socket-Pfad
hosts-form-socketPathHint = Ein Unix-Socket oder unter Windows eine Named Pipe wie { $pipe }.
hosts-form-address = Adresse
hosts-form-addressHint = host:port, zum Beispiel { $example }.
hosts-form-tcpWarning-title = Unverschlüsselt und nicht authentifiziert
hosts-form-tcpWarning-description = Wer diesen Port erreichen kann, kontrolliert die Engine. Verwenden Sie außerhalb eines vertrauenswürdigen Netzwerks besser SSH oder TLS.
hosts-form-loadFromFile = Aus Datei laden…
hosts-incus-trust = Vertrauen
hosts-incus-trust-token = Trust-Token
hosts-incus-trust-certificate = Vorhandenes Zertifikat
hosts-incus-token = Trust-Token
hosts-incus-tokenHint = Führen Sie dies auf dem Incus-Server aus und fügen Sie die Ausgabe ein. Homeport erstellt ein eigenes Zertifikat, prüft das des Servers anhand des Tokens und registriert sich selbst.
hosts-incus-tokenReads = Für { $name }, unter { $addresses }
hosts-incus-tokenFingerprint = Serverzertifikat { $fingerprint }
hosts-incus-tokenExpires = Läuft ab { $time }
hosts-incus-serverCert = Zu vertrauendes Serverzertifikat
hosts-incus-serverCert-choose = Serverzertifikat wählen
hosts-incus-serverCertHint = Das eigene Zertifikat von Incus (/var/lib/incus/server.crt). Homeport akzeptiert genau dieses, unabhängig von der Adresse.
hosts-incus-clientCert = Client-Zertifikat
hosts-incus-clientCertHint = Bereits von Incus als vertrauenswürdig eingestuft: incus config trust add-certificate.
hosts-incus-clientKey = Client-Schlüssel
hosts-keychain-title = Kein Schlüsselbund des Betriebssystems
hosts-keychain-description = Schlüssel und Zertifikate werden mit einem Schlüssel verschlüsselt, der im Schlüsselbund des Betriebssystems liegt, und es ist keiner verfügbar. Verwenden Sie stattdessen ssh-agent, einen lokalen Socket oder TCP.
hosts-ssh-server = Server
hosts-ssh-port = Port
hosts-ssh-user = Benutzer
hosts-ssh-userHint = Benötigt Zugriff auf den Docker-Socket, meist durch Mitgliedschaft in der Gruppe docker.
hosts-ssh-socketPath = Entfernter Socket-Pfad
hosts-ssh-socketPathHint = Rootless Docker verwendet { $path }.
hosts-ssh-auth = Authentifizierung
hosts-ssh-auth-generated = Neuer Schlüssel für diesen Host
hosts-ssh-auth-imported = Vorhandener Schlüssel
hosts-ssh-generated-editing = Dieser Host behält seinen Schlüssel. Erzeugen Sie ihn auf der Host-Seite neu.
hosts-ssh-generated-new = Homeport erstellt einen Ed25519-Schlüssel für diesen Host und zeigt Ihnen den Befehl, der ihn auf dem Server autorisiert.
hosts-ssh-privateKey = Privater Schlüssel
hosts-ssh-privateKey-choose = Privaten Schlüssel wählen
hosts-ssh-privateKeyHint = Verschlüsselt gespeichert; er verlässt diesen Rechner nie.
hosts-ssh-passphrase = Passphrase
hosts-bastions-title = Bastionen
hosts-bastions-description = Server, bei denen Homeport sich zuerst der Reihe nach anmeldet, wenn dieser nur SSH von ihnen annimmt (ProxyJump).
hosts-bastions-add = Bastion hinzufügen
hosts-bastions-one = Bastion { $n }
hosts-bastions-server = Server der Bastion { $n }
hosts-bastions-port = Port der Bastion { $n }
hosts-bastions-user = Benutzer der Bastion { $n }
hosts-bastions-moveUp = Bastion { $n } nach oben verschieben
hosts-bastions-moveDown = Bastion { $n } nach unten verschieben
hosts-bastions-remove = Bastion { $n } entfernen
hosts-bastions-auth-sameAsHost = Wie der Host
hosts-bastions-auth-imported = Eigener Schlüssel
hosts-bastions-auth-agent = ssh-agent
hosts-import-title = Importieren
hosts-import-description = Fügen Sie die Server hinzu, die Ihre SSH-Konfiguration beschreibt, oder die Engines Ihrer Docker-Kontexte.
hosts-import-sshConfig-button = Aus ~/.ssh/config importieren
hosts-import-sshConfig-title = Aus ~/.ssh/config importieren
hosts-import-sshConfig-description = Alle Server, die Ihre SSH-Konfiguration nennt, mit Benutzer, Port, Schlüssel und ProxyJump-Bastionen. Schlüssel werden einmal gelesen und verschlüsselt gespeichert; die Datei wird nicht geändert.
hosts-import-sshConfig-empty = Ihre SSH-Konfiguration nennt keinen Server ohne Platzhalter.
hosts-import-dockerContexts-button = Docker-Kontexte importieren
hosts-import-dockerContexts-title = Docker-Kontexte importieren
hosts-import-dockerContexts-description = Die Kontexte der Docker-CLI mit ihren TLS-Zertifikaten. Zertifikate werden einmal gelesen und verschlüsselt gespeichert; ~/.docker wird nicht geändert.
hosts-import-dockerContexts-empty = Die Docker-CLI hat außer default keinen Kontext.
hosts-import-column-name = Host-Name
hosts-import-column-address = Adresse und Socket
hosts-import-column-auth = Meldet sich an mit
hosts-import-column-notes = Hinweise
hosts-import-pick = { $name } importieren
hosts-import-name = Host-Name für { $name }
hosts-import-passphrase = Passphrase für den Schlüssel von { $name }
hosts-import-socket = Entfernter Socket-Pfad von { $name }
hosts-import-via = über { $bastions }
hosts-import-nameTaken = Ein Host hat bereits diesen Namen; benennen Sie ihn um, um ihn zu importieren.
hosts-import-auth-key = seinem Schlüssel
hosts-import-auth-agent = ssh-agent
hosts-import-auth-certificates = TLS-Zertifikaten
hosts-import-auth-none = nichts
hosts-import-problem-nameEmpty = Geben Sie ihm einen Namen.
hosts-import-problem-nameTwice = Zwei Hosts würden diesen Namen erhalten.
hosts-import-problem-nameTaken = Wählen Sie einen anderen Namen.
hosts-import-problem-passphrase = Geben Sie die Passphrase des Schlüssels ein.
hosts-import-problem-socket = Der Socket-Pfad muss absolut sein.
hosts-import-submit = { $count ->
        [0] Importieren
        [one] { NUM($count) } Host importieren
       *[other] { NUM($count) } Hosts importieren
    }
hosts-import-failed = { $name } wurde nicht importiert:
hosts-import-done = { $count ->
        [one] { NUM($count) } Host importiert
       *[other] { NUM($count) } Hosts importiert
    }
hosts-bundle-fileType = Homeport-Export
hosts-bundle-passphrase = Passphrase
hosts-bundle-passphraseHint = Mindestens { $min } Zeichen. Homeport bewahrt sie nicht auf: Ohne sie lässt sich die Datei nicht öffnen.
hosts-bundle-confirmation = Passphrase wiederholen
hosts-bundle-problem-short = Verwenden Sie mindestens { $min } Zeichen.
hosts-bundle-problem-mismatch = Die beiden Passphrasen unterscheiden sich.
hosts-bundle-export-button = Exportieren
hosts-bundle-export-title = Hosts exportieren
hosts-bundle-export-description = Schreibt die gewählten Hosts mit ihren Schlüsseln, Zertifikaten und vertrauenswürdigen Serverschlüsseln in eine mit einer Passphrase verschlüsselte Datei. Importieren Sie sie auf einem anderen Computer, um sie dort zu verwenden.
hosts-bundle-export-hosts = Zu exportierende Hosts
hosts-bundle-export-saveTitle = Export speichern
hosts-bundle-export-submit = { $count ->
        [0] Exportieren
        [one] { NUM($count) } Host exportieren…
       *[other] { NUM($count) } Hosts exportieren…
    }
hosts-bundle-export-done = { $count ->
        [one] { NUM($count) } Host exportiert
       *[other] { NUM($count) } Hosts exportiert
    }
hosts-bundle-export-withoutSecrets = Diese wurden ohne ihre Zugangsdaten exportiert, da sich diese auf diesem Computer nicht öffnen lassen:
hosts-bundle-import-button = Importieren
hosts-bundle-import-title = Hosts aus einem Export importieren
hosts-bundle-import-description = Öffnen Sie eine aus Homeport exportierte Datei mit ihrer Passphrase und wählen Sie dann die hinzuzufügenden Hosts. Ihre Zugangsdaten werden verschlüsselt auf diesem Computer gespeichert.
hosts-bundle-import-choose = Datei wählen…
hosts-bundle-import-chooseTitle = Homeport-Export wählen
hosts-bundle-import-open = Öffnen
hosts-bundle-import-renamed = Ein Host heißt bereits { $name }, daher wird dieser umbenannt.
hosts-bundle-import-secretsMissing = Ohne seine Zugangsdaten exportiert; geben Sie sie nach dem Import auf seiner Seite ein.
hosts-tls-ca = CA-Zertifikat
hosts-tls-ca-choose = CA-Zertifikat wählen
hosts-tls-cert = Client-Zertifikat
hosts-tls-cert-choose = Client-Zertifikat wählen
hosts-tls-key = Client-Schlüssel
hosts-tls-key-choose = Client-Schlüssel wählen
hosts-palette-add = Host hinzufügen
hosts-palette-manage = Hosts verwalten
