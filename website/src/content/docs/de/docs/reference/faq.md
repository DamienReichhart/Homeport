---
title: FAQ
description: Häufige Fragen zu Homeport.
---

### Ist Homeport kostenlos?

Ja. Homeport ist kostenlos nutzbar, für private wie für berufliche Zwecke, und quelloffen unter der [GNU Affero General Public License, Version 3](https://github.com/DamienReichhart/Homeport/blob/main/LICENSE) (AGPL).

### Was bedeutet die AGPL für mich?

Nichts ändert sich, wenn Sie Homeport verwenden: Installieren Sie es, nutzen Sie es zu Hause oder bei der Arbeit, auf so vielen Computern, wie Sie möchten. Die AGPL verlangt nur etwas von denen, die Homeport **weitergeben**, verändert oder nicht, oder anderen eine veränderte Version über ein Netzwerk anbieten: Sie müssen den vollständigen Quellcode unter derselben Lizenz veröffentlichen. Jede Kopie von Homeport verlinkt unter **Einstellungen → Lizenzen** auf den Quellcode ihrer genauen Version.

Versionen bis 0.11 wurden unter der MIT-Lizenz veröffentlicht und bleiben unter ihr verfügbar.

### Ersetzt Homeport Docker Desktop?

Nicht ganz. Homeport ist ein Verwaltungsprogramm: Es führt selbst keine Docker Engine aus. Installieren Sie unter Linux Docker Engine und verwenden Sie Homeport als Oberfläche dafür. Unter macOS und Windows brauchen Sie weiterhin etwas, das die Engine ausführt – Docker Desktop, Colima, OrbStack, Rancher Desktop oder Podman –, und Homeport kann auf jedem davon aufsetzen, zusammen mit Ihren entfernten Servern.

### Funktioniert es mit containerd?

Ja, direkt, ohne Docker: Die Engine eines Hosts kann [containerd](/docs/hosts/containerd/) sein, erreicht über seinen Socket auf diesem Rechner oder über SSH – das von k3s, das eigene von Docker Engine oder der containerd-Dienst auf einem Server. Homeport listet seine Namespaces, seine Container und deren Tasks (starten, stoppen, pausieren, Signal senden, entfernen), ihre Logs dort, wo nerdctl oder Kubernetes sie geschrieben haben, ein Terminal darin, seine Images (pullen, entfernen) und seine Snapshots auf. Es erstellt dort keine Container und stellt keine Stacks bereit; das erledigt nerdctl.

### Funktioniert es mit Incus?

Ja: Die Engine eines Hosts kann [Incus](/docs/hosts/incus/) 6.0 oder neuer sein, erreicht über seinen Socket auf diesem Rechner, über SSH oder über HTTPS mit einem Trust-Token aus `incus config trust add`. Homeport listet die [Instanzen](/docs/guides/instances/) eines Projekts auf, Systemcontainer und virtuelle Maschinen, startet, stoppt, friert ein, löscht und erstellt sie, öffnet ihre Konsole und ein Terminal darin, erstellt Snapshots und stellt sie wieder her und verwaltet Images, Profile, Speicherpools und Netzwerke. Es stellt dort keine Stacks bereit und installiert nichts auf dem Host.

### Funktioniert es mit LXD?

Nicht als solches: Homeport spricht Incus, den Community-Fork von LXD, und lehnt einen LXD-Server ab, dessen API sich auseinanderentwickelt hat. Migrieren Sie ihn mit `lxd-to-incus` und fügen Sie ihn dann als Incus-Host hinzu.

### Funktioniert es mit Podman?

Ja, über den Docker-kompatiblen API-Socket von Podman. Homeport erkennt die üblichen Podman-Sockets unter Linux, jede laufende [Podman-Maschine unter macOS](/docs/hosts/local/#podman-maschinen-unter-macos) und die Pipe der Standard-Podman-Maschine unter Windows. Alles andere kann [von Hand hinzugefügt](/docs/hosts/local/#einen-socket-von-hand-hinzufügen) werden.

Homeport erkennt Podman außerdem beim Verbinden und bietet dann an, was nur Podman hat, über die eigene API von Podman auf demselben Socket: [Pods](/docs/guides/podman-pods/), die Sie auflisten, starten, stoppen, erstellen und entfernen und in denen Sie über den Ausführen-Dialog Container ausführen können; und [Quadlet-Units](/docs/guides/quadlet/), die Sie auflisten, lesen, installieren, bearbeiten und entfernen sowie über SSH starten oder stoppen können.

### Funktioniert es mit Kubernetes, Nomad oder Docker Swarm?

Docker Swarm: ja. Homeport findet einen Swarm über seine Manager-Hosts, sobald einer davon ein Host ist, und verwaltet ihn vollständig: seine Nodes, Services, Tasks, Secrets, Configs und Swarm-Stacks, einschließlich darin bereitgestellter verwalteter, verknüpfter und synchronisierter Stacks. Es kann auch einen Swarm auf einem Host starten und einem beitreten oder ihn verlassen. Alles läuft über die Engine API eines verbundenen Managers; auf den Nodes wird nichts installiert. Siehe [Swarms](/docs/guides/swarm/).

Kubernetes: ja. Fügen Sie einen Cluster aus Ihrer kubeconfig hinzu – der dieses Computers oder einer, die Sie einfügen –, und Homeport findet lokale Cluster (Docker Desktop, OrbStack, Rancher Desktop, kind, k3d, minikube) für Sie. EKS, GKE und AKS funktionieren über das Exec-Plugin, das ihre kubeconfig nennt. Sie können Namespaces, Workloads, Pods, Services, Ingresses, ConfigMaps, Secrets, Nodes, Events und jede andere API-Ressource, benutzerdefinierte eingeschlossen, live durchsuchen; Logs verfolgen, ein Terminal in einem Pod öffnen, Ports weiterleiten, skalieren, neu starten und löschen sowie YAML mit Vorschau anwenden oder bearbeiten. Auf dem Cluster wird nichts installiert: Homeport spricht nur mit der Kubernetes-API, die Ihre kubeconfig bereits erreicht, mit den Rechten ihrer Anmeldedaten. Mit der `helm`-CLI auf diesem Computer listet Homeport außerdem [Helm-Releases](/docs/clusters/helm/) auf, installiert, upgradet, setzt sie zurück und deinstalliert sie, und verwaltet die Chart-Repositorys von helm. Siehe [Cluster](/docs/clusters/).

Nomad: ja. Fügen Sie einen Cluster über die Adresse seines Agenten hinzu, mit einem ACL-Token und TLS-Material, wenn er sie verwendet. Sie können Jobs und ihre Allocations live auflisten; einen Job aus HCL oder JSON ausführen, nachdem Sie den Plan von Nomad gesehen haben; einen bearbeiten und erneut ausführen, ihn stoppen oder bereinigen und seine Task Groups skalieren; die Logs eines Tasks verfolgen und eine Shell darin öffnen; Nodes leeren und ihre Eignung umschalten; und einen Namespace wählen. Auf dem Cluster wird nichts installiert, und keine `nomad`-CLI wird benötigt: Homeport spricht mit den Rechten des Tokens mit der HTTP-API von Nomad. Siehe [Nomad-Cluster](/docs/clusters/nomad/).

### Brauche ich kubectl?

Nein. Homeport liest Ihre kubeconfig und spricht selbst mit der Kubernetes-API. Die einzigen Programme, die es ausführt, sind die [Exec-Plugins](/docs/clusters/exec-plugins/), die eine kubeconfig nennt, etwa die AWS CLI für EKS, `gke-gcloud-auth-plugin` für GKE oder `kubelogin` für AKS – dieselben, die `kubectl` ausführen würde. `kubectl` zur Hand zu haben, hilft trotzdem, um zu prüfen, was Ihre Anmeldedaten dürfen (`kubectl auth can-i`).

### Muss ich etwas auf meinen Servern installieren?

Nein – nur eine Docker Engine und einen SSH-Server, die Sie höchstwahrscheinlich schon haben. Homeport installiert keine Agenten.

Auch ein Swarm braucht nicht mehr als die Engine. Seine Nodes müssen einander jedoch erreichen, was Homeport nicht für Sie einrichten kann: Öffnen Sie zwischen ihnen 2377/tcp (zu den Managern), 7946/tcp und 7946/udp sowie 4789/udp. Siehe [Ports zwischen Nodes](/docs/guides/swarm/#ports-zwischen-nodes).

### Muss Docker auf meinem Computer installiert sein, um entfernte Server zu verwalten?

Nein. Homeport spricht die Engine API und SSH selbst. Die einzigen Ausnahmen sind das Bereitstellen verwalteter, verknüpfter und synchronisierter Stacks, das den Befehl `docker compose` auf Ihrem Computer verwendet, und ihr Bereitstellen in einem Swarm, das den Befehl `docker` verwendet (`docker stack deploy`). Keines von beiden braucht eine Engine auf Ihrem Computer.

### Hält Homeport meine Stacks synchron, wenn es geschlossen ist?

Nein. [Synchronisierte Stacks](/docs/guides/synced-stacks/) werden nur auf neue Commits geprüft und, wenn Sie das einschalten, automatisch bereitgestellt, solange Homeport geöffnet ist. Homeport installiert nichts auf Ihren Servern – keinen Agenten, keinen Webhook-Empfänger –, daher überwacht nichts Ihre Repositorys, wenn es geschlossen ist. Wenn Sie es wieder öffnen, wartet es ein Intervall und prüft dann alle. Für Bereitstellungen, die rund um die Uhr jedem Push folgen müssen, führen Sie sie aus Ihrer CI aus.

Damit Homeport ohne sein Fenster weiterläuft, schalten Sie **In den Infobereich schließen** sowie **Bei der Anmeldung starten** mit **Verborgen im Infobereich starten** unter [Einstellungen → Desktop](/docs/guides/settings/#desktop) ein.

### Kann Homeport mir sagen, wenn ein Container abstürzt?

Ja, solange es läuft – auch verborgen im Infobereich. Es zeigt eine Benachrichtigung des Betriebssystems, wenn ein Container mit einem Fehler stoppt, keinen Arbeitsspeicher mehr hat oder ungesund wird, und wenn ein Host nicht mehr erreichbar ist oder wieder zurückkommt. Jede Art lässt sich unter [Einstellungen → Benachrichtigungen](/docs/guides/settings/#benachrichtigungen) ausschalten. Es erfährt dies aus dem Ereignisstrom, den es ohnehin mit jedem Host hält, daher wird nichts auf Ihren Servern installiert; wenn Homeport nicht läuft, wird niemand benachrichtigt.

### Braucht Homeport git?

Nur für [synchronisierte Stacks](/docs/guides/synced-stacks/), die das auf Ihrem Computer installierte `git` verwenden, mit dem eigenen Deploy-Key oder Token jedes Repositorys. **Einstellungen → Dieser Rechner** zeigt, ob Homeport es gefunden hat. Ihre eigene git-Konfiguration, Credential-Helper und SSH-Schlüssel werden nie verwendet.

### Können mehrere Personen die Hosts von Homeport teilen?

Jede Person führt ihr eigenes Homeport mit ihren eigenen Hosts und Schlüsseln aus. Mit einem von Homeport erstellten Schlüssel pro Person und pro Server können Sie den Zugriff jeder Person in `authorized_keys` sehen und widerrufen.

### Kann ich Homeport auf einen anderen Computer umziehen?

Ja, auf zwei Arten:

- **Alles**: **Einstellungen → Daten → Sichern…** speichert jeden Host, Cluster, Stack, jede Registry-Anmeldung, jedes Repository und jede Einstellung, mit ihren Geheimnissen, in eine Datei, die durch eine Passphrase mit mindestens 12 Zeichen geschützt ist. Auf dem neuen Computer öffnet **Wiederherstellen…** an derselben Stelle sie, listet auf, was sie enthält, und ersetzt, was dort ist. Alles verbindet sich und wird bereitgestellt, ohne Sie erneut nach einem Schlüssel oder Passwort zu fragen. Siehe [Ihre Daten und Sicherungen](/docs/reference/data-and-backups/#in-homeport-sichern).
- **Einige Hosts**: Auf der Seite **Hosts** **exportieren** Sie die gewählten Hosts und **importieren** sie auf dem anderen Computer, neben den Hosts, die er bereits hat. Siehe [Auf einen anderen Computer umziehen](/docs/reference/security/#auf-einen-anderen-computer-umziehen).

Das Kopieren von `homeport.db` funktioniert für Geheimnisse nicht: Sie sind mit einem Schlüssel verschlüsselt, der im Schlüsselbund dieses Computers liegt, und erscheinen anderswo als **Zugangsdaten verloren**. Siehe [Die Dateien selbst kopieren](/docs/reference/data-and-backups/#die-dateien-selbst-kopieren).

### Wo bewahrt Homeport seine Daten auf?

In einer einzigen SQLite-Datenbank, `homeport.db`, im Anwendungsdatenordner Ihres Benutzers, und den Schlüssel, der ihre Geheimnisse verschlüsselt, im Schlüsselbund Ihres Betriebssystems. Nichts wird auf Ihren Servern oder irgendwo online aufbewahrt. [Ihre Daten und Sicherungen](/docs/reference/data-and-backups/) nennt den Ordner auf jedem System, was zu sichern ist und wie Sie es wiederherstellen.

### Enthält eine Sicherung meine Container und Volumes?

Nein. Eine Sicherung enthält, was Homeport aufbewahrt – wie Ihre Hosts erreicht werden, die Definitionen und Umgebungen Ihrer Stacks, Anmeldungen und Einstellungen –, nicht, was auf Ihren Hosts läuft. Ihre Container, Images und Volumes bleiben auf den Hosts. Um die Daten eines Volumes zu kopieren, [sichern Sie es in eine Datei](/docs/guides/volumes-networks/#sichern-und-wiederherstellen) über die Seite „Volumes“.

### Kann Homeport Images bauen?

Ja, an zwei Stellen. Die Seite **Images** baut ein Image aus einem Ordner auf diesem Computer. Und ein [verknüpfter Stack](/docs/guides/stacks/#verknüpfte-stacks) – ein Stack, dessen Compose-Dateien in einem Ordner auf diesem Computer liegen – kann `build:` verwenden: Homeport führt `docker compose` in diesem Ordner aus, sodass der Build-Kontext von dort an die Engine des Hosts gesendet wird, die ihn baut. **Neu bauen und bereitstellen** baut nach einer Codeänderung neu. Ein verwalteter Stack kann nicht bauen, da Homeport nur sein YAML hält und es keinen Ordner gibt, aus dem gebaut werden könnte: Verknüpfen Sie stattdessen den Ordner oder pushen Sie das Image in eine Registry.

### Warum erscheint mein Stack auf meinem anderen Computer als extern?

Ein mit `docker compose up` in einem Ordner gestarteter Stack erscheint nur auf einem Computer als [verknüpft](/docs/guides/stacks/#wie-verknüpfte-stacks-gefunden-werden), auf dem dieser Ordner und seine Compose-Dateien unter demselben Pfad existieren und die laufenden Services definieren. Auf einem anderen Computer existiert der Pfad nicht, daher hat Homeport keine Compose-Datei dafür und zeigt ihn als extern an. Um ihn auch von dort zu verwalten, klonen Sie das Projekt auf diesen Computer und verknüpfen Sie es mit **Neuer Stack → Ordner auf diesem Computer verwenden**, oder übernehmen Sie ihn als verwalteten Stack.

### Sammelt Homeport Daten über mich?

Nein. Es gibt kein Konto, keine Telemetrie, keine Analysen und keine Absturzberichte. Die einzige Anfrage, die Homeport von sich aus von Ihrem Computer stellt, ist die Update-Prüfung, die eine kleine Datei von dieser Website herunterlädt und nichts über Sie sendet; Sie können sie unter **Einstellungen → Updates** ausschalten. Unabhängig davon fragt die Prüfung auf neuere Images die Registrys Ihrer Images über die Engine jedes Hosts, auf welche Version ihre Tags verweisen; bei einem lokalen Host geht auch diese Anfrage von Ihrem Computer aus. Siehe [Sicherheit und Ihre Daten](/docs/reference/security/#womit-sich-homeport-verbindet).

### Kann Homeport meine privaten Registrys verwenden?

Ja. Fügen Sie unter **Einstellungen → Registrys** eine Anmeldung für jede Registry hinzu – Docker Hub, GHCR oder Ihre eigene. Homeport übergibt sie der Engine zum Pullen, Pushen und Bauen auf jedem Host und pullt die Images eines verwalteten, verknüpften oder synchronisierten Stacks damit vor dem Bereitstellen. Passwörter und Tokens werden mit dem Schlüsselbund Ihres Betriebssystems verschlüsselt. Homeport liest die Anmeldungen von `docker login` auf diesem Computer nicht.

### Woher weiß Homeport, dass es ein neueres Image gibt?

Es fragt die Registry des Images über die Engine des Hosts, auf welchen Digest der Tag jetzt verweist, und vergleicht ihn mit dem, den der Host gepullt hat. Nichts wird heruntergeladen, aber Docker Hub rechnet jede Frage auf sein Pull-Ratenlimit an, wie ein `docker pull`, der nichts Neues findet. Daher bewahrt Homeport jede Antwort 6 Stunden auf, fragt höchstens vier Registrys gleichzeitig und fragt nur früher erneut, wenn Sie **Nach neueren Images suchen** wählen. Eine Docker-Hub-Anmeldung unter **Einstellungen → Registrys** hebt das Limit an. Ist eine Registry nicht erreichbar oder begrenzt sie die Rate, zeigt Homeport für ihre Images keine Markierung und schreibt eine Zeile in sein Log.

### Wie aktualisiere ich?

Unter Windows, macOS und mit dem AppImage sagt Homeport Ihnen, wenn eine neue Version erschienen ist: Wählen Sie **Installieren und neu starten**. Unter Linux werden die Pakete `.deb` und `.rpm` zusammen mit dem Rest Ihres Systems aus dem Paket-Repository von Homeport aktualisiert. Siehe [Aktualisieren](/docs/reference/updating/).

### Verliere ich durch ein Update meine Hosts?

Nein. Jede Version von Homeport öffnet die Daten jeder früheren Version, zurück bis 0.1.0, und aktualisiert sie, ohne etwas zu verlieren: Hosts, Stacks, Cluster, Einstellungen und Geheimnisse. Der Wechsel zurück zu einer älteren Version ist das Einzige, was nicht unterstützt wird: Das ältere Homeport hält mit einer Meldung an, statt etwas zu ändern. Siehe [Ihre Daten über Versionen hinweg](/docs/reference/updating/#ihre-daten-über-versionen-hinweg).

### Gibt es Homeport bei Homebrew, winget, Flathub oder im AUR?

Noch nicht. Unter Linux hat Homeport eigene APT- und RPM-Repositorys, sodass `apt`, `dnf` und `zypper` es installieren und aktualisieren. Homebrew, winget, Scoop, das AUR, Flathub und Snap sind geplant; die [Installationsseite](/docs/installation/#später-verfügbar) verlinkt auf das Issue zu jedem davon.

### Ist Homeport in meiner Sprache verfügbar?

Homeport spricht Englisch, Französisch, Spanisch, Deutsch, Italienisch, brasilianisches Portugiesisch, Russisch, vereinfachtes Chinesisch, Japanisch, Koreanisch, Hindi und Arabisch und folgt der Sprache Ihres Betriebssystems, sofern Sie keine unter [Einstellungen → Darstellung](/docs/guides/settings/#darstellung) wählen. Die Website und diese Dokumentation gibt es ebenfalls in all diesen Sprachen; nur die rechtlichen Seiten sind auf Englisch. Um eine Sprache hinzuzufügen, siehe [Mitwirken](/docs/project/contributing/#homeport-übersetzen).

### Ist Homeport barrierefrei?

Es lässt sich allein mit der Tastatur bedienen, folgt der Einstellung Ihres Systems für reduzierte Bewegung, erfüllt den Kontrast nach WCAG 2.2 AA, und jede Seite wird mit automatisierten Barrierefreiheitswerkzeugen geprüft. Eine praktische Prüfung mit Screenreadern ist im Gange. Siehe [Tastatur und Barrierefreiheit](/docs/reference/accessibility/) für das, was funktioniert, die bekannten Grenzen und wie Sie eine Barriere melden.

### Gibt es eine Befehlszeilenschnittstelle?

Nein – Homeport ist eine Desktop-App. Ihre vorhandene `docker`-CLI funktioniert daneben, und Homeport aktualisiert sich live, wenn Sie im Terminal etwas ändern.

### Wie kann ich helfen?

Melden Sie Fehler und schlagen Sie Funktionen in den [GitHub-Issues](https://github.com/DamienReichhart/Homeport/issues) vor, oder siehe [Mitwirken](/docs/project/contributing/).
