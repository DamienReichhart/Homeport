---
title: Exec-Plugins
description: Wie Homeport sich bei EKS-, GKE- und AKS-Clustern mit dem Exec-Credential-Plugin anmeldet, das ihre kubeconfig nennt, und was wo läuft.
---

Cloud-kubeconfigs enthalten weder ein Passwort noch ein Zertifikat: Ihr Benutzer nennt ein **Exec-Plugin**, ein Programm auf Ihrem Rechner, das auf Anfrage ein kurzlebiges Token ausgibt. Homeport führt dasselbe Programm aus, das `kubectl` ausführen würde, auf diesem Rechner, nach dem Kubernetes-Protokoll `ExecCredential` (`client.authentication.k8s.io/v1` und `v1beta1`).

## Gängige Anbieter

| Anbieter | Der Benutzer der kubeconfig führt aus | Installation                                                                  |
| -------- | ------------------------------------- | ----------------------------------------------------------------------------- |
| EKS      | `aws eks get-token --cluster-name …`  | die AWS CLI, Version 2                                                        |
| GKE      | `gke-gcloud-auth-plugin`              | die Google Cloud CLI, dann `gcloud components install gke-gcloud-auth-plugin` |
| AKS      | `kubelogin get-token …`               | `az aks install-cli` oder kubelogin aus seinen Releases                       |

Schreiben Sie die kubeconfig wie gewohnt – `aws eks update-kubeconfig`, `gcloud container clusters get-credentials`, `az aks get-credentials` gefolgt von `kubelogin convert-kubeconfig` – und [fügen Sie dann ihre Kontexte hinzu](/docs/clusters/).

## Was wo läuft

- Das Plugin läuft **auf diesem Rechner**, als Sie, mit den Argumenten und Umgebungsvariablen, die die kubeconfig ihm gibt (zum Beispiel `AWS_PROFILE`). Homeport verlangt von ihm nie Interaktion und gibt ihm keine Eingabe.
- Ein Desktop-Starter startet Homeport mit einem kurzen `PATH`, daher sucht Homeport auch an den üblichen Installationsorten der AWS-, Google-Cloud- und Azure-CLIs und von kubelogin (Homebrew, der eigene Ordner des SDK, `~/.local/bin`, `~/.krew/bin`, Program Files…). Ein Plugin, das in der kubeconfig mit vollständigem Pfad angegeben ist, läuft von dort.
- Das Token oder Zertifikat, das es ausgibt, wird **nur im Arbeitsspeicher** gehalten, bis kurz vor seinem Ablauf; dann läuft das Plugin erneut. Es läuft einmal pro Token-Lebensdauer, nicht einmal pro Anfrage. Wenn die API ein Token ablehnt, fragt Homeport das Plugin einmal nach einem neuen, bevor es **Zugriff verweigert** anzeigt.
- Die eigenen Caches des Plugins (`~/.kube/cache`, `~/.aws/sso/cache`, die gcloud-Konfiguration) sind seine Sache; Homeport schreibt dort nichts.

## Wenn die Anmeldung nicht gelingt

- **„… ist auf diesem Computer nicht installiert“** – das Programm wurde nicht gefunden. Installieren Sie es (die Meldung sagt wie, mit dem `installHint` der kubeconfig, wenn sie einen hat), prüfen Sie, ob es in einem Terminal funktioniert, und wählen Sie dann **Neu verbinden**.
- **„… erfordert eine Anmeldung, die innerhalb von Homeport nicht möglich ist“** – das Plugin möchte einen Browser oder einen Gerätecode (AWS SSO, Azure-Geräteanmeldung, eine abgelaufene `gcloud`-Sitzung). Führen Sie `kubectl` einmal in einem Terminal mit diesem Kontext aus, melden Sie sich dort an und wählen Sie dann **Neu verbinden**: Das Plugin findet dann seine zwischengespeicherte Anmeldung.
- **„… konnte keine Zugangsdaten für … abrufen“** – es lief, ist aber fehlgeschlagen; **Details** zeigt das Ende seiner Ausgabe. Homeport versucht es von selbst erneut.

Siehe [Fehlerbehebung](/docs/reference/troubleshooting/#clusters) für die anderen Meldungen.
