---
title: Glossar
description: Die Begriffe, die Homeport verwendet, und was sie bedeuten.
---

Homeport benennt Dinge genau. Diese Begriffe erscheinen in der gesamten App und in dieser Dokumentation. Jeder Eintrag trägt den englischen Begriff des Produkts, gefolgt von dem Wort, das die App auf Deutsch verwendet.

### Host (dt. _Host_)

Eine Engine, mit der sich Homeport verbindet, festgelegt durch einen Namen und den Weg, sie zu erreichen: ein lokaler Socket, TCP, TLS oder SSH. Jeder Container, jedes Image, jedes Volume, jedes Netzwerk und jeder Stack gehört zu genau einem Host. Siehe [Hosts und Verbindungen](/docs/hosts/).

### Engine kind (dt. _Engine-Art_)

Welche Art von Engine ein Host anspricht: Docker-kompatibel (Docker Engine oder Podman über seine kompatible API), containerd oder Incus. Sie bestimmt, welche Seiten ein Host zeigt: containerd- und Incus-Hosts erhalten eigene. Podman ist keine eigene Art: Homeport erkennt es bei jeder Verbindung. Siehe [Engine-Arten](/docs/hosts/#engine-arten).

### Namespace (containerd’s) (dt. _containerd-Namespace_)

Auf einem containerd-Host der Raum, in dem jeder Container, jedes Image und jeder Snapshot lebt: nerdctl verwendet `default`, Kubernetes `k8s.io`, Docker `moby`. Die Seiten eines containerd-Hosts zeigen jeweils einen. Die Namespaces eines Clusters sind die von Kubernetes. Siehe [containerd-Namespaces](/docs/hosts/containerd/#namespaces).

### Snapshot (dt. _Snapshot_)

Auf einem containerd-Host eine Schicht von Dateien auf der Festplatte: eine entpackte Image-Schicht oder die beschreibbare Schicht eines Containers. Siehe [Snapshots](/docs/hosts/containerd/#snapshots). Auf einem Incus-Host ein gespeicherter Zustand einer Instanz, auf den sie zurückgesetzt werden kann; siehe [Instanz-Snapshots](/docs/guides/instances/#snapshots).

### Instance (dt. _Instanz_)

Was ein Incus-Host ausführt: ein Systemcontainer, der eine ganze Linux-Distribution auf dem Kernel des Hosts bootet, oder eine virtuelle Maschine mit eigenem Kernel. Kein Container, der eine einzige Anwendung ausführt. Siehe [Instanzen](/docs/guides/instances/).

### Project (Incus’s) (dt. _Projekt_)

Auf einem Incus-Host der Raum, in dem seine Instanzen, Images, Profile, benutzerdefinierten Volumes und Netzwerke leben. Die Seiten eines Incus-Hosts zeigen jeweils eines, gewählt unter dem Host-Umschalter. Siehe [Projekte](/docs/hosts/incus/#projekte).

### Profile (dt. _Profil_)

Auf einem Incus-Host Konfiguration und Geräte, die von den Instanzen geteilt werden, die es aufführen. Siehe [Profile](/docs/guides/instances/#profile).

### Trust token (dt. _Trust-Token_)

Was `incus config trust add` auf einem Incus-Server ausgibt und einmal in Homeport eingefügt wird, um diesen Server über HTTPS hinzuzufügen. Siehe [Trust-Tokens](/docs/hosts/incus/#über-https-mit-einem-trust-token).

### Pod (dt. _Pod_ / _Podman-Pod_)

Auf einem Podman-Host eine Gruppe von Containern, die sich ein Netzwerk teilen und zusammen starten und stoppen, um einen Infra-Container herum, der die Ports hält, die der Pod veröffentlicht. Die Pods eines Clusters sind Kubernetes-Pods. Siehe [Podman-Pods](/docs/guides/podman-pods/).

### Quadlet unit (dt. _Quadlet-Unit_)

Eine Datei auf einem Podman-Host, etwa `web.container`, die systemd in einen Dienst verwandelt, `web.service`, sodass die Container von Podman beim Booten starten und systemd sie am Laufen hält. Siehe [Quadlet-Units](/docs/guides/quadlet/).

### Bastion (dt. _Bastion_)

Ein SSH-Server, über den ein SSH-Host erreicht wird, wenn der eigene SSH-Server des Hosts nur Verbindungen von ihm annimmt: das, was OpenSSH `ProxyJump` nennt. Ein Host kann mehrere haben, in einer Reihenfolge, jede mit ihrem eigenen vertrauten Serverschlüssel. Eine Bastion ist kein Host: Auf ihr läuft keine Engine, mit der Homeport spricht. Siehe [Über eine Bastion](/docs/hosts/ssh/#über-eine-bastion).

### Cluster (dt. _Cluster_)

Ein Kubernetes-Cluster oder ein Nomad-Cluster, mit dem sich Homeport über seine API verbindet. Ein Kubernetes-Cluster wird aus einem Kontext einer kubeconfig hinzugefügt: ein Name, die Adresse des API-Servers und die Anmeldedaten, die die kubeconfig liefert, verschlüsselt gespeichert. Ein Nomad-Cluster wird über die Adresse eines seiner Agenten hinzugefügt, mit einem ACL-Token und TLS-Material, verschlüsselt gespeichert. Homeport installiert auf keinem von beiden etwas und erreicht nur, was diese Anmeldedaten erlauben. Kein Swarm, der über seine Hosts gefunden und nie hinzugefügt wird. Die eigenen Objekte von Kubernetes behalten ihre Kubernetes-Namen: Namespace, Pod, Deployment, Service, Node… Die von Nomad behalten die von Nomad: Job, Task Group, Task, Allocation, Client-Node, Namespace. Siehe [Cluster](/docs/clusters/) und [Nomad-Cluster](/docs/clusters/nomad/).

### Allocation (dt. _Allocation_)

Auf einem Nomad-Cluster eine Platzierung der Task Group eines Jobs auf einem Client-Node, die ihre Tasks ausführt. Ihre Logs und eine Shell darin laufen über die Server von Nomad zum Node. Siehe [Logs und Shell](/docs/clusters/nomad/#logs-und-shell).

### Swarm (dt. _Swarm_)

Ein Docker-Swarm-Cluster, gefunden über die Hosts, die zu ihm gehören, und nie für sich allein hinzugefügt. Homeport erreicht ihn über jeden verbundenen Manager-Host, unterscheidet Swarms anhand ihrer Swarm-ID und speichert nichts über einen. Er ist nach seinem ersten Manager-Host benannt, da Docker jedem Swarm denselben Namen gibt. Siehe [Swarms](/docs/guides/swarm/).

### Port forward (dt. _Portweiterleitung_)

Ein Port auf dem Loopback dieses Computers, den Homeport über die Kubernetes-API mit einem Port eines Pods verbindet, oder mit dem eines Services über einen seiner bereiten Pods, wie `kubectl port-forward`. Sie endet, wenn Homeport beendet wird. Siehe [Portweiterleitungen](/docs/clusters/port-forward/).

### Release (dt. _Release_)

Ein Helm-Release: eine Installation eines Charts auf einem Cluster, benannt in seinem Namespace, mit einer nummerierten Revision für jede Installation, jedes Upgrade und jedes Zurücksetzen. Helm bewahrt es auf dem Cluster auf; ein in einem Terminal erstelltes Release und ein in Homeport erstelltes sind dasselbe. Siehe [Helm-Releases](/docs/clusters/helm/).

### Chart repository (dt. _Chart-Repository_)

Ein Server, von dem helm Charts herunterlädt, in der eigenen Liste von helm auf diesem Computer – derjenigen, die `helm repo list` zeigt. Charts in einer OCI-Registry werden stattdessen über ihre `oci://`-Referenz installiert. Siehe [Chart-Repositorys](/docs/clusters/helm/#chart-repositorys).

### Node (dt. _Node_)

Eine Engine in einem Swarm: ein Manager oder ein Worker. Die Nodes eines Clusters sind Kubernetes-Nodes, die Rechner, auf denen er Pods ausführt. Ein Node ist nur dann ein Host, wenn Homeport sich mit ihm verbindet, und die Seite eines Hosts verlinkt auf seinen Node.

### Service (dt. _Service_)

Was ein Swarm ausführt: ein Image, seine Einstellungen und eine Anzahl von Replikas oder, bei einem globalen Service, ein Task auf jedem Node.

### Task (dt. _Task_)

Ein Platz eines Services auf einem Node, dort als Container ausgeführt. Wenn ein Service nicht konvergiert, sagen seine Tasks, warum. Auf einem containerd-Host ist ein Task das, was für einen Container läuft, der nur Metadaten ist: Das Starten eines Containers erstellt seinen Task.

### Stack (dt. _Stack_)

Ein Compose-Projekt, das auf einem Host läuft, oder ein Swarm-Stack, der auf einem Swarm läuft, identifiziert über seinen Projektnamen. Jeder Stack ist verwaltet, verknüpft, synchronisiert oder extern. Siehe [Stacks](/docs/guides/stacks/) und [Swarm-Stacks](/docs/guides/swarm/#swarm-stacks).

### Managed Stack (dt. _verwalteter Stack_)

Ein Stack, dessen Compose-Definition und Umgebung Homeport hält, sodass er aus Homeport bereitgestellt, neu bereitgestellt und bearbeitet werden kann.

### Linked Stack (dt. _verknüpfter Stack_)

Ein Stack, dessen Compose-Dateien ein Ordner auf diesem Computer sind. Homeport merkt sich, wo der Ordner und seine Compose-Dateien liegen, liest und schreibt die Dateien an Ort und Stelle und stellt mit diesem Ordner als Projektverzeichnis bereit, sodass `build:`, `env_file:` und relative Pfade funktionieren. Gefunden über die Compose-Labels laufender Container oder aus einem Ordner erstellt. Er wird extern, solange seine Dateien fehlen. Siehe [verknüpfte Stacks](/docs/guides/stacks/#verknüpfte-stacks).

### Synced Stack (dt. _synchronisierter Stack_)

Ein Stack, dessen Compose-Definition aus einem git-Repository stammt. Homeport bewahrt einen Klon davon auf diesem Computer auf, ruft einen Branch oder Tag bei Bedarf oder in einem Intervall ab, solange Homeport geöffnet ist, zeigt, was sich geändert hat, und stellt einen Commit aus dem Ordner der Compose-Datei bereit, sodass `build:` und relative Pfade funktionieren. Er wird nie in Homeport bearbeitet: Seine Dateien ändern sich über git; nur seine Umgebung und seine Synchronisierungseinstellungen gehören Homeport. Siehe [synchronisierte Stacks](/docs/guides/synced-stacks/).

### Repository (dt. _Repository_)

Ein git-Repository, das Homeport für synchronisierte Stacks lesen darf: seine URL und wie sich Homeport dort authentifiziert – mit einem von ihm erzeugten Deploy-Key, einem Benutzernamen und Token oder bei einem öffentlichen mit nichts. Einmal für alle Hosts aufbewahrt und von jedem daraus erstellten synchronisierten Stack geteilt.

### Deploy key (dt. _Deploy-Key_)

Der Ed25519-Schlüssel, den Homeport für ein Repository erzeugt und den Sie dort in Ihrer Git-Forge nur mit Lesezugriff hinzufügen. Nie Ihr eigener SSH-Schlüssel.

### External Stack (dt. _externer Stack_)

Ein auf einem Host laufend gefundener Stack, dessen Compose-Definition Homeport nicht hat, weder gespeichert noch in einem Ordner auf diesem Computer noch in einem Repository, dem es folgt. Er kann gestartet, gestoppt und entfernt, aber nicht neu bereitgestellt oder bearbeitet werden.

### Adopt (dt. _übernehmen_)

Einen externen Stack in einen verwalteten Stack verwandeln, indem man seine Compose-Definition liefert.

### Registry login (dt. _Registry-Anmeldung_)

Der Benutzername und das Passwort oder Zugriffstoken, die Homeport für eine Container-Registry wie Docker Hub oder GHCR aufbewahrt und auf jedem Host zum Pullen, Pushen und Bauen verwendet. Siehe [Einstellungen](/docs/guides/settings/#registrys).

### Helper container (dt. _Hilfscontainer_)

Ein kurzlebiger `busybox`-Container, den Homeport auf einem Host erstellt, um ein Volume zu öffnen – um es zu durchsuchen, zu sichern, wiederherzustellen oder zu klonen –, da die Engine API keines lesen kann. Er hat kein Netzwerk, bindet das Volume schreibgeschützt ein, sofern er nicht hineinschreiben muss, und wird entfernt, wenn die Aktion endet. Er erscheint nie in der Containerliste oder auf der Seite „Ereignisse“. Siehe [Die Dateien eines Volumes durchsuchen](/docs/guides/volumes-networks/#der-hilfscontainer).

### Board (dt. _Board_)

Die Seite, die die Container jedes verbundenen Hosts in einer Tabelle auflistet, jeweils mit ihrem Host beschriftet. Siehe [Board](/docs/guides/overview/#board).

### Command palette (dt. _Befehlspalette_)

Das mit <kbd>Mod</kbd> <kbd>K</kbd> geöffnete Suchfeld: zu einer Seite wechseln, den Host wechseln, einen Befehl ausführen oder einen Container, ein Image, ein Volume, ein Netzwerk oder einen Stack auf einem beliebigen verbundenen Host finden. Siehe [Tastatur](/docs/guides/keyboard/#befehlspalette).

### Engine (dt. _Engine_)

Die Software, auf die ein Host zeigt, die installiert, gestartet und gestoppt wird: die Docker Engine (`dockerd`) oder eine kompatible wie Docker Desktop, Podman, Colima oder OrbStack, mit der Homeport über die Docker Engine API spricht; oder containerd oder Incus, über deren eigene. Kein Synonym für Host: „Starten Sie die Docker Engine“ betrifft die Engine, „Homeport kann prod-server nicht erreichen“ betrifft den Host.

### Socket (dt. _Socket_)

Die Adresse, auf der eine Engine lauscht: eine Unix-Socket-Datei wie `/var/run/docker.sock` oder unter Windows eine Named Pipe wie `//./pipe/docker_engine`.

### Keychain (dt. _Schlüsselbund_)

Der sichere Speicher Ihres Betriebssystems für Passwörter und Schlüssel – macOS-Schlüsselbund, Windows-Anmeldeinformationsverwaltung oder ein Linux-Secret-Service –, in dem Homeport den Schlüssel aufbewahrt, der seine Geheimnisse verschlüsselt.

### Trust on first use (dt. _Vertrauen bei erster Verwendung_)

Wie Homeport SSH-Server prüft: Sie bestätigen bei der ersten Verbindung den Fingerabdruck des Serverschlüssels, und Homeport verweigert die Verbindung, wenn sich der Schlüssel später ändert, bis Sie erneut bestätigen.

## Auf Deutsch

Die deutsche App, Dokumentation und Website verwenden diese Begriffe, damit ein Bildschirm und eine Seite immer dasselbe sagen. Jede andere Sprache hat ihre eigene Tabelle in [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n).

| Englisch       | Deutsch                |
| -------------- | ---------------------- |
| Host           | Host (m.)              |
| Engine         | Engine (f.)            |
| Instance       | Instanz (f.)           |
| Swarm          | Swarm (m.)             |
| Cluster        | Cluster (m.)           |
| Stack          | Stack (m.)             |
| Managed Stack  | verwalteter Stack      |
| Linked Stack   | verknüpfter Stack      |
| Synced Stack   | synchronisierter Stack |
| External Stack | externer Stack         |
| Adopt          | übernehmen             |
| Board          | Board (n.)             |
| Bastion        | Bastion (f.)           |
| Deploy key     | Deploy-Key (m.)        |
| Port forward   | Portweiterleitung (f.) |
| Release (Helm) | Release (n.)           |
