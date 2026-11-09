---
title: Inoltro di porte
description: Raggiungi la porta di un Pod o di un Service da questo computer, come kubectl port-forward, senza esporla.
---

Un inoltro di porta rende una porta di un Pod, o di un Service, raggiungibile su questo computer: un database, un’interfaccia di amministrazione o un’API interna, senza un `LoadBalancer`, un Ingress o un terminale lasciato aperto con `kubectl port-forward`.

## Inoltrare una porta

- Nella pagina di un **Pod**, i container elencano le porte che dichiarano; **Inoltra…** si trova accanto a ogni porta TCP.
- Nella pagina di un **Service**, **Inoltra…** si trova accanto a ognuna delle sue porte TCP.

Scegli la porta locale: parte dallo stesso numero quando quella porta è libera su questo computer, ed è vuota altrimenti. Lasciala vuota e Homeport ne sceglie una libera. Una notifica dice dove è in ascolto, con **Apri** per caricarla nel tuo browser.

L’inoltro è in ascolto solo sugli indirizzi di loopback — `127.0.0.1`, e `[::1]` quando la stessa porta è libera anche lì — quindi nient’altro sulla tua rete può raggiungerlo.

### Pod e Service

Un inoltro verso un **Pod** raggiunge quel Pod. Se il Pod scompare (un rollout lo ha sostituito), l’inoltro termina e Homeport lo dice.

Un inoltro verso un **Service** passa da uno dei suoi Pod pronti, come fa `kubectl port-forward svc/…`, verso la porta a cui il Service invia quella porta (il suo `targetPort`, per numero o per nome di una porta di container). Quando quel Pod scompare, la connessione successiva ne sceglie un altro pronto, quindi un inoltro verso un Service sopravvive ai rollout. Non bilancia le connessioni tra i Pod.

Si può inoltrare solo TCP. I Service senza selettore (che puntano a endpoint esterni) non possono essere inoltrati.

## Inoltri di porta

**Inoltri di porta**, nella barra laterale di un cluster, elenca ogni inoltro in esecuzione, in tutti i tuoi cluster: il suo indirizzo locale, il Pod o il Service e la porta che raggiunge (e, per un Service, il Pod attraverso cui passa ora), il suo Namespace e il suo cluster, e quante connessioni sono aperte attraverso di esso. **Apri** lo carica nel tuo browser; **Arresta** lo termina e chiude le sue connessioni.

Gli inoltri durano solo quanto Homeport: chiuderlo li arresta tutti, e nessuno torna al successivo avvio. Rimuovere o disattivare un cluster arresta anche i suoi inoltri.

## Quando non funziona

- **Porta in uso**: un altro programma è già in ascolto su quella porta locale. Scegline un’altra, o lasciala vuota.
- **Porte sotto la 1024**: la maggior parte dei sistemi non permette a un’app di mettersi in ascolto su di esse; scegli una porta locale più alta.
- **Non in esecuzione**, **nessun Pod pronto**: non c’è ancora nulla verso cui inoltrare; la pagina del Pod dice perché.
- **Non consentito**: l’inoltro richiede `create` su `pods/portforward`; Homeport lo indica, con il comando `kubectl auth can-i` per verificarlo.
