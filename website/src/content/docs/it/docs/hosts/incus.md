---
title: Incus
description: Connettiti a Incus su questo computer, tramite SSH o tramite HTTPS con un token di fiducia, e scegli il progetto mostrato dalle sue pagine.
---

**Incus** esegue container di sistema e macchine virtuali, le **istanze** della [guida](/docs/guides/instances/). Homeport vi si connette tramite la sua API REST, come stabilisce l’[ADR 0021](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0021-incus-over-its-rest-api.md). Un host Incus ha pagine proprie nella barra laterale: **Istanze**, **Immagini**, **Profili**, **Archiviazione** e **Reti**. Le pagine di Docker, gli stack e la Bacheca non sono disponibili su di esso.

## Aggiungere un host Incus

Scegli **Incus** nel campo **Engine** del modulo, poi come raggiungerlo:

| Connessione       | Quando                                                 | Cosa richiede                                                         |
| ----------------- | ------------------------------------------------------ | --------------------------------------------------------------------- |
| **Socket locale** | Incus è in esecuzione su questo computer (Linux)       | il tuo utente nel gruppo `incus-admin`, o `incus` (vedi sotto)        |
| **SSH**           | Incus è in esecuzione su un server a cui puoi accedere | l’utente SSH nel gruppo `incus-admin`, o `incus`                      |
| **TLS** (HTTPS)   | Incus è in ascolto sulla rete, o SSH non è consentito  | un token di fiducia, o un certificato client di cui Incus si fida già |

**TCP** semplice è disattivato: Incus non serve la sua API senza TLS. Homeport si connette a **Incus 6.0** (la release con supporto a lungo termine) e successivi.

### I suoi socket

| Socket                            | Chi può usarlo                   | Cosa raggiunge                                         |
| --------------------------------- | -------------------------------- | ------------------------------------------------------ |
| `/var/lib/incus/unix.socket`      | il gruppo `incus-admin`          | tutto: ogni progetto, pool di archiviazione e rete     |
| `/var/lib/incus/unix.socket.user` | il gruppo `incus` (`incus-user`) | solo il progetto dell’utente, creato al primo utilizzo |

Su Linux, **Trovati su questo computer** li elenca entrambi quando rispondono, quello limitato come **Incus (restricted to your project)**. Un socket che non puoi ancora usare viene elencato con il comando che risolve il problema:

```bash
sudo usermod -aG incus-admin "$USER"
```

Le modifiche ai gruppi valgono per i nuovi accessi: esci e accedi di nuovo (o riconnettiti tramite SSH) prima di aggiungere l’host. Un Incus arrestato indica quali unità lo avviano:

```bash
sudo systemctl enable --now incus.socket incus.service
```

### Tramite HTTPS con un token di fiducia

Incus è in ascolto sulla rete una volta impostato `core.https_address`. Sul server Incus:

```bash
incus config set core.https_address :8443
incus config trust add homeport
```

Il secondo comando stampa un **token di fiducia**: una riga di lettere e cifre. In Homeport, scegli **Incus** e **TLS**, lascia **Token di fiducia** e incollalo. Homeport mostra ciò che legge (il nome a cui è destinato, gli indirizzi del server e l’impronta del suo certificato, e quando scade), poi con **Aggiungi**:

1. crea il proprio certificato client, una chiave ECDSA P-384 creata in memoria;
2. prova gli indirizzi del token in ordine, e tiene il primo il cui certificato ha l’impronta indicata dal token; qualsiasi altro viene rifiutato, con entrambe le impronte;
3. invia il token con il suo certificato, e verifica che Incus ora se ne fidi.

L’host compare poi in `incus config trust list` con il nome del token. Un token funziona una sola volta e scade; uno usato, revocato o scaduto viene rifiutato con il comando che ne crea uno nuovo. Per revocare Homeport in seguito, rimuovi lì il suo certificato:

```bash
incus config trust list
incus config trust remove <fingerprint>
```

**Certificato esistente** accetta un certificato client e una chiave di cui Incus si fida già (`incus config trust add-certificate`), con il certificato di Incus stesso da considerare attendibile (`/var/lib/incus/server.crt`, o `cluster.crt` in un cluster).

### Il certificato del server è fissato

Il certificato di Incus è autofirmato, quindi non c’è una CA con cui verificarlo. Homeport conserva con l’host il certificato che ha considerato attendibile e da quel momento accetta esattamente quello, confrontato tramite la sua impronta SHA-256, qualunque sia l’indirizzo da cui viene raggiunto. Se Incus viene reinstallato o il suo certificato rinnovato, l’host viene rifiutato con l’impronta attesa e quella ricevuta: controlla quella nuova sul server con `incus info`, poi aggiungi di nuovo l’host con un nuovo token. La chiave client ed entrambi i certificati vengono memorizzati cifrati come ogni altro segreto; vedi [sicurezza](/docs/reference/security/#incus-certificates).

## Progetti

Incus conserva istanze, immagini, profili, volumi personalizzati e reti nei **progetti**. Le pagine mostrano un progetto alla volta, scelto sotto il selettore degli host, che elenca ciascuno con quante istanze e immagini contiene. Homeport ricorda la scelta per ogni host tra un avvio e l’altro. Finché non ne scegli uno, un host mostra `default`, o il suo unico progetto sul socket limitato. I pool di archiviazione appartengono all’intero server; i loro volumi vengono mostrati per il progetto selezionato.

## Eventi

Le pagine si aggiornano dagli eventi del ciclo di vita di Incus, in ogni progetto: un’istanza che si avvia o si arresta, uno snapshot creato, un’immagine aggiunta, un profilo o una rete modificati. Vengono anche conservati nella pagina **Eventi**. Incus non ne conserva la cronologia, quindi ciò che è successo mentre Homeport era chiuso non viene recuperato.

Mentre è connesso, Homeport chiede a Incus `GET /1.0` ogni 15 secondi. Un Incus arrestato rende l’host **Non raggiungibile**, e Homeport si riconnette da solo quando risponde di nuovo.

## Cosa fa Homeport sull’host

Su un host Incus non viene eseguito nulla oltre a Incus stesso: console e terminali passano attraverso i websocket di Incus, e le immagini remote vengono scaricate da Incus. Homeport legge da questo computer l’elenco delle immagini di un server di immagini remoto, per i selettori di immagini.
