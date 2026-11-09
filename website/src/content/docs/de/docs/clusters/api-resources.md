---
title: API-Ressourcen
description: Jede Objektart durchsuchen, die ein Cluster bereitstellt, benutzerdefinierte Ressourcen eingeschlossen, mit den Spalten, die kubectl get zeigt.
---

Homeport hat eigene Seiten für Workloads, Pods, Services, Ingresses, ConfigMaps, Secrets, Nodes und Events. **API-Ressourcen** in der Seitenleiste eines Clusters deckt alles andere ab: PersistentVolumes und ihre Claims, NetworkPolicies, ServiceAccounts und RBAC sowie die Ressourcen, die Operatoren mit ihren CRDs hinzufügen – die Certificates von cert-manager, die Applications von Argo CD, die ServiceMonitors von Prometheus.

## Eine Art finden

Die linke Spalte listet jede Art auf, die der Cluster bereitstellt, gruppiert nach API-Gruppe (`core` zuerst), jeweils in der bevorzugten Version der Gruppe, wie bei `kubectl api-resources`. Suchen Sie nach Kind, Plural oder Gruppe, oder tippen Sie einen Kurznamen: `pvc`, `netpol`, `sa`, `cert`.

Ein Schloss markiert die Arten, die Sie nicht auflisten können: die, die Ihre Zugangsdaten im gewählten Namespace nicht auflisten dürfen (wie der Cluster Homeport über ein `SelfSubjectRulesReview` mitteilt), und die wenigen, deren API gar nicht auflisten kann. Wenn Sie eine öffnen, steht dort, warum. Manche Cluster, etwa solche, deren Autorisierer ein Webhook ist, können das nicht sagen; dann ist nichts markiert, und eine abgelehnte Liste sagt das.

Das Installieren oder Entfernen einer CRD aktualisiert die Liste von selbst; **Aktualisieren** liest sie erneut.

## Objekte auflisten

Wählen Sie eine Art, und ihre Objekte werden mit denselben Spalten wie bei `kubectl get` aufgelistet: Der API-Server sagt, welche, einschließlich der `additionalPrinterColumns` einer CRD, sodass die Certificates von cert-manager **Ready**, **Secret** und **Age** zeigen wie in einem Terminal. Die Liste folgt Änderungen live über einen Watch.

Arten mit Namespace haben die Namespace-Auswahl wie die anderen Seiten von Homeport; mit **Alle Namespaces** kommt eine Spalte **Namespace** hinzu.

## Die Seite eines Objekts

Klicken Sie auf ein Objekt, um seine Seite zu öffnen:

- **Übersicht**: sein Namespace, die Objekte, denen es gehört – mit Link zu ihrer Seite –, sein Alter, seine Labels und Annotationen.
- **Bedingungen**, wenn sein `status` welche hat, wie auf den eigenen Seiten von Homeport.
- **Events** dazu und sein **YAML**, ohne `managedFields`.

Wenn seine Art es erlaubt, öffnet **Bearbeiten** sein YAML, um es über eine Vorschau und Server-side Apply zu ändern (siehe [YAML anwenden und bearbeiten](/docs/clusters/yaml/)), und **Löschen** entfernt es nach einer Bestätigung, die es und den Cluster nennt (siehe [Löschen](/docs/clusters/actions/#deleting)). In der Liste haken Sie mehrere Objekte an, um sie zusammen zu löschen.
