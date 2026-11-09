---
title: Log e terminali
description: Segui i log di un Pod o di un intero carico di lavoro, scopri perché un container si è arrestato in modo anomalo, e apri un terminale nel container di un Pod.
---

La pagina di un Pod ha le schede **Log** e **Terminale**, e la pagina di un carico di lavoro una scheda **Log** per tutti i suoi Pod: i `kubectl logs` e `kubectl exec -it` di ogni giorno, tramite l’API di Kubernetes con le credenziali del cluster stesso.

## Log

**Log** trasmette i log di un container man mano che vengono scritti. È lo stesso visualizzatore di quello di un container su un host:

- **Cronologia**: parti dalle ultime 100, 1.000 o 10.000 righe, o da tutte.
- **Da**: solo le righe degli ultimi minuti, ore o giorni.
- **Orari**: l’ora in cui Kubernetes ha registrato ogni riga.
- **Segui**: mantieni in vista la riga più recente; disattivalo per rileggere mentre continuano ad arrivare nuove righe.
- **Trova**, **Cancella** e **Scarica** (come file di testo, le righe mostrate fino a quel momento).

Un Pod con più container ha un selettore **Container**, con gli init container contrassegnati. Si apre sul container che sceglierebbe `kubectl`: quello indicato dall’annotazione `kubectl.kubernetes.io/default-container` del Pod, altrimenti il suo primo container applicativo.

Kubernetes tiene insieme lo standard output e lo standard error di un container, quindi non c’è un filtro stdout/stderr come per i container di un host.

Il flusso si interrompe quando lasci la scheda, e lo segnala quando il container si arresta.

### Perché si è arrestato in modo anomalo?

Quando un container si è riavviato, **Istanza precedente** mostra i log dell’esecuzione prima dell’ultimo riavvio — per un Pod in `CrashLoopBackOff`, l’esecuzione che si è arrestata in modo anomalo — come `kubectl logs --previous`. È disattivato per un container che non si è mai riavviato, poiché non c’è nulla da mostrare.

Quando un container non è ancora partito (`ContainerCreating`, `CrashLoopBackOff` mentre attende di ripartire), i log lo dicono con il motivo, al loro posto. La pagina del suo Pod dice perché è in attesa.

### I log di un carico di lavoro

La scheda **Log** di un Deployment, StatefulSet, DaemonSet o Job alterna i log di tutti i suoi Pod man mano che vengono scritti, con ogni riga etichettata con il suo Pod nel suo colore, come `kubectl logs -l app=api --prefix`. Scegli un Pod per leggerlo da solo, e il container quando il suo template ne ha più di uno.

## Terminale

**Terminale** apre una shell in uno dei container in esecuzione del Pod: `bash` quando l’immagine lo ha, altrimenti `sh`. Digita invece un comando nel campo (`psql -U app shop`, `top`) e **Connetti** lo esegue, con le virgolette interpretate come farebbe una shell.

È un vero TTY: `vi`, `top`, i colori e i tasti freccia funzionano, e ridimensionare la finestra ridimensiona il terminale nel container. Quando il processo termina, il terminale dice con quale codice; **Riconnetti** ne avvia un altro. Lasciare la scheda termina il processo nel container.

### Quando non si apre

- **Nessuna shell**: le immagini minime (distroless) non hanno alcuna shell. Digita un comando presente nell’immagine, o collega un container di debug da un terminale con `kubectl debug -it <pod> --image=busybox --target=<container>`; Homeport non crea container di debug.
- **Non in esecuzione**: un container in attesa o terminato non può eseguire nulla. I Pod di un Job completato si sono arrestati definitivamente.
- **Non consentito**: aprire un terminale richiede `create` su `pods/exec`, e leggere i log `get` su `pods/log`, che spesso mancano ai ruoli di sola lettura. Homeport dice quale, con il comando `kubectl auth can-i` per verificarlo.

Homeport non copia file dentro o fuori da un Pod (`kubectl cp`).
