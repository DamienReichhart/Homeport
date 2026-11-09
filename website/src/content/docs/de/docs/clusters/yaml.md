---
title: YAML anwenden und bearbeiten
description: Manifeste auf einen Cluster anwenden oder das YAML eines Objekts bearbeiten und sehen, was sich ändert, bevor sich etwas ändert, mit Server-side Apply.
---

Zwei Wege, einen Cluster mit YAML zu ändern, für alles, was die eigenen Aktionen nicht abdecken: **YAML anwenden** für Manifeste, wie `kubectl apply -f`, und **Bearbeiten** im Tab **YAML** eines beliebigen Objekts, wie `kubectl edit`. Beide zeigen, was sich ändern würde, bevor sich etwas ändert.

## YAML anwenden

**YAML anwenden** in der Seitenleiste eines Clusters nimmt ein oder mehrere Manifeste entgegen: Fügen Sie sie ein oder laden Sie mit **Datei öffnen…** eine `.yaml`-, `.yml`- oder `.json`-Datei von diesem Rechner. Mehrere durch `---` getrennte Dokumente werden zusammen angewendet, der Reihe nach.

Ein Dokument ohne Namespace kommt in den oben rechts gewählten Namespace (oder, mit **Alle Namespaces**, in den Standard-Namespace des Clusters). Objekte, die zu keinem Namespace gehören, etwa ein Namespace oder eine ClusterRole, ignorieren ihn.

1. **Änderungen anzeigen** fragt den Cluster mit einem serverseitigen Probelauf, was jedes Dokument bewirken würde: Noch ändert sich nichts. Jedes Dokument wird als **Erstellt**, **Geändert** oder **Keine Änderungen** angezeigt, mit den Zeilen, die es ändern würde.
2. **Anwenden** bittet Sie um Bestätigung, nennt dabei den Cluster und wendet dann an.

Wenn Sie dieselben Manifeste zweimal anwenden, wird beim zweiten Mal **Keine Änderungen** gemeldet. Wenn Sie das YAML nach einer Vorschau bearbeiten, wird zuerst eine neue verlangt.

## Ein Objekt bearbeiten

Der Tab **YAML** jedes Objekts hat **Bearbeiten**: sein YAML, wie der Cluster es hält, ohne das, was der Server verwaltet (`managedFields`, `status`, seine `resourceVersion`, UID und Erstellungszeit). Ändern Sie es, sehen Sie mit **Änderungen anzeigen** nur die Zeilen, die Sie geändert haben, und wählen Sie dann **Anwenden**.

Die Vorschau vergleicht Ihr YAML mit dem Objekt, wie es zum Zeitpunkt der Vorschau ist, sodass eine zwischenzeitliche Änderung im Diff erscheint. Wenn die Bearbeitung ein Feld ändert, das ein anderes Werkzeug verwaltet, führt die Vorschau es als Konflikt auf, statt es zu überschreiben.

Secrets können in Homeport nicht bearbeitet werden, da es nie alle ihre Werte auf einmal zeigt; und Events schreibt Kubernetes selbst.

## Server-side Apply

Homeport wendet mit dem Server-side Apply von Kubernetes an, unter dem Field Manager `homeport`: Der Cluster zeichnet auf, welche Felder Homeport gesetzt hat, neben denen, die `kubectl`, Helm, Controller oder andere Werkzeuge gesetzt haben, was Sie in den `managedFields` eines Objekts sehen.

Wenn ein Dokument ein Feld ändert, das ein anderes Werkzeug verwaltet – `.spec.replicas`, gesetzt durch `kubectl apply` oder verwaltet von einem Autoscaler –, sagt die Vorschau, welche Felder und welche Manager. Lassen Sie diese Felder weg, oder wählen Sie nach einer zweiten Bestätigung **Trotzdem anwenden**: Homeport übernimmt sie, und das andere Werkzeug setzt sie womöglich zurück.

## Wenn ein Dokument nicht angewendet werden kann

Die Vorschau zeigt das Problem jedes Dokuments an Ort und Stelle, und **Anwenden** bleibt deaktiviert, bis jedes Dokument angewendet werden kann:

- **Ungültiges YAML**, mit der Zeile, an der es bricht.
- **Unbekannte Art**: Der Cluster stellt diese `apiVersion` und diesen `kind` nicht bereit. Eine benutzerdefinierte Ressource braucht zuerst ihre installierte CRD.
- **Vom Cluster abgelehnt**: die Antwort seiner Validierung für jedes Feld, etwa ein fehlendes Pflichtfeld oder ein Wert vom falschen Typ.
- **Nicht erlaubt**: Anwenden braucht `patch` auf der Ressource (und `create` für neue Objekte), mit dem Befehl `kubectl auth can-i` zur Prüfung.
