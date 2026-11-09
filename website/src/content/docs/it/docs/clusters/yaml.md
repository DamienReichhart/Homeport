---
title: Applicare e modificare YAML
description: Applica manifest a un cluster o modifica lo YAML di un oggetto, vedendo cosa cambia prima che cambi qualcosa, con il server-side apply.
---

Due modi per cambiare un cluster con lo YAML, per tutto ciò che le azioni dedicate non coprono: **Applica YAML** per i manifest, come `kubectl apply -f`, e **Modifica** nella scheda **YAML** di qualsiasi oggetto, come `kubectl edit`. Entrambi mostrano cosa cambierebbe prima che cambi qualcosa.

## Applica YAML

**Applica YAML**, nella barra laterale di un cluster, accetta uno o più manifest: incollali, o usa **Apri file…** per caricare un file `.yaml`, `.yml` o `.json` da questo computer. Più documenti separati da `---` vengono applicati insieme, in ordine.

Un documento senza namespace va nel namespace scelto in alto a destra (o, con **Tutti i namespace**, nel namespace predefinito del cluster). Gli oggetti che non stanno in un namespace, come un Namespace o un ClusterRole, lo ignorano.

1. **Anteprima delle modifiche** chiede al cluster cosa farebbe ogni documento, con un dry run lato server: per ora non cambia nulla. Ogni documento compare come **Creato**, **Modificato** o **Nessuna modifica**, con le righe che cambierebbe.
2. **Applica** ti chiede di confermare, indicando il cluster, poi applica.

Applicare due volte gli stessi manifest riporta **Nessuna modifica** la seconda volta. Modificare lo YAML dopo un’anteprima ne richiede prima una nuova.

## Modificare un oggetto

La scheda **YAML** di ogni oggetto ha **Modifica**: il suo YAML come lo conserva il cluster, senza ciò che gestisce il server (`managedFields`, `status`, la sua `resourceVersion`, l’uid e l’ora di creazione). Cambialo, usa **Anteprima delle modifiche** per vedere solo le righe che hai cambiato, poi **Applica**.

L’anteprima confronta il tuo YAML con l’oggetto com’è al momento dell’anteprima, quindi una modifica fatta nel frattempo compare nel diff. Se la modifica cambia un campo gestito da un altro strumento, l’anteprima lo elenca come conflitto invece di sovrascriverlo.

I Secret non possono essere modificati in Homeport, poiché non mostra mai tutti i loro valori insieme; e gli eventi sono scritti da Kubernetes stesso.

## Server-side apply

Homeport applica con il server-side apply di Kubernetes, con il field manager `homeport`: il cluster registra quali campi ha impostato Homeport, accanto a quelli impostati da `kubectl`, Helm, i controller o altri strumenti, che vedrai nei `managedFields` di un oggetto.

Quando un documento cambia un campo gestito da un altro strumento — `.spec.replicas` impostato da `kubectl apply` o gestito da un autoscaler — l’anteprima dice quali campi e quali manager. Lascia fuori quei campi, o usa **Applica comunque** dopo una seconda conferma: Homeport ne prende il controllo, e l’altro strumento potrebbe reimpostarli.

## Quando un documento non può essere applicato

L’anteprima mostra il problema di ogni documento al suo posto, e **Applica** resta disattivato finché ogni documento non può essere applicato:

- **YAML non valido**, con la riga in cui si interrompe.
- **Kind sconosciuto**: il cluster non serve quella `apiVersion` e quel `kind`. Una risorsa personalizzata richiede prima l’installazione della sua CRD.
- **Rifiutato dal cluster**: la risposta della sua validazione per ogni campo, come un campo obbligatorio mancante o un valore del tipo sbagliato.
- **Non consentito**: applicare richiede `patch` sulla risorsa (e `create` per i nuovi oggetti), con il comando `kubectl auth can-i` per verificarlo.
