---
title: Ridimensionare, riavviare ed eliminare
description: Ridimensiona Deployment e StatefulSet, riavvia un rollout, sospendi ed esegui i CronJob, ed elimina qualsiasi oggetto di un cluster, ogni volta dopo una conferma che lo indica.
---

Le modifiche quotidiane a un cluster sono a un clic di distanza nelle sue pagine, ognuna inviata all’API di Kubernetes con le credenziali del cluster stesso, come farebbe `kubectl`. Gli elenchi seguono la modifica in tempo reale tramite il loro watch.

Ogni azione che rimuove o sostituisce qualcosa chiede prima conferma, e la conferma indica il cluster, il namespace, il kind e il nome, così una modifica non finisce mai per errore sul cluster o sul namespace sbagliato.

## Carichi di lavoro

La pagina di un carico di lavoro ha le sue azioni nell’intestazione; in **Carichi di lavoro**, ogni riga le ha nel suo menu **⋯**.

| Azione           | Su                                 | Fa, come                                                                |
| ---------------- | ---------------------------------- | ----------------------------------------------------------------------- |
| **Ridimensiona** | Deployment, StatefulSet            | `kubectl scale --replicas=N`                                            |
| **Riavvia**      | Deployment, StatefulSet, DaemonSet | `kubectl rollout restart`                                               |
| **Sospendi**     | CronJob                            | `kubectl patch cronjob … -p '{"spec":{"suspend":true}}'`                |
| **Riprendi**     | CronJob sospesi                    | lo stesso, con `false`                                                  |
| **Esegui ora**   | CronJob                            | `kubectl create job --from=cronjob/…`                                   |
| **Elimina**      | Tutti                              | `kubectl delete` (i suoi Pod e Job se ne vanno con esso, in background) |

### Ridimensionare

**Ridimensiona** chiede quanti Pod eseguire, mostrando quanti ne esegue ora. Ridimensionare a `0` arresta il carico di lavoro finché non lo ridimensioni di nuovo verso l’alto. Se un HorizontalPodAutoscaler gestisce il carico di lavoro, la finestra di dialogo lo dice: l’autoscaler reimposterà il numero al suo prossimo controllo, quindi cambia invece il minimo e il massimo dell’autoscaler.

### Riavviare

**Riavvia** aggiunge l’annotazione `kubectl.kubernetes.io/restartedAt` al template dei Pod, esattamente come fa `kubectl rollout restart`: il carico di lavoro distribuisce un nuovo insieme di Pod secondo la sua strategia (un nuovo ReplicaSet per un Deployment), e i vecchi Pod se ne vanno quando i nuovi sono pronti.

### CronJob

**Sospendi** impedisce a un CronJob di avviare nuovi Job (quelli già in esecuzione continuano) e **Riprendi** gli permette di avviarli di nuovo; un CronJob sospeso compare come **In pausa**. **Esegui ora** avvia subito un Job dal suo template, chiamato `<cronjob>-manual-<id>`, di proprietà del CronJob e marcato `cronjob.kubernetes.io/instantiate: manual` come quelli di kubectl. Compare nella scheda **Job** del CronJob.

## Eliminare

Ogni pagina di un oggetto — un carico di lavoro, un Pod, un Service, un Ingress, una ConfigMap, un Secret, un Node, o qualsiasi oggetto in **Risorse API** — ha **Elimina** nella sua intestazione. In un elenco, spunta le righe ed **Elimina N** le rimuove tutte dopo un’unica conferma che le elenca.

Gli oggetti vengono eliminati con la propagazione `Background`, come fa `kubectl delete`: l’oggetto se ne va subito e il garbage collector rimuove ciò che possiede (i ReplicaSet e i Pod di un Deployment, i Job di un CronJob).

La conferma dice cosa succede dopo:

- **Un Pod posseduto da un controller** — un ReplicaSet, uno StatefulSet, un DaemonSet o un Job — torna: il controller ne avvia uno nuovo al suo posto. Elimina o ridimensiona il carico di lavoro per sbarazzartene.
- **Un Namespace** porta con sé tutto ciò che contiene: i suoi carichi di lavoro, Pod, Service, ConfigMap e Secret.
- **Un Node** viene rimosso dall’elenco del cluster senza essere prima svuotato.

## Quando non funziona

- **Non consentito**: una modifica che le tue credenziali non possono fare lo dice con il suo verbo e la sua risorsa — «Le tue credenziali non consentono “patch” su deployments in shop.» — e il comando `kubectl auth can-i` per verificarlo.
- **Modificato nel frattempo**: quando qualcun altro ha modificato l’oggetto tra il momento in cui l’hai guardato e la tua azione, Homeport lo dice; guardalo di nuovo e riprova.
- **Già sparito**: eliminare qualcosa di già eliminato dice che non esiste più.
