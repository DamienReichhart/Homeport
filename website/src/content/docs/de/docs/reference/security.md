---
title: Sicherheit und Ihre Daten
description: Was Homeport speichert, wo, wie Geheimnisse verschlüsselt werden und was es nie tut.
---

Homeport verwahrt Anmeldedaten, die Ihre Server öffnen. Diese Seite erklärt genau, was es aufbewahrt und wie.

## Was Homeport speichert

Alles liegt in einer einzigen SQLite-Datenbank, `homeport.db`, im Anwendungsdatenordner Ihres Benutzers:

| System  | Ordner                                            |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

Sie enthält:

| Daten                                                                       | Gespeichert       |
| --------------------------------------------------------------------------- | ----------------- |
| Host-Namen, Adressen, SSH-Benutzer und Socket-Pfade                         | Klartext          |
| Öffentliche SSH-Schlüssel und die Serverschlüssel, denen Sie vertraut haben | Klartext          |
| Private SSH-Schlüssel und ihre Passphrasen                                  | **Verschlüsselt** |
| TLS-CA-Zertifikate, Client-Zertifikate und Schlüssel                        | **Verschlüsselt** |
| Client-Schlüssel und fixierte Zertifikate von Incus-Hosts                   | **Verschlüsselt** |
| Compose-Dateien verwalteter Stacks                                          | Klartext          |
| Umgebungen verwalteter Stacks                                               | **Verschlüsselt** |
| Ordner und Compose-Dateipfade verknüpfter Stacks                            | Klartext          |
| Bereitstellungsverlauf: Compose-Dateien und Profile                         | Klartext          |
| Bereitstellungsverlauf: Umgebungen                                          | **Verschlüsselt** |
| Registry-Adressen und Benutzernamen                                         | Klartext          |
| Registry-Passwörter und Zugriffstokens                                      | **Verschlüsselt** |
| Repository-URLs, öffentliche Hälften von Deploy-Keys                        | Klartext          |
| Serverschlüssel von Repositorys, denen Sie vertraut haben                   | Klartext          |
| Deploy-Keys und Zugriffstokens von Repositorys                              | **Verschlüsselt** |
| Cluster-Namen, API-Server-URLs und Namespaces                               | Klartext          |
| Wie sich jeder Cluster anmeldet, sein kubeconfig-Kontext                    | Klartext          |
| Cluster-CA, Client-Zertifikate und Schlüssel                                | **Verschlüsselt** |
| Cluster-Tokens, Passwörter und OIDC-Tokens                                  | **Verschlüsselt** |
| Befehle, Argumente und Umgebung von Exec-Plugins                            | **Verschlüsselt** |
| Ihr Design, Ihre letzte Auswahl und der Namespace pro Cluster               | Klartext          |
| Die Engine-Ereignisse jedes Hosts, für die Seite „Ereignisse“               | Klartext          |

Homeport kopiert sonst nichts aus Ihren Engines: Container, Images, Volumes und Logs werden live gelesen und nie gespeichert, außer wenn Sie selbst Logs oder Dateien herunterladen oder ein Volume sichern. Die Engine-Ereignisse, die es aufbewahrt – was mit welchem Container, Image, Volume oder Netzwerk geschah, mit ihren Labels –, werden standardmäßig 30 Tage lang aufbewahrt ([Einstellungen → Daten](/docs/guides/settings/#data)), höchstens 10.000 pro Host, und mit ihrem Host oder mit **Verlauf löschen** entfernt.

### Ordner verknüpfter Stacks

Für einen [verknüpften Stack](/docs/guides/stacks/#linked-stacks) speichert Homeport nur, wo sein Ordner und seine Compose-Dateien liegen, sowie einen Fingerabdruck der Dateien, um zu erkennen, wann sie sich ändern – nie eine Kopie davon. Es liest die Compose-Dateien, wenn Sie den Stack öffnen, und schreibt nur in sie, wenn Sie im Tab **Definition** **Speichern** wählen, wobei jede bearbeitete Datei in einem Schritt ersetzt wird und ihre Berechtigungen erhalten bleiben; eine Datei, die sich seit dem Öffnen geändert hat, überschreibt es nur, wenn Sie es ausdrücklich sagen. Homeport liest oder schreibt nie die `.env` des Ordners: Compose liest sie, wenn es läuft. Das Vergessen eines verknüpften Stacks tastet den Ordner nie an.

### Repositorys

Für [synchronisierte Stacks](/docs/guides/synced-stacks/) bewahrt Homeport die URL jedes Repositorys auf und entweder einen Deploy-Key, den es für dieses Repository erzeugt hat (seine private Hälfte verschlüsselt), oder Ihren Benutzernamen und ein Zugriffstoken (verschlüsselt). Der Inhalt eines Repositorys ist kein Geheimnis: Sein Klon liegt in einem Ordner `repositories` neben `homeport.db`.

### Swarms

Homeport speichert nichts über einen [Swarm](/docs/guides/swarm/): Es findet ihn bei jeder Verbindung über seine Manager-Hosts. Die Werte von Swarm-Secrets gehen an den Swarm und werden von Homeport nicht aufbewahrt: Ein Wert, den Sie eingeben oder aus einer Datei laden, wird einmal an die Engine gesendet, nie in `homeport.db`, eine Datei oder das Log geschrieben und aus dem Arbeitsspeicher entfernt, wenn der Dialog geschlossen wird. Die Engine sendet den Wert eines Secrets nie zurück, daher kann Homeport ihn auch nicht anzeigen. Join-Tokens werden bei Bedarf von einem Manager abgerufen und nie gespeichert oder geloggt.

### Cluster

Die Anmeldedaten eines [Clusters](/docs/clusters/) stammen aus Ihrer kubeconfig und werden nur verschlüsselt gespeichert, wie SSH-Schlüssel; Dateien, auf die die kubeconfig verweist, werden einmal gelesen und eingebettet, eine eingefügte kubeconfig wird nie in eine Datei geschrieben, und Ihre kubeconfig wird nie verändert. Ein [Exec-Plugin](/docs/clusters/exec-plugins/) läuft auf diesem Computer, als Sie, und das Token oder Zertifikat, das es ausgibt, wird nur im Arbeitsspeicher aufbewahrt, bis es abläuft. Homeport speichert sonst nichts darüber, was auf einem Cluster läuft: Pods, Workloads und der Rest werden live gelesen. [Portweiterleitungen](/docs/clusters/port-forward/) lauschen nur auf dem Loopback dieses Computers (`127.0.0.1` und `::1`), sodass nichts anderes in Ihrem Netzwerk sie verwenden kann, und sie enden, wenn Homeport beendet wird. Änderungen mit [YAML anwenden oder Bearbeiten](/docs/clusters/yaml/) werden auf dem Cluster unter dem Field Manager `homeport` vermerkt.

[Helm](/docs/clusters/helm/) führt Ihre `helm`-CLI gegen einen Cluster aus, ohne Ihre kubeconfig und ohne Cluster-Anmeldedaten auf die Festplatte zu schreiben: Für jeden Befehl öffnet Homeport eine private Verbindung auf dem Loopback dieses Computers (`127.0.0.1`, ein zufälliger Port), die nur Anfragen mit einem zufälligen Token annimmt, und gibt helm eine temporäre kubeconfig, nur für Sie lesbar, die dieses Token enthält und sonst nichts. Homeport fügt jeder Anfrage unterwegs die eigenen Anmeldedaten des Clusters hinzu und löscht die kubeconfig und schließt die Verbindung, wenn der Befehl endet. Chart-Werte gehen über die Standardeingabe an helm, nie in eine Datei. Die eigenen Chart-Repositorys, Registry-Anmeldungen und Caches von helm bleiben die von helm, wie in einem Terminal; Homeport fügt nur öffentliche Repositorys hinzu und speichert kein Repository-Passwort.

Das ACL-Token, das CA-Zertifikat, das Client-Zertifikat und der Schlüssel eines [Nomad-Clusters](/docs/clusters/nomad/) werden nur verschlüsselt gespeichert, wie SSH-Schlüssel; seine Adresse, Region und sein TLS-Servername werden im Klartext aufbewahrt. Das Token reist im Header `X-Nomad-Token` jeder Anfrage, nie in einer URL, auch im WebSocket einer Shell. Homeport bewahrt keine Kopie der Spezifikation eines Jobs auf: Nomad speichert sie mit dem Job.

**Werte von Kubernetes-Secrets** werden nur abgerufen, wenn Sie für einen Schlüssel **Anzeigen** wählen, einen nach dem anderen: Listen, die Seite eines Secrets und sein YAML enthalten seine Schlüssel und Größen, nie seine Werte. Ein angezeigter Wert bleibt im Arbeitsspeicher dieser Seite – in keinem Cache, nicht in `homeport.db`, nicht im Log – und wird wieder ausgeblendet, wenn Sie die Seite verlassen.

## Wie Geheimnisse verschlüsselt werden

- Jedes Geheimnis wird einzeln mit **XChaCha20-Poly1305** versiegelt, einer authentifizierten Verschlüsselung, sodass Manipulationen erkannt und das Lesen verhindert wird.
- Der 256-Bit-Schlüssel ist zufällig, wird beim ersten Start erzeugt und im **Schlüsselbund Ihres Betriebssystems** aufbewahrt – macOS-Schlüsselbund, Windows-Anmeldeinformationsverwaltung oder Secret Service unter Linux – unter dem Dienst `com.homeport.app`. Er wird nie neben den Daten gespeichert.
- Es gibt kein Master-Passwort: Der Schlüsselbund bindet den Schlüssel bereits an Ihr Benutzerkonto.

Wer nur eine Kopie von `homeport.db` hat, kann Namen und Adressen von Hosts und Clustern lesen, aber nicht Ihre Schlüssel, Zertifikate, Tokens, Stack-Umgebungen oder Registry-Passwörter.

### Ohne Schlüsselbund

Homeport zeigt ein Banner an und speichert keine Geheimnisse. Hosts mit lokalem Socket, TCP und ssh-agent funktionieren weiterhin. Siehe [Voraussetzungen](/docs/requirements/#optional-an-os-keychain).

### Wenn der Schlüssel im Schlüsselbund verloren geht

Das Zurücksetzen des Schlüsselbunds oder das Verschieben von `homeport.db` auf einen anderen Computer oder zu einem anderen Benutzer hinterlässt Geheimnisse, die nicht entschlüsselt werden können. Homeport löscht nichts: Betroffene Hosts zeigen **Zugangsdaten verloren**, bis Sie ihre Anmeldedaten erneut eingeben. Siehe [Fehlerbehebung](/docs/reference/troubleshooting/#credentials-lost). Um Ihre Daten umzuziehen, sichern Sie sie stattdessen oder exportieren Sie Ihre Hosts.

### Auf einen anderen Computer umziehen

**Exportieren** auf der Seite „Hosts“ schreibt die von Ihnen gewählten Hosts in eine Datei, mit ihren Schlüsseln, Zertifikaten und vertrauten Serverschlüsseln, verschlüsselt mit einer Passphrase Ihrer Wahl:

- Die ganze Datei ist verschlüsselt (XChaCha20-Poly1305, mit dem per Argon2id aus der Passphrase abgeleiteten Schlüssel), sodass sie ohne die Passphrase nichts preisgibt – nicht einmal Host-Namen oder Adressen. Jede Änderung an der Datei macht sie unlesbar.
- Die Passphrase braucht mindestens 12 Zeichen. Homeport speichert oder loggt sie nie; wenn Sie sie verlieren, kann die Datei nicht geöffnet werden.
- **Importieren** auf dem anderen Computer fragt nach der Passphrase und speichert dann die Geheimnisse jedes Hosts verschlüsselt unter dem eigenen Schlüsselbund-Schlüssel dieses Computers, wie alle anderen.
- Ein Host, dessen Anmeldedaten auf diesem Computer bereits verloren sind, wird ohne sie exportiert, und der Export sagt, welcher.

Behandeln Sie die Datei wie die Schlüssel, die sie enthält: Löschen Sie sie nach dem Import. Das Format ist in [ADR 0013](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0013-passphrase-protected-export-bundles.md) beschrieben.

### Sicherungen

**Einstellungen → Daten → Sichern…** schreibt alles oben Genannte – Hosts, Cluster, Stacks, Registry-Anmeldungen, Repositorys und Einstellungen, mit ihren geöffneten Geheimnissen – in eine Datei im selben verschlüsselten Format, mit einer Passphrase, die Sie zweimal eingeben. Engine-Ereignisse und Bereitstellungsverlauf sind nicht enthalten. Ein Geheimnis, das auf diesem Computer nicht geöffnet werden kann, wird ausgelassen, und die Sicherung sagt, welches.

**Wiederherstellen…** fragt nach der Passphrase und listet auf, was die Datei enthält, bevor etwas geändert wird. Die Wiederherstellung dann:

1. kopiert die aktuelle Datenbank nach `homeport.db.before-restore-<date and time>`, neben `homeport.db`;
2. ersetzt jeden Host, Cluster, Stack, jede Anmeldung, jedes Repository und jede Einstellung in einem Schritt, sodass ein Fehler Ihre Daten unverändert lässt;
3. verschlüsselt jedes Geheimnis erneut mit dem Schlüsselbund-Schlüssel dieses Computers.

Eine von einem neueren Homeport erstellte Sicherung wird abgelehnt; eine von einem älteren Homeport erstellte wird wiederhergestellt. Die Sicherung enthält jedes Geheimnis, das Homeport hat: Bewahren Sie sie und ihre Passphrase so sicher auf wie die Schlüssel selbst.

## Schlüssel berühren die Festplatte nie im Klartext

Homeport hat seinen eigenen SSH-Client, daher schreibt es nie einen privaten Schlüssel in eine Datei, damit `ssh` ihn liest, und verändert nie `~/.ssh`. Entschlüsselte Schlüssel bleiben im Arbeitsspeicher.

Wenn `docker compose` einen Stack auf einem SSH- oder TLS-Host bereitstellt, öffnet Homeport einen kurzlebigen privaten Socket auf diesem Computer – nur für Ihren Benutzer lesbar, oder unter Windows eine Named Pipe, die entfernte Clients ablehnt – und leitet ihn über seine eigene Verbindung weiter, für diesen einen Befehl.

Auch Repository-Schlüssel und -Tokens erreichen nie die Festplatte. Für jeden `git`-Befehl stellt Homeport einen privaten SSH-Agenten bereit, der nur den Deploy-Key dieses Repositorys hält – auf einem Socket, den nur Ihr Benutzer öffnen kann, oder unter Windows einer Named Pipe, die entfernte Clients ablehnt –, oder übergibt das Token in einem HTTPS-Header über die Umgebung des Befehls. Ihre eigene git-Konfiguration, Credential-Helper und SSH-Schlüssel bleiben außen vor, sodass nichts in Ihrem Schlüsselbund oder in `~/.git-credentials` zwischengespeichert wird.

Auch Registry-Anmeldungen erreichen `docker compose` nie: Homeport pullt die Images eines Stacks selbst, über die Engine API, bevor es `docker compose up` ausführt, sodass die Umgebung des Befehls und die Dateien, die er erhält, kein Passwort und kein Token enthalten.

## Servern vertrauen

Wenn sich Homeport zum ersten Mal mit einem SSH-Server verbindet, bittet es Sie, den Fingerabdruck des Serverschlüssels zu bestätigen, und es verweigert die Verbindung, falls sich dieser Schlüssel später ändert, bis Sie erneut bestätigen. TLS-Hosts prüfen das Zertifikat des Servers gegen Ihre CA und den Namen des Servers; Incus-Hosts über HTTPS akzeptieren nur das Zertifikat, mit dem sie hinzugefügt wurden. Siehe [SSH-Hosts](/docs/hosts/ssh/#first-connection).

### Incus-Zertifikate

Das Hinzufügen eines [Incus-Hosts über HTTPS](/docs/hosts/incus/#over-https-with-a-trust-token) erzeugt ein Client-Zertifikat und seinen privaten Schlüssel im Arbeitsspeicher, einen ECDSA-P-384-Schlüssel, und speichert sie verschlüsselt mit den anderen Geheimnissen des Hosts, nie im Klartext auf der Festplatte. Das eigene Zertifikat von Incus wird auf dieselbe Weise gespeichert und fixiert: Homeport akzeptiert genau dieses Zertifikat vom Server, verglichen über seinen SHA-256-Fingerabdruck, und lehnt ein anderes ab, wobei es beide Fingerabdrücke zeigt. Das Trust-Token wird einmal verwendet und nicht aufbewahrt. Entfernen Sie das Zertifikat von Homeport auf dem Server mit `incus config trust remove`, um es zu widerrufen.

## Womit sich Homeport verbindet

- **Mit den Hosts, die Sie hinzufügen.** Wenn Sie ein Image pullen, pushen oder bauen, kontaktiert die Engine die Registry, nicht Homeport. Homeport übergibt ihr mit der Anfrage die gespeicherte Anmeldung für diese Registry, über die Verbindung des Hosts; die Engine bewahrt sie nicht auf.
- **Mit den Registrys Ihrer Images, über die Engine jedes Hosts**, um festzustellen, ob ein [neueres Image](/docs/guides/containers/#newer-images) veröffentlicht ist: wenn die Seite „Container“, „Images“ oder „Stacks“ lädt (jede Antwort wird 6 Stunden aufbewahrt) und wenn Sie **Nach neueren Images suchen** wählen. Die Engine fragt die Registry nach dem Digest, auf den ein Tag verweist, bei privaten Registrys mit der gespeicherten Anmeldung, und lädt nichts herunter. Bei einem lokalen Host läuft die Engine auf Ihrem Computer, daher geht die Anfrage von ihm aus.
- **Mit Docker Hub, wenn Sie Tags durchsuchen** im Pull-Dialog: Homeport listet die Tags eines Repositorys von `hub.docker.com` auf, von diesem Computer aus, mit Ihrer Docker-Hub-Anmeldung, falls eine gespeichert ist. Die Suche nach Repositorys läuft stattdessen über die Engine des Hosts.
- **Mit einer Registry, wenn Sie ihre Anmeldung prüfen** in **Einstellungen → Registrys**: Homeport fragt den Endpunkt `/v2/` der Registry und ihren Token-Dienst von diesem Computer aus an und sendet Benutzername und Passwort oder Token über HTTPS (einfaches HTTP nur für eine Registry auf diesem Computer, etwa `localhost:5000`).
- **Mit den Servern Ihrer Repositorys**, für [synchronisierte Stacks](/docs/guides/synced-stacks/): wenn Sie den Zugriff testen, einen synchronisierten Stack erstellen oder synchronisieren, und in einem Intervall, solange Homeport geöffnet ist, falls die Abfrage aktiviert ist. Homeport führt `git` von diesem Computer aus mit dem Deploy-Key oder Token dieses Repositorys aus.
- **Mit den Clustern, die Sie hinzufügen**, über den Kubernetes-API-Server, den ihre kubeconfig nennt (und seinen Proxy, falls vorhanden), mit den Anmeldedaten, die sie liefert, wobei das Zertifikat des Servers gegen die CA der kubeconfig geprüft wird. Exec-Plugins kontaktieren ihre eigene Cloud (AWS, Google Cloud, Azure) von diesem Computer aus, wie sie es für `kubectl` tun.
- **Mit den Nomad-Clustern, die Sie hinzufügen**, über die Agentenadresse, die Sie angeben, und mit nichts sonst: Die Logs und Shells eines Jobs laufen über diesen Agenten, der sie an den Client-Node weiterleitet, sodass sich Homeport nie mit den Nodes selbst verbindet. Das Zertifikat des Agenten wird gegen die von Ihnen angegebene CA oder die CAs dieses Computers geprüft.
- **Mit Incus-Image-Servern**, für die Image-Auswahl auf einem [Incus-Host](/docs/guides/instances/#making-an-instance): Homeport lädt die Image-Liste eines Servers (`streams/v1/images.json`, von `images.linuxcontainers.org`, sofern Sie keinen anderen wählen) von diesem Computer herunter und bewahrt sie zehn Minuten auf. Incus lädt die Images selbst herunter.
- **Mit Chart-Repositorys und OCI-Registrys, über helm**, wenn Sie ein Repository hinzufügen oder aktualisieren, Charts suchen oder ein Chart installieren oder upgraden: `helm` lädt Indizes und Charts von diesem Computer herunter, wie in einem Terminal.
- **Mit dieser Website, um nach Updates zu suchen**: beim Start und alle 6 Stunden lädt Homeport `updates/latest.json` herunter, eine kleine Datei mit der neuesten Version. Die Anfrage enthält nichts über Sie, Ihre Hosts oder Ihre Container, erreicht aber wie jede Web-Anfrage GitHub Pages, das diese Website hostet, von Ihrer IP-Adresse aus. Schalten Sie dies unter **Einstellungen → Updates** aus. Die Updates selbst werden von GitHub Releases heruntergeladen, und Homeport installiert eines nur, wenn es mit dem Update-Schlüssel von Homeport signiert ist. Siehe [Aktualisieren](/docs/reference/updating/).

## Was Homeport auf Ihren Hosts ausführt

Homeport spricht über ihre API mit der Engine eines Hosts und führt nur für das, was diese API nicht kann, Befehle auf dem Server eines Hosts aus:

| Wann                                                                                            | Wo                                                                         | Befehl                                                                                                                       |
| ----------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| Der Server eines SSH-Hosts weigert sich, den Socket seiner Engine zu öffnen                     | der SSH-Server                                                             | eine POSIX-Shell-Prüfung, ob der Socket existiert und lesbar ist und ob eine Engine installiert ist                          |
| **Starten**, **Stoppen** oder **Neu starten** einer [Quadlet-Unit](/docs/guides/quadlet/)       | der SSH-Server oder, bei einem lokalen Socket unter Linux, dieser Computer | `systemctl [--user] start\|stop\|restart '<unit>.service'`                                                                   |
| Die [Logs eines containerd-Containers](/docs/hosts/containerd/#logs)                            | der SSH-Server oder, bei einem lokalen Socket unter Linux, dieser Computer | `tail -n <lines> [-F] -- '<log file>'`, nach `ls` des Logordners eines Kubernetes-Containers, wenn CRI die Datei nicht nennt |
| Ein [Terminal in einem containerd-Container](/docs/hosts/containerd/#a-terminal-in-a-container) | der SSH-Server oder, bei einem lokalen Socket unter Linux, dieser Computer | `mktemp -d`, `mkfifo -m 600` für zwei FIFOs in diesem Ordner, `cat` durch sie hindurch, dann `rm -rf` des Ordners            |

Sie laufen mit den Rechten des SSH-Benutzers (oder Ihren, auf diesem Computer) und nie mit `sudo`. Werte werden geprüft und maskiert, bevor sie in einen Befehl eingehen. Das Installieren, Bearbeiten und Entfernen von Quadlet-Units läuft über die API von Podman, die die Dateien selbst schreibt. Auf einem Incus-Host läuft nichts: Seine Konsole und seine Terminals laufen über die eigene API von Incus. Die einzigen Dateien, die Homeport auf einem Host anlegt, sind die zwei FIFOs eines containerd-Terminals, in einem privaten temporären Ordner, der beim Schließen des Terminals entfernt wird.

## Was Homeport nie tut

- **Kein Konto.** Es gibt nichts, wofür Sie sich registrieren müssten.
- **Keine Telemetrie, keine Analysen und keine Absturzberichte.**
- **Nichts wird auf Ihren Clustern installiert.** Homeport verwendet nur die Kubernetes-API, mit den Rechten Ihrer Anmeldedaten, und ändert nichts, ohne dass Sie es verlangen: Alles, was etwas entfernt oder ersetzt, fragt zuerst und nennt dabei den Cluster.
- **Nichts wird auf Ihren Hosts installiert.** Die einzigen Befehle, die Homeport auf einem Host ausführt, sind [oben aufgeführt](#what-homeport-runs-on-your-hosts). Der einzige Container, den Homeport je von sich aus startet, ist ein kurzlebiger [Hilfscontainer](/docs/guides/volumes-networks/#the-helper-container), wenn Sie ein Volume öffnen, sichern, wiederherstellen oder klonen; er hat kein Netzwerk und wird entfernt, wenn er fertig ist. Sein Image wird erst gepullt, wenn Sie zustimmen.
- **Keine stillen Installationen.** Ein Update wird nur installiert, wenn Sie **Installieren und neu starten** wählen; Linux-Pakete werden nur von Ihrem Paketmanager aktualisiert.

Die [Datenschutzerklärung](/privacy/) legt dar, was das rechtlich bedeutet: wer für Ihre Daten verantwortlich ist, auf welcher Grundlage und wie Sie Ihre Rechte ausüben.

## Logs

Homeport schreibt eine kleine Logdatei – einige Dutzend Kilobyte, ersetzt, wenn sie voll ist – zur Diagnose von Problemen. Sie enthält Verbindungsereignisse und Fehler, nicht Ihre Anmeldedaten oder Container-Ausgaben.

| System  | Ordner                                  |
| ------- | --------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` |
| macOS   | `~/Library/Logs/com.homeport.app/`      |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` |

## Einem Download vertrauen

Releases werden von GitHub Actions aus dem öffentlichen Quellcode gebaut. Jede Datei kommt mit einer SHA-256-Prüfsumme und einer signierten Build-Provenienz-Attestierung, die sie an den genauen Commit und Workflow bindet, der sie gebaut hat. Siehe [Downloads prüfen](/docs/reference/verifying/).

## Eine Sicherheitslücke melden

Bitte melden Sie Sicherheitsprobleme vertraulich über die [private Meldung von Sicherheitslücken auf GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new) statt über ein öffentliches Issue.
