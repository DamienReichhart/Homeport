---
title: CPU e memoria
description: Guarda quanta CPU e memoria usano i Pod, i loro container e i Node, rispetto a richieste, limiti e allocabile, grazie a metrics-server.
---

Quando un cluster esegue [metrics-server](https://github.com/kubernetes-sigs/metrics-server), Homeport mostra quanta CPU e memoria usa ogni Pod e ogni Node: negli elenchi, e in grafici dal vivo nelle loro pagine. Legge la Metrics API (`metrics.k8s.io`) che metrics-server espone, come fa `kubectl top`.

## Cosa serve

metrics-server deve già essere in esecuzione sul cluster: Homeport non installa nulla su un cluster. Se c’è dipende da come è stato creato il cluster:

| Cluster                  | metrics-server                                                                    |
| ------------------------ | --------------------------------------------------------------------------------- |
| GKE                      | Installato                                                                        |
| AKS                      | Installato                                                                        |
| k3s e k3d                | Installato (a meno che k3s sia stato avviato con `--disable metrics-server`)      |
| Docker Desktop           | Installato con il suo Kubernetes                                                  |
| kind                     | Non installato                                                                    |
| EKS                      | Non installato: aggiungi il componente aggiuntivo metrics-server, o installalo tu |
| minikube                 | Non installato: `minikube addons enable metrics-server`                           |
| Altri cluster (kubeadm…) | Di solito non installato                                                          |

Per installarlo tu, segui [le sue istruzioni](https://github.com/kubernetes-sigs/metrics-server#installation); su un cluster locale i cui kubelet hanno certificati autofirmati (kind), metrics-server ha bisogno anche di `--kubelet-insecure-tls`.

Senza metrics-server non si rompe nulla: le colonne di CPU e memoria non sono mostrate, e la scheda **Metriche** di un Pod o di un Node dice che metrics-server non è installato, con un link. Subito dopo l’installazione, metrics-server ha bisogno di circa un minuto prima di avere campioni; nel frattempo la scheda dice che la Metrics API non risponde ancora.

Le tue credenziali devono anche poter leggere le metriche: `list` e `get` su `pods.metrics.k8s.io` nel Namespace (o in tutti i Namespace), e su `nodes.metrics.k8s.io` per i Node. Quando non possono, la scheda **Metriche** dice quale permesso manca, e tutte le altre pagine continuano a funzionare.

## Negli elenchi

In **Pod**, le colonne **CPU** e **Memoria** mostrano quanto usa ogni Pod, sommando i suoi container, e che quota è del suo limite, o della sua richiesta se non ha un limite. Un Pod ha un limite solo quando ce l’hanno tutti i suoi container: un container senza limite può usare tutto il Node. La CPU è in millicore, come la scrive `kubectl top` (`250m` è un quarto di core), e la memoria è il working set, su cui il kubelet si basa per sfrattare i Pod.

In **Node**, **CPU usata** e **Memoria usata** mostrano quanto usa ogni Node, e la sua quota di ciò che è **allocabile** ai Pod.

Fai clic sull’intestazione di una colonna per ordinare in base a essa: prima i Pod o i Node più carichi.

## Nella pagina di un Pod o di un Node

La scheda **Metriche** di un Pod traccia CPU e memoria di ogni container, con una linea tratteggiata alla sua richiesta e un’altra al suo limite, quando sono impostati. Quella di un Node traccia la sua CPU e la sua memoria con una linea all’allocabile.

metrics-server legge i kubelet ogni 15 secondi, e Homeport chiede un nuovo campione allo stesso ritmo, così un cambio di carico compare nel grafico entro 30 secondi. I campioni restano finché la pagina è aperta, per 15 minuti al massimo, qualunque scheda tu stia guardando, e solo in memoria: chiudere la pagina ferma le letture e li dimentica. Homeport non conserva alcuno storico delle metriche; per quello, usa un sistema di monitoraggio come Prometheus.
