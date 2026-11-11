---
title: Impostazioni
description: Tema e lingua, l’area di notifica, l’avvio all’accesso, le notifiche, gli aggiornamenti, gli accessi ai registri, i repository, la frequenza di controllo degli stack sincronizzati, cosa supporta questo computer, versione e licenze.
---

**Impostazioni** è l’ultima pagina della barra laterale, sopra i collegamenti **Documentazione** e **GitHub**, che si aprono nel browser.

## Aspetto

**Tema** è **Come il sistema** per impostazione predefinita, e segue la modalità chiara o scura del sistema operativo quando cambia. Scegli **Chiaro** o **Scuro** per fissarlo.

**Lingua** è **Come il sistema** per impostazione predefinita: Homeport si mostra nella lingua del sistema operativo quando la conosce, e in inglese altrimenti. L’opzione indica la lingua trovata, come **Come il sistema (Français)**. Scegli una lingua per usare quella; l’intera finestra cambia subito, senza riavvio, e così il menu dell’area di notifica e le notifiche.

Homeport parla inglese, francese (Français), spagnolo (Español), tedesco (Deutsch), italiano (Italiano), portoghese brasiliano (Português), russo (Русский), cinese semplificato (简体中文), giapponese (日本語), coreano (한국어), hindi (हिन्दी) e arabo (العربية), che mostra da destra a sinistra. Le varianti regionali seguono la loro lingua: il francese canadese mostra il francese, il portoghese europeo mostra il portoghese brasiliano. Il cinese tradizionale non è ancora disponibile e mostra l’inglese.

Anche date, numeri e dimensioni seguono la lingua, nello stile regionale del sistema quando ne ha uno: _1.5 GB_ e _4 Nov 2026_ in inglese britannico, _1,5 Go_ e _4 nov. 2026_ in francese. Nomi, ID, comandi, log e ciò che dicono gli Engine e i cluster restano come sono, e i **Dettagli** di un errore mantengono il testo originale del sistema o della libreria che ha avuto il problema, così una segnalazione di bug si legge allo stesso modo in ogni lingua.

## Desktop

Come funziona Homeport su questo computer.

- **Chiudi nell’area di notifica**: disattivato per impostazione predefinita, quindi chiudere la finestra fa uscire Homeport. Attivalo per nascondere invece la finestra nell’[area di notifica](/docs/guides/overview/#area-di-notifica): Homeport resta in esecuzione con le sue connessioni agli host aperte. Riaprilo dall’icona nell’area di notifica, o aprendo di nuovo Homeport; esci con **Esci** nell’area di notifica. L’interruttore è disattivato, con una spiegazione, su un desktop senza area di notifica.
- **Avvia all’accesso**: disattivato per impostazione predefinita. Attivalo e Homeport si avvia quando accedi, così controlla i tuoi host fin dall’inizio. L’interruttore mostra ciò che ha il tuo sistema: se rimuovi tu stesso Homeport dagli elementi di accesso, risulta disattivato. Su Windows è elencato in **Gestione attività → App di avvio**, su macOS in **System Settings → General → Login Items**, su Linux in `~/.config/autostart/`. La voce di un’AppImage segue il file quando lo sposti, a partire dalla volta successiva in cui lo apri.
- **Avvia nascosto nell’area di notifica**: disattivato per impostazione predefinita, e disponibile solo quando **Avvia all’accesso** è attivo. Quando Homeport si avvia all’accesso, resta nell’[area di notifica](/docs/guides/overview/#area-di-notifica) senza aprire la finestra. Aprire Homeport tu stesso mostra sempre la finestra, e senza un’area di notifica si apre sempre.

## Notifiche

Homeport mostra una notifica del sistema operativo quando succede qualcosa su un host, che la sua finestra sia aperta, ridotta a icona o [chiusa nell’area di notifica](#desktop). Ogni tipo ha il suo interruttore, tutti attivi per impostazione predefinita, e un interruttore ha effetto subito:

| Interruttore                                                     | Esempio                                                                                        |
| ---------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| **Un container si arresta con un errore o esaurisce la memoria** | _web-1 si è arrestato su prod-server (codice di uscita 137)_, _db ha esaurito la memoria su …_ |
| **Un container diventa non integro**                             | _api non è integro su prod-server_                                                             |
| **Un host connesso diventa non raggiungibile**                   | _prod-server non è raggiungibile: …_                                                           |
| **Un host non raggiungibile torna disponibile**                  | _prod-server è di nuovo connesso._                                                             |

Arrestare un container (con **Arresta** in Homeport, `docker stop`, `docker kill` o `docker compose down`) non è un crash, quindi non mostra nulla; né lo fa un container che esce con codice 0. Nemmeno un host che non riesce a connettersi all’avvio di Homeport è un’interruzione: lo è solo uno che era connesso. Un container bloccato in riavvio, o un host che continua a cadere, invia al massimo una notifica ogni 5 minuti.

Le notifiche arrivano dalle connessioni che Homeport ha già aperte, quindi sui tuoi host non viene installato nulla, e si fermano quando esci da Homeport. È il sistema operativo a decidere come appaiono e se vengono mostrate: se non ne compare nessuna, controlla le sue impostazioni di notifica per Homeport.

## Aggiornamenti

- **Cerca aggiornamenti automaticamente**: attivo per impostazione predefinita; Homeport cerca una nuova versione all’avvio e ogni 6 ore, e mostra una barra in cima alla finestra quando ce n’è una.
- **Controlla ora**: controlla subito, anche con i controlli automatici disattivati, e dice se hai l’ultima versione.

Quando è disponibile un aggiornamento, la scheda mostra cosa puoi fare, che dipende da come hai installato Homeport:

- **Windows, macOS e l’AppImage**: **Installa e riavvia** scarica l’aggiornamento, ne verifica la firma, lo installa e riapre Homeport.
- **Pacchetti `.deb` e `.rpm`**: è il tuo gestore di pacchetti a installare gli aggiornamenti, quindi la scheda mostra il comando da eseguire, con un pulsante **Copia il comando**.

**Novità** apre il [registro delle modifiche](/changelog/). Vedi [aggiornare](/docs/reference/updating/) per saperne di più.

## Registri

Gli accessi ai registri di container (Docker Hub, GitHub Container Registry (GHCR) o il tuo) vengono conservati una sola volta in Homeport e usati su ogni host: quando scarichi un’immagine, esegui un container da un’immagine che non c’è ancora, invii, crei una build o distribuisci uno stack gestito, collegato o sincronizzato. C’è un accesso per ogni indirizzo di registro.

- **Aggiungi accesso** chiede il **Registro** (`docker.io`, `ghcr.io`, `registry.example.com:5000`), il **Nome utente** e la **Password o token di accesso**. GHCR, e gli account Docker Hub con l’autenticazione a due fattori, richiedono un token di accesso invece della password.
- **Verifica l’accesso** chiede al registro, da questo computer, se accetta l’accesso, come fa `docker login`, senza salvare nulla. Un accesso rifiutato lo dice con un suggerimento; un registro che Homeport non riesce a raggiungere dice invece questo.
- **Modifica** cambia l’indirizzo o il nome utente; lascia vuota la password per mantenere quella memorizzata. **Rimuovi** dimentica l’accesso: le immagini di quel registro vengono di nuovo scaricate in forma anonima.

Docker Hub si può inserire come `docker.io`, `index.docker.io` o `https://index.docker.io/v1/`; sono un unico registro. Un’immagine senza un registro nel nome, come `nginx` o `bitnami/redis`, usa l’accesso a Docker Hub.

Password e token sono cifrati con la chiave del portachiavi del sistema operativo e non vengono mai più mostrati, registrati nei log o inseriti nei dettagli degli errori. Senza un portachiavi, la scheda spiega che gli accessi non possono essere memorizzati, e le immagini vengono scaricate in forma anonima. Vedi [sicurezza](/docs/reference/security/).

Homeport non legge `~/.docker/config.json` né i suoi credential helper.

## Repository

I repository git da cui vengono distribuiti gli [stack sincronizzati](/docs/guides/synced-stacks/), conservati una sola volta e utilizzabili su ogni host.

- **Aggiungi repository** chiede l’URL del repository e come Homeport lo raggiunge: una **Chiave di deploy** che crea per quel repository (URL SSH), un **Token** con il suo nome utente (URL HTTPS), oppure **Pubblico** per uno che chiunque può leggere.
- Il pulsante con l’ingranaggio mostra la chiave di deploy di un repository, con dove aggiungerla su GitHub, GitLab, Gitea e Forgejo, e **Nuova chiave** per sostituirla; oppure **Sostituisci il token**. **Verifica l’accesso** ne elenca i branch e i tag da questo computer. Una volta che Homeport ha considerato attendibile la chiave di un server SSH, lì compare la sua impronta, con **Reimposta** per quando la forge l’ha cambiata davvero.
- **Rimuovi** dimentica il repository ed elimina il suo clone su questo computer; viene rifiutato finché uno stack sincronizzato lo usa.

Chiavi e token sono cifrati con la chiave del portachiavi del sistema operativo e non vengono mai più mostrati, registrati nei log o inseriti nei dettagli degli errori. La tua configurazione git, i credential helper e le chiavi SSH non vengono mai usati. Vedi [repository](/docs/guides/synced-stacks/#repository).

## Stack sincronizzati

**Controlla i repository ogni** imposta quanti minuti Homeport attende tra un controllo e l’altro dei repository degli stack sincronizzati alla ricerca di nuovi commit: 5 per impostazione predefinita, almeno 1. Homeport controlla solo mentre è aperto; quando è chiuso non si sincronizza nulla. Vedi [polling e distribuzione automatica](/docs/guides/synced-stacks/#polling-e-distribuzione-automatica).

## Dati

Cosa conserva Homeport su questo computer.

### Backup e ripristino

**Esegui backup…** salva ogni host, cluster, stack, accesso al registro, repository e impostazione, con le loro chiavi, certificati, token, password e ambienti, in un unico file `.homeport`. Digita due volte una passphrase di almeno 12 caratteri: l’intero file viene cifrato con essa, e non può essere recuperata. Gli eventi degli Engine e la cronologia delle distribuzioni non sono inclusi, né nulla di ciò che si trova sui tuoi host o cluster: container, immagini, volumi e risorse Kubernetes restano dove sono.

**Ripristina…** apre un file del genere con la sua passphrase ed elenca ciò che contiene: i suoi host e cluster per nome, e quanti stack, accessi ai registri e repository. **Sostituisci tutto** sostituisce poi tutti i dati di Homeport con quelli del backup, su questo o su un altro computer:

- Tutto si connette e distribuisce subito: host SSH e TLS, registri e repository, senza chiedere di nuovo una chiave o una password, né di considerare attendibile la chiave di un server.
- Uno [stack collegato](/docs/guides/stacks/#stack-collegati) la cui cartella non è su questo computer appare come esterno finché la cartella non c’è, allo stesso percorso.
- Gli [stack sincronizzati](/docs/guides/synced-stacks/) clonano di nuovo il loro repository alla sincronizzazione successiva.
- I dati attuali vengono prima copiati in `homeport.db.before-restore-<date>` nella cartella dei dati di Homeport. Ripristinare un file sbagliato, uno di un Homeport più recente o uno che fallisce a metà non cambia nulla.

Per aggiungere invece alcuni host a quelli che hai, usa **Esporta** e **Importa** nella pagina Host. [I tuoi dati e i backup](/docs/reference/data-and-backups/) spiega dove si trovano i file di Homeport, cosa include un backup e come copiare tu stesso il database.

### Conservare gli eventi degli Engine

**Conserva gli eventi degli Engine per** imposta per quanti giorni vengono conservati gli [eventi dell’Engine](/docs/guides/overview/#eventi) di ogni host: 30 per impostazione predefinita, da 1 a 365. Ogni host conserva al massimo 10.000 eventi, qualunque sia questo valore. Quelli più vecchi vengono rimossi all’avvio di Homeport e ogni ora, e subito quando abbassi il numero.

## Questo computer

Cosa può usare Homeport su questo computer:

- **Portachiavi del sistema operativo**: **Disponibile** quando chiavi SSH, certificati, ambienti degli stack e accessi ai registri possono essere memorizzati cifrati; **Non disponibile** altrimenti. Vedi [requisiti](/docs/requirements/#facoltativo-un-portachiavi-del-sistema-operativo).
- **CLI di Compose**: la versione di Compose trovata, come `docker compose v2.29.1`, oppure **Non trovata** se gli stack possono essere avviati e arrestati ma non distribuiti. Vedi [stack](/docs/guides/stacks/#cosa-serve-per-distribuire).
- **git**: il git trovato, come `/usr/bin/git 2.43.0`, oppure cosa installare se gli stack sincronizzati non si possono usare. Vedi [requisiti](/docs/requirements/#facoltativo-git).
- **CLI di Helm**: l’helm trovato, come `/usr/local/bin/helm v3.16.2`, oppure cosa installare per questo sistema se le [release Helm](/docs/clusters/helm/) non si possono elencare né installare. Helm 2 viene segnalato come non supportato. Vedi [requisiti](/docs/requirements/#facoltativo-helm).

Homeport controlla le CLI ogni volta che questa pagina si apre, quindi una installata dopo l’avvio compare; dopo aver installato un portachiavi, esci e riapri Homeport.

## Informazioni

Mostra la versione di Homeport e la piattaforma per cui è stato compilato, per esempio `v1.0.0 · macos/aarch64`.

**Apri la cartella dei log** mostra dove Homeport scrive il suo [log](/docs/reference/security/#log).

**Esporta diagnostica…** salva uno zip da allegare quando [segnali un problema](https://github.com/DamienReichhart/Homeport/issues/new/choose): un riepilogo delle versioni e dei tipi di host e di cluster che usi, e i log con i segreti rimossi. La finestra di dialogo ne mostra prima il contenuto, e nulla viene inviato da nessuna parte. Vedi [Esportare la diagnostica](/docs/reference/troubleshooting/#esportare-la-diagnostica-per-una-segnalazione-di-bug).

**Licenze** mostra la licenza di Homeport (la GNU AGPL, versione 3), un collegamento al codice sorgente della versione che usi e le note di ogni pacchetto open source con cui è costruito.
