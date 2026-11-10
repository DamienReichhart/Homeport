---
title: Hosts und Verbindungen
description: Wie sich Homeport mit Docker Engines, containerd und Incus verbindet, was jeder Host-Status bedeutet und wie die Neuverbindung funktioniert.
---

Ein **Host** ist eine Docker Engine, ein containerd oder ein Incus, mit dem sich Homeport verbindet. Er hat einen Namen und einen Weg, ihn zu erreichen. Jeder Container, jede Instanz, jedes Image, jedes Volume, jedes Netzwerk und jeder Stack, den Sie sehen, gehört zu genau einem Host.

## Verbindungsarten

| Typ                | Erreicht eine Engine…                                                  | Anmeldedaten                                                              | Anleitung                                                        |
| ------------------ | ---------------------------------------------------------------------- | ------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| **Lokaler Socket** | auf diesem Rechner, über einen Socket oder eine Pipe                   | keine                                                                     | [Lokale Engines](/docs/hosts/local/)                             |
| **SSH**            | auf einem Server, an dem Sie sich anmelden können                      | ein Homeport-Schlüssel, Ihr eigener Schlüssel oder ssh-agent              | [SSH](/docs/hosts/ssh/)                                          |
| **TLS**            | freigegeben mit `dockerd --tlsverify`, oder das HTTPS von Incus        | CA-Zertifikat, Client-Zertifikat und Schlüssel; für Incus ein Trust-Token | [TLS und TCP](/docs/hosts/tls-tcp/), [Incus](/docs/hosts/incus/) |
| **TCP**            | ohne Verschlüsselung freigegeben, in einem vertrauenswürdigen Netzwerk | keine                                                                     | [TLS und TCP](/docs/hosts/tls-tcp/)                              |

Bevorzugen Sie für einen entfernten Server **SSH**: Es erfordert keine Freigabe über den SSH-Port hinaus und ist am einfachsten sicher einzurichten.

## Engine-Arten

Neben dem Weg, ihn zu erreichen, gibt jeder Host im Feld **Engine** des Formulars an, auf welche Art von Engine er verweist. Sie bestimmt, welche Seiten die Seitenleiste für diesen Host zeigt.

| Engine                | Was sie umfasst                                                                        | Seiten                                                                                                  |
| --------------------- | -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| **Docker-kompatibel** | Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop, Podman               | Container, Images, Volumes, Netzwerke, Stacks, Ereignisse; und bei Podman auch seine eigenen Seiten     |
| **containerd**        | der eigene Dienst von containerd, der von k3s, das eigene containerd von Docker Engine | Container, Images, Snapshots und Namespaces von containerd; siehe [containerd](/docs/hosts/containerd/) |
| **Incus**             | Incus 6.0 und neuer: Systemcontainer und virtuelle Maschinen                           | Instanzen, Images, Profile, Speicher und Netzwerke; siehe [Incus](/docs/hosts/incus/)                   |

Das Formular wählt die Engine anhand des Socket-Pfads, wenn dieser den Socket von containerd oder Incus nennt. containerd wird nur über seinen Socket erreicht, auf diesem Computer oder über SSH, daher sind TCP und TLS dafür ausgegraut; Incus über seinen Socket oder über HTTPS (TLS), daher ist nur TCP ausgegraut. Hosts, die vor Homeport 0.18 hinzugefügt wurden, sind Docker-kompatibel, ohne dass sich sonst etwas ändert.

**Podman** ist eine Docker-kompatible Engine, die Homeport bei jeder Verbindung erkennt, weil derselbe Socket das Produkt wechseln kann: Der Status des Hosts zeigt dann **Podman** und dessen Version statt der der Engine, und die Seitenleiste fügt die Seiten hinzu, die nur Podman hat. Eine Seite, die der ausgewählte Host nicht anzeigen kann, steht nicht in der Seitenleiste, und wenn Sie sie über einen Link öffnen, wird erklärt, warum.

Ein Host, der zu einem Docker Swarm gehört, zeigt **Manager von** oder **Worker von** seinem Swarm, und es genügt, einen Manager als Host hinzuzufügen, damit der Swarm in der Seitenleiste erscheint. Die Seite eines Hosts hat außerdem eine Karte **Swarm**, um einen Swarm zu starten, einem beizutreten oder ihn zu verlassen. Siehe [Swarms](/docs/guides/swarm/).

## Hosts hinzufügen, bearbeiten und entfernen

- **Fügen** Sie einen Host mit **Host hinzufügen…** im Host-Umschalter oben in der Seitenleiste **hinzu**, oder mit **Host hinzufügen** in der Übersicht.
- **Hosts verwalten** im Host-Umschalter listet jeden Host mit Typ, Adresse und Status auf. Wählen Sie einen aus, um seine Seite zu öffnen.
- Auf der Seite eines Hosts können Sie seine Verbindungsdetails ändern und **Speichern** wählen; Homeport verbindet sich mit den neuen Einstellungen neu. Der Verbindungstyp eines Hosts lässt sich nicht ändern – entfernen Sie ihn stattdessen und fügen Sie ihn erneut hinzu.
- Der Schalter **Aktiviert** trennt einen Host, ohne ihn zu vergessen. Ein deaktivierter Host versucht es nicht erneut, bis Sie ihn wieder einschalten.
- **Host entfernen** vergisst den Host, seine gespeicherten Anmeldedaten, die Compose-Definitionen seiner verwalteten Stacks, seine verknüpften Stacks und den Bereitstellungsverlauf seiner Stacks. Nichts, was auf dem Host läuft, wird angetastet: Container, Images und Volumes bleiben, wie sie sind, ebenso die Ordner verknüpfter Stacks. Die Stacks eines Swarms, den der Host verwaltet, gehören zum Swarm und bleiben erhalten.

Host-Namen müssen eindeutig sein, ohne Berücksichtigung der Groß- und Kleinschreibung.

## Hosts importieren

**Host hinzufügen** bietet zwei Wege, viele Hosts auf einmal hinzuzufügen, aus dem, was dieser Computer bereits beschreibt. Jeder öffnet eine Vorschau: Wählen Sie die zu importierenden Einträge aus, benennen Sie sie um und wählen Sie **Importieren**.

- **Aus ~/.ssh/config importieren** fügt die Server hinzu, die Ihre SSH-Konfiguration nennt, mit ihrem Benutzer, Port, Schlüssel und ihren `ProxyJump`-Bastionen. Siehe [Aus ~/.ssh/config importieren](/docs/hosts/ssh/#aus-sshconfig-importieren).
- **Docker-Kontexte importieren** fügt die Engines der Kontexte der Docker CLI hinzu (`docker context ls`), mit ihren TLS-Zertifikaten.

Schlüssel und Zertifikate werden einmal gelesen, beim Import, und wie alle anderen verschlüsselt gespeichert; weder `~/.ssh` noch `~/.docker` wird je verändert.

### Docker-Kontexte

Homeport liest jeden Kontext unter `~/.docker/contexts` (oder unter `$DOCKER_CONFIG`, wenn gesetzt) und ordnet seinen Docker-Endpunkt zu:

| Endpunkt                                                 | Wird zu                                                                 |
| -------------------------------------------------------- | ----------------------------------------------------------------------- |
| `unix:///…`, `npipe:////./pipe/…`                        | einem Host mit **Lokaler Socket**                                       |
| `tcp://host:port` ohne TLS-Dateien                       | einem **TCP**-Host                                                      |
| `tcp://host:port` mit `ca.pem`, `cert.pem` und `key.pem` | einem **TLS**-Host, mit seinen Zertifikaten                             |
| `ssh://user@host:port`                                   | einem **SSH**-Host, der ssh-agent verwendet, mit `/var/run/docker.sock` |

Der integrierte Kontext `default` wird nicht angeboten: Die [lokale Erkennung](/docs/hosts/local/) findet die Engine dieses Rechners bereits. Ein Kontext wird angezeigt, kann aber nicht ausgewählt werden, wenn:

- er `SkipTLSVerify` setzt, da Homeport das Zertifikat der Engine immer prüft;
- sein TLS-Ordner nur einige der drei Dateien enthält;
- sein Endpunkt etwas anderes ist, etwa ein Kubernetes-Endpunkt;
- ein Host bereits dieselbe Engine erreicht. Sein Name sagt, welcher.

Ein Kontext, dessen Name bereits der eines Hosts ist, wird als `name (2)` angeboten.

## Auf einen anderen Computer umziehen

**Exportieren** auf der Seite „Hosts“ speichert die von Ihnen gewählten Hosts in einer durch eine Passphrase geschützten Datei; **Importieren** auf der Seite „Hosts“ des anderen Computers öffnet sie und fügt die von Ihnen gewählten Hosts hinzu, bereit zum Verbinden. Ein Host, dessen Name dort vergeben ist, wird als `name (2)` vorgeschlagen, und Sie können ihn umbenennen oder überspringen. Siehe [Auf einen anderen Computer umziehen](/docs/reference/security/#auf-einen-anderen-computer-umziehen), wie die Datei geschützt ist. Um alles auf einmal umzuziehen, Cluster, Stacks, Registry-Anmeldungen, Repositorys und Einstellungen eingeschlossen, verwenden Sie stattdessen eine [Sicherung](/docs/reference/data-and-backups/#backing-up-in-homeport).

## Einen Host auswählen

Jede Ressourcenseite – Container, Images, Volumes, Netzwerke, Stacks und Ereignisse – zeigt den im **Host-Umschalter** ausgewählten Host. Die Seiten eines containerd-Hosts zeigen außerdem jeweils einen Namespace, gewählt [unter dem Umschalter](/docs/hosts/containerd/#namespaces), und die eines Incus-Hosts ein [Projekt](/docs/hosts/incus/#projekte). Wechseln Sie den Host dort, oder wählen Sie die Karte eines Hosts in der [Übersicht](/docs/guides/overview/). Homeport merkt sich Ihre Wahl zwischen den Starts.

## Status

Jeder Host zeigt neben seinem Namen einen Status. Ist der ausgewählte Host nicht verbunden, zeigen die Ressourcenseiten seinen Status mit **Jetzt erneut versuchen** und **Host-Einstellungen öffnen**.

| Status                                     | Bedeutung                                                                                                           | Wie es weitergeht                                                                                        |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| **Verbunden**                              | Die Engine hat geantwortet. Ihre Version wird angezeigt, oder die von Podman, containerd oder Incus.                | Homeport überwacht die Verbindung weiter.                                                                |
| **Wird verbunden**                         | Homeport erreicht gerade die Engine.                                                                                | —                                                                                                        |
| **Deaktiviert**                            | Der Schalter **Aktiviert** des Hosts ist aus.                                                                       | Nichts, bis Sie ihn einschalten.                                                                         |
| **Nicht erreichbar**                       | Netzwerk, Socket oder Engine haben nicht geantwortet.                                                               | Versucht es selbst erneut und wartet jedes Mal länger.                                                   |
| **Zugriff verweigert**                     | Der SSH-Server hat den Schlüssel abgelehnt, ssh-agent ist fehlgeschlagen oder der TLS-Handshake ist fehlgeschlagen. | Beheben Sie die Ursache und wählen Sie dann **Neu verbinden**.                                           |
| **Serverschlüssel nicht vertrauenswürdig** | Erste SSH-Verbindung: Der Schlüssel des Servers oder einer Bastion braucht Ihre Zustimmung.                         | [Dem Schlüssel vertrauen](/docs/hosts/ssh/#dem-schlüssel-des-servers-vertrauen) auf der Seite des Hosts. |
| **Serverschlüssel geändert**               | Der SSH-Server hat einen anderen Schlüssel vorgelegt als den, dem Sie vertraut haben.                               | [Zuerst untersuchen](/docs/hosts/ssh/#wenn-sich-der-schlüssel-des-servers-ändert).                       |
| **Zugangsdaten verloren**                  | Die gespeicherten Anmeldedaten lassen sich nicht entschlüsseln.                                                     | [Erneut eingeben](/docs/reference/troubleshooting/#zugangsdaten-verloren).                               |
| **Schlüsselbund nicht verfügbar**          | Der Host braucht gespeicherte Geheimnisse, aber es gibt keinen Schlüsselbund des Betriebssystems.                   | Siehe [Voraussetzungen](/docs/requirements/#optional-ein-schlüsselbund-des-betriebssystems).             |
| **Falsch konfiguriert**                    | In den Einstellungen des Hosts fehlt etwas oder ist ungültig.                                                       | Die Meldung sagt, was; beheben Sie es und wählen Sie **Speichern**.                                      |

## Neuverbindung

Homeport verbindet sich beim Start mit jedem aktivierten Host, und jeder Host hat seine eigene Verbindung, sodass ein langsamer Server nie einen anderen aufhält.

Solange die Verbindung besteht, hört Homeport den Ereignisstrom der Engine ab – so aktualisieren sich die Seiten von selbst – und prüft die Engine alle 15 Sekunden. Bricht die Verbindung ab, wird der Host **Nicht erreichbar**, und Homeport versucht es nach 1 Sekunde erneut, dann nach 2, 4, 8 und so weiter, bis zu einmal pro Minute. Eine erfolgreiche Verbindung setzt die Wartezeit zurück.

Nur **Nicht erreichbar** wird automatisch erneut versucht. Jedes andere Problem braucht etwas von Ihnen, und dann versucht es **Neu verbinden** (auf der Seite des Hosts) oder **Jetzt erneut versuchen** sofort erneut.

## Ereignisse und Live-Aktualisierungen

Homeport fragt nicht regelmäßig ab. Wenn die Engine eine Änderung meldet – ein Container startet, ein Image wird gepullt –, aktualisieren sich die betroffenen Seiten innerhalb eines Sekundenbruchteils. Die Seite [Ereignisse](/docs/guides/overview/#ereignisse) zeigt den rohen Strom.
