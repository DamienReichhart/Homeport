---
title: containerd
description: Connettiti a containerd su questo computer o tramite SSH, scegli un namespace, esegui i task dei suoi container, leggine i log, apri una shell e gestisci immagini e snapshot.
---

**containerd** funziona sotto k3s, nerdctl, BuildKit e molti server che non hanno Docker. Homeport vi si connette direttamente, tramite la sua API gRPC, come stabilisce l’[ADR 0020](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0020-containerd-over-grpc.md). Un host containerd ha pagine proprie nella barra laterale: **Container**, **Immagini**, **Snapshot** e **Namespace**. Le pagine di Docker, gli stack e la Bacheca non sono disponibili su di esso.

## Aggiungere un host containerd

Scegli **containerd** nel campo **Engine** del modulo. Raggiunge containerd solo tramite il suo socket, quindi la connessione è **Socket locale** o **SSH**; TCP e TLS sono disattivati, perché containerd non serve la sua API sulla rete.

| containerd                      | Socket                                                                       |
| ------------------------------- | ---------------------------------------------------------------------------- |
| il servizio containerd stesso   | `/run/containerd/containerd.sock`                                            |
| k3s                             | `/run/k3s/containerd/containerd.sock`                                        |
| il containerd del Docker Engine | `/run/containerd/containerd.sock`, con i suoi container nel namespace `moby` |

Su Linux, **Trovati su questo computer** elenca il containerd di questo computer e quello di k3s quando rispondono. Un socket che non puoi ancora usare viene invece elencato con il modo per risolvere il problema.

### Consentire al tuo utente di usare il socket

Il socket di containerd appartiene a root. Per usarlo come altro utente, in locale o tramite SSH, dai a un gruppo l’accesso in `/etc/containerd/config.toml` e aggiungi l’utente a quel gruppo:

```bash
sudo groupadd -f containerd
sudo usermod -aG containerd "$USER"
getent group containerd | cut -d: -f3
```

Metti il numero stampato sotto `[grpc]` in `/etc/containerd/config.toml`, riavvia containerd e accedi di nuovo:

```toml
[grpc]
  gid = 1001
```

```bash
sudo systemctl restart containerd
```

**containerd rootless** (`containerd-rootless.sh`) tiene il suo socket all’interno del namespace di RootlessKit, dove nulla dall’esterno può raggiungerlo, nemmeno Homeport. Homeport lo segnala quando lo trova; usa invece il servizio di sistema di containerd.

### Cosa consente ogni versione e connessione

Homeport si connette a containerd **1.7** e successivi. Il download passa per il servizio Transfer di containerd, stabile dalla **2.0**.

| Azione                                          | containerd       | Connessione                      |
| ----------------------------------------------- | ---------------- | -------------------------------- |
| Namespace, container e task, immagini, snapshot | 1.7 e successivi | socket locale o SSH              |
| Scaricare un’immagine                           | 2.0 e successivi | socket locale o SSH              |
| Log e un terminale                              | 1.7 e successivi | SSH, o un socket locale su Linux |

Un containerd troppo vecchio per connettersi lascia l’host in **Configurazione errata**, indicandone la versione; uno troppo vecchio per scaricare lo segnala quando scarichi.

## Namespace

Ogni oggetto di containerd vive in un **namespace**: nerdctl usa `default`, Kubernetes `k8s.io`, Docker `moby`, BuildKit `buildkit`. Le pagine mostrano un namespace alla volta, scelto sotto il selettore degli host, che elenca ciascuno con quanti container e immagini contiene. Homeport ricorda la scelta per ogni host tra un avvio e l’altro. Finché non ne scegli uno, un host mostra `default`, o l’unico namespace che contiene qualcosa quando `default` è vuoto, così un host k3s si apre su `k8s.io`.

La pagina **Namespace** crea namespace vuoti e rimuove quelli vuoti. containerd si rifiuta di rimuovere un namespace che contiene ancora qualcosa, e Homeport dice cosa contiene ancora: rimuovi prima i suoi container, immagini e snapshot.

## Container e task

In containerd, un **container** è solo metadati: la sua immagine, la sua spec, il suo snapshot. Ciò che viene eseguito è il suo **task**. La pagina **Container** elenca i container del namespace con il loro task: **In esecuzione**, **In pausa**, **Arrestato** con il suo codice di uscita, o **Nessun task**. Ciascuno indica quale strumento lo ha creato — **Kubernetes** (con il suo Pod), **nerdctl**, **Docker** o **ctr o altro** — e i container pause dei Pod sono nascosti a meno che tu non li richieda.

| Azione            | Cosa fa Homeport                                                                                                                                                                                                                                                                                           |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Avvia**         | Crea il task sullo snapshot del container, con i log dove li scrive nerdctl, e lo avvia. Un task arrestato viene sostituito, come richiede `ctr task start`.                                                                                                                                               |
| **Arresta**       | Invia il segnale di arresto dell’immagine (SIGTERM a meno che non ne indichi un altro), poi SIGKILL dopo 10 secondi. Un task ancora attivo dopo questo viene segnalato.                                                                                                                                    |
| **Sospendi**      | Congela il task; **Riprendi** lo scongela.                                                                                                                                                                                                                                                                 |
| **Invia segnale** | Invia qualsiasi segnale, per nome (`SIGHUP`) o numero, al processo principale del task.                                                                                                                                                                                                                    |
| **Rimuovi**       | Chiede prima conferma; termina un task in esecuzione, rimuove il task e il container, e il suo snapshot con i file che ha modificato, a meno che tu non deselezioni **Rimuovi anche**: uno snapshot conservato viene marcato `containerd.io/gc.root`, così il garbage collector di containerd lo conserva. |

La pagina di un container mostra il suo runtime, snapshot, etichette e spec OCI. Homeport non crea container su containerd: containerd non ha una propria gestione della rete, che aggiungono nerdctl e CNI.

Il **kubelet** gestisce i container di Kubernetes: li riavvia dopo un arresto e li ricrea dopo una rimozione. Le loro pagine lo dicono; modifica invece il Pod sul suo cluster. Rimuovere da qui un container nerdctl lascia i record di nerdctl che lo riguardano (il suo nome e la sua rete), come farebbe `ctr`; per quelli, `nerdctl rm` è il modo più pulito.

## Log

containerd non conserva log: lo strumento che avvia un task decide dove va il suo output. La scheda **Log** lo legge dove quello strumento lo ha scritto:

| Creato da             | Dove va il suo output                                                                           | Cosa mostra Homeport                                            |
| --------------------- | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| nerdctl               | un file di log JSON nella cartella di stato di nerdctl, indicato dalle sue etichette `nerdctl/` | quel file, seguito                                              |
| Kubernetes (`k8s.io`) | `/var/log/pods/<pod>/<container>/N.log`, indicato nel record CRI che lo riguarda                | quel file, seguito                                              |
| `ctr` e altri client  | dove lo hanno indirizzato, di solito da nessuna parte                                           | **L’output di questo container non viene registrato**, e perché |

I container nerdctl che scrivono i log con un altro driver (journald, fluentd) lo indicano invece. Homeport legge il file con `tail` sul computer dell’host (vedi [sotto](#what-runs-on-the-host)); i file di log di solito appartengono a root, quindi l’utente SSH deve avere il permesso di leggerli.

## Un terminale in un container

La scheda **Exec** apre una shell in un container in esecuzione — bash quando l’immagine ce l’ha, altrimenti sh — o il comando che indichi, come utente numerico (`uid` o `uid:gid`) e in una cartella a tua scelta. **Apri di nuovo** ne avvia uno nuovo con ciò che hai digitato.

containerd accetta l’input e l’output di un processo come percorsi sul suo computer. Quindi Homeport esegue lì un breve script che crea due FIFO in una cartella temporanea privata, e vi copia attraverso il terminale; containerd avvia il processo su quelle FIFO, dalla spec di processo del container stesso. Chiudere la scheda termina il processo e rimuove la cartella.

## Immagini

La pagina **Immagini** elenca le immagini del namespace con le loro piattaforme, i container creati da esse e la loro dimensione per la piattaforma dell’host, come la conta `ctr images ls`.

**Scarica immagine** recupera un’immagine tramite containerd stesso, poi la decomprime per la piattaforma dell’host nel suo snapshotter predefinito, così `nerdctl run` può usarla subito. Fornisci un riferimento completo (`ghcr.io/home/app:1.2`) o uno breve (`nginx`, che diventa `docker.io/library/nginx:latest`). L’immagine di un registro privato viene scaricata con il suo accesso da **Impostazioni → Registri**, inviato a containerd solo quando il registro lo richiede, e mai memorizzato sull’host.

Un’immagine da cui è stato creato un container del namespace non può essere rimossa; rimuovi prima i container. containerd libera il contenuto di un’immagine quando nulla più lo trattiene.

## Snapshot

Gli snapshot sono dove va il disco di containerd: i layer delle immagini decompresse, e il layer scrivibile di ogni container. La pagina **Snapshot** mostra quelli di uno snapshotter (`overlayfs` per primo, o qualsiasi altro caricato dall’host) come un albero per genitore, dal più grande, con il loro utilizzo come lo conta `ctr snapshot usage` e il totale in alto.

Ciascuno indica cosa lo usa: un container, tramite il suo snapshot, o un’immagine, tramite i suoi layer. Gli snapshot che nulla usa e su cui nulla è costruito — come uno conservato quando il suo container è stato rimosso da qui — possono essere mostrati da soli con **Solo inutilizzati**, e rimossi. Gli altri non possono essere rimossi da qui. Uno snapshot a cui nulla fa riferimento e che nulla marca come radice, come quello lasciato da `ctr containers rm --keep-snapshot`, non dura: il garbage collector di containerd lo rimuove alla sua esecuzione successiva.

## Cosa viene eseguito sull’host

Tutto quanto sopra passa per l’API di containerd, tranne log e terminali, che richiedono i file del computer di containerd. Per quelli Homeport esegue questi comandi, tramite la sessione SSH che ha già aperta, o su questo computer per un socket locale su Linux, con i diritti dell’utente SSH e mai con `sudo`:

| Per          | Comando                                                                                                                          |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------- |
| Log          | `tail -n <lines> [-F] -- '<log file>'`; per un container Kubernetes senza il record CRI, prima un `ls` della sua cartella di log |
| Un terminale | `mktemp -d`, `mkfifo -m 600` per due FIFO in quella cartella, `cat` attraverso di esse, e `rm -rf` della cartella                |

Un socket locale su macOS o Windows non può eseguirli, e log e terminali dicono allora che richiedono SSH. La [pagina sulla sicurezza](/docs/reference/security/#what-homeport-runs-on-your-hosts) elenca ogni comando che Homeport esegue su un host.

## Eventi

Le pagine si aggiornano dagli eventi di containerd stesso, in ogni namespace: un task che si avvia o termina, un container o un’immagine creati o rimossi, uno snapshot salvato. containerd non ne conserva la cronologia, quindi ciò che è successo mentre Homeport era chiuso non viene recuperato.
