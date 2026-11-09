---
title: Risorse API
description: Sfoglia ogni tipo di oggetto servito da un cluster, risorse personalizzate comprese, con le colonne mostrate da kubectl get.
---

Homeport ha pagine proprie per carichi di lavoro, Pod, Service, Ingress, ConfigMap, Secret, Node ed eventi. **Risorse API**, nella barra laterale di un cluster, copre tutto il resto: i PersistentVolume e le loro claim, le NetworkPolicy, i ServiceAccount e l’RBAC, e le risorse che gli operator aggiungono con le loro CRD — i Certificate di cert-manager, le Application di Argo CD, i ServiceMonitor di Prometheus.

## Trovare un tipo

La colonna di sinistra elenca ogni tipo servito dal cluster, raggruppato per gruppo API (`core` per primo), alla versione preferita di ogni gruppo, come fa `kubectl api-resources`. Cerca per kind, plurale o gruppo, o digita un nome breve: `pvc`, `netpol`, `sa`, `cert`.

Un lucchetto segna i tipi che non puoi elencare: quelli che le tue credenziali non hanno il permesso di elencare nel namespace scelto (come il cluster comunica a Homeport tramite una `SelfSubjectRulesReview`), e i pochi la cui API non permette affatto di elencare. Aprirne uno spiega perché. Alcuni cluster, come quelli il cui authorizer è un webhook, non possono dirlo; allora nulla viene segnato e un elenco rifiutato lo dice.

Installare o rimuovere una CRD aggiorna l’elenco da solo; **Aggiorna** lo legge di nuovo.

## Elencare gli oggetti

Scegli un tipo e i suoi oggetti vengono elencati con le stesse colonne di `kubectl get`: è il server API a dire quali, comprese le `additionalPrinterColumns` di una CRD, così i Certificate di cert-manager mostrano **Ready**, **Secret** e **Age** come in un terminale. L’elenco segue le modifiche in tempo reale, tramite un watch.

I tipi con namespace hanno il selettore del namespace, come le altre pagine di Homeport; con **Tutti i namespace**, viene aggiunta una colonna **Namespace**.

## La pagina di un oggetto

Fai clic su un oggetto per aprire la sua pagina:

- **Panoramica**: il suo namespace, gli oggetti che lo possiedono — con un link alla loro pagina — la sua età, le etichette e le annotazioni.
- **Condizioni**, quando il suo `status` ne ha, come per le pagine proprie di Homeport.
- Gli **Eventi** che lo riguardano, e il suo **YAML**, senza `managedFields`.

Quando il suo tipo lo consente, **Modifica** apre il suo YAML per cambiarlo tramite un’anteprima e il server-side apply (vedi [Applicare e modificare YAML](/docs/clusters/yaml/)), ed **Elimina** lo rimuove dopo una conferma che indica l’oggetto e il cluster (vedi [Eliminare](/docs/clusters/actions/#eliminare)). Nell’elenco, spunta più oggetti per eliminarli insieme.
