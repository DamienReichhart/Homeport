---
title: Systemvoraussetzungen
description: Was Homeport zum Laufen braucht, und die optionalen Bestandteile, die SSH, TLS, Stack-Bereitstellungen, Kubernetes- und Nomad-Cluster sowie Helm ermöglichen.
---

## Betriebssysteme

| System  | Versionen                                                           | Architekturen        |
| ------- | ------------------------------------------------------------------- | -------------------- |
| Windows | Windows 10 und 11                                                   | x64, ARM64           |
| macOS   | macOS 10.15 Catalina oder neuer                                     | Apple Silicon, Intel |
| Linux   | glibc 2.35 oder neuer: Ubuntu 22.04, Debian 12, Fedora 36 und neuer | x64, ARM64           |

Homeport zeichnet sein Fenster mit der Web-Ansicht des Systems: WebView2 unter Windows (in Windows 10 und 11 enthalten), WebKit unter macOS und WebKitGTK unter Linux (als Abhängigkeit der Pakete `.deb` und `.rpm` installiert und im AppImage gebündelt).

## Eine Docker Engine zum Verwalten

Homeport enthält keine Container-Laufzeitumgebung. Es verwaltet Docker Engines, die bereits irgendwo laufen:

- **Auf diesem Rechner** – Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop oder Podman. Homeport findet sie für Sie; siehe [lokale Engines](/docs/hosts/local/).
- **Auf einem Server** – erreichbar über [SSH](/docs/hosts/ssh/) oder über [TLS oder TCP](/docs/hosts/tls-tcp/).

Jede Engine, die die Docker Engine API spricht, funktioniert, einschließlich des Docker-kompatiblen Sockets von Podman.

Außerdem verwaltet es [containerd](/docs/hosts/containerd/) 1.7 und neuer direkt, über seinen Socket auf diesem Rechner oder über SSH: den von k3s, den eigenen von Docker Engine oder den containerd-Dienst auf einem Server. Das Pullen von Images erfordert dort containerd 2.0 oder neuer, und Logs und Terminals erfordern, dass der Host über SSH erreicht wird, oder ein containerd auf diesem Computer (Linux).

Außerdem verwaltet es [Incus](/docs/hosts/incus/) 6.0 und neuer, über seinen Socket auf diesem Rechner, über SSH oder über HTTPS mit einem Trust-Token. Virtuelle Maschinen benötigen Hardware-Virtualisierung (`/dev/kvm`) auf dem Incus-Server, und ein Terminal darin benötigt den `incus-agent`, den ihr Image ausführt.

Auf einem Podman-Host nutzt Homeport zudem die eigene API von Podman: [Pods](/docs/guides/podman-pods/) benötigen Podman 4.0 oder neuer, und [Quadlet-Units](/docs/guides/quadlet/) werden ab Podman 5.7 aufgelistet und ab 5.8 installiert, bearbeitet und entfernt. Das Starten und Stoppen einer Quadlet-Unit erfordert, dass der Host über SSH erreicht wird, oder ein Podman auf diesem Computer (Linux).

## Optional: Kubernetes-Cluster

Um [Cluster](/docs/clusters/) zu durchsuchen, benötigt Homeport eine kubeconfig, die sie erreicht (die von `kubectl` verwendete oder eine, die Sie einfügen), und Netzwerkzugriff auf ihre API-Server. Es spricht die Kubernetes-API selbst, daher wird `kubectl` nicht benötigt.

- **Kubernetes-Versionen**: Cluster mit Kubernetes 1.24 oder neuer. Homeport verwendet nur stabile APIs (`v1`, `apps/v1`, `batch/v1`, `networking.k8s.io/v1`) und ist gegen die API 1.33 gebaut.
- **Exec-Plugins**: Eine Cloud-kubeconfig nennt ein Programm, das auf diesem Computer für ihre Anmeldedaten ausgeführt wird – die [AWS CLI](https://aws.amazon.com/cli/) (Version 2) für EKS, `gke-gcloud-auth-plugin` aus der Google Cloud CLI für GKE, `kubelogin` für AKS. Installieren Sie das Programm, das Ihre kubeconfig nennt; siehe [Exec-Plugins](/docs/clusters/exec-plugins/).

## Optional: Nomad-Cluster

Um [Nomad-Cluster](/docs/clusters/nomad/) zu verwalten, benötigt Homeport Netzwerkzugriff auf die HTTP-API eines Nomad-Agenten (standardmäßig Port 4646) und, wenn im Cluster ACLs aktiviert sind, ein Token; siehe [die ACL-Policy](/docs/clusters/nomad/#acl-policies), die es braucht. Es spricht die API von Nomad selbst, daher wird die `nomad`-CLI nicht benötigt, und auf dem Cluster wird nichts installiert.

- **Nomad-Versionen**: Nomad 1.6 oder neuer. **Bearbeiten und ausführen** geht von der eingereichten Spezifikation eines Jobs aus, die Nomad ab 1.6 aufbewahrt.

## Optional: ein Schlüsselbund des Betriebssystems

Homeport verschlüsselt SSH-Schlüssel, TLS-Zertifikate, die kubeconfig-Anmeldedaten von Clustern, Stack-Umgebungen, Registry-Anmeldungen sowie die Deploy-Keys und Tokens von Repositorys mit einem Schlüssel, den es im Schlüsselbund Ihres Betriebssystems aufbewahrt:

- **macOS** – Schlüsselbund
- **Windows** – Anmeldeinformationsverwaltung
- **Linux** – ein Secret-Service-Anbieter wie GNOME Keyring oder KWallet

Ohne einen solchen zeigt Homeport ein Banner an und funktioniert weiterhin mit lokalen Sockets, einfachem TCP und SSH über **ssh-agent**. TLS-Hosts, SSH-Hosts mit gespeicherten Schlüsseln, Cluster, Stack-Umgebungen, Registry-Anmeldungen und private Repositorys benötigen den Schlüsselbund; ohne ihn werden Images anonym gepullt, und nur öffentliche Repositorys können hinzugefügt werden. Siehe [Sicherheit und Ihre Daten](/docs/reference/security/).

## Optional: Docker Compose

Um [verwaltete, verknüpfte und synchronisierte Stacks](/docs/guides/stacks/) bereitzustellen, führt Homeport den Befehl `docker compose` auf **diesem** Rechner aus – nicht auf dem Host. Installieren Sie die Docker CLI mit dem Compose-Plugin (Docker Desktop enthält es) oder das eigenständige `docker-compose` v2.

Ohne sie erscheinen Stacks weiterhin – verknüpfte Stacks eingeschlossen – und können gestartet, gestoppt und entfernt werden; nur das Bereitstellen benötigt Compose. **Einstellungen → Dieser Rechner** zeigt, ob Homeport es gefunden hat.

Um einen Stack in einem [Swarm](/docs/guides/swarm/) bereitzustellen, führt Homeport stattdessen `docker stack deploy` auf diesem Rechner aus und benötigt daher den Befehl `docker` selbst (das Compose-Plugin wird nicht verwendet). Docker Desktop hat ihn; installieren Sie unter Linux das Docker-CLI-Paket Ihrer Distribution (`docker-ce-cli`, `docker.io` oder `docker`). Hier muss keine Engine laufen.

## Optional: git

[Synchronisierte Stacks](/docs/guides/synced-stacks/) verwenden das auf **diesem** Rechner installierte `git`, um ihre Repositorys zu klonen und abzurufen: Git for Windows, die Befehlszeilentools von Apple (`xcode-select --install`) oder das `git`-Paket Ihrer Distribution. Unter Windows verweist Homeport es für SSH-Repositorys auf das OpenSSH des Systems. Alle anderen Funktionen arbeiten ohne git; **Einstellungen → Dieser Rechner** zeigt, ob Homeport es gefunden hat.

## Optional: Helm

Um [Helm-Releases](/docs/clusters/helm/) auf einem Cluster zu verwalten, führt Homeport den Befehl `helm` ab Version 3 auf **diesem** Rechner aus. Es verweist helm über eine private Verbindung auf den Cluster, sodass keine Cluster-Anmeldedaten auf die Festplatte geschrieben werden, und helm verwendet seine eigenen Chart-Repositorys, wie in einem Terminal. Installieren Sie es mit `brew install helm` (macOS), `winget install Helm.Helm`, Chocolatey oder Scoop (Windows) oder mit `snap install helm --classic`, dem Paket Ihrer Distribution oder dem Skript von [helm.sh](https://helm.sh/docs/intro/install/) (Linux). Jede andere Cluster-Seite funktioniert ohne helm; **Einstellungen → Dieser Rechner** zeigt, ob Homeport es gefunden hat.

## Optional: ein Infobereich

Homeport legt ein Symbol in den Infobereich (die Menüleiste unter macOS, den Infobereich der Taskleiste unter Windows), das jeden Host mit seinem Status auflistet. Windows und macOS haben immer einen. Unter Linux benötigt er die Bibliothek **appindicator**: Das AppImage enthält sie, und die Pakete `.deb` und `.rpm` empfehlen sie, sodass `apt`, `dnf` und `zypper` sie installieren, sofern Sie empfohlene Pakete nicht deaktiviert haben. Andernfalls installieren Sie `libayatana-appindicator3-1` (Debian, Ubuntu, openSUSE) oder `libayatana-appindicator-gtk3` (Fedora). Manche Desktops blenden Symbole im Infobereich aus – GNOME zeigt sie nur mit einer Erweiterung wie AppIndicator Support. Ohne Infobereich arbeitet Homeport über sein Fenster, und das Schließen des Fensters beendet es.

## Was jede Funktion benötigt

| Funktion                                                  | Schlüsselbund | Compose-CLI | git | helm |
| --------------------------------------------------------- | :-----------: | :---------: | :-: | :--: |
| Stacks in einem Swarm bereitstellen                       |       —       | docker CLI  |  —  |  —   |
| Hosts mit lokalem Socket und TCP                          |       —       |      —      |  —  |  —   |
| SSH-Hosts mit ssh-agent                                   |       —       |      —      |  —  |  —   |
| SSH-Hosts mit einem Homeport- oder importierten Schlüssel |      Ja       |      —      |  —  |  —   |
| TLS-Hosts                                                 |      Ja       |      —      |  —  |  —   |
| Container, Images, Volumes, Netzwerke                     |       —       |      —      |  —  |  —   |
| Stacks starten, stoppen, entfernen                        |       —       |      —      |  —  |  —   |
| Verwaltete und verknüpfte Stacks bereitstellen            |       —       |     Ja      |  —  |  —   |
| Umgebung eines verwalteten oder synchronisierten Stacks   |      Ja       |      —      |  —  |  —   |
| Registry-Anmeldungen (private Images, Pushes)             |      Ja       |      —      |  —  |  —   |
| Synchronisierte Stacks aus einem öffentlichen Repository  |       —       |     Ja      | Ja  |  —   |
| Synchronisierte Stacks mit Deploy-Key oder Token          |      Ja       |     Ja      | Ja  |  —   |
| Ein Image aus einem Ordner bauen                          |       —       |      —      |  —  |  —   |
| Kubernetes-Cluster                                        |      Ja       |      —      |  —  |  —   |
| Nomad-Cluster                                             |      Ja       |      —      |  —  |  —   |
| Helm-Releases                                             |      Ja       |      —      |  —  |  Ja  |
