---
title: CPU und Arbeitsspeicher
description: Sehen Sie, was Pods, ihre Container und Nodes an CPU und Arbeitsspeicher nutzen, gemessen an ihren Anforderungen, Limits und dem Zuweisbaren, von metrics-server.
---

Wenn auf einem Cluster [metrics-server](https://github.com/kubernetes-sigs/metrics-server) läuft, zeigt Homeport, was jeder Pod und jeder Node an CPU und Arbeitsspeicher nutzt: in den Listen und in Live-Diagrammen auf ihren Seiten. Es liest die Metrics API (`metrics.k8s.io`), die metrics-server bereitstellt, wie `kubectl top`.

## Was Sie brauchen

metrics-server muss bereits auf dem Cluster laufen: Homeport installiert nichts auf einem Cluster. Ob er da ist, hängt davon ab, wie der Cluster erstellt wurde:

| Cluster                   | metrics-server                                                                                |
| ------------------------- | --------------------------------------------------------------------------------------------- |
| GKE                       | Installiert                                                                                   |
| AKS                       | Installiert                                                                                   |
| k3s und k3d               | Installiert (außer k3s wurde mit `--disable metrics-server` gestartet)                        |
| Docker Desktop            | Mit seinem Kubernetes installiert                                                             |
| kind                      | Nicht installiert                                                                             |
| EKS                       | Nicht installiert: Fügen Sie das Add-on metrics-server hinzu oder installieren Sie ihn selbst |
| minikube                  | Nicht installiert: `minikube addons enable metrics-server`                                    |
| Andere Cluster (kubeadm…) | Meist nicht installiert                                                                       |

Um ihn selbst zu installieren, folgen Sie [seiner Anleitung](https://github.com/kubernetes-sigs/metrics-server#installation); auf einem lokalen Cluster, dessen Kubelets selbstsignierte Zertifikate haben (kind), braucht metrics-server außerdem `--kubelet-insecure-tls`.

Ohne metrics-server geht nichts kaputt: Die Spalten für CPU und Arbeitsspeicher werden nicht angezeigt, und der Tab **Metriken** eines Pods oder Nodes sagt, dass metrics-server nicht installiert ist, mit einem Link dorthin. Direkt nach der Installation braucht metrics-server etwa eine Minute, bis er Messwerte hat; bis dahin sagt der Tab, dass die Metrics API noch nicht antwortet.

Ihre Anmeldedaten müssen Metriken auch lesen dürfen: `list` und `get` auf `pods.metrics.k8s.io` im Namespace (oder in jedem Namespace) und auf `nodes.metrics.k8s.io` für Nodes. Dürfen sie es nicht, sagt der Tab **Metriken**, welche Berechtigung fehlt, und alle anderen Seiten funktionieren weiter.

## In den Listen

Auf **Pods** zeigen die Spalten **CPU** und **Arbeitsspeicher**, was jeder Pod nutzt, seine Container zusammengezählt, und welchen Anteil das an seinem Limit ausmacht, oder an seiner Anforderung, wenn er kein Limit hat. Ein Pod hat nur dann ein Limit, wenn alle seine Container eines haben: Ein Container ohne Limit kann den ganzen Node nutzen. CPU wird in Millicores angegeben, wie `kubectl top` sie schreibt (`250m` ist ein Viertel eines Kerns), und Arbeitsspeicher ist das Working Set, nach dem das Kubelet Pods verdrängt.

Auf **Nodes** zeigen **CPU belegt** und **Arbeitsspeicher belegt**, was jeder Node nutzt, und seinen Anteil an dem, was Pods **zuweisbar** ist.

Klicken Sie auf den Kopf einer Spalte, um danach zu sortieren: die am stärksten ausgelasteten Pods oder Nodes zuerst.

## Auf der Seite eines Pods oder Nodes

Der Tab **Metriken** eines Pods zeichnet CPU und Arbeitsspeicher jedes Containers auf, mit einer gestrichelten Linie bei seiner Anforderung und einer weiteren bei seinem Limit, sofern gesetzt. Der eines Nodes zeichnet seine CPU und seinen Arbeitsspeicher mit einer Linie beim Zuweisbaren auf.

metrics-server liest die Kubelets alle 15 Sekunden, und Homeport fragt im selben Takt nach einem neuen Messwert, sodass eine Laständerung innerhalb von 30 Sekunden im Diagramm erscheint. Die Messwerte bleiben erhalten, solange die Seite offen ist, höchstens 15 Minuten lang, unabhängig vom angezeigten Tab, und nur im Arbeitsspeicher: Das Schließen der Seite beendet das Lesen und verwirft sie. Homeport speichert keinen Verlauf von Metriken; dafür nutzen Sie ein Monitoring-System wie Prometheus.
