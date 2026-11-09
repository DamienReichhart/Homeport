---
title: Logs und Terminals
description: Den Logs eines Pods oder eines ganzen Workloads folgen, sehen, warum ein Container abgestürzt ist, und ein Terminal im Container eines Pods öffnen.
---

Die Seite eines Pods hat die Tabs **Logs** und **Terminal**, und die Seite eines Workloads einen Tab **Logs** für all seine Pods: das alltägliche `kubectl logs` und `kubectl exec -it`, über die Kubernetes-API mit den eigenen Zugangsdaten des Clusters.

## Logs

**Logs** streamt die Logs eines Containers, während sie geschrieben werden. Es ist derselbe Viewer wie für einen Container auf einem Host:

- **Verlauf**: mit den letzten 100, 1.000 oder 10.000 Zeilen beginnen, oder mit allen.
- **Seit**: nur Zeilen der letzten Minuten, Stunden oder Tage.
- **Zeitstempel**: die Zeit, zu der Kubernetes jede Zeile erfasst hat.
- **Mitverfolgen**: die neueste Zeile im Blick behalten; schalten Sie es aus, um zurückzulesen, während weiter neue Zeilen eintreffen.
- **Suchen**, **Leeren** und **Herunterladen** (als Textdatei, die bisher angezeigten Zeilen).

Ein Pod mit mehreren Containern hat eine Auswahl **Container**, Init-Container markiert. Sie öffnet mit dem Container, den `kubectl` wählen würde: dem, den die Annotation `kubectl.kubernetes.io/default-container` des Pods nennt, sonst seinem ersten App-Container.

Kubernetes hält Standardausgabe und Standardfehler eines Containers zusammen, daher gibt es keinen stdout/stderr-Filter wie für die Container eines Hosts.

Der Stream stoppt, wenn Sie den Tab verlassen, und sagt es, wenn der Container stoppt.

### Warum ist er abgestürzt?

Wenn ein Container neu gestartet wurde, zeigt **Vorherige Instanz** die Logs des Laufs vor dem letzten Neustart – bei einem Pod in `CrashLoopBackOff` den Lauf, der abgestürzt ist –, wie `kubectl logs --previous`. Bei einem Container, der nie neu gestartet wurde, ist es ausgeschaltet, da es nichts zu zeigen gibt.

Wenn ein Container noch nicht gestartet ist (`ContainerCreating`, `CrashLoopBackOff`, während er auf den nächsten Start wartet), sagen die Logs das an Ort und Stelle mit dem Grund. Die Seite seines Pods sagt, warum er wartet.

### Die Logs eines Workloads

Der Tab **Logs** eines Deployments, StatefulSets, DaemonSets oder Jobs verschränkt die Logs all seiner Pods, während sie geschrieben werden, jede Zeile mit ihrem Pod in einer eigenen Farbe beschriftet, wie `kubectl logs -l app=api --prefix`. Wählen Sie einen Pod, um ihn allein zu lesen, und den Container, wenn sein Template mehrere hat.

## Terminal

**Terminal** öffnet eine Shell in einem der laufenden Container des Pods: `bash`, wenn das Image es hat, sonst `sh`. Tippen Sie stattdessen einen Befehl in das Feld (`psql -U app shop`, `top`), und **Verbinden** führt ihn aus, zitiert, wie eine Shell es täte.

Es ist ein echtes TTY: `vi`, `top`, Farben und Pfeiltasten funktionieren, und eine Größenänderung des Fensters ändert die Größe des Terminals im Container. Wenn der Prozess endet, sagt das Terminal, mit welchem Code; **Neu verbinden** startet einen weiteren. Das Verlassen des Tabs beendet den Prozess im Container.

### Wenn es sich nicht öffnen lässt

- **Keine Shell**: Minimale (distroless) Images haben überhaupt keine Shell. Tippen Sie einen Befehl, den das Image hat, oder hängen Sie aus einem Terminal mit `kubectl debug -it <pod> --image=busybox --target=<container>` einen Debug-Container an; Homeport erstellt keine Debug-Container.
- **Läuft nicht**: Ein Container, der wartet oder fertig ist, kann nichts ausführen. Die Pods eines abgeschlossenen Jobs sind endgültig gestoppt.
- **Nicht erlaubt**: Ein Terminal zu öffnen braucht `create` auf `pods/exec`, Logs zu lesen `get` auf `pods/log`, was Rollen mit reinem Lesezugriff oft fehlt. Homeport sagt, welches, mit dem Befehl `kubectl auth can-i` zur Prüfung.

Homeport kopiert keine Dateien in einen Pod oder aus ihm heraus (`kubectl cp`).
