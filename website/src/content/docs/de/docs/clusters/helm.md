---
title: Helm-Releases
description: Die Helm-Releases eines Clusters auflisten, installieren, upgraden, zurücksetzen und deinstallieren und Chart-Repositorys verwalten, über die helm-CLI auf diesem Rechner.
---

**Helm** in der Seitenleiste eines Clusters zeigt die [Helm](https://helm.sh)-Releases darauf: dieselben Releases, die `helm list` in einem Terminal zeigt, ob sie aus Homeport installiert wurden oder nicht.

## Was Homeport braucht

Homeport führt den Befehl `helm` ab Version 3 auf **diesem** Rechner aus. Installieren Sie ihn mit `brew install helm` unter macOS, `winget install Helm.Helm` (oder Chocolatey oder Scoop) unter Windows und `snap install helm --classic`, dem Paket Ihrer Distribution oder dem Skript von helm.sh unter Linux. **Einstellungen → Dieser Rechner** zeigt das helm, das Homeport gefunden hat; ohne eines sagt die Seite Helm, was Sie für Ihr System installieren müssen, und jede andere Seite des Clusters funktioniert.

Homeport gibt helm nicht Ihre kubeconfig. Für jeden helm-Befehl öffnet es eine private Verbindung zum Cluster auf der Loopback-Adresse dieses Rechners und gibt helm eine temporäre kubeconfig, die darauf zeigt, mit einem Einmal-Token und ohne Zugangsdaten des Clusters. Homeport fügt unterwegs jeder Anfrage die eigenen Zugangsdaten des Clusters hinzu – Zertifikate, Tokens, die Tokens von Exec-Plugins –, wie bei jeder anderen Seite, und löscht die kubeconfig, wenn der Befehl endet. So erreicht helm jeden Cluster, den Homeport erreicht, Cloud-Cluster mit Exec-Plugins eingeschlossen, und keine Zugangsdaten werden auf die Festplatte geschrieben.

Helm bewahrt seine Releases als Secrets im Namespace des jeweiligen Release auf: Homeport installiert nichts auf dem Cluster.

## Releases

Die Liste zeigt für jedes Release den Namen, den Namespace, das Chart und die Chart-Version, die App-Version, die Revision, den Status und wann es zuletzt geändert wurde. Wählen Sie oben rechts einen Namespace oder alle. helm hat keine Live-Aktualisierungen, daher fragt die Liste alle 30 Sekunden erneut, solange sie geöffnet ist, und **Aktualisieren** fragt sofort.

Ein Release mit Status **Fehlgeschlagen** ist rot markiert, und eines, das auf eine Installation, ein Upgrade oder ein Zurücksetzen **wartet** (`pending-install`, `pending-upgrade`, `pending-rollback`), gelb: Ein wartendes Release ist gesperrt, bis dieser Vorgang endet.

Wenn Ihre Zugangsdaten Secrets nicht in jedem Namespace auflisten dürfen, listet Homeport stattdessen die Releases des Standard-Namespace des Clusters auf und sagt das; wählen Sie einen anderen Namespace, um seine Releases zu sehen.

## Ein Chart installieren

**Chart installieren** (oder <kbd>n</kbd>) im Tab Releases installiert ein Chart auf dem ausgewählten Cluster:

1. **Das Chart wählen.** Durchsuchen Sie die Charts der [Repositorys](#chart-repositories) von helm und klicken Sie auf **Verwenden**, oder geben Sie unter **Aus einer OCI-Registry** eine Referenz wie `oci://registry-1.docker.io/bitnamicharts/nginx` an, die ohne Hinzufügen eines Repositorys installiert.
2. **Version, Name und Namespace wählen.** Die Versionen eines Repository-Charts werden aufgelistet, die neueste zuerst; die Version eines OCI-Charts wird eingetippt oder für die neueste leer gelassen. Das Release wird zunächst nach dem Chart benannt: höchstens 53 Kleinbuchstaben, Ziffern, Bindestriche und Punkte. Der Namespace ist der, den die Seite zeigt (oder der Standard des Clusters), und wird erstellt, falls er nicht existiert, es sei denn, Sie entfernen dieses Häkchen.
3. **Seine Values bearbeiten.** Der Editor beginnt mit den Standard-Values des Charts (`helm show values`); ändern Sie, was Sie brauchen, und **Auf die Standardwerte des Charts zurücksetzen** stellt sie wieder her. Values werden vor dem Lauf von helm als YAML geprüft und gehen über seine Standardeingabe an helm – nie in einer Datei.
4. **Vorschau**, optional: helm rendert das Chart mit Ihren Values und mit Hilfe des Clusters (`--dry-run=server`), und Homeport zeigt das Manifest, das es anwenden würde, Hooks eingeschlossen, sowie die Hinweise des Charts. Auf dem Cluster ändert sich nichts.
5. **Installieren.** Die Ausgabe von helm läuft ein, wie `helm install` sie ausgibt. Mit **Warten, bis seine Ressourcen bereit sind** wartet helm, bis die Deployments, Pods und Services des Release bereit sind, höchstens die angegebenen Minuten (standardmäßig 5), und markiert das Release als fehlgeschlagen, wenn sie es nicht sind. **Release öffnen** öffnet dann seine Seite.

Charts aus einem lokalen Ordner oder einer `.tgz`-Datei werden nicht aus Homeport installiert, ebenso wenig Charts aus OCI-Registrys, die eine Anmeldung brauchen, es sei denn, Sie haben sich mit `helm registry login` in einem Terminal angemeldet.

## Die Seite eines Release

Klicken Sie auf ein Release, um es zu öffnen:

- **Verlauf** – jede Revision, die neueste zuerst, mit Datum, Status, Chart, App-Version und der Beschreibung von helm, was passiert ist (`Install complete`, `Upgrade "web" failed: …`).
- **Values** – die bei Installation oder Upgrade angegebenen Values (**Vom Benutzer angegeben**) oder jeder Wert, mit dem das Chart gerendert wurde, seine Standardwerte eingeschlossen (**Berechnet**).
- **Hinweise** – was das Chart nach einer Installation ausgibt (seine `NOTES.txt`).
- **Manifest** – die Kubernetes-Objekte, die die Revision gerendert hat, als YAML.
- **Ressourcen** – diese Objekte nach Art, Name und Namespace, jedes öffnet seine Seite in Homeport: seine eigene Seite für Deployments, Pods, Services und dergleichen, seine Seite unter [API-Ressourcen](/docs/clusters/api-resources/) für den Rest.

Values, Hinweise, Manifest und Ressourcen zeigen die aktuelle Revision; wählen Sie darüber eine frühere, um zu sehen, womit sie installiert wurde.

## Ein Release upgraden

**Upgrade** auf der Seite eines Release ändert seine Chart-Version, seine Values oder beides:

- **Chart** – das Chart, aus dem upgegradet wird. helm zeichnet nicht auf, woher ein Chart stammt, daher sucht Homeport in Ihren Repositorys nach einem Chart dieses Namens, die mit der aktuellen Version des Release zuerst. Wählen Sie mit **Anderes Chart…** ein anderes, als `repository/chart` oder als `oci://`-Referenz – was ein aus einer OCI-Registry installiertes Release braucht.
- **Chart-Version** – die Versionen dieses Charts, die neueste zuerst, oder bei einem OCI-Chart eingetippt.
- **Values** – der Editor beginnt mit den vom Benutzer angegebenen Values des Release. Sie ersetzen die des Release: Die Standardwerte des Charts füllen den Rest. **Änderungen an den Values** zeigt sie neben den aktuellen, und **Auf die aktuellen Values zurücksetzen** stellt diese wieder her.
- **Warten, bis seine Ressourcen bereit sind**, höchstens die angegebenen Minuten.

**Vorschau** rendert das Upgrade mit Hilfe des Clusters, ohne etwas zu ändern, und zeigt das neue Manifest und was sich darin gegenüber dem aktuellen ändert. **Upgrade** führt `helm upgrade` aus, mit der Ausgabe von helm live. Jedes Upgrade ist eine neue Revision im **Verlauf**.

## Zurücksetzen

Im **Verlauf** fragt **Auf Revision N zurücksetzen** bei jeder früheren Revision zuerst nach, nennt dabei die Chart- und App-Versionen jetzt und danach, und führt dann `helm rollback` aus: helm stellt Chart und Values dieser Revision als neue Revision wieder her, sodass auch ein Zurücksetzen zurückgesetzt werden kann. Haken Sie **Warten, bis seine Ressourcen bereit sind** an, damit helm auf sie wartet.

Ein Release, das nach einer unterbrochenen Installation, einem Upgrade oder einem Zurücksetzen auf **Ausstehend** hängt, ist gesperrt: helm lehnt Änderungen ab, bis der Vorgang endet. Es auf seine zuletzt bereitgestellte Revision zurückzusetzen, hebt die Sperre auf.

## Deinstallieren

**Deinstallieren** auf der Seite eines Release nennt das Release, seinen Namespace und Cluster und listet die Objekte auf, die mit ihm gelöscht werden, bevor `helm uninstall` läuft. Objekte, die ein Chart zum Behalten markiert (`helm.sh/resource-policy: keep`), und die PersistentVolumeClaims von StatefulSets bleiben auf dem Cluster. **Verlauf behalten** lässt das Release als `uninstalled` in der Liste, sodass es zurückgesetzt werden kann; ohne diese Option verschwindet das Release aus der Liste. **Warten, bis seine Objekte gelöscht sind** lässt helm auf sie warten.

## Chart-Repositorys

**Chart-Repositorys**, der zweite Tab der Seite Helm, listet die Repositorys auf, die helm auf diesem Rechner kennt: dieselbe Liste wie `helm repo list` in einem Terminal, gemeinsam für jeden Cluster. Homeport führt keine eigene Liste, daher erscheint ein in einem Terminal hinzugefügtes Repository hier, und eines, das hier hinzugefügt wird, erscheint in `helm repo list`.

- **Repository hinzufügen** fragt nach einem Namen (Buchstaben, Ziffern, Punkte, Bindestriche und Unterstriche) und der `https://`-URL des Repositorys, etwa `https://charts.bitnami.com/bitnami`, und lädt seinen Index herunter.
- **Alle aktualisieren** oder **Aktualisieren** bei einem Repository lädt die neuesten Indizes herunter, wie `helm repo update`, mit der Ausgabe von helm live. Charts, die seit der letzten Aktualisierung veröffentlicht wurden, erscheinen erst nach einer.
- **Entfernen** vergisst ein Repository, nachdem Sie bestätigt haben. Daraus installierte Releases laufen weiter.

**Charts finden** durchsucht die Charts dieser Repositorys nach Name oder Beschreibung, wie `helm search repo`: Tippen Sie `postgres`, nachdem Sie das Repository von Bitnami hinzugefügt haben, um seine PostgreSQL-Charts mit ihren neuesten Chart- und App-Versionen zu sehen.

Homeport fügt nur öffentliche Repositorys hinzu. Eines, das eine Anmeldung braucht, wird in einem Terminal mit `helm repo add --username …` hinzugefügt, wobei helm sein Passwort wie immer aufbewahrt; es erscheint dann hier und funktioniert wie die anderen. Ein Chart in einer OCI-Registry (`oci://…`) braucht kein Repository.

## Wenn es nicht funktioniert

- **Die helm-CLI ist nicht installiert**: Installieren Sie helm 3 wie oben beschrieben und öffnen Sie die Seite dann erneut.
- **Helm 2**: Homeport steuert nur Helm 3 und neuer. Installieren Sie ein aktuelles helm.
- **Nicht erlaubt**: helm liest Releases aus Secrets; um sie aufzulisten, braucht es `list` auf `secrets` im Namespace. Details nennt, was abgelehnt wurde.
- **Eine andere Installation, ein Upgrade oder ein Zurücksetzen läuft noch**: Das Release ist durch einen ausstehenden Vorgang gesperrt. Warten Sie, bis er endet, oder setzen Sie es auf seine zuletzt bereitgestellte Revision zurück.
- **Ein Release mit diesem Namen existiert bereits**: Release-Namen sind in einem Namespace eindeutig. Wählen Sie einen anderen oder upgraden Sie das bestehende Release.
- **Chart oder Version nicht gefunden**: Prüfen Sie Name und Version; ein Repository-Chart braucht sein hinzugefügtes und [aktualisiertes](#chart-repositories) Repository. Um ein Release zu upgraden, dessen Chart in keinem Ihrer Repositorys ist, fügen Sie zuerst dieses Repository hinzu.
- **Die Values sind kein gültiges YAML**: Die Meldung nennt die Zeile; korrigieren Sie sie und versuchen Sie es erneut.
- **Warten aufgegeben**: Mit **Warten** war eine Ressource nicht rechtzeitig bereit. Sehen Sie sich die Pods und Events des Release an, um zu finden, was hängt.
- **Kein Chart-Repository oder nicht erreichbar**: Die URL liefert keine `index.yaml`, oder dieser Rechner kann sie nicht erreichen. Details zeigt, was helm gemeldet hat.
- **Nicht verbunden**: helm geht über die Verbindung des Clusters in Homeport; die Seite des Clusters sagt, warum er nicht verbunden ist.
