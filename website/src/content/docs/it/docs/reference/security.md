---
title: La sicurezza e i tuoi dati
description: Cosa memorizza Homeport, dove, come vengono cifrati i segreti, e cosa non fa mai.
---

Homeport conserva credenziali che aprono i tuoi server. Questa pagina spiega esattamente cosa conserva, e come è protetto. [I tuoi dati e i backup](/docs/reference/data-and-backups/) spiega cosa copiare per farne un backup, e come ripristinarlo.

## Cosa memorizza Homeport

Tutto vive in un unico database SQLite, `homeport.db`, nella cartella dei dati delle applicazioni del tuo utente:

| Sistema | Cartella                                          |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

Contiene:

| Dati                                                                | Memorizzazione  |
| ------------------------------------------------------------------- | --------------- |
| Nomi degli host, indirizzi, utenti SSH e percorsi dei socket        | Testo in chiaro |
| Chiavi pubbliche SSH e le chiavi dei server considerate attendibili | Testo in chiaro |
| Chiavi private SSH e le loro passphrase                             | **Cifrate**     |
| Certificati CA TLS, certificati client e chiavi                     | **Cifrati**     |
| Chiavi client e certificati fissati degli host Incus                | **Cifrati**     |
| File compose degli stack gestiti                                    | Testo in chiaro |
| Ambienti degli stack gestiti                                        | **Cifrati**     |
| Cartelle e percorsi dei file compose degli stack collegati          | Testo in chiaro |
| Cronologia delle distribuzioni: file compose e profili              | Testo in chiaro |
| Cronologia delle distribuzioni: ambienti                            | **Cifrati**     |
| Indirizzi dei registri e nomi utente                                | Testo in chiaro |
| Password e token di accesso dei registri                            | **Cifrati**     |
| URL dei repository, metà pubbliche delle chiavi di deploy           | Testo in chiaro |
| Chiavi dei server dei repository considerate attendibili            | Testo in chiaro |
| Chiavi di deploy e token di accesso dei repository                  | **Cifrati**     |
| Nomi dei cluster, URL dei server API e namespace                    | Testo in chiaro |
| Come accede ogni cluster, il suo contesto kubeconfig                | Testo in chiaro |
| CA, certificati client e chiavi dei cluster                         | **Cifrati**     |
| Token, password e token OIDC dei cluster                            | **Cifrati**     |
| Comandi, argomenti e ambiente dei plugin exec                       | **Cifrati**     |
| Il tuo tema, l’ultima selezione e il namespace di ogni cluster      | Testo in chiaro |
| Gli eventi dell’Engine di ogni host, per la pagina Eventi           | Testo in chiaro |

Homeport non copia nient’altro dai tuoi Engine: container, immagini, volumi e log vengono letti in tempo reale e mai salvati, tranne quando sei tu a scaricare log o file, o a eseguire il backup di un volume. Gli eventi dell’Engine che conserva — cosa è successo a quale container, immagine, volume o rete, con le loro etichette — vengono conservati per 30 giorni per impostazione predefinita ([Impostazioni → Dati](/docs/guides/settings/#dati)), al massimo 10.000 per host, e rimossi con il loro host o con **Cancella la cronologia**.

### Cartelle degli stack collegati

Per uno [stack collegato](/docs/guides/stacks/#stack-collegati), Homeport memorizza solo dove si trovano la sua cartella e i suoi file compose, e un’impronta dei file per capire quando cambiano — mai una loro copia. Legge i file compose quando apri lo stack, e vi scrive solo quando scegli **Salva** nella sua scheda **Definizione**, sostituendo ogni file modificato in un solo passaggio e conservandone i permessi; non sovrascrive un file cambiato da quando lo hai aperto a meno che tu non lo dica. Homeport non legge né scrive mai il `.env` della cartella: lo legge Compose quando viene eseguito. Dimenticare uno stack collegato non tocca mai la cartella.

### Repository

Per gli [stack sincronizzati](/docs/guides/synced-stacks/), Homeport conserva l’URL di ogni repository, e una chiave di deploy che ha generato per quel repository (con la metà privata cifrata) oppure il tuo nome utente e un token di accesso (cifrato). Il contenuto di un repository non è un segreto: il suo clone si trova in una cartella `repositories` accanto a `homeport.db`.

### Swarm

Homeport non memorizza nulla su uno [Swarm](/docs/guides/swarm/): lo trova tramite i suoi host manager a ogni connessione. I valori dei segreti Swarm vanno allo Swarm e non vengono conservati da Homeport: un valore che digiti o carichi da un file viene inviato all’Engine una sola volta, mai scritto in `homeport.db`, in un file o nel log, e rimosso dalla memoria quando la finestra di dialogo si chiude. L’Engine non restituisce mai il valore di un segreto, quindi nemmeno Homeport può mostrarlo. I token di adesione vengono recuperati da un manager quando ti servono, e mai memorizzati né registrati nei log.

### Cluster

Le credenziali di un [cluster](/docs/clusters/) provengono dal tuo kubeconfig e vengono memorizzate solo cifrate, come le chiavi SSH; i file a cui il kubeconfig fa riferimento vengono letti una sola volta e incorporati, un kubeconfig incollato non viene mai scritto in un file, e il tuo kubeconfig non viene mai modificato. Un [plugin exec](/docs/clusters/exec-plugins/) viene eseguito su questo computer, come te, e il token o il certificato che stampa viene conservato solo in memoria, fino alla sua scadenza. Homeport non memorizza nient’altro su ciò che è in esecuzione su un cluster: Pod, carichi di lavoro e il resto vengono letti in tempo reale. Gli [inoltri di porta](/docs/clusters/port-forward/) sono in ascolto solo sul loopback di questo computer (`127.0.0.1` e `::1`), quindi nient’altro sulla tua rete può usarli, e terminano quando Homeport si chiude. Le modifiche fatte con [Applica YAML o Modifica](/docs/clusters/yaml/) vengono registrate sul cluster con il field manager `homeport`.

[Helm](/docs/clusters/helm/) esegue la tua CLI `helm` su un cluster senza il tuo kubeconfig e senza scrivere su disco alcuna credenziale del cluster: per ogni comando, Homeport apre una connessione privata sul loopback di questo computer (`127.0.0.1`, una porta casuale) che accetta solo richieste con un token casuale, e fornisce a helm un kubeconfig temporaneo, leggibile solo da te, che contiene quel token e nient’altro. Homeport aggiunge a ogni richiesta, al suo passaggio, le credenziali del cluster stesso, ed elimina il kubeconfig e chiude la connessione quando il comando termina. I valori dei chart vanno a helm sul suo standard input, mai in un file. I repository di chart, gli accessi ai registri e le cache di helm restano di helm, come in un terminale; Homeport aggiunge solo repository pubblici e non memorizza alcuna password di repository.

Il token ACL, il certificato CA, il certificato client e la chiave di un [cluster Nomad](/docs/clusters/nomad/) vengono memorizzati solo cifrati, come le chiavi SSH; il suo indirizzo, la regione e il nome del server TLS vengono conservati in chiaro. Il token viaggia nell’intestazione `X-Nomad-Token` di ogni richiesta, mai in un URL, compreso il WebSocket di una shell. Homeport non conserva alcuna copia della specifica di un job: la memorizza Nomad con il job.

**I valori dei Secret di Kubernetes** vengono recuperati solo quando scegli **Mostra** per una chiave, una alla volta: gli elenchi, la pagina di un Secret e il suo YAML contengono le sue chiavi e le loro dimensioni, mai i valori. Un valore mostrato resta nella memoria di quella pagina — non in una cache, non in `homeport.db`, non nel log — e viene nascosto di nuovo quando lasci la pagina.

## Come vengono cifrati i segreti

- Ogni segreto viene sigillato singolarmente con **XChaCha20-Poly1305**, un cifrario autenticato, così le manomissioni vengono rilevate oltre a impedire la lettura.
- La chiave a 256 bit è casuale, creata al primo avvio, e conservata nel **portachiavi del sistema operativo** — il Portachiavi di macOS, Gestione credenziali di Windows, o il Secret Service su Linux — sotto il servizio `com.homeport.app`. Non viene mai memorizzata accanto ai dati.
- Non c’è una password principale: il portachiavi lega già la chiave al tuo account utente.

Chi ha una copia del solo `homeport.db` può leggere i nomi e gli indirizzi degli host e dei cluster, ma non le tue chiavi, i certificati, i token, gli ambienti degli stack o le password dei registri.

### Senza un portachiavi

Homeport mostra un banner e non memorizza segreti. Gli host con socket locale, TCP e ssh-agent funzionano comunque. Vedi i [requisiti](/docs/requirements/#facoltativo-un-portachiavi-del-sistema-operativo).

### Se la chiave del portachiavi va persa

Reimpostare il portachiavi, o spostare `homeport.db` su un altro computer o utente, lascia segreti che non possono essere decifrati. Homeport non elimina nulla: gli host e i cluster interessati mostrano **Credenziali perse** finché non inserisci di nuovo le loro credenziali; [Se i segreti vanno persi](/docs/reference/data-and-backups/#se-i-segreti-vanno-persi) elenca cosa inserire di nuovo, e dove. Per spostare i tuoi dati, [esegui invece un backup](#backup) o [esporta i tuoi host](#passare-a-un-altro-computer).

Homeport non crea mai una nuova chiave sopra i segreti che ha già. Quando il portachiavi risponde ma non ha la voce `com.homeport.app` mentre il database contiene segreti cifrati — questa volta risponde un altro portachiavi, come il Secret Service di KeePassXC invece di GNOME Keyring —, Homeport non memorizza alcun segreto e segnala che **manca la voce del portachiavi con cui sono stati sigillati i segreti di Homeport**. Torna al portachiavi che contiene la voce e riavvia Homeport. Se la voce è sparita definitivamente, scegli **Ricomincia con una nuova chiave** in **Impostazioni → Questo computer**: i segreti sigillati con la vecchia chiave appaiono allora come **Credenziali perse**, da inserire di nuovo o da ripristinare da un backup.

### Passare a un altro computer

**Esporta** nella pagina Host scrive gli host che scegli in un unico file, con le loro chiavi, i certificati e le chiavi dei server considerate attendibili, cifrato con una passphrase che scegli tu:

- L’intero file è cifrato (XChaCha20-Poly1305, con la chiave derivata dalla passphrase tramite Argon2id), quindi senza la passphrase non rivela nulla — nemmeno i nomi o gli indirizzi degli host. Qualsiasi modifica al file lo rende illeggibile.
- La passphrase richiede almeno 12 caratteri. Homeport non la memorizza né la registra mai nei log; se la perdi, il file non può essere aperto.
- **Importa** sull’altro computer chiede la passphrase, poi memorizza i segreti di ogni host cifrati con la chiave del portachiavi di quel computer, come tutti gli altri.
- Un host le cui credenziali sono già perse su questo computer viene esportato senza di esse, e l’esportazione indica quale.

Tratta il file come le chiavi che contiene: eliminalo dopo l’importazione. Il formato è descritto nell’[ADR 0013](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0013-passphrase-protected-export-bundles.md).

### Backup

**Impostazioni → Dati → Esegui backup…** scrive tutto quanto sopra — host, cluster, stack, accessi ai registri, repository e impostazioni, con i loro segreti decifrati — in un unico file nello stesso formato cifrato, con una passphrase che digiti due volte. Gli eventi dell’Engine e la cronologia delle distribuzioni non sono inclusi. Un segreto che non può essere decifrato su questo computer viene escluso, e il backup indica quale.

**Ripristina…** chiede la passphrase ed elenca ciò che il file contiene prima di modificare qualsiasi cosa. Il ripristino poi:

1. copia il database attuale in `homeport.db.before-restore-<date and time>`, accanto a `homeport.db`;
2. sostituisce ogni host, cluster, stack, accesso, repository e impostazione in un solo passaggio, così un errore lascia i tuoi dati com’erano;
3. cifra di nuovo ogni segreto con la chiave del portachiavi di questo computer.

Un backup creato da un Homeport più recente viene rifiutato; uno creato da un Homeport più vecchio viene ripristinato. Il backup contiene ogni segreto di Homeport: conservalo, insieme alla sua passphrase, al sicuro quanto le chiavi stesse. [I tuoi dati e i backup](/docs/reference/data-and-backups/) contiene i passaggi, cosa include e cosa esclude un backup, e come copiare tu stesso il database.

## Le chiavi non toccano mai il disco in chiaro

Homeport ha un proprio client SSH, quindi non scrive mai una chiave privata in un file perché `ssh` la legga, e non modifica mai `~/.ssh`. Le chiavi decifrate restano in memoria.

Quando `docker compose` distribuisce uno stack su un host SSH o TLS, Homeport apre su questo computer un socket privato di breve durata — leggibile solo dal tuo utente, o una named pipe che rifiuta i client remoti su Windows — e lo inoltra tramite la propria connessione, per quel solo comando.

Nemmeno le chiavi e i token dei repository raggiungono mai il disco. Per ogni comando `git`, Homeport serve un agente SSH privato che contiene solo la chiave di deploy di quel repository — su un socket che solo il tuo utente può aprire, o una named pipe che rifiuta i client remoti su Windows — oppure passa il token in un’intestazione HTTPS tramite l’ambiente del comando. La tua configurazione git, i tuoi credential helper e le tue chiavi SSH vengono esclusi, quindi nulla viene messo in cache nel tuo portachiavi o in `~/.git-credentials`.

Nemmeno gli accessi ai registri raggiungono mai `docker compose`: Homeport scarica da sé le immagini di uno stack, tramite la Engine API, prima di eseguire `docker compose up`, quindi l’ambiente del comando e i file che riceve non contengono alcuna password né token.

## Fidarsi dei server

La prima volta che Homeport si connette a un server SSH, ti chiede di confermare l’impronta della chiave del server, e si rifiuta di connettersi se quella chiave cambia in seguito, finché non confermi di nuovo. Gli host TLS verificano il certificato del server rispetto alla tua CA e al nome del server; gli host Incus tramite HTTPS accettano solo il certificato con cui sono stati aggiunti. Vedi [Host SSH](/docs/hosts/ssh/#prima-connessione).

### Certificati Incus

Aggiungere un [host Incus tramite HTTPS](/docs/hosts/incus/#tramite-https-con-un-token-di-fiducia) crea in memoria un certificato client e la sua chiave privata, una chiave ECDSA P-384, e li memorizza cifrati con gli altri segreti dell’host, mai in chiaro su disco. Il certificato di Incus stesso viene memorizzato allo stesso modo e fissato: Homeport accetta dal server esattamente quel certificato, confrontato tramite la sua impronta SHA-256, e ne rifiuta uno diverso, mostrando entrambe le impronte. Il token di fiducia viene usato una volta e non conservato. Rimuovi il certificato di Homeport sul server con `incus config trust remove` per revocarlo.

## A cosa si connette Homeport

- **Gli host che aggiungi.** Quando scarichi, invii o compili un’immagine, è l’Engine a contattare il registro, non Homeport. Homeport gli passa con la richiesta l’accesso memorizzato per quel registro, tramite la connessione dell’host; l’Engine non lo conserva.
- **I registri delle tue immagini, tramite l’Engine di ogni host**, per sapere se è pubblicata un’[immagine più recente](/docs/guides/containers/#immagini-più-recenti): quando la pagina Container, Immagini o Stack viene caricata (ogni risposta viene conservata per 6 ore) e quando scegli **Cerca immagini più recenti**. L’Engine chiede al registro il digest a cui punta un tag, con l’accesso memorizzato per i registri privati, e non scarica nulla. Per un host locale, l’Engine viene eseguito sul tuo computer, quindi la richiesta parte da esso.
- **Docker Hub, quando sfogli i tag** nella finestra di download: Homeport elenca i tag di un repository da `hub.docker.com`, da questo computer, con il tuo accesso a Docker Hub quando ne è memorizzato uno. La ricerca dei repository passa invece per l’Engine dell’host.
- **Un registro, quando ne verifichi l’accesso** in **Impostazioni → Registri**: Homeport interroga l’endpoint `/v2/` del registro, e il suo servizio di token, da questo computer, inviando il nome utente e la password o il token tramite HTTPS (HTTP semplice solo per un registro su questo computer, come `localhost:5000`).
- **I server dei tuoi repository**, per gli [stack sincronizzati](/docs/guides/synced-stacks/): quando verifichi l’accesso, crei o sincronizzi uno stack sincronizzato, e a intervalli mentre Homeport è aperto se il polling è attivo. Homeport esegue `git` da questo computer con la chiave di deploy o il token di quel repository.
- **I cluster che aggiungi**, tramite il server API di Kubernetes indicato dal loro kubeconfig (e il suo proxy, quando ne ha uno), con le credenziali che fornisce, verificando il certificato del server rispetto alla CA del kubeconfig. I plugin exec contattano il proprio cloud (AWS, Google Cloud, Azure) da questo computer, come fanno per `kubectl`.
- **I cluster Nomad che aggiungi**, tramite l’indirizzo dell’agente che fornisci, e nient’altro: i log e le shell di un job passano per quell’agente, che li inoltra al client node, quindi Homeport non si connette mai ai nodi stessi. Il certificato dell’agente viene verificato rispetto alla CA che fornisci, o alle CA di questo computer.
- **I server di immagini Incus**, per i selettori di immagini su un [host Incus](/docs/guides/instances/#creare-unistanza): Homeport scarica l’elenco delle immagini di un server (`streams/v1/images.json`, da `images.linuxcontainers.org` a meno che tu non ne scelga un altro) da questo computer, e lo conserva per dieci minuti. Le immagini stesse vengono scaricate da Incus.
- **I repository di chart e i registri OCI, tramite helm**, quando aggiungi o aggiorni un repository, cerchi chart, o installi o aggiorni un chart: `helm` scarica indici e chart da questo computer, come fa in un terminale.
- **Questo sito, per controllare gli aggiornamenti**: all’avvio e ogni 6 ore, Homeport scarica `updates/latest.json`, un piccolo file che indica l’ultima versione. La richiesta non contiene nulla su di te, sui tuoi host o sui tuoi container, ma come ogni richiesta web raggiunge GitHub Pages, che ospita questo sito, dal tuo indirizzo IP. Disattivala in **Impostazioni → Aggiornamenti**. Gli aggiornamenti stessi vengono scaricati da GitHub Releases, e Homeport ne installa uno solo se è firmato con la chiave di aggiornamento di Homeport. Vedi [aggiornamenti](/docs/reference/updating/).

## Cosa esegue Homeport sui tuoi host

Homeport comunica con l’Engine di un host tramite la sua API, ed esegue comandi sul server di un host solo per ciò che quell’API non può fare:

| Quando                                                                                          | Dove                                                           | Comando                                                                                                                             |
| ----------------------------------------------------------------------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Il server di un host SSH si rifiuta di aprire il socket del suo Engine                          | il server SSH                                                  | un controllo in shell POSIX per sapere se il socket esiste ed è leggibile, e se è installato un Engine                              |
| **Avvia**, **Arresta** o **Riavvia** di un’[unità Quadlet](/docs/guides/quadlet/)               | il server SSH, o questo computer per un socket locale su Linux | `systemctl [--user] start\|stop\|restart '<unit>.service'`                                                                          |
| I [log di un container containerd](/docs/hosts/containerd/#log)                                 | il server SSH, o questo computer per un socket locale su Linux | `tail -n <lines> [-F] -- '<log file>'`, dopo un `ls` della cartella di log di un container Kubernetes quando CRI non indica il file |
| Un [terminale in un container containerd](/docs/hosts/containerd/#un-terminale-in-un-container) | il server SSH, o questo computer per un socket locale su Linux | `mktemp -d`, `mkfifo -m 600` per due FIFO in quella cartella, `cat` attraverso di esse, poi `rm -rf` della cartella                 |

Vengono eseguiti con i diritti dell’utente SSH (o i tuoi, su questo computer) e mai con `sudo`. I valori vengono controllati e messi tra virgolette prima di entrare in un comando. L’installazione, la modifica e la rimozione delle unità Quadlet passano per l’API di Podman, che scrive i file da sé. Su un host Incus non viene eseguito nulla: la sua console e i suoi terminali passano per l’API di Incus stesso. Gli unici file che Homeport crea su un host sono le due FIFO di un terminale containerd, in una cartella temporanea privata rimossa quando il terminale si chiude.

## Cosa Homeport non fa mai

- **Nessun account.** Non c’è nulla a cui iscriversi.
- **Nessuna telemetria, analisi o segnalazione degli arresti anomali.** Quando segnali un bug, il [pacchetto di diagnostica](/docs/reference/troubleshooting/#esportare-la-diagnostica-per-una-segnalazione-di-bug) è un file che esporti, leggi e alleghi tu stesso.
- **Nulla installato sui tuoi cluster.** Homeport usa solo l’API di Kubernetes, con i diritti delle tue credenziali, e non cambia nulla senza che tu lo chieda: tutto ciò che rimuove o sostituisce qualcosa chiede prima conferma, indicando il cluster.
- **Nulla installato sui tuoi host.** Gli unici comandi che Homeport esegue su un host sono [elencati sopra](#cosa-esegue-homeport-sui-tuoi-host). L’unico container che Homeport avvia di sua iniziativa è un [container ausiliario](/docs/guides/volumes-networks/#il-container-ausiliario) di breve durata quando apri, esegui il backup, ripristini o cloni un volume; non ha rete e viene rimosso al termine. La sua immagine viene scaricata solo dopo il tuo consenso.
- **Nessuna installazione silenziosa.** Un aggiornamento viene installato solo quando scegli **Installa e riavvia**; i pacchetti Linux vengono aggiornati solo dal tuo gestore di pacchetti.

L’[informativa sulla privacy](/privacy/) spiega cosa significa tutto ciò dal punto di vista legale: chi è responsabile dei tuoi dati, su quale base, e come esercitare i tuoi diritti.

## Log

Homeport scrive un piccolo file di log — qualche decina di kilobyte, sostituito quando è pieno — per diagnosticare i problemi. Contiene eventi di connessione ed errori, non le tue credenziali né l’output dei container.

| Sistema | Cartella                                |
| ------- | --------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` |
| macOS   | `~/Library/Logs/com.homeport.app/`      |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` |

Homeport non ha telemetria né segnalazione degli arresti anomali: un log non lascia mai il tuo computer a meno che non sia tu a inviarlo. Per una segnalazione di bug, **Impostazioni → Informazioni → Esporta diagnostica…** salva uno zip con un riepilogo delle versioni e questi log, dopo aver sostituito tutto ciò che ha l’aspetto di un segreto (chiavi, password, token, credenziali negli URL) e, salvo diversa indicazione, i nomi e gli indirizzi dei tuoi host e cluster. Vedi [Esportare la diagnostica](/docs/reference/troubleshooting/#esportare-la-diagnostica-per-una-segnalazione-di-bug).

## Fidarsi di un download

Le release vengono compilate da GitHub Actions a partire dal codice sorgente pubblico. Ogni file è accompagnato da un checksum SHA-256 e da un’attestazione firmata della provenienza della build che lo lega al commit e al workflow esatti che lo hanno compilato. Vedi [verificare i download](/docs/reference/verifying/).

## Segnalare una vulnerabilità

Segnala i problemi di sicurezza in privato tramite la [segnalazione privata delle vulnerabilità di GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new) invece che con una issue pubblica.
