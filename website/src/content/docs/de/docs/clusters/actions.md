---
title: Skalieren, neu starten und löschen
description: Deployments und StatefulSets skalieren, einen Rollout neu starten, CronJobs aussetzen und ausführen und jedes Objekt eines Clusters löschen, jeweils nach einer Bestätigung, die es nennt.
---

Die alltäglichen Änderungen an einem Cluster sind auf seinen Seiten einen Klick entfernt, jede wird mit den eigenen Zugangsdaten des Clusters an die Kubernetes-API gesendet, wie `kubectl` es tun würde. Die Listen folgen der Änderung live über ihren Watch.

Jede Aktion, die etwas entfernt oder ersetzt, fragt vorher nach, und die Bestätigung nennt den Cluster, den Namespace, die Art und den Namen, sodass eine Änderung nie versehentlich im falschen Cluster oder Namespace landet.

## Workloads

Die Seite eines Workloads hat ihre Aktionen in der Kopfzeile; unter **Workloads** hat jede Zeile sie in ihrem Menü **⋯**.

| Aktion              | Für                                   | Bewirkt, wie                                                     |
| ------------------- | ------------------------------------- | ---------------------------------------------------------------- |
| **Skalieren**       | Deployments, StatefulSets             | `kubectl scale --replicas=N`                                     |
| **Neu starten**     | Deployments, StatefulSets, DaemonSets | `kubectl rollout restart`                                        |
| **Aussetzen**       | CronJobs                              | `kubectl patch cronjob … -p '{"spec":{"suspend":true}}'`         |
| **Fortsetzen**      | Ausgesetzte CronJobs                  | dasselbe, mit `false`                                            |
| **Jetzt ausführen** | CronJobs                              | `kubectl create job --from=cronjob/…`                            |
| **Löschen**         | Alle                                  | `kubectl delete` (seine Pods und Jobs gehen mit, im Hintergrund) |

### Skalieren

**Skalieren** fragt, wie viele Pods laufen sollen, und zeigt, wie viele jetzt laufen. Skalieren auf `0` stoppt den Workload, bis Sie ihn wieder hochskalieren. Wenn ein HorizontalPodAutoscaler den Workload verwaltet, sagt der Dialog das: Der Autoscaler setzt die Anzahl bei seiner nächsten Prüfung zurück, ändern Sie also stattdessen Minimum und Maximum des Autoscalers.

### Neu starten

**Neu starten** fügt dem Pod-Template die Annotation `kubectl.kubernetes.io/restartedAt` hinzu, genau wie `kubectl rollout restart`: Der Workload rollt nach seiner Strategie einen neuen Satz Pods aus (ein neues ReplicaSet bei einem Deployment), und die alten Pods gehen, sobald die neuen bereit sind.

### CronJobs

**Aussetzen** hindert einen CronJob daran, neue Jobs zu starten (bereits laufende laufen weiter), und **Fortsetzen** lässt ihn wieder welche starten; ein ausgesetzter CronJob wird als **Pausiert** angezeigt. **Jetzt ausführen** startet sofort einen Job aus seinem Template, benannt `<cronjob>-manual-<id>`, im Besitz des CronJobs und wie bei kubectl mit `cronjob.kubernetes.io/instantiate: manual` markiert. Er erscheint im Tab **Jobs** des CronJobs.

## Löschen

Jede Seite eines Objekts – eines Workloads, eines Pods, eines Services, eines Ingress, einer ConfigMap, eines Secrets, eines Nodes oder jedes Objekts unter **API-Ressourcen** – hat **Löschen** in ihrer Kopfzeile. In einer Liste haken Sie Zeilen an, und **N löschen** entfernt sie alle nach einer einzigen Bestätigung, die sie auflistet.

Objekte werden mit der Propagation `Background` gelöscht, wie bei `kubectl delete`: Das Objekt geht sofort, und der Garbage Collector entfernt, was ihm gehört (die ReplicaSets und Pods eines Deployments, die Jobs eines CronJobs).

Die Bestätigung sagt, was als Nächstes passiert:

- **Ein Pod, der einem Controller gehört** – einem ReplicaSet, StatefulSet, DaemonSet oder Job –, kommt zurück: Der Controller startet einen neuen an seiner Stelle. Löschen oder skalieren Sie den Workload, um ihn loszuwerden.
- **Ein Namespace** nimmt alles darin mit: seine Workloads, Pods, Services, ConfigMaps und Secrets.
- **Ein Node** wird aus der Liste des Clusters entfernt, ohne vorher geleert zu werden.

## Wenn es nicht funktioniert

- **Nicht erlaubt**: Eine Änderung, die Ihre Zugangsdaten nicht vornehmen dürfen, sagt das mit ihrem Verb und ihrer Ressource – „Ihre Zugangsdaten erlauben ‚patch‘ für deployments in shop nicht.“ – und nennt den Befehl `kubectl auth can-i` zur Prüfung.
- **Zwischenzeitlich geändert**: Wenn jemand anderes das Objekt zwischen Ihrem Blick und Ihrer Aktion geändert hat, sagt Homeport das; sehen Sie es sich erneut an und versuchen Sie es noch einmal.
- **Bereits weg**: Etwas zu löschen, das bereits gelöscht ist, meldet, dass es nicht mehr existiert.
