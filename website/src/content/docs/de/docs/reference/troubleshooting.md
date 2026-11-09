---
title: Fehlerbehebung
description: Lösungen für die Probleme, auf die Sie am ehesten stoßen, geordnet nach der Meldung, die Homeport anzeigt.
---

Steht Ihr Problem nicht hier, prüfen Sie die [Logdatei](/docs/reference/security/#logs) und [eröffnen Sie ein Issue](https://github.com/DamienReichhart/Homeport/issues) mit Ihrer Homeport-Version aus den **Einstellungen**.

## Installieren und Starten

### Windows meldet „Der Computer wurde durch Windows geschützt“

Das Installationsprogramm ist noch nicht mit einem kommerziellen Zertifikat signiert. Wählen Sie **Weitere Informationen** und dann **Trotzdem ausführen**. Sie können zuerst [den Download prüfen](/docs/reference/verifying/).

### macOS meldet, dass Homeport „nicht geöffnet werden kann“

Homeport ist noch nicht von Apple notarisiert. Öffnen Sie **Systemeinstellungen → Datenschutz & Sicherheit** und wählen Sie neben Homeport **Dennoch öffnen**, oder führen Sie Folgendes aus:

```sh
xattr -dr com.apple.quarantine /Applications/Homeport.app
```

### Das AppImage startet nicht

Wenn es FUSE erwähnt, installieren Sie `libfuse2` (`libfuse2t64` unter Ubuntu 24.04 und neuer). Prüfen Sie, dass die Datei ausführbar ist: `chmod +x Homeport*.AppImage`.

### Homeport öffnet sich an der falschen Stelle

Homeport öffnet sich wieder mit der Größe, Position und dem Maximierungszustand, die sein Fenster beim Schließen hatte. Ist der Bildschirm, auf dem es war, nicht mehr vorhanden, öffnet es sich zentriert auf einem verbleibenden. Um die gespeicherte Stelle zu vergessen, beenden Sie Homeport und löschen Sie `.window-state.json` aus seinem Einstellungsordner:

| System  | Datei                                                               |
| ------- | ------------------------------------------------------------------- |
| Linux   | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%APPDATA%\com.homeport.app\.window-state.json`                     |

### Ein erneutes Öffnen von Homeport öffnet kein zweites Fenster

Es läuft immer nur ein Homeport: Wenn Sie es öffnen, während es bereits läuft, wird sein Fenster in den Vordergrund geholt, auch wenn es im [Infobereich](/docs/guides/overview/#infobereich) verborgen ist. Zwei Kopien würden sich mit jedem Host zweimal verbinden und in dieselben Daten schreiben.

### Es gibt kein Symbol im Infobereich

Unter Linux benötigt der Infobereich die Bibliothek appindicator, und manche Desktops blenden Symbole im Infobereich aus: Installieren Sie unter GNOME die Erweiterung AppIndicator Support. Findet Homeport keine appindicator-Bibliothek, weist **Einstellungen → Desktop** darauf hin, und das Schließen des Fensters beendet es. Siehe [Voraussetzungen](/docs/requirements/#optional-ein-infobereich).

### Ein rotes Banner meldet, dass kein Schlüsselbund des Betriebssystems verfügbar ist

Homeport konnte Ihren Schlüsselbund nicht erreichen. Unter Linux bedeutet das meist, dass kein Secret Service läuft – häufig bei minimalen Desktops und Fenstermanagern. Installieren und starten Sie GNOME Keyring (`gnome-keyring`) oder aktivieren Sie den Secret Service von KWallet und starten Sie Homeport dann neu. Bis dahin verwenden Sie Hosts mit lokalem Socket, TCP oder ssh-agent.

## Updates und das Paket-Repository

### „Homeport konnte nicht nach Updates suchen“

Homeport konnte die Update-Informationen nicht von dieser Website herunterladen: Sie sind möglicherweise offline oder hinter einem Proxy oder einer Firewall, die `damienreichhart.github.io` blockiert. Homeport funktioniert weiter; es prüft später erneut, oder wählen Sie **Einstellungen → Updates → Jetzt prüfen**. Sie können die neueste Version jederzeit von der [Download-Seite](/download/) herunterladen.

### Ein Update lässt sich nicht installieren

Homeport installiert nur ein Update, dessen Signatur zu seinem Update-Schlüssel passt, und bricht ab, wenn der Download unvollständig oder verändert ist. Versuchen Sie es später erneut; schlägt es weiterhin fehl, laden Sie die neue Version von der [Download-Seite](/download/) herunter und installieren Sie sie über die alte.

### apt meldet „NO_PUBKEY“ oder „The repository is not signed“

Der Homeport-Schlüssel fehlt in `/etc/apt/keyrings/homeport.asc`. Führen Sie die [Schnellinstallation](/docs/installation/#schnellinstallation) erneut aus oder die ersten beiden Befehle der [manuellen Einrichtung](/docs/installation/#manuelle-einrichtung).

### dnf oder zypper fragt, ob ein Schlüssel importiert werden soll

Das ist der Paketschlüssel von Homeport, beim ersten Zugriff Ihres Paketmanagers auf das Repository. Prüfen Sie, dass der angezeigte Fingerabdruck mit dem auf [Downloads prüfen](/docs/reference/verifying/#paketsignaturen) übereinstimmt, und stimmen Sie dann zu.

### apt meldet „Hash Sum mismatch“ direkt nach einem Release

Das Repository wurde aktualisiert, während apt es las. Führen Sie `sudo apt update` einige Minuten später erneut aus.

### Mein Paketmanager sieht die neue Version nicht

Prüfen Sie, dass das Repository noch eingerichtet ist: `/etc/apt/sources.list.d/homeport.sources`, `/etc/yum.repos.d/homeport.repo` oder `/etc/zypp/repos.d/homeport.repo`. Wenn Sie es gelöscht haben, führen Sie die [Schnellinstallation](/docs/installation/#schnellinstallation) aus, um es erneut hinzuzufügen. Wenn Sie 0.2.0 oder älter aus einer heruntergeladenen Datei installiert haben, wurde das Repository nie hinzugefügt: siehe [Aktualisieren von 0.2.0](/docs/reference/updating/#aktualisieren-von-020-oder-älter).

## Verbinden

### Nicht erreichbar: Permission denied

Ihr Benutzer kann den Docker-Socket nicht öffnen. Fügen Sie ihn unter Linux der Gruppe `docker` hinzu und melden Sie sich erneut an:

```sh
sudo usermod -aG docker $USER
```

### Nicht erreichbar: No such file or directory

Unter dem Socket-Pfad lauscht nichts. Starten Sie Ihre Engine – Docker Desktop, Colima, OrbStack, Rancher Desktop – oder prüfen Sie den Pfad. Siehe [lokale Engines](/docs/hosts/local/#einen-socket-von-hand-hinzufügen).

### „Die Engine hat nicht rechtzeitig geantwortet“ oder „Zeitüberschreitung der SSH-Verbindung“

Homeport wartet 10 Sekunden auf eine Verbindung. Prüfen Sie Adresse und Port, ob der Server läuft und ob keine Firewall im Weg ist. Homeport versucht es von selbst weiter.

### Zugriff verweigert: „Der Server hat den Schlüssel für … abgelehnt“

Der Server akzeptiert den Schlüssel von Homeport noch nicht. Führen Sie den Befehl **Öffentlichen Schlüssel hinzufügen** von der Seite des Hosts auf dem Server aus und wählen Sie dann **Neu verbinden**. Siehe [SSH-Hosts](/docs/hosts/ssh/#den-schlüssel-von-homeport-autorisieren).

### „Der SSH-Server konnte /var/run/docker.sock nicht öffnen“

Homeport hat sich angemeldet, konnte Docker aber nicht erreichen. Prüfen Sie:

- den **Entfernten Socket-Pfad** – rootless Docker verwendet `/run/user/<uid>/docker.sock`;
- dass der Benutzer Docker verwenden kann: `sudo usermod -aG docker <user>` auf dem Server;
- dass der Server die Socket-Weiterleitung erlaubt (`AllowStreamLocalForwarding` in `sshd_config`, standardmäßig aktiviert).

### „Es läuft kein ssh-agent“

Starten Sie Ihren SSH-Agenten und fügen Sie Ihren Schlüssel mit `ssh-add` hinzu. Wenn Sie Homeport über ein Desktop-Menü geöffnet haben, sieht es möglicherweise nicht den Agenten, den Ihr Terminal verwendet; melden Sie sich ab und wieder an oder starten Sie Homeport aus einem Terminal, in dem `ssh-add -l` Ihren Schlüssel auflistet.

Unter Windows verwendet Homeport entweder den Dienst OpenSSH Authentication Agent oder Pageant von PuTTY. Öffnen Sie Pageant und laden Sie Ihren Schlüssel, oder starten Sie den Dienst in einer PowerShell mit Administratorrechten:

```powershell
Get-Service ssh-agent | Set-Service -StartupType Automatic
Start-Service ssh-agent
ssh-add
```

### Zugriff verweigert: „TLS-Handshake fehlgeschlagen“

- Das CA-Zertifikat muss dasjenige sein, das das Zertifikat des Servers signiert hat.
- Die eingegebene Adresse muss mit einem Namen oder einer IP-Adresse im Zertifikat des Servers übereinstimmen.
- Der Server muss mit `--tlsverify` laufen und der CA Ihres Client-Zertifikats vertrauen.

### Serverschlüssel geändert

Der SSH-Server hat einen anderen Schlüssel vorgelegt als den, dem Sie vertraut haben. Wenn Sie den Server neu installiert oder seine Schlüssel geändert haben, wählen Sie **Dem neuen Schlüssel vertrauen**. Wenn Sie nicht wissen, warum er sich geändert hat, verbinden Sie sich nicht: Fragen Sie, wer den Server betreibt. Siehe [SSH-Hosts](/docs/hosts/ssh/#wenn-sich-der-schlüssel-des-servers-ändert).

### Zugangsdaten verloren

Der Schlüssel, der Ihre Geheimnisse verschlüsselt, fehlt im Schlüsselbund oder hat sich geändert – nach einem Zurücksetzen des Schlüsselbunds, einem neuen Benutzerprofil oder dem Kopieren der Daten von Homeport auf einen anderen Computer (verwenden Sie dafür [Exportieren und Importieren](/docs/reference/security/#auf-einen-anderen-computer-umziehen)). Homeport behält den Host; geben Sie seine Anmeldedaten auf der Seite des Hosts erneut ein:

- **TLS** – laden Sie die drei Zertifikatsdateien erneut und wählen Sie **Speichern**.
- **SSH mit einem importierten Schlüssel** – fügen Sie den privaten Schlüssel und die Passphrase erneut ein und wählen Sie **Speichern**.
- **SSH mit einem Homeport-Schlüssel** – wählen Sie **Schlüssel neu erzeugen** und autorisieren Sie dann den neuen Schlüssel auf dem Server.

Auch Umgebungen verwalteter Stacks, die mit dem alten Schlüssel versiegelt wurden, müssen neu eingegeben werden.

## Cluster

### Zugriff verweigert: „… hat die Zugangsdaten aus seiner kubeconfig abgelehnt“

Der API-Server hat mit 401 geantwortet: Das Zertifikat oder Token ist abgelaufen oder wurde widerrufen. Prüfen Sie, dass `kubectl --context <context> get namespaces` noch funktioniert, und verwenden Sie dann **Aus kubeconfig aktualisieren** auf der Seite des Clusters. Bei einem Exec-Plugin hat Homeport es bereits einmal nach einem neuen Token gefragt.

### „… ist auf diesem Computer nicht installiert“

Das [Exec-Plugin](/docs/clusters/exec-plugins/) des Clusters (`aws`, `gke-gcloud-auth-plugin`, `kubelogin`…) wurde weder im `PATH` noch an seinen üblichen Installationsorten gefunden. Installieren Sie es, prüfen Sie, dass es in einem Terminal läuft, und wählen Sie dann **Neu verbinden**.

### „… erfordert eine Anmeldung, die innerhalb von Homeport nicht möglich ist“

Das Exec-Plugin möchte einen Browser oder einen Gerätecode. Führen Sie `kubectl --context <context> get namespaces` einmal in einem Terminal aus, melden Sie sich dort an und wählen Sie dann **Neu verbinden**.

### „… konnte keine Zugangsdaten für … abrufen“

Das Exec-Plugin lief und ist fehlgeschlagen. **Details** zeigt das Ende seiner Ausgabe, oft ein abgelaufenes Profil oder eine falsche Region. Homeport versucht es von selbst erneut.

### „… meldet sich mit dem Auth-Provider gcp an, den Kubernetes nicht mehr unterstützt“

Die Auth-Provider `gcp` und `azure` wurden aus Kubernetes entfernt. Rufen Sie die Anmeldedaten mit installiertem `gke-gcloud-auth-plugin` erneut ab (GKE) oder führen Sie `kubelogin convert-kubeconfig` aus (AKS) und fügen Sie den Kontext dann erneut hinzu.

### „Ihre Zugangsdaten erlauben „…“ für … in … nicht“

Das RBAC des Clusters erlaubt dem Benutzer Ihrer kubeconfig das nicht. Wählen Sie in der Auswahl oben rechts einen Namespace, den Sie sehen dürfen, oder fragen Sie den Administrator des Clusters. Die Meldung nennt den Befehl `kubectl auth can-i`, der es prüft. Andere Seiten funktionieren weiter.

### Nicht erreichbar: „… hat die Verbindung abgelehnt“ oder „… hat nicht rechtzeitig geantwortet“

Der API-Server ist von diesem Computer aus nicht erreichbar: Der Cluster ist gestoppt (bei einem lokalen), hinter einem VPN, oder die Serveradresse der kubeconfig hat sich geändert. Homeport versucht es von selbst weiter; prüfen Sie `kubectl cluster-info` mit demselben Kontext.

### „Das Zertifikat von … ist nicht von der CA in seiner kubeconfig signiert“

kind, k3d und minikube erzeugen bei jeder Neuerstellung eines Clusters neue Zertifikate. Verwenden Sie **Aus kubeconfig aktualisieren** auf der Seite des Clusters.

### Nomad: „… kennt das ACL-Token nicht, das Homeport gesendet hat“

Das Token wurde falsch eingegeben, ist abgelaufen oder wurde gelöscht. Erstellen Sie ein neues mit der Policy aus [ACL-Policies](/docs/clusters/nomad/#acl-policies) (`nomad acl token create -name=homeport -policy=homeport`) und fügen Sie seine Secret-ID in den **Einstellungen** des Clusters ein. Der Cluster wartet als **Zugriff verweigert**, bis Sie das tun.

### Nomad: „… hat abgelehnt: Dem ACL-Token fehlt eine dafür nötige Berechtigung“

Die Policies des Tokens gewähren nicht, was diese Seite oder Aktion braucht; die Meldung sagt, was (Jobs oder Logs lesen, Jobs ausführen oder stoppen, skalieren, eine Shell öffnen, Nodes ändern). Fügen Sie die Capability zur Policy des Tokens hinzu; [ACL-Policies](/docs/clusters/nomad/#acl-policies) listet auf, was jede braucht. Andere Seiten funktionieren weiter.

### Nomad: „… hat keinen Leader: Seine Server können sich auf keinen einigen“

Weniger als die Mehrheit der Server des Clusters laufen oder können einander erreichen, daher kann Nomad nicht antworten. Prüfen Sie sie mit `nomad server members` auf einem Server und bringen Sie genug davon zurück. Homeport versucht es weiter.

### Nomad: „… hat keinen Job …“

Der Job wurde gestoppt und bereinigt oder von der Garbage Collection entfernt, oder er lebt in einem anderen Namespace als dem gewählten. Öffnen Sie die Seite „Jobs“ erneut, bei Bedarf mit **Alle Namespaces**.

### Nomad: „Die Task Group … kann nicht mehr als … ausführen“ oder „nicht weniger als …“

Der Block `scaling` der Task Group setzt ein `min` und ein `max`, und die angeforderte Anzahl liegt außerhalb davon. Ändern Sie die Grenzen in der Spezifikation des Jobs und führen Sie ihn erneut aus, um weiterzugehen.

### Nomad: „Nomad skaliert … nicht, während ein Deployment davon läuft“

Das Ausführen einer neuen Version eines Service-Jobs startet ein Deployment, und Nomad weigert sich, seine Anzahl zu ändern, bis jede Allocation der neuen Version gesund ist. Warten Sie, bis sich die Allocations des Jobs eingependelt haben, und skalieren Sie dann erneut.

### Nomad: „Die Job-Spezifikation lässt sich nicht parsen, in Zeile …, Spalte …“

Der Parser von Nomad hat an dieser Stelle ein Problem gefunden; **Details** enthält seine Meldung. `variable`-Blöcke brauchen einen Standardwert, da Homeport keine Variablendateien übergibt. Eine JSON-Spezifikation muss ein Job im API-Format von Nomad sein, wie `nomad job inspect` ihn ausgibt.

### Nomad: „Nomad kann nicht alle Allocations von … platzieren“

Der Plan hat keinen geeigneten Node mit den Ressourcen gefunden, die die Task Group anfordert, oder der zu ihren Constraints passt; der Plan sagt, welche. **Trotzdem ausführen** stellt in die Warteschlange, was nicht platziert werden kann, bis ein Node es aufnehmen kann; oder senken Sie die Ressourcen, ändern Sie die Constraints oder fügen Sie Nodes hinzu.

### Nomad: „… hat sich seit der Planung geändert und wurde daher nicht ausgeführt“

Jemand anderes hat den Job zwischen Ihrem Plan und **Ausführen** ausgeführt oder gestoppt, und Homeport überschreibt diese Änderung nicht. Gehen Sie zurück, planen Sie erneut und prüfen Sie den neuen Diff vor dem Ausführen.

### Nomad: „Der Task … der Allocation … läuft nicht“

Eine Shell braucht einen laufenden Task: Dieser hat noch nicht begonnen, ist beendet oder fehlgeschlagen. Seine Logs bleiben lesbar, bis die Allocation von der Garbage Collection entfernt wird.

### Nomad: „… kann den Node nicht erreichen, auf dem die Allocation … läuft“

Logs und Shells laufen über die Server zum Client-Node, und die Server können ihn nicht erreichen: Der Node ist ausgefallen, getrennt, oder sein Agent wurde gestoppt. Prüfen Sie ihn auf der Seite „Nodes“ oder mit `nomad node status`.

### Helm: „Die helm-CLI ist auf diesem Computer nicht installiert“

[Helm-Releases](/docs/clusters/helm/) benötigen `helm` 3 oder neuer auf diesem Computer. Installieren Sie es (`brew install helm`, `winget install Helm.Helm` oder `snap install helm --classic`) und öffnen Sie dann die Helm-Seite erneut; **Einstellungen → Dieser Rechner** zeigt das von Homeport gefundene. Ein `helm` 2 wird als nicht unterstützt gemeldet.

### Helm: „Eine andere Installation, ein Upgrade oder ein Rollback dieses Releases läuft noch“

Das Release ist `pending-…`: Eine Installation, ein Upgrade oder ein Zurücksetzen wurde nicht abgeschlossen, oft weil es unterbrochen wurde. helm weigert sich, es zu ändern, bis das endet. Warten Sie darauf oder setzen Sie das Release über seinen **Verlauf** auf seine zuletzt bereitgestellte Revision zurück.

## Homeport verwenden

### Bereitstellen ist ausgegraut: „Die docker compose-CLI ist auf diesem Computer nicht installiert“

Homeport braucht `docker compose` auf diesem Computer, um Stacks bereitzustellen. Installieren Sie Docker Desktop oder die Docker CLI mit dem Compose-Plugin und starten Sie Homeport dann neu. **Einstellungen → Dieser Rechner** zeigt, ob es gefunden wird.

Ist es installiert, wird aber nicht gefunden, hat Homeport möglicherweise einen anderen `PATH` als Ihr Terminal – häufig, wenn es unter macOS über ein Desktop-Menü gestartet wurde. Stellen Sie sicher, dass `docker` an einem Standardort wie `/usr/local/bin` liegt.

### Bereitstellen schlägt fehl: „verwendet build:“

Ein verwalteter Stack hat keinen Ordner, aus dem gebaut werden könnte. [Verknüpfen Sie stattdessen den Projektordner](/docs/guides/stacks/#einen-verknüpften-stack-erstellen) – ein verknüpfter Stack baut –, oder bauen Sie das Image, pushen Sie es in eine Registry und verwenden Sie dann `image:`.

### Mein Stack wurde extern

Ein [verknüpfter Stack](/docs/guides/stacks/#wenn-der-ordner-verschoben-wird-oder-verschwindet) erscheint als extern, markiert mit ⚠ **Seine Compose-Dateien sind nicht mehr auf diesem Computer**, wenn sein Ordner verschoben, umbenannt oder gelöscht wurde oder eine Compose-Datei darin. Nichts geht verloren: Legen Sie den Ordner zurück an seinen Platz, und der Stack ist bei der nächsten Aktualisierung wieder verknüpft. Ist der Ordner endgültig umgezogen, wählen Sie **Vergessen** im Tab **Definition** des Stacks und verknüpfen Sie ihn dann von seinem neuen Ort aus erneut mit **Neuer Stack → Ordner auf diesem Computer verwenden**.

Ein aus einem Ordner auf einem anderen Computer gestarteter Stack ist hier immer extern, da dieser Computer den Pfad nicht hat.

### „Außerhalb von Homeport geändert“

Die Compose-Dateien eines verknüpften Stacks wurden außerhalb von Homeport bearbeitet – in Ihrem Editor, durch `git pull` –, seit Homeport sie zuletzt gelesen hat. Wählen Sie **Neu laden**, um sie so zu sehen, wie sie sind. Wenn Sie gleichzeitig in Homeport bearbeitet haben, wird das Speichern mit **… wurde außerhalb von Homeport geändert, seit Sie sie geöffnet haben** abgelehnt, damit keine der Änderungen stillschweigend verloren geht: Laden Sie neu und wiederholen Sie Ihre Bearbeitung, oder wählen Sie **Überschreiben**, um Ihre zu behalten. Siehe [einen verknüpften Stack bearbeiten](/docs/guides/stacks/#einen-verknüpften-stack-bearbeiten).

### Das Pullen eines privaten Images schlägt fehl

Fügen Sie unter **Einstellungen → Registrys** eine Anmeldung für die Registry des Images hinzu und verwenden Sie **Anmeldung prüfen**, um zu bestätigen, dass die Registry sie akzeptiert. GHCR sowie Docker-Hub-Konten mit Zwei-Faktor-Authentifizierung brauchen ein Zugriffstoken statt des Passworts. Der Pull-Dialog sagt, welche Anmeldung er verwenden wird; zeigt er **anonym**, passt die Registry des Images zu keiner gespeicherten Anmeldung – `ghcr.io/org/app` braucht eine Anmeldung für `ghcr.io` und `org/app` eine für Docker Hub.

### Ein Build schlägt fehl

Die Meldung nennt den fehlgeschlagenen Schritt, etwa **Der Build ist bei Schritt 3/5 (RUN make) fehlgeschlagen**; **Details** enthält das Ende der Build-Ausgabe. Homeport verwendet den klassischen Builder der Engine, daher schlagen Dockerfiles fehl, die BuildKit brauchen (`RUN --mount`, `--secret`, Heredocs); bauen Sie diese in einem Terminal. Ein privates `FROM`-Image braucht eine Anmeldung für seine Registry unter **Einstellungen → Registrys**.

### Keine Markierung für neuere Images erscheint

Die Markierung erscheint nur, wenn die Registry gefragt wurde und geantwortet hat. Lokal gebaute oder per Digest gepullte Images zeigen nie eine. Ist eine Registry nicht erreichbar oder begrenzt sie Anfragen (Docker Hub rechnet diese Prüfungen auf sein Pull-Limit an), schreibt Homeport eine Zeile in sein [Log](/docs/reference/security/#logs) und zeigt nichts; Antworten werden außerdem 6 Stunden aufbewahrt, wählen Sie also **Nach neueren Images suchen**, um jetzt erneut zu fragen. Eine Docker-Hub-Anmeldung unter **Einstellungen → Registrys** hebt das Limit an.

### Die Suche auf Docker Hub zeigt einen Fehler

Die Suche läuft auf dem ausgewählten Host, braucht also dessen Internetzugang, und Tags werden von diesem Computer aus von `hub.docker.com` gelesen. Beides kann einem Ratenlimit unterliegen oder von einem Proxy blockiert werden. Der Dialog pullt trotzdem jede vollständige Referenz, die Sie eingeben, etwa `postgres:17-alpine`.

### Der Tab „Dateien“ meldet, dass der Container keine Tools find und stat hat

Minimale und Distroless-Images enthalten weder `find` noch `stat`, die Homeport zum Auflisten von Dateien verwendet. Verwenden Sie für diese Container `docker cp`.

### Das Terminal öffnet sich nicht

Der Container muss laufen und `/bin/sh` oder `/bin/bash` haben. Verwenden Sie **Neues Terminal**, um ein anderes Programm auszuführen.

### Ein Volume oder Netzwerk lässt sich nicht löschen

Die Engine weigert sich, solange ein Container – auch ein gestoppter – es noch verwendet. Entfernen Sie zuerst diese Container; die Bestätigung listet sie auf.

### „Dafür ist Podman 5.8 oder neuer nötig“

Die API von Podman liest, installiert, bearbeitet und entfernt [Quadlet-Units](/docs/guides/quadlet/) ab Podman 5.8 und listet sie ab 5.7 auf. Aktualisieren Sie Podman auf dem Host oder verwenden Sie dort `podman quadlet` in einem Terminal. Das Starten und Stoppen von Units funktioniert mit jedem Podman.

### Quadlet: Starten und Stoppen sind ausgegraut

Homeport startet und stoppt den Dienst einer Unit mit `systemctl`, was es nur über SSH oder für ein Podman auf diesem Computer (unter Linux) ausführen kann. Erreichen Sie den Host über SSH statt über TCP oder TLS. Eine Unit, deren Dienst **noch nicht erzeugt** anzeigt, hat nichts zu starten: Der Generator von systemd hat ihre Datei übersprungen.

### Quadlet: „Das systemd des Benutzers läuft auf dem Host nicht“

Benutzer-Units (rootless Podman) brauchen die systemd-Instanz des Benutzers, die nur läuft, während der Benutzer angemeldet ist, sofern Lingering nicht aktiviert ist. Führen Sie als dieser Benutzer auf dem Host `loginctl enable-linger "$USER"` aus und versuchen Sie es dann erneut.

### Quadlet: „systemd hat diesem Benutzer nicht erlaubt, … zu verwalten“

System-Units (rootful Podman) können nur von einem Benutzer gestartet und gestoppt werden, der sie verwalten darf, etwa `root`, und Homeport verwendet nie `sudo`. Verbinden Sie sich als ein solcher Benutzer oder verwenden Sie rootless Podman, dessen Units dem Benutzer selbst gehören.

### containerd: „darf containerd unter /run/containerd/containerd.sock nicht verwenden“

Der Socket von containerd gehört root. Geben Sie einer Gruppe mit `gid` unter `[grpc]` in `/etc/containerd/config.toml` Zugriff darauf, fügen Sie Ihren Benutzer (oder den SSH-Benutzer) dieser Gruppe hinzu, starten Sie containerd neu und melden Sie sich erneut an; [die containerd-Seite](/docs/hosts/containerd/#ihrem-benutzer-die-verwendung-des-sockets-erlauben) enthält die Befehle. Rootless containerd ist überhaupt nicht erreichbar: Sein Socket existiert nur im Namespace von RootlessKit.

### containerd: „Die Ausgabe dieses Containers wird nicht aufgezeichnet“

containerd bewahrt keine Logs auf; das Werkzeug, das den Task gestartet hat, entscheidet, wohin seine Ausgabe geht. Mit `ctr` erstellte Container leiten sie meist nirgendwohin. Die Container von nerdctl und Kubernetes loggen in Dateien, die Homeport liest; siehe [Logs](/docs/hosts/containerd/#logs).

### containerd: Logs oder das Terminal melden, dass sie SSH brauchen

Sie brauchen die Dateien des containerd-Rechners, die Homeport über SSH erreicht, oder auf diesem Computer bei einem lokalen Socket unter Linux. Fügen Sie den Host über SSH hinzu. Kann die Logdatei nicht gelesen werden, gehört sie meist root: Verbinden Sie sich als Benutzer, der sie lesen darf.

### containerd: „kann für Homeport keine Images pullen“

Das Pullen läuft über den Transfer-Service von containerd, der ab containerd 2.0 stabil ist. Aktualisieren Sie containerd oder pullen Sie mit `ctr images pull` oder `nerdctl pull` auf dem Host; das Image erscheint dann auf der Seite „Images“.

### Incus: „darf Incus unter /var/lib/incus/unix.socket nicht verwenden“

Der Admin-Socket von Incus ist für die Gruppe `incus-admin`, und der eingeschränkte `unix.socket.user` für die Gruppe `incus`. Fügen Sie Ihren Benutzer (oder den SSH-Benutzer) mit `sudo usermod -aG incus-admin "$USER"` hinzu, melden Sie sich dann ab und wieder an oder verbinden Sie sich über SSH neu. Siehe [die Sockets von Incus](/docs/hosts/incus/#seine-sockets).

### Incus: „Homeport konnte Incus unter … nicht erreichen“ beim Hinzufügen mit einem Trust-Token

Incus lauscht erst im Netzwerk, wenn `core.https_address` gesetzt ist. Führen Sie auf dem Server `incus config set core.https_address :8443` aus, prüfen Sie, dass eine Firewall Port 8443 durchlässt, und erstellen Sie ein neues Token: Das Token listet die Adressen auf, auf denen Incus bei seiner Erstellung lauschte.

### Incus: „Incus hat das Trust-Token abgelehnt“

Ein Token funktioniert einmal und läuft ab. Erstellen Sie auf dem Server mit `incus config trust add homeport` ein neues und fügen Sie es innerhalb seiner Gültigkeitsdauer ein.

### Incus: „… hat ein Zertifikat mit dem Fingerabdruck … vorgelegt, nicht den vertrauenswürdigen …“

Das Zertifikat des Servers ist nicht das, dem Homeport vertraut hat: Incus wurde neu installiert, sein Zertifikat wurde erneuert, oder etwas anderes antwortet unter dieser Adresse. Prüfen Sie den Fingerabdruck auf dem Server mit `incus info`; ist es der angezeigte, entfernen Sie den Host und fügen Sie ihn mit einem neuen Token erneut hinzu. Siehe [fixierte Zertifikate](/docs/hosts/incus/#das-zertifikat-des-servers-ist-fixiert).

### Incus: „Dieser Host kann keine virtuellen Maschinen ausführen“

Incus braucht für virtuelle Maschinen KVM (`/dev/kvm`) und QEMU. Aktivieren Sie die Virtualisierung in der Firmware des Servers (oder die verschachtelte Virtualisierung bei einem virtuellen Server) oder erstellen Sie stattdessen einen Systemcontainer.

### Incus: „Der Incus-Agent der virtuellen Maschine läuft nicht“

Exec in einer virtuellen Maschine läuft über den `incus-agent` darin, den die eigenen Images von Incus beim Booten starten. Warten Sie, bis der Bootvorgang abgeschlossen ist, oder verwenden Sie den Tab **Konsole**, der keinen Agenten braucht.

### „Host … ist nicht verbunden“

Die Aktion brauchte den Host, aber er hat die Verbindung verloren. Warten Sie, bis er sich neu verbindet, oder wählen Sie **Jetzt erneut versuchen**.
