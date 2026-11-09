---
title: Host e connessioni
description: Come Homeport si connette a Docker Engine, containerd e Incus, cosa significa ogni stato di un host, e come funziona la riconnessione.
---

Un **host** è un Docker Engine, un containerd o un Incus a cui Homeport si connette. Ha un nome e un modo per raggiungerlo. Ogni container, istanza, immagine, volume, rete e stack che vedi appartiene esattamente a un host.

## Tipi di connessione

| Tipo              | Raggiunge un Engine…                                  | Credenziali                                                                 | Guida                                                          |
| ----------------- | ----------------------------------------------------- | --------------------------------------------------------------------------- | -------------------------------------------------------------- |
| **Socket locale** | su questo computer, tramite un socket o una pipe      | nessuna                                                                     | [Engine locali](/docs/hosts/local/)                            |
| **SSH**           | su un server a cui puoi accedere                      | una chiave di Homeport, la tua chiave o ssh-agent                           | [SSH](/docs/hosts/ssh/)                                        |
| **TLS**           | esposto con `dockerd --tlsverify`, o l’HTTPS di Incus | certificato CA, certificato e chiave client; per Incus, un token di fiducia | [TLS e TCP](/docs/hosts/tls-tcp/), [Incus](/docs/hosts/incus/) |
| **TCP**           | esposto senza cifratura, su una rete attendibile      | nessuna                                                                     | [TLS e TCP](/docs/hosts/tls-tcp/)                              |

Per un server remoto, preferisci **SSH**: non richiede nulla di esposto oltre alla porta SSH ed è il più semplice da configurare in modo sicuro.

## Tipi di Engine

Oltre al modo per raggiungerlo, ogni host indica a quale tipo di Engine punta, nel campo **Engine** del modulo. Questo decide quali pagine la barra laterale mostra per quell’host.

| Engine                     | Cosa comprende                                                                | Pagine                                                                                              |
| -------------------------- | ----------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| **Compatibile con Docker** | Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop, Podman      | Container, Immagini, Volumi, Reti, Stack, Eventi; e su Podman, anche le sue pagine proprie          |
| **containerd**             | il servizio containerd stesso, quello di k3s, il containerd del Docker Engine | Container, Immagini, Snapshot e Namespace di containerd; vedi [containerd](/docs/hosts/containerd/) |
| **Incus**                  | Incus 6.0 e successivi: container di sistema e macchine virtuali              | Istanze, Immagini, Profili, Archiviazione e Reti; vedi [Incus](/docs/hosts/incus/)                  |

Il modulo sceglie l’Engine dal percorso del socket quando questo indica il socket di containerd o di Incus. containerd si raggiunge solo tramite il suo socket, su questo computer o tramite SSH, quindi TCP e TLS sono disattivati per esso; Incus tramite il suo socket o tramite HTTPS (TLS), quindi solo TCP è disattivato. Gli host aggiunti prima di Homeport 0.18 sono compatibili con Docker, senza altre modifiche.

**Podman** è un Engine compatibile con Docker che Homeport riconosce a ogni connessione, perché lo stesso socket può cambiare prodotto: lo stato dell’host mostra allora **Podman** e la sua versione invece di quella dell’Engine, e la barra laterale aggiunge le pagine che solo Podman ha. Una pagina che l’host selezionato non può mostrare non compare nella barra laterale, e aprirla da un link spiega il perché.

Un host che appartiene a un Docker Swarm mostra **Manager di** o **Worker di** il suo Swarm, e aggiungere un manager come host basta perché lo Swarm compaia nella barra laterale. La pagina di un host ha anche una scheda **Swarm** per avviare uno Swarm, unirsi a uno o lasciarlo. Vedi [Swarm](/docs/guides/swarm/).

## Aggiungere, modificare e rimuovere host

- **Aggiungi** un host con **Aggiungi host…** nel selettore degli host in cima alla barra laterale, o con **Aggiungi host** nella Panoramica.
- **Gestisci gli host** nel selettore degli host elenca ogni host con il suo tipo, indirizzo e stato. Selezionane uno per aprire la sua pagina.
- Nella pagina di un host puoi cambiare i dettagli della connessione e scegliere **Salva**; Homeport si riconnette con le nuove impostazioni. Il tipo di connessione di un host non può essere cambiato — rimuovilo e aggiungilo di nuovo.
- L’interruttore **Attivato** disconnette un host senza dimenticarlo. Un host disattivato non riprova finché non lo riattivi.
- **Rimuovi l’host** dimentica l’host, le sue credenziali memorizzate, le definizioni compose dei suoi stack gestiti, i suoi stack collegati e la cronologia delle distribuzioni dei suoi stack. Nulla di ciò che è in esecuzione sull’host viene toccato: container, immagini e volumi restano come sono, e anche le cartelle degli stack collegati. Gli stack di uno Swarm gestito dall’host appartengono allo Swarm, e vengono conservati.

I nomi degli host devono essere univoci, senza distinzione tra maiuscole e minuscole.

## Importare host

**Aggiungi host** offre due modi per aggiungere molti host in una volta, da ciò che questo computer descrive già. Ciascuno apre un’anteprima: scegli le voci da importare, rinominale e scegli **Importa**.

- **Importa da ~/.ssh/config** aggiunge i server indicati nella tua configurazione SSH, con il loro utente, porta, chiave e bastion `ProxyJump`. Vedi [Importa da ~/.ssh/config](/docs/hosts/ssh/#importa-da-sshconfig).
- **Importa i contesti Docker** aggiunge gli Engine dei contesti della Docker CLI (`docker context ls`), con i loro certificati TLS.

Chiavi e certificati vengono letti una sola volta, al momento dell’importazione, e memorizzati cifrati come tutti gli altri; né `~/.ssh` né `~/.docker` vengono mai modificati.

### Contesti Docker

Homeport legge ogni contesto sotto `~/.docker/contexts` (o sotto `$DOCKER_CONFIG` quando è impostato) e converte il suo endpoint Docker:

| Endpoint                                               | Diventa                                                       |
| ------------------------------------------------------ | ------------------------------------------------------------- |
| `unix:///…`, `npipe:////./pipe/…`                      | un host **Socket locale**                                     |
| `tcp://host:port` senza file TLS                       | un host **TCP**                                               |
| `tcp://host:port` con `ca.pem`, `cert.pem` e `key.pem` | un host **TLS**, con i suoi certificati                       |
| `ssh://user@host:port`                                 | un host **SSH** che usa ssh-agent, con `/var/run/docker.sock` |

Il contesto integrato `default` non viene proposto: il [rilevamento locale](/docs/hosts/local/) trova già l’Engine di questo computer. Un contesto viene mostrato ma non può essere scelto quando:

- imposta `SkipTLSVerify`, dato che Homeport verifica sempre il certificato dell’Engine;
- la sua cartella TLS contiene solo alcuni dei tre file;
- il suo endpoint è di altro tipo, come uno Kubernetes;
- un host raggiunge già lo stesso Engine. Il suo nome indica quale.

Un contesto il cui nome è già quello di un host viene proposto come `name (2)`.

## Passare a un altro computer

**Esporta** nella pagina Host salva gli host che scegli in un file protetto da una passphrase; **Importa** nella pagina Host dell’altro computer lo apre e aggiunge gli host che scegli, pronti a connettersi. Un host il cui nome è già in uso lì viene proposto come `name (2)`, e puoi rinominarlo o saltarlo. Vedi [passare a un altro computer](/docs/reference/security/#passare-a-un-altro-computer) per sapere come è protetto il file.

## Selezionare un host

Ogni pagina di risorse — Container, Immagini, Volumi, Reti, Stack ed Eventi — mostra l’host selezionato nel **selettore degli host**. Le pagine di un host containerd mostrano anche un namespace alla volta, scelto [sotto il selettore](/docs/hosts/containerd/#namespace), e quelle di un host Incus un [progetto](/docs/hosts/incus/#progetti). Cambia host lì, o seleziona la scheda di un host nella [Panoramica](/docs/guides/overview/). Homeport ricorda la tua scelta tra un avvio e l’altro.

## Stati

Ogni host mostra uno stato accanto al suo nome. Quando l’host selezionato non è connesso, le pagine di risorse mostrano il suo stato con **Riprova ora** e **Apri le impostazioni dell’host**.

| Stato                                 | Significato                                                                                     | Cosa succede poi                                                                                                       |
| ------------------------------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| **Connesso**                          | L’Engine ha risposto. Viene mostrata la sua versione, o quella di Podman, containerd o Incus.   | Homeport continua a sorvegliare la connessione.                                                                        |
| **Connessione in corso**              | Homeport sta raggiungendo l’Engine.                                                             | —                                                                                                                      |
| **Disattivato**                       | L’interruttore **Attivato** dell’host è spento.                                                 | Nulla finché non lo attivi.                                                                                            |
| **Non raggiungibile**                 | La rete, il socket o l’Engine non ha risposto.                                                  | Riprova da solo, attendendo ogni volta più a lungo.                                                                    |
| **Accesso negato**                    | Il server SSH ha rifiutato la chiave, ssh-agent non ha funzionato, o l’handshake TLS è fallito. | Correggi la causa, poi **Riconnetti**.                                                                                 |
| **Chiave del server non attendibile** | Prima connessione SSH: la chiave del server o di un bastion richiede la tua approvazione.       | [Considera attendibile la chiave](/docs/hosts/ssh/#considera-attendibile-la-chiave-del-server) nella pagina dell’host. |
| **Chiave del server cambiata**        | Il server SSH ha presentato una chiave diversa da quella che avevi considerato attendibile.     | [Indaga prima](/docs/hosts/ssh/#quando-la-chiave-del-server-cambia).                                                   |
| **Credenziali perse**                 | Le credenziali memorizzate non possono essere decifrate.                                        | [Inseriscile di nuovo](/docs/reference/troubleshooting/#credenziali-perse).                                            |
| **Portachiavi non disponibile**       | L’host richiede segreti memorizzati ma non c’è un portachiavi del sistema operativo.            | Vedi i [requisiti](/docs/requirements/#facoltativo-un-portachiavi-del-sistema-operativo).                              |
| **Configurazione errata**             | Qualcosa nelle impostazioni dell’host manca o non è valido.                                     | Il messaggio dice cosa; correggilo e scegli **Salva**.                                                                 |

## Riconnessione

Homeport si connette a ogni host attivato all’avvio, e ogni host ha la propria connessione, così un server lento non blocca mai un altro.

Mentre è connesso, Homeport ascolta il flusso di eventi dell’Engine — ed è così che le pagine si aggiornano da sole — e controlla l’Engine ogni 15 secondi. Se la connessione cade, l’host diventa **Non raggiungibile** e Homeport riprova dopo 1 secondo, poi 2, 4, 8 e così via, fino a una volta al minuto. Una connessione riuscita azzera l’attesa.

Solo **Non raggiungibile** riprova automaticamente. Ogni altro problema richiede qualcosa da parte tua, e poi **Riconnetti** (nella pagina dell’host) o **Riprova ora** riprova subito.

## Eventi e aggiornamenti in tempo reale

Homeport non interroga periodicamente. Quando l’Engine segnala una modifica — un container che si avvia, un’immagine che viene scaricata — le pagine interessate si aggiornano in una frazione di secondo. La pagina [Eventi](/docs/guides/overview/#eventi) mostra il flusso grezzo.
