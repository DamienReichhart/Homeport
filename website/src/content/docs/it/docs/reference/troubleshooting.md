---
title: Risoluzione dei problemi
description: Soluzioni ai problemi che è più probabile incontrare, in base al messaggio mostrato da Homeport.
---

Se il tuo problema non è qui, controlla il [file di log](/docs/reference/security/#logs) e [apri una issue](https://github.com/DamienReichhart/Homeport/issues) indicando la tua versione di Homeport da **Impostazioni**.

## Installazione e avvio

### Windows dice «Windows ha protetto il PC»

Il programma di installazione non è ancora firmato con un certificato commerciale. Scegli **Ulteriori informazioni**, poi **Esegui comunque**. Puoi prima [verificare il download](/docs/reference/verifying/).

### macOS dice che Homeport «non può essere aperto»

Homeport non è ancora autenticato da Apple. Apri **Impostazioni di Sistema → Privacy e sicurezza** e scegli **Apri comunque** accanto a Homeport, oppure esegui:

```sh
xattr -dr com.apple.quarantine /Applications/Homeport.app
```

### L’AppImage non si avvia

Se menziona FUSE, installa `libfuse2` (`libfuse2t64` su Ubuntu 24.04 e successivi). Controlla che il file sia eseguibile: `chmod +x Homeport_*.AppImage`.

### Homeport si apre nel posto sbagliato

Homeport si riapre con la dimensione, la posizione e lo stato massimizzato che la sua finestra aveva quando l’hai chiusa. Se lo schermo su cui si trovava non c’è più, si apre al centro di uno rimasto. Per dimenticare la posizione salvata, esci da Homeport ed elimina `.window-state.json` dalla sua cartella delle impostazioni:

| Sistema | File                                                                |
| ------- | ------------------------------------------------------------------- |
| Linux   | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%APPDATA%\com.homeport.app\.window-state.json`                     |

### Riaprire Homeport non apre una seconda finestra

Viene eseguito un solo Homeport alla volta: aprirlo mentre è già in esecuzione porta la sua finestra in primo piano, anche quando è nascosto nell’[area di notifica](/docs/guides/overview/#system-tray). Due copie si connetterebbero due volte a ogni host e scriverebbero negli stessi dati.

### Non c’è l’icona nell’area di notifica

Su Linux, l’area di notifica richiede la libreria appindicator, e alcuni desktop nascondono le icone dell’area di notifica: su GNOME, installa l’estensione AppIndicator Support. Se Homeport non trova alcuna libreria appindicator, **Impostazioni → Desktop** lo segnala e chiudere la finestra chiude l’app. Vedi i [requisiti](/docs/requirements/#optional-a-system-tray).

### Un banner rosso dice che non è disponibile alcun portachiavi del sistema operativo

Homeport non è riuscito a raggiungere il tuo portachiavi. Su Linux questo di solito significa che nessun Secret Service è in esecuzione — cosa comune sui desktop minimali e sui window manager. Installa e avvia GNOME Keyring (`gnome-keyring`) o attiva il Secret Service di KWallet, poi riavvia Homeport. Fino ad allora, usa host con socket locale, TCP o ssh-agent.

## Aggiornamenti e repository dei pacchetti

### «Impossibile cercare aggiornamenti»

Homeport non è riuscito a scaricare le informazioni sugli aggiornamenti da questo sito: potresti essere offline, o dietro un proxy o un firewall che blocca `damienreichhart.github.io`. Homeport continua a funzionare; controlla di nuovo più tardi, oppure scegli **Impostazioni → Aggiornamenti → Controlla ora**. Puoi sempre scaricare l’ultima versione dalla [pagina di download](/download/).

### Un aggiornamento non si installa

Homeport installa solo un aggiornamento la cui firma corrisponde alla sua chiave di aggiornamento, e si ferma se il download è incompleto o alterato. Riprova più tardi; se continua a non riuscire, scarica la nuova versione dalla [pagina di download](/download/) e installala sopra quella vecchia.

### apt dice «NO_PUBKEY» o «The repository is not signed»

Manca la chiave di Homeport in `/etc/apt/keyrings/homeport.asc`. Esegui di nuovo l’[installazione rapida](/docs/installation/#quick-install), o i primi due comandi della [configurazione manuale](/docs/installation/#manual-setup).

### dnf o zypper chiede se importare una chiave

È la chiave dei pacchetti di Homeport, la prima volta che il tuo gestore di pacchetti usa il repository. Controlla che l’impronta mostrata corrisponda a quella in [verificare i download](/docs/reference/verifying/#package-signatures), poi accetta.

### apt dice «Hash Sum mismatch» subito dopo una release

Il repository era in fase di aggiornamento mentre apt lo leggeva. Esegui di nuovo `sudo apt update` qualche minuto dopo.

### Il mio gestore di pacchetti non vede la nuova versione

Controlla che il repository sia ancora configurato: `/etc/apt/sources.list.d/homeport.sources`, `/etc/yum.repos.d/homeport.repo` o `/etc/zypp/repos.d/homeport.repo`. Se l’hai eliminato, esegui l’[installazione rapida](/docs/installation/#quick-install) per aggiungerlo di nuovo. Se hai installato la 0.2.0 o precedenti da un file scaricato, il repository non è mai stato aggiunto: vedi [aggiornare dalla 0.2.0](/docs/reference/updating/#updating-from-020-or-earlier).

## Connessione

### Non raggiungibile: Permission denied

Il tuo utente non può aprire il socket Docker. Su Linux, aggiungilo al gruppo `docker` e accedi di nuovo:

```sh
sudo usermod -aG docker $USER
```

### Non raggiungibile: No such file or directory

Nulla è in ascolto al percorso del socket. Avvia il tuo Engine — Docker Desktop, Colima, OrbStack, Rancher Desktop — o controlla il percorso. Vedi [Engine locali](/docs/hosts/local/#adding-a-socket-by-hand).

### «Il Docker Engine non ha risposto in tempo» o «… non ha risposto in tempo»

Homeport attende 10 secondi per una connessione. Controlla l’indirizzo e la porta, che il server sia acceso, e che nessun firewall sia d’intralcio. Homeport continua a riprovare da solo.

### Accesso negato: «Il server SSH non ha consentito l’accesso a …»

Il server non accetta ancora la chiave di Homeport. Esegui sul server il comando **Aggiungi la chiave pubblica** dalla pagina dell’host, poi scegli **Riconnetti**. Vedi [Host SSH](/docs/hosts/ssh/#authorise-homeports-key).

### «… non riesce ad aprire il socket del Docker Engine in /var/run/docker.sock …»

Homeport ha effettuato l’accesso, ma non è riuscito a raggiungere Docker. Controlla:

- il **Percorso del socket remoto** — Docker rootless usa `/run/user/<uid>/docker.sock`;
- che l’utente possa usare Docker: `sudo usermod -aG docker <user>` sul server;
- che il server consenta l’inoltro del socket (`AllowStreamLocalForwarding` in `sshd_config`, attivo per impostazione predefinita).

### «Nessun ssh-agent è in esecuzione.»

Avvia il tuo agente SSH e aggiungi la tua chiave con `ssh-add`. Se hai aperto Homeport da un menu del desktop, potrebbe non vedere l’agente usato dal tuo terminale; esci e accedi di nuovo, o avvia Homeport da un terminale in cui `ssh-add -l` elenca la tua chiave.

Su Windows, Homeport usa il servizio OpenSSH Authentication Agent oppure Pageant di PuTTY. Apri Pageant e carica la tua chiave, o avvia il servizio in un PowerShell da amministratore:

```powershell
Get-Service ssh-agent | Set-Service -StartupType Automatic
Start-Service ssh-agent
ssh-add
```

### Accesso negato: «La connessione sicura a … non è riuscita.»

- Il certificato CA deve essere quello che ha firmato il certificato del server.
- L’indirizzo che hai inserito deve corrispondere a un nome o a un indirizzo IP nel certificato del server.
- Il server deve essere eseguito con `--tlsverify` e fidarsi della CA del tuo certificato client.

### Chiave del server cambiata

Il server SSH ha presentato una chiave diversa da quella che avevi considerato attendibile. Se hai reinstallato il server o cambiato le sue chiavi, scegli **Considera attendibile la nuova chiave**. Se non sai perché è cambiata, non connetterti: chiedi a chi gestisce il server. Vedi [Host SSH](/docs/hosts/ssh/#when-the-servers-key-changes).

### Credenziali perse

La chiave che cifra i tuoi segreti manca dal portachiavi o è cambiata — dopo un ripristino del portachiavi, un nuovo profilo utente, o la copia dei dati di Homeport su un altro computer (per questo usa [Esporta e Importa](/docs/reference/security/#moving-to-another-computer)). Homeport conserva l’host; inserisci di nuovo le sue credenziali nella pagina dell’host:

- **TLS** — carica di nuovo i tre file dei certificati e scegli **Salva**.
- **SSH con una chiave importata** — incolla di nuovo la chiave privata e la passphrase e scegli **Salva**.
- **SSH con una chiave di Homeport** — scegli **Rigenera la chiave**, poi autorizza la nuova chiave sul server.

Anche gli ambienti degli stack gestiti sigillati con la vecchia chiave vanno inseriti di nuovo.

## Cluster

### Accesso negato: «… ha rifiutato le credenziali del suo kubeconfig.»

Il server API ha risposto 401: il certificato o il token è scaduto o è stato revocato. Controlla che `kubectl --context <context> get namespaces` funzioni ancora, poi usa **Aggiorna dal kubeconfig** nella pagina del cluster. Per un plugin exec, Homeport gli ha già chiesto una volta un nuovo token.

### «… non è installato su questo computer.»

Il [plugin exec](/docs/clusters/exec-plugins/) del cluster (`aws`, `gke-gcloud-auth-plugin`, `kubelogin`…) non è stato trovato nel `PATH` né nelle sue posizioni di installazione abituali. Installalo, controlla che funzioni in un terminale, poi scegli **Riconnetti**.

### «… richiede che tu effettui l’accesso, cosa che non può fare all’interno di Homeport.»

Il plugin exec vuole un browser o un codice dispositivo. Esegui una volta `kubectl --context <context> get namespaces` in un terminale, accedi lì, poi scegli **Riconnetti**.

### «… non è riuscito a ottenere le credenziali per …»

Il plugin exec è stato eseguito e non è riuscito. **Dettagli** mostra la parte finale di ciò che ha stampato, spesso un profilo scaduto o una regione sbagliata. Homeport riprova da solo.

### «Il suo utente accede con l’auth provider gcp, che Kubernetes non supporta più.»

Gli auth provider `gcp` e `azure` sono stati rimossi da Kubernetes. Recupera di nuovo le credenziali con `gke-gcloud-auth-plugin` installato (GKE), o esegui `kubelogin convert-kubeconfig` (AKS), poi aggiungi di nuovo il contesto.

### «Le tue credenziali non consentono «…» su … in …»

L’RBAC del cluster non consente all’utente del tuo kubeconfig di farlo. Scegli un namespace che hai il permesso di vedere nel selettore in alto a destra, o chiedi all’amministratore del cluster. Il messaggio fornisce il comando `kubectl auth can-i` che lo verifica. Le altre pagine continuano a funzionare.

### Non raggiungibile: «… ha rifiutato la connessione.» o «… non ha risposto in tempo.»

Il server API non è raggiungibile da questo computer: il cluster è arrestato (uno locale), dietro una VPN, o l’indirizzo del server nel kubeconfig è cambiato. Homeport continua a riprovare da solo; controlla `kubectl cluster-info` con lo stesso contesto.

### «Il certificato di … non è firmato dalla CA del suo kubeconfig.»

kind, k3d e minikube creano nuovi certificati ogni volta che ricreano un cluster. Usa **Aggiorna dal kubeconfig** nella pagina del cluster.

### Nomad: «… non conosce il token ACL inviato da Homeport.»

Il token è stato digitato male, è scaduto o è stato eliminato. Creane uno nuovo con la policy indicata in [Policy ACL](/docs/clusters/nomad/#acl-policies) (`nomad acl token create -name=homeport -policy=homeport`) e incolla il suo secret ID nelle **Impostazioni** del cluster. Il cluster resta in **Accesso negato** finché non lo fai.

### Nomad: «… ha rifiutato: al token ACL manca un permesso necessario per questa operazione.»

Le policy del token non concedono ciò di cui questa pagina o azione ha bisogno; il messaggio dice cosa (leggere job o log, eseguire o arrestare job, ridimensionare, aprire una shell, modificare nodi). Aggiungi la capability alla policy del token; [Policy ACL](/docs/clusters/nomad/#acl-policies) elenca ciò che serve a ciascuna. Le altre pagine continuano a funzionare.

### Nomad: «… non ha un leader: i suoi server non riescono a concordarne uno.»

Meno della maggioranza dei server del cluster è in esecuzione o riesce a raggiungersi, quindi Nomad non può rispondere. Controllali con `nomad server members` su un server, e riattivane abbastanza. Homeport continua a riprovare.

### Nomad: «… non ha alcun job …»

Il job è stato arrestato ed eliminato definitivamente, o rimosso dal garbage collector, oppure si trova in un namespace diverso da quello scelto. Apri di nuovo la pagina Job, con **Tutti i namespace** se necessario.

### Nomad: «Il task group … non può eseguirne più di …» o «meno di …»

Il blocco `scaling` del task group imposta un `min` e un `max`, e il numero richiesto è fuori da questi limiti. Cambia i limiti nella specifica del job ed eseguilo di nuovo per andare oltre.

### Nomad: «Nomad non ridimensiona … mentre è in corso un suo deployment.»

Eseguire una nuova versione di un job di servizio avvia un deployment, e Nomad si rifiuta di cambiarne il numero finché ogni allocation della nuova versione non è integra. Attendi che le allocation del job si stabilizzino, poi ridimensiona di nuovo.

### Nomad: «La specifica del job non può essere analizzata, alla riga …, colonna ….»

Il parser di Nomad ha trovato un problema in quel punto; **Dettagli** contiene il suo messaggio. I blocchi `variable` richiedono un valore predefinito, poiché Homeport non passa file di variabili. Una specifica JSON deve essere un job nel formato dell’API di Nomad, come lo stampa `nomad job inspect`.

### Nomad: «Nomad non riesce a collocare tutte le allocation di …»

Il piano non ha trovato alcun nodo idoneo con le risorse richieste dal task group, o che soddisfi i suoi vincoli; il piano dice quale. **Esegui comunque** mette in coda ciò che non può essere collocato finché un nodo non può accoglierlo; oppure riduci le risorse, cambia i vincoli o aggiungi nodi.

### Nomad: «… è cambiato dopo la pianificazione, quindi non è stato eseguito.»

Qualcun altro ha eseguito o arrestato il job tra il tuo piano e **Esegui**, e Homeport non sovrascrive la sua modifica. Torna indietro, pianifica di nuovo e controlla il nuovo diff prima di eseguire.

### Nomad: «Il task … dell’allocation … non è in esecuzione»

Una shell richiede un task in esecuzione: questo non è stato avviato, è terminato o non è riuscito. I suoi log restano leggibili finché l’allocation non viene rimossa dal garbage collector.

### Nomad: «… non riesce a raggiungere il nodo su cui è in esecuzione l’allocation ….»

Log e shell passano dai server al client node, e i server non riescono a raggiungerlo: il nodo è spento, disconnesso, o il suo agente si è arrestato. Controllalo nella pagina Nodi o con `nomad node status`.

### Helm: «La CLI helm non è installata su questo computer.»

Le [release Helm](/docs/clusters/helm/) richiedono `helm` 3 o successivo su questo computer. Installalo (`brew install helm`, `winget install Helm.Helm`, o `snap install helm --classic`), poi riapri la pagina Helm; **Impostazioni → Questo computer** mostra quello trovato da Homeport. Un `helm` 2 viene segnalato come non supportato.

### Helm: «Un’altra installazione, aggiornamento o rollback di questa release è ancora in corso.»

La release è `pending-…`: un’installazione, un aggiornamento o un rollback non è terminato, spesso perché è stato interrotto. helm si rifiuta di modificarla finché non termina. Attendi, oppure riporta la release all’ultima revisione distribuita dalla sua **Cronologia**.

## Usare Homeport

### Distribuisci è disattivato: «La CLI docker compose non è installata su questo computer.»

Homeport ha bisogno di `docker compose` su questo computer per distribuire gli stack. Installa Docker Desktop o la Docker CLI con il plugin Compose, poi riavvia Homeport. **Impostazioni → Questo computer** mostra se viene trovato.

Se è installato ma non viene trovato, Homeport potrebbe avere un `PATH` diverso da quello del tuo terminale — cosa comune quando viene avviato da un menu del desktop su macOS. Assicurati che `docker` si trovi in una posizione standard come `/usr/local/bin`.

### La distribuzione non riesce: «usa build:»

Uno stack gestito non ha una cartella da cui compilare. [Collega invece la cartella del progetto](/docs/guides/stacks/#creating-a-linked-stack) — uno stack collegato può compilare — oppure compila e invia l’immagine a un registro, poi usa `image:`.

### Il mio stack è diventato esterno

Uno [stack collegato](/docs/guides/stacks/#when-the-folder-moves-or-disappears) appare come esterno, contrassegnato con ⚠ **I suoi file compose non sono più su questo computer**, quando la sua cartella è stata spostata, rinominata o eliminata, o lo è stato un file compose al suo interno. Nulla va perso: rimetti la cartella dov’era e lo stack torna collegato al prossimo aggiornamento. Se la cartella è stata spostata definitivamente, scegli **Dimentica** nella scheda **Definizione** dello stack, poi collegalo di nuovo dalla sua nuova posizione con **Nuovo stack → Usa una cartella di questo computer**.

Uno stack avviato da una cartella su un altro computer è sempre esterno qui, perché questo computer non ha quel percorso.

### «Modificati fuori da Homeport»

I file compose di uno stack collegato sono stati modificati fuori da Homeport — nel tuo editor, da `git pull` — da quando Homeport li ha letti l’ultima volta. Scegli **Ricarica** per vederli come sono. Se stavi modificando in Homeport nello stesso momento, il salvataggio viene rifiutato con **… è cambiato fuori da Homeport da quando l’hai aperto** così nessuna delle due modifiche va persa in silenzio: ricarica e rifai la tua modifica, oppure scegli **Sovrascrivi** per conservare la tua. Vedi [modificare uno stack collegato](/docs/guides/stacks/#editing-a-linked-stack).

### Il download di un’immagine privata non riesce

Aggiungi un accesso per il registro dell’immagine in **Impostazioni → Registri**, e usa **Verifica l’accesso** per confermare che il registro lo accetta. GHCR, e gli account Docker Hub con l’autenticazione a due fattori, richiedono un token di accesso invece della password. La finestra di download indica quale accesso userà; se dice **in forma anonima**, il registro dell’immagine non corrisponde a nessun accesso memorizzato — `ghcr.io/org/app` richiede un accesso per `ghcr.io`, e `org/app` uno per Docker Hub.

### Una build non riesce

Il messaggio indica il passaggio che non è riuscito, come **La build non è riuscita al passaggio 3/5 (RUN make)**; **Dettagli** contiene la parte finale dell’output della build. Homeport usa il builder classico dell’Engine, quindi i Dockerfile che richiedono BuildKit (`RUN --mount`, `--secret`, heredoc) non riescono; compilali da un terminale. Un’immagine `FROM` privata richiede un accesso per il suo registro in **Impostazioni → Registri**.

### Non compare alcun segno di immagine più recente

Il segno compare solo quando il registro è stato interrogato e ha risposto. Le immagini compilate in locale o scaricate tramite digest non ne mostrano mai uno. Se un registro non è raggiungibile o limita le richieste (Docker Hub conta questi controlli nel suo limite di download), Homeport scrive una riga nel suo [log](/docs/reference/security/#logs) e non mostra nulla; le risposte vengono inoltre conservate per 6 ore, quindi scegli **Cerca immagini più recenti** per chiedere di nuovo subito. Un accesso a Docker Hub in **Impostazioni → Registri** alza il limite.

### La ricerca su Docker Hub mostra un errore

La ricerca viene eseguita sull’host selezionato, quindi richiede l’accesso a internet di quell’host, e i tag vengono letti da `hub.docker.com` da questo computer. Entrambi possono essere soggetti a limiti di richieste o bloccati da un proxy. La finestra di dialogo scarica comunque qualsiasi riferimento completo che digiti, come `postgres:17-alpine`.

### La scheda File dice che il container non ha gli strumenti find e stat

Le immagini minimali e distroless non includono `find` e `stat`, che Homeport usa per elencare i file. Usa `docker cp` per quei container.

### Il terminale non si apre

Il container deve essere in esecuzione, e avere `/bin/sh` o `/bin/bash`. Usa **Nuovo terminale** per eseguire un programma diverso.

### Un volume o una rete non si elimina

L’Engine lo rifiuta finché un container — anche uno arrestato — lo usa ancora. Rimuovi prima quei container; la conferma li elenca.

### «Richiede Podman 5.8 o successivo»

L’API di Podman legge, installa, modifica e rimuove le [unità Quadlet](/docs/guides/quadlet/) da Podman 5.8, e le elenca dalla 5.7. Aggiorna Podman sull’host, o usa `podman quadlet` in un terminale lì. Avviare e arrestare le unità funziona con ogni Podman.

### Quadlet: Avvia e Arresta sono disattivati

Homeport avvia e arresta il servizio di un’unità con `systemctl`, che può eseguire solo tramite SSH o per un Podman su questo computer (su Linux). Raggiungi l’host tramite SSH invece di TCP o TLS. Un’unità il cui servizio mostra **non ancora generato** non ha nulla da avviare: il generatore di systemd ha saltato il suo file.

### Quadlet: «Il systemd dell’utente non è in esecuzione sull’host»

Le unità utente (Podman rootless) richiedono l’istanza systemd dell’utente, che viene eseguita solo mentre l’utente ha effettuato l’accesso, a meno che il lingering non sia attivo. Come quell’utente sull’host, esegui `loginctl enable-linger "$USER"`, poi riprova.

### Quadlet: «systemd non ha consentito a questo utente di gestire …»

Le unità di sistema (Podman rootful) possono essere avviate e arrestate solo da un utente autorizzato a gestirle, come `root`, e Homeport non usa mai `sudo`. Connettiti come un utente di questo tipo, o usa Podman rootless, le cui unità sono dell’utente stesso.

### containerd: «non ha il permesso di usare containerd in /run/containerd/containerd.sock»

Il socket di containerd appartiene a root. Dai a un gruppo l’accesso con `gid` sotto `[grpc]` in `/etc/containerd/config.toml`, aggiungi il tuo utente (o l’utente SSH) a quel gruppo, riavvia containerd e accedi di nuovo; [la pagina su containerd](/docs/hosts/containerd/#letting-your-user-use-the-socket) contiene i comandi. containerd rootless non può essere raggiunto affatto: il suo socket esiste solo all’interno del namespace di RootlessKit.

### containerd: «L’output di questo container non viene registrato»

containerd non conserva log; lo strumento che ha avviato il task decide dove va il suo output. I container creati con `ctr` di solito non lo mandano da nessuna parte. I container di nerdctl e di Kubernetes scrivono i log in file che Homeport legge; vedi [Log](/docs/hosts/containerd/#logs).

### containerd: i log o il terminale dicono che richiedono SSH

Richiedono i file del computer di containerd, che Homeport raggiunge tramite SSH, o su questo computer per un socket locale su Linux. Aggiungi l’host tramite SSH. Se il file di log non può essere letto, di solito appartiene a root: connettiti come un utente autorizzato a leggerlo.

### containerd: «non può scaricare immagini per Homeport»

Il download passa per il servizio Transfer di containerd, stabile da containerd 2.0. Aggiorna containerd, o scarica con `ctr images pull` o `nerdctl pull` sull’host; l’immagine compare poi nella pagina Immagini.

### Incus: «non ha il permesso di usare Incus in /var/lib/incus/unix.socket»

Il socket di amministrazione di Incus è per il gruppo `incus-admin`, e quello limitato `unix.socket.user` per il gruppo `incus`. Aggiungi il tuo utente (o l’utente SSH) con `sudo usermod -aG incus-admin "$USER"`, poi esci e accedi di nuovo, o riconnettiti tramite SSH. Vedi [i socket di Incus](/docs/hosts/incus/#its-sockets).

### Incus: «Homeport non è riuscito a raggiungere Incus su …» quando aggiungi con un token di fiducia

Incus non è in ascolto sulla rete finché non viene impostato `core.https_address`. Sul server, esegui `incus config set core.https_address :8443`, controlla che un firewall lasci passare la porta 8443, e crea un nuovo token: il token elenca gli indirizzi su cui Incus era in ascolto quando è stato creato.

### Incus: «Incus ha rifiutato il token di fiducia.»

Un token funziona una sola volta, e scade. Creane uno nuovo sul server con `incus config trust add homeport` e incollalo entro la sua validità.

### Incus: «… ha presentato un certificato con impronta …, non quella attendibile …»

Il certificato del server non è quello che Homeport ha considerato attendibile: Incus è stato reinstallato, il suo certificato è stato rinnovato, o qualcos’altro risponde a quell’indirizzo. Controlla l’impronta sul server con `incus info`; se è quella mostrata, rimuovi l’host e aggiungilo di nuovo con un nuovo token. Vedi [certificati fissati](/docs/hosts/incus/#the-servers-certificate-is-pinned).

### Incus: «Questo host non può eseguire macchine virtuali.»

Incus richiede KVM (`/dev/kvm`) e QEMU per le macchine virtuali. Attiva la virtualizzazione nel firmware del server (o la virtualizzazione annidata per un server virtuale), oppure crea invece un container di sistema.

### Incus: «L’agente Incus della macchina virtuale non è in esecuzione»

L’esecuzione di comandi in una macchina virtuale passa per l’`incus-agent` al suo interno, che le immagini di Incus avviano all’accensione. Attendi la fine dell’avvio, o usa la scheda **Console**, che non richiede alcun agente.

### «… non è connesso.»

L’azione richiedeva l’host, ma si è disconnesso. Attendi che si riconnetta, o scegli **Riprova ora**.
