---
title: containerd
description: Mit containerd auf diesem Rechner oder über SSH verbinden, einen Namespace wählen, die Tasks seiner Container ausführen, ihre Logs lesen, eine Shell öffnen und Images und Snapshots verwalten.
---

**containerd** läuft unter k3s, nerdctl, BuildKit und vielen Servern, die kein Docker haben. Homeport verbindet sich direkt damit, über seine eigene gRPC-API, wie [ADR 0020](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0020-containerd-over-grpc.md) festlegt. Ein containerd-Host erhält eigene Seiten in der Seitenleiste: **Container**, **Images**, **Snapshots** und **Namespaces**. Die Seiten von Docker, Stacks und das Board werden dafür nicht angeboten.

## Einen containerd-Host hinzufügen

Wählen Sie **containerd** im Feld **Engine** des Formulars. containerd wird nur über seinen Socket erreicht, daher ist die Verbindung **Lokaler Socket** oder **SSH**; TCP und TLS sind ausgegraut, weil containerd seine API nicht über das Netzwerk anbietet.

| containerd                              | Socket                                                                 |
| --------------------------------------- | ---------------------------------------------------------------------- |
| der eigene Dienst von containerd        | `/run/containerd/containerd.sock`                                      |
| k3s                                     | `/run/k3s/containerd/containerd.sock`                                  |
| das eigene containerd von Docker Engine | `/run/containerd/containerd.sock`, seine Container im Namespace `moby` |

Unter Linux listet **Auf diesem Rechner gefunden** das containerd dieses Computers und das von k3s auf, wenn sie antworten. Ein Socket, den Sie noch nicht verwenden dürfen, wird stattdessen mit einer Anleitung zur Behebung aufgelistet.

### Ihrem Benutzer die Verwendung des Sockets erlauben

Der Socket von containerd gehört root. Um ihn als anderer Benutzer zu verwenden, lokal oder über SSH, geben Sie einer Gruppe in `/etc/containerd/config.toml` Zugriff darauf und fügen Sie den Benutzer dieser Gruppe hinzu:

```bash
sudo groupadd -f containerd
sudo usermod -aG containerd "$USER"
getent group containerd | cut -d: -f3
```

Tragen Sie die ausgegebene Zahl unter `[grpc]` in `/etc/containerd/config.toml` ein, starten Sie containerd neu und melden Sie sich erneut an:

```toml
[grpc]
  gid = 1001
```

```bash
sudo systemctl restart containerd
```

**Rootless containerd** (`containerd-rootless.sh`) hält seinen Socket im Namespace von RootlessKit, wo nichts außerhalb davon ihn erreichen kann, nicht einmal Homeport. Homeport weist darauf hin, wenn es ihn findet; verwenden Sie stattdessen den Systemdienst von containerd.

### Was jede Version und Verbindung erlaubt

Homeport verbindet sich mit containerd **1.7** und neuer. Das Pullen läuft über den Transfer-Service von containerd, der ab **2.0** stabil ist.

| Aktion                                             | containerd    | Verbindung                              |
| -------------------------------------------------- | ------------- | --------------------------------------- |
| Namespaces, Container und Tasks, Images, Snapshots | 1.7 und neuer | lokaler Socket oder SSH                 |
| Ein Image pullen                                   | 2.0 und neuer | lokaler Socket oder SSH                 |
| Logs und ein Terminal                              | 1.7 und neuer | SSH oder ein lokaler Socket unter Linux |

Ein containerd, das zu alt zum Verbinden ist, lässt den Host **Falsch konfiguriert** und nennt seine Version; eines, das zu alt zum Pullen ist, sagt es, wenn Sie pullen.

## Namespaces

Jedes containerd-Objekt lebt in einem **Namespace**: nerdctl verwendet `default`, Kubernetes `k8s.io`, Docker `moby`, BuildKit `buildkit`. Die Seiten zeigen jeweils einen Namespace, gewählt unter dem Host-Umschalter, der jeden mit der Anzahl seiner Container und Images auflistet. Homeport merkt sich die Wahl für jeden Host zwischen den Starts. Bis Sie einen wählen, zeigt ein Host `default` oder, wenn `default` leer ist, den einzigen Namespace, der etwas enthält, sodass ein k3s-Host auf `k8s.io` öffnet.

Die Seite **Namespaces** erstellt leere Namespaces und entfernt leere. containerd weigert sich, einen Namespace zu entfernen, der noch etwas enthält, und Homeport sagt, was er noch enthält: Entfernen Sie zuerst seine Container, Images und Snapshots.

## Container und Tasks

In containerd ist ein **Container** nur Metadaten: sein Image, seine Spezifikation, sein Snapshot. Was läuft, ist sein **Task**. Die Seite **Container** listet die Container des Namespace mit ihrem Task auf: **Läuft**, **Pausiert**, **Gestoppt** mit seinem Exit-Code oder **Kein Task**. Jeder nennt das Werkzeug, das ihn erstellt hat – **Kubernetes** (mit seinem Pod), **nerdctl**, **Docker** oder **ctr oder andere** –, und die Pause-Container von Pods sind ausgeblendet, sofern Sie nicht danach fragen.

| Aktion            | Was Homeport tut                                                                                                                                                                                                                                                                                                                          |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Starten**       | Erstellt den Task auf dem Snapshot des Containers, mit Logs dort, wo nerdctl sie ablegt, und startet ihn. Ein gestoppter Task wird ersetzt, wie es `ctr task start` erfordert.                                                                                                                                                            |
| **Stoppen**       | Sendet das Stoppsignal des Images (SIGTERM, sofern es kein anderes nennt), dann nach 10 Sekunden SIGKILL. Ein Task, der danach noch lebt, wird gemeldet.                                                                                                                                                                                  |
| **Pausieren**     | Friert den Task ein; **Fortsetzen** taut ihn auf.                                                                                                                                                                                                                                                                                         |
| **Signal senden** | Sendet ein beliebiges Signal, per Name (`SIGHUP`) oder Nummer, an den Hauptprozess des Tasks.                                                                                                                                                                                                                                             |
| **Entfernen**     | Fragt zuerst; beendet einen laufenden Task, entfernt den Task und den Container sowie seinen Snapshot mit den von ihm geänderten Dateien, es sei denn, Sie deaktivieren **Auch seinen Snapshot … entfernen**: Ein behaltener Snapshot wird mit `containerd.io/gc.root` markiert, sodass die Garbage Collection von containerd ihn behält. |

Die Seite eines Containers zeigt seine Runtime, seinen Snapshot, seine Labels und seine OCI-Spezifikation. Homeport erstellt keine Container auf containerd: containerd hat kein eigenes Networking, das fügen nerdctl und CNI hinzu.

Das **kubelet** verwaltet die Container von Kubernetes: Es startet sie nach einem Stopp wieder und erstellt sie nach einem Entfernen neu. Ihre Seiten weisen darauf hin; ändern Sie stattdessen den Pod auf seinem Cluster. Das Entfernen eines nerdctl-Containers von hier lässt die eigenen Aufzeichnungen von nerdctl darüber zurück (seinen Namen und sein Netzwerk), wie es `ctr` täte; `nerdctl rm` ist dafür der sauberere Weg.

## Logs

containerd bewahrt keine Logs auf: Das Werkzeug, das einen Task startet, entscheidet, wohin seine Ausgabe geht. Der Tab **Logs** liest sie dort, wo dieses Werkzeug sie geschrieben hat:

| Erstellt von             | Wohin seine Ausgabe geht                                                              | Was Homeport zeigt                                                    |
| ------------------------ | ------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| nerdctl                  | eine JSON-Logdatei im Statusordner von nerdctl, benannt nach seinen `nerdctl/`-Labels | diese Datei, fortlaufend verfolgt                                     |
| Kubernetes (`k8s.io`)    | `/var/log/pods/<pod>/<container>/N.log`, genannt im CRI-Eintrag dazu                  | diese Datei, fortlaufend verfolgt                                     |
| `ctr` und andere Clients | wohin auch immer sie sie geleitet haben, meist nirgendwohin                           | **Die Ausgabe dieses Containers wird nicht aufgezeichnet**, und warum |

nerdctl-Container, die an einen anderen Treiber loggen (journald, fluentd), nennen stattdessen diesen. Homeport liest die Datei mit `tail` auf dem Rechner des Hosts (siehe [unten](#was-auf-dem-host-ausgeführt-wird)); Logdateien gehören meist root, daher muss der SSH-Benutzer sie lesen dürfen.

## Ein Terminal in einem Container

Der Tab **Exec** öffnet eine Shell in einem laufenden Container – bash, wenn das Image sie hat, sonst sh – oder den von Ihnen angegebenen Befehl, als numerischer Benutzer (`uid` oder `uid:gid`) und in einem Ordner Ihrer Wahl. **Erneut öffnen** startet eine neue mit dem, was Sie eingegeben haben.

containerd nimmt Ein- und Ausgabe eines Prozesses als Pfade auf seinem Rechner entgegen. Daher führt Homeport dort ein kurzes Skript aus, das zwei FIFOs in einem privaten temporären Ordner anlegt, und kopiert das Terminal durch sie hindurch; containerd startet den Prozess auf diesen FIFOs, anhand der eigenen Prozessspezifikation des Containers. Das Schließen des Tabs beendet den Prozess und entfernt den Ordner.

## Images

Die Seite **Images** listet die Images des Namespace mit ihren Plattformen, den daraus erstellten Containern und ihrer Größe für die Plattform des Hosts auf, so wie `ctr images ls` sie zählt.

**Image pullen** ruft ein Image über containerd selbst ab und entpackt es dann für die Plattform des Hosts in seinen Standard-Snapshotter, sodass `nerdctl run` es sofort verwenden kann. Geben Sie eine vollständige Referenz (`ghcr.io/home/app:1.2`) oder eine kurze (`nginx`, die zu `docker.io/library/nginx:latest` wird) an. Das Image einer privaten Registry wird mit ihrer Anmeldung aus **Einstellungen → Registrys** gepullt, die nur dann an containerd gesendet wird, wenn die Registry danach fragt, und nie auf dem Host gespeichert wird.

Ein Image, aus dem ein Container des Namespace erstellt wurde, kann nicht entfernt werden; entfernen Sie zuerst die Container. containerd gibt den Inhalt eines Images frei, sobald nichts ihn mehr hält.

## Snapshots

In Snapshots steckt der Speicherplatz von containerd: die Schichten entpackter Images und die beschreibbare Schicht jedes Containers. Die Seite **Snapshots** zeigt die eines Snapshotters (zuerst `overlayfs` oder einen anderen, den der Host geladen hat) als Baum nach Elternteil, die größten zuerst, mit ihrer Belegung, wie `ctr snapshot usage` sie zählt, und der Summe oben.

Jeder nennt, was ihn verwendet: einen Container über seinen Snapshot oder ein Image über seine Schichten. Snapshots, die nichts verwendet und auf denen nichts aufbaut – etwa einer, der beim Entfernen seines Containers hier behalten wurde –, lassen sich mit **Nur unbenutzte** allein anzeigen und entfernen. Die anderen können von hier aus nicht entfernt werden. Ein Snapshot, auf den nichts verweist und den nichts als Root markiert, etwa einer, den `ctr containers rm --keep-snapshot` zurücklässt, bleibt nicht bestehen: Die Garbage Collection von containerd entfernt ihn bei ihrem nächsten Lauf.

## Was auf dem Host ausgeführt wird

Alles oben Genannte läuft über die API von containerd, außer Logs und Terminals, die die Dateien des containerd-Rechners brauchen. Dafür führt Homeport diese Befehle aus, über die SSH-Sitzung, die es bereits hält, oder bei einem lokalen Socket unter Linux auf diesem Computer, mit den Rechten des SSH-Benutzers und nie mit `sudo`:

| Für          | Befehl                                                                                                                |
| ------------ | --------------------------------------------------------------------------------------------------------------------- |
| Logs         | `tail -n <lines> [-F] -- '<log file>'`; für einen Kubernetes-Container ohne CRI-Eintrag zuerst `ls` seines Logordners |
| Ein Terminal | `mktemp -d`, `mkfifo -m 600` für zwei FIFOs in diesem Ordner, `cat` durch sie hindurch und `rm -rf` des Ordners       |

Ein lokaler Socket unter macOS oder Windows kann sie nicht ausführen, und Logs und Terminals sagen dann, dass sie SSH brauchen. Die [Sicherheitsseite](/docs/reference/security/#was-homeport-auf-ihren-hosts-ausführt) listet jeden Befehl auf, den Homeport auf einem Host ausführt.

## Ereignisse

Die Seiten aktualisieren sich anhand der eigenen Ereignisse von containerd, in jedem Namespace: ein Task, der startet oder endet, ein Container oder Image, das erstellt oder entfernt wird, ein Snapshot, der festgeschrieben wird. containerd bewahrt keinen Verlauf davon auf, daher wird nicht nachgetragen, was geschah, während Homeport geschlossen war.
