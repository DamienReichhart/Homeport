---
title: Unità Quadlet
description: Elenca le unità Quadlet di un host Podman con il loro stato systemd, leggi i loro file, installale, modificale e rimuovile, e avvia o arresta i loro servizi.
---

**Quadlet** è il modo in cui Podman esegue servizi di lunga durata sotto systemd. Un’unità Quadlet è un file (`web.container`, `shop.pod`, `cache.volume`, `lan.network`, `app.kube`, `base.image`, `api.build`) che il generatore di systemd trasforma in un servizio, `web.service`, che avvia il container all’avvio del sistema e lo riavvia quando si arresta. Podman rootful tiene le sue unità in `/etc/containers/systemd/`; Podman rootless tiene quelle di ogni utente in `~/.config/containers/systemd/`.

La pagina **Quadlet** compare nella barra laterale quando è selezionato un host Podman. Passa dall’API di Podman stesso sul socket che l’host usa già, e da `systemctl` per avviare e arrestare, come stabilisce l’[ADR 0019](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0019-quadlet-units-through-podmans-api-and-systemctl.md).

## Cosa consente ogni versione di Podman e ogni connessione

L’API di Podman ha acquisito il supporto per Quadlet nel corso di due release, e non avvia né arresta mai il servizio di un’unità. Quindi ciò che Homeport può fare dipende dal Podman dell’host e da come Homeport lo raggiunge:

| Azione                                     | Podman                | Connessione                      |
| ------------------------------------------ | --------------------- | -------------------------------- |
| Elencare le unità con il loro stato        | 5.7 e successivi      | qualsiasi                        |
| Elencare le unità dai loro container       | da 4.0 a 5.6          | qualsiasi                        |
| Leggere il file di un’unità                | 5.8 e successivi      | qualsiasi                        |
| Installare, modificare e rimuovere unità   | 5.8 e successivi      | qualsiasi                        |
| Avviare, arrestare e riavviare un servizio | qualsiasi con Quadlet | SSH, o un socket locale su Linux |

Su Podman precedente alla 5.7, le unità vengono trovate tramite l’etichetta `PODMAN_SYSTEMD_UNIT` che Podman applica ai container che creano, quindi un’unità compare solo dopo aver creato un container, con il nome del suo servizio. La pagina lo indica sopra l’elenco.

Avviare e arrestare richiedono `systemctl`, che Homeport esegue dove gira Podman: nella sessione SSH di un host SSH, oppure su questo computer per un socket locale su Linux. Un host TCP o TLS, o un socket locale su macOS o Windows (dove Podman gira in una macchina virtuale), può fare tutto il resto; i pulsanti sono disattivati, con il motivo.

## La pagina Quadlet

| Colonna       | Mostra                                                                                                                                                      |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Unità**     | Il nome del file dell’unità, e l’applicazione con cui è stata installata.                                                                                   |
| **Tipo**      | `container`, `pod`, `volume`, `network`, `kube`, `image`, `build` o `artifact`.                                                                             |
| **Servizio**  | Il servizio systemd che genera, oppure **non ancora generato** finché systemd non l’ha caricata.                                                            |
| **Stato**     | **In esecuzione**, **Avvio in corso**, **Arresto in corso**, **Arrestato**, **Non riuscito**, **Non caricato da systemd** o **Modello**, come dice systemd. |
| **Container** | Quanti container ha creato.                                                                                                                                 |

Ogni riga ha **Avvia** per un’unità arrestata o **Arresta** per una in esecuzione. Seleziona un’unità per aprirne la pagina. L’elenco si aggiorna da solo man mano che i suoi container si avviano e si arrestano.

Un’unità **Non caricato da systemd** è installata, ma il generatore di systemd l’ha saltata: di solito per un errore nel file. Sull’host, `systemctl --user daemon-reload` (o `systemctl daemon-reload` per Podman rootful) e poi `journalctl` dicono perché.

Nella pagina **Container**, un container creato da un’unità mostra il servizio dell’unità accanto al proprio nome; selezionalo per aprire l’unità.

## La pagina di un’unità

- L’intestazione ha **Avvia**, **Arresta** e **Riavvia**, **Modifica** e **Rimuovi**.
- **Servizio** mostra l’unità systemd, se è un’unità di sistema o un’unità utente (`systemctl --user`), dove si trova il suo file sull’host, e la sua applicazione e il suo pod, se li ha.
- **File dell’unità** mostra il file come lo legge Podman, con l’evidenziazione della sintassi.
- **Container** elenca i container che ha creato, ognuno dei quali apre la sua pagina di container.

Arrestare il container di un’unità dalla pagina Container va contro systemd, che lo riavvia secondo `Restart=`. Arresta invece l’unità.

## Installare un’unità

**Nuova unità** (o `n` nella pagina Quadlet, o **Nuova unità** nel riquadro comandi) apre un editor con un punto di partenza per il tipo che scegli, oppure un file che apri da questo computer con **Apri file…**. Dagli un nome con la sua estensione, come `web.container`; il file richiede la sezione del suo tipo, `[Container]` per un `.container`.

**Aggiungi file…** mette altri file accanto all’unità (un `Containerfile` per un `.build`, lo YAML di un `.kube`, un file di ambiente) in una cartella dell’applicazione che porta il nome dell’unità, come fa `podman quadlet install`.

Podman scrive i file dove vanno le unità del suo utente e ricarica systemd, così il servizio esiste subito. L’installazione non lo avvia: scegli **Avvia** una volta che c’è. Un’unità con un nome già usato viene rifiutata; modifica invece quella esistente.

## Modificare e rimuovere

**Modifica** apre il file dell’unità. **Salva** lo scrive al suo posto e ricarica systemd; i file installati accanto restano. Un servizio in esecuzione mantiene le vecchie impostazioni finché non scegli **Riavvia**. Se il file è cambiato sull’host da quando l’hai aperto, **Salva** viene rifiutato e lo dice, invece di sovrascrivere quella modifica: riapri l’unità per vederla, oppure scegli **Sovrascrivi**.

**Rimuovi** elimina il file dell’unità e ricarica systemd. **Arresta prima il suo servizio in esecuzione** è attivo per impostazione predefinita; senza, Podman si rifiuta di rimuovere un’unità in esecuzione. I container, i volumi e le immagini creati dall’unità vengono conservati.

## Cosa esegue Homeport sull’host

Per **Avvia**, **Arresta** e **Riavvia**, e per nient’altro, Homeport esegue un solo comando sul server dell’host tramite SSH (o su questo computer per un socket locale), con i diritti dell’utente SSH:

```bash
systemctl --user start 'web.service'
```

`--user` c’è per le unità di un Podman rootless e viene omesso per quelle di un Podman rootful; `start` diventa `stop` o `restart` per quelle azioni. Homeport verifica il nome del servizio e lo mette tra virgolette, e non usa mai `sudo`. Quindi:

- Le **unità di sistema** (Podman rootful) richiedono un utente SSH autorizzato a gestirle, come `root`.
- Le **unità utente** (Podman rootless) richiedono l’istanza systemd dell’utente. Perché giri senza una sessione di accesso, e perché le unità si avviino all’avvio del sistema, abilita il lingering una volta, come quell’utente sull’host:

```bash
loginctl enable-linger "$USER"
```

La [pagina sulla sicurezza](/docs/reference/security/#cosa-esegue-homeport-sui-tuoi-host) elenca ogni comando che Homeport esegue su un host.
