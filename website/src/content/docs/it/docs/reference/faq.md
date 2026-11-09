---
title: Domande frequenti
description: Domande comuni su Homeport.
---

### Homeport è gratuito?

Sì. Homeport è gratuito, per uso personale e commerciale, e open source sotto la [GNU Affero General Public License, versione 3](https://github.com/DamienReichhart/Homeport/blob/main/LICENSE) (AGPL).

### Cosa significa l’AGPL per me?

Non cambia nulla se usi Homeport: installalo, eseguilo a casa o al lavoro, su tutti i computer che vuoi. L’AGPL chiede qualcosa solo a chi **distribuisce** Homeport, modificato o no, o offre ad altri una versione modificata tramite una rete: deve pubblicarne il codice sorgente completo sotto la stessa licenza. Ogni copia di Homeport rimanda al codice sorgente della sua versione esatta in **Impostazioni → Licenze**.

Le versioni fino alla 0.11 sono state rilasciate sotto la licenza MIT e restano disponibili sotto di essa.

### Homeport sostituisce Docker Desktop?

Non del tutto. Homeport è un gestore: non esegue un Docker Engine da sé. Su Linux, installa Docker Engine e usa Homeport come sua interfaccia. Su macOS e Windows ti serve comunque qualcosa che esegua l’Engine — Docker Desktop, Colima, OrbStack, Rancher Desktop o Podman — e Homeport può funzionare sopra uno qualsiasi di essi, insieme ai tuoi server remoti.

### Funziona con containerd?

Sì, direttamente, senza Docker: l’Engine di un host può essere [containerd](/docs/hosts/containerd/), raggiunto tramite il suo socket su questo computer o tramite SSH — quello di k3s, quello del Docker Engine stesso, o il servizio containerd su un server. Homeport elenca i suoi namespace, i suoi container e i loro task (avvio, arresto, pausa, segnale, rimozione), i loro log dove li hanno scritti nerdctl o Kubernetes, un terminale al loro interno, le sue immagini (download, rimozione) e i suoi snapshot. Non crea container né distribuisce stack lì; lo fa nerdctl.

### Funziona con Incus?

Sì: l’Engine di un host può essere [Incus](/docs/hosts/incus/) 6.0 o successivo, raggiunto tramite il suo socket su questo computer o tramite SSH, o tramite HTTPS con un token di fiducia di `incus config trust add`. Homeport elenca le [istanze](/docs/guides/instances/) di un progetto, container di sistema e macchine virtuali, le avvia, arresta, congela, elimina e crea, apre la loro console e un terminale al loro interno, crea e ripristina snapshot, e gestisce immagini, profili, pool di archiviazione e reti. Non distribuisce stack lì, e non installa nulla sull’host.

### Funziona con LXD?

Non in quanto tale: Homeport parla Incus, il fork comunitario di LXD, e rifiuta un server LXD, la cui API si è discostata. Migralo con `lxd-to-incus`, poi aggiungilo come host Incus.

### Funziona con Podman?

Sì, tramite il socket dell’API compatibile con Docker di Podman. Homeport rileva i socket Podman abituali su Linux, ogni [macchina Podman in esecuzione su macOS](/docs/hosts/local/#macchine-podman-su-macos), e la pipe della macchina Podman predefinita su Windows. Qualsiasi altro può essere [aggiunto a mano](/docs/hosts/local/#aggiungere-un-socket-a-mano).

Homeport riconosce anche Podman quando si connette, e offre allora ciò che solo Podman ha, tramite l’API propria di Podman sullo stesso socket: i [pod](/docs/guides/podman-pods/), che puoi elencare, avviare, arrestare, creare e rimuovere, ed eseguirvi container dalla finestra Esegui; e le [unità Quadlet](/docs/guides/quadlet/), che puoi elencare, leggere, installare, modificare e rimuovere, e avviare o arrestare tramite SSH.

### Funziona con Kubernetes, Nomad o Docker Swarm?

Docker Swarm, sì. Homeport trova uno Swarm tramite i suoi host manager, non appena uno di essi è un host, e lo gestisce completamente: i suoi nodi, servizi, task, segreti, config e stack swarm, compresi gli stack gestiti, collegati e sincronizzati distribuiti su di esso. Può anche avviare uno Swarm su un host, e unirsi a uno o lasciarlo. Tutto passa per la Engine API di un manager connesso; nulla viene installato sui nodi. Vedi [Swarm](/docs/guides/swarm/).

Kubernetes, sì. Aggiungi un cluster dal tuo kubeconfig — quello di questo computer o uno che incolli — e Homeport trova per te quelli locali (Docker Desktop, OrbStack, Rancher Desktop, kind, k3d, minikube). EKS, GKE e AKS funzionano tramite il plugin exec indicato dal loro kubeconfig. Puoi esplorare Namespace, carichi di lavoro, Pod, Service, Ingress, ConfigMap, Secret, Node, Event e qualsiasi altra risorsa API, comprese quelle personalizzate, in tempo reale; seguire i log, aprire un terminale in un Pod, inoltrare porte, ridimensionare, riavviare ed eliminare, e applicare o modificare YAML con un’anteprima. Nulla viene installato sul cluster: Homeport comunica solo con l’API di Kubernetes che il tuo kubeconfig raggiunge già, con i diritti delle sue credenziali. Con la CLI `helm` su questo computer, Homeport elenca, installa, aggiorna, ripristina e disinstalla anche le [release Helm](/docs/clusters/helm/), e gestisce i repository di chart di helm. Vedi [Cluster](/docs/clusters/).

Nomad, sì. Aggiungi un cluster tramite l’indirizzo del suo agente, con un token ACL e il materiale TLS quando li usa. Puoi elencare i job e le loro allocation, in tempo reale; eseguire un job da HCL o JSON dopo aver visto il piano di Nomad; modificarne e rieseguirne uno, arrestarlo o eliminarlo definitivamente, e ridimensionarne i task group; seguire i log di un task e aprirvi una shell; svuotare i nodi e cambiarne l’idoneità; e scegliere un namespace. Nulla viene installato sul cluster e non serve la CLI `nomad`: Homeport comunica con l’API HTTP di Nomad con i diritti del token. Vedi [cluster Nomad](/docs/clusters/nomad/).

### Mi serve kubectl?

No. Homeport legge il tuo kubeconfig e comunica da sé con l’API di Kubernetes. Gli unici programmi che esegue sono i [plugin exec](/docs/clusters/exec-plugins/) indicati da un kubeconfig, come l’AWS CLI per EKS, `gke-gcloud-auth-plugin` per GKE o `kubelogin` per AKS — gli stessi che eseguirebbe `kubectl`. Avere `kubectl` a disposizione aiuta comunque a verificare cosa possono fare le tue credenziali (`kubectl auth can-i`).

### Devo installare qualcosa sui miei server?

No — solo un Docker Engine e un server SSH, che molto probabilmente hai già. Homeport non installa agenti.

Neanche uno Swarm richiede altro oltre all’Engine. I suoi nodi devono però potersi raggiungere tra loro, cosa che Homeport non può configurare per te: apri 2377/tcp (verso i manager), 7946/tcp e 7946/udp, e 4789/udp tra di essi. Vedi [porte tra i nodi](/docs/guides/swarm/#porte-tra-i-nodi).

### Mi serve Docker installato sul computer per gestire server remoti?

No. Homeport parla da sé la Engine API e SSH. Le uniche eccezioni sono la distribuzione di stack gestiti, collegati e sincronizzati, che usa il comando `docker compose` sul tuo computer, e la loro distribuzione su uno Swarm, che usa il comando `docker` (`docker stack deploy`). Nessuno dei due richiede un Engine sul tuo computer.

### Homeport mantiene sincronizzati i miei stack quando è chiuso?

No. Gli [stack sincronizzati](/docs/guides/synced-stacks/) vengono controllati per nuovi commit, e distribuiti automaticamente se lo attivi, solo mentre Homeport è aperto. Homeport non installa nulla sui tuoi server — nessun agente, nessun ricevitore di webhook — quindi quando è chiuso, nulla sorveglia i tuoi repository. Quando lo riapri, attende un intervallo, poi li controlla tutti. Per le distribuzioni che devono seguire ogni push a qualsiasi ora, eseguile dalla tua CI.

Per tenere Homeport in esecuzione senza la sua finestra, attiva **Chiudi nell’area di notifica**, e **Avvia all’accesso** con **Avvia nascosto nell’area di notifica**, in [Impostazioni → Desktop](/docs/guides/settings/#desktop).

### Homeport può avvisarmi quando un container si blocca?

Sì, mentre è in esecuzione — anche nascosto nell’area di notifica. Mostra una notifica del sistema operativo quando un container si arresta con un errore, esaurisce la memoria o diventa non integro, e quando un host diventa non raggiungibile o torna disponibile. Ogni tipo può essere disattivato in [Impostazioni → Notifiche](/docs/guides/settings/#notifiche). Lo apprende dal flusso di eventi che mantiene già con ogni host, quindi nulla viene installato sui tuoi server; quando Homeport non è in esecuzione, nessuno viene avvisato.

### Homeport ha bisogno di git?

Solo per gli [stack sincronizzati](/docs/guides/synced-stacks/), che eseguono il `git` installato sul tuo computer, con la chiave di deploy o il token di ciascun repository. **Impostazioni → Questo computer** indica se Homeport l’ha trovato. La tua configurazione git, i tuoi credential helper e le tue chiavi SSH non vengono mai usati.

### Più persone possono condividere gli host di Homeport?

Ogni persona esegue il proprio Homeport con i propri host e le proprie chiavi. Con una chiave creata da Homeport per persona e per server, puoi vedere e revocare l’accesso di ciascuno in `authorized_keys`.

### Posso spostare Homeport su un altro computer?

Sì, in due modi:

- **Tutto**: **Impostazioni → Dati → Esegui backup…** salva ogni host, cluster, stack, accesso al registro, repository e impostazione, con i loro segreti, in un unico file protetto da una passphrase di almeno 12 caratteri. Sul nuovo computer, **Ripristina…** nello stesso punto lo apre, elenca ciò che contiene e sostituisce ciò che c’è. Tutto si connette e si distribuisce senza chiederti di nuovo una chiave o una password. Vedi [backup e ripristino](/docs/guides/settings/#backup-e-ripristino).
- **Alcuni host**: nella pagina **Host**, **Esporta** gli host che scegli, e **Importa**li sull’altro computer, accanto agli host che ha già. Vedi [passare a un altro computer](/docs/reference/security/#passare-a-un-altro-computer).

Copiare `homeport.db` non funziona per i segreti: sono cifrati con una chiave conservata nel portachiavi di questo computer, e altrove appaiono come **Credenziali perse**.

### Un backup include i miei container e volumi?

No. Un backup contiene ciò che Homeport conserva — come raggiungere i tuoi host, le definizioni e gli ambienti dei tuoi stack, gli accessi e le impostazioni — non ciò che è in esecuzione sui tuoi host. I tuoi container, immagini e volumi restano sugli host. Per copiare i dati di un volume, [esegui il backup in un file](/docs/guides/volumes-networks/#backup-e-ripristino) dalla pagina Volumi.

### Homeport può compilare immagini?

Sì, in due punti. La pagina **Immagini** compila un’immagine da una cartella di questo computer. E uno [stack collegato](/docs/guides/stacks/#stack-collegati) — uno stack i cui file compose sono in una cartella di questo computer — può usare `build:`: Homeport esegue `docker compose` in quella cartella, quindi il contesto di build viene inviato da lì all’Engine dell’host, che lo compila. **Ricompila e distribuisci** ricompila dopo una modifica al codice. Uno stack gestito non può compilare, perché Homeport ne conserva solo lo YAML e non c’è una cartella da cui compilare: collega invece la cartella, o invia l’immagine a un registro.

### Perché il mio stack appare come esterno sull’altro mio computer?

Uno stack avviato con `docker compose up` in una cartella appare come [collegato](/docs/guides/stacks/#come-vengono-trovati-gli-stack-collegati) solo su un computer dove quella cartella e i suoi file compose esistono, allo stesso percorso, e definiscono i servizi in esecuzione. Su un altro computer il percorso non esiste, quindi Homeport non ha un file compose per esso e lo mostra come esterno. Per gestirlo anche da lì, clona il progetto su quel computer e collegalo con **Nuovo stack → Usa una cartella di questo computer**, o adottalo come stack gestito.

### Homeport raccoglie dati su di me?

No. Non ci sono account, telemetria o analisi. L’unica richiesta che Homeport fa di sua iniziativa dal tuo computer è il controllo degli aggiornamenti, che scarica un piccolo file da questo sito e non invia nulla su di te; puoi disattivarlo in **Impostazioni → Aggiornamenti**. Separatamente, il controllo delle immagini più recenti chiede ai registri delle tue immagini, tramite l’Engine di ogni host, a quale versione puntano i loro tag; per un host locale anche quella richiesta parte dal tuo computer. Vedi [la sicurezza e i tuoi dati](/docs/reference/security/#a-cosa-si-connette-homeport).

### Homeport può usare i miei registri privati?

Sì. Aggiungi un accesso per ogni registro in **Impostazioni → Registri** — Docker Hub, GHCR o il tuo. Homeport lo passa all’Engine per scaricare, inviare e compilare su qualsiasi host, e lo usa per scaricare le immagini di uno stack gestito, collegato o sincronizzato prima di distribuirlo. Password e token sono cifrati con il portachiavi del tuo sistema operativo. Homeport non legge gli accessi di `docker login` su questo computer.

### Come fa Homeport a sapere che esiste un’immagine più recente?

Chiede al registro dell’immagine, tramite l’Engine dell’host, a quale digest punta ora il tag, e lo confronta con quello scaricato dall’host. Non viene scaricato nulla, ma Docker Hub conta ogni domanda nel suo limite di download, come un `docker pull` che non trova nulla di nuovo. Per questo Homeport conserva ogni risposta per 6 ore, interroga al massimo quattro registri alla volta, e chiede di nuovo prima solo quando scegli **Cerca immagini più recenti**. Un accesso a Docker Hub in **Impostazioni → Registri** alza il limite. Se un registro non è raggiungibile o sta limitando le richieste, Homeport non mostra alcun segno per le sue immagini e scrive una riga nel suo log.

### Come aggiorno?

Su Windows, macOS e con l’AppImage, Homeport ti avvisa quando esce una nuova versione: scegli **Installa e riavvia**. Su Linux, i pacchetti `.deb` e `.rpm` si aggiornano insieme al resto del sistema dal repository dei pacchetti di Homeport. Vedi [aggiornamenti](/docs/reference/updating/).

### Homeport è su Homebrew, winget, Flathub o AUR?

Non ancora. Sono previsti; la [pagina di installazione](/docs/installation/#in-arrivo) rimanda alla issue di ciascuno.

### Homeport è disponibile nella mia lingua?

Homeport parla inglese, francese, spagnolo, tedesco, italiano, portoghese brasiliano, russo, cinese semplificato, giapponese, coreano, hindi e arabo, e segue la lingua del tuo sistema operativo a meno che tu non ne scelga una in [Impostazioni → Aspetto](/docs/guides/settings/#aspetto). Anche il sito è disponibile in tutte queste lingue; la documentazione è interamente tradotta in francese, e le altre lingue mostrano la pagina inglese dove una traduzione non esiste ancora. Per aggiungere una lingua, vedi [Contribuire](/docs/project/contributing/#tradurre-homeport).

### Homeport è accessibile?

Può essere usato con la sola tastiera, segue l’impostazione di movimento ridotto del sistema, rispetta il contrasto WCAG 2.2 AA, e ogni pagina viene controllata con strumenti automatici di accessibilità. È in corso una verifica pratica con screen reader. Vedi [Tastiera e accessibilità](/docs/reference/accessibility/) per cosa funziona, i limiti noti e come segnalare una barriera.

### Esiste un’interfaccia da riga di comando?

No — Homeport è un’app desktop. La tua CLI `docker` esistente funziona accanto ad essa, e Homeport si aggiorna in tempo reale quando cambi qualcosa dal terminale.

### Come posso aiutare?

Segnala bug e suggerisci funzioni nelle [issue di GitHub](https://github.com/DamienReichhart/Homeport/issues), o vedi [contribuire](/docs/project/contributing/).
