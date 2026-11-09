---
title: Glossario
description: Le parole che Homeport usa, e cosa significano.
---

Homeport chiama le cose con precisione. Questi termini compaiono in tutta l’app e in questa documentazione. Ogni voce riporta il termine inglese del prodotto, seguito dalla parola che l’app usa in italiano.

### Host (it. _host_)

Un Docker Engine a cui Homeport si connette, definito da un nome e da un modo per raggiungerlo: un socket locale, TCP, TLS o SSH. Ogni container, immagine, volume, rete e stack appartiene esattamente a un host. Vedi [Host e connessioni](/docs/hosts/).

### Engine kind (it. _tipo di Engine_)

Il tipo di Engine a cui punta un host: compatibile con Docker (Docker Engine, o Podman tramite la sua API compatibile), containerd o Incus. Decide quali pagine mostra un host: gli host containerd e Incus hanno le loro. Podman non è un tipo a sé: Homeport lo riconosce a ogni connessione. Vedi [Tipi di Engine](/docs/hosts/#engine-kinds).

### Namespace (containerd’s) (it. _namespace containerd_)

Su un host containerd, lo spazio in cui vive ogni container, immagine e snapshot: nerdctl usa `default`, Kubernetes `k8s.io`, Docker `moby`. Le pagine di un host containerd ne mostrano uno alla volta. I Namespace di un cluster sono quelli di Kubernetes. Vedi [namespace di containerd](/docs/hosts/containerd/#namespaces).

### Snapshot (it. _snapshot_)

Su un host containerd, un layer di file su disco: un layer di un’immagine decompressa, o il layer scrivibile di un container. Vedi [snapshot](/docs/hosts/containerd/#snapshots). Su un host Incus, uno stato salvato di un’istanza a cui può essere ripristinata; vedi [snapshot delle istanze](/docs/guides/instances/#snapshots).

### Instance (it. _istanza_)

Ciò che esegue un host Incus: un container di sistema, che avvia un’intera distribuzione Linux sul kernel dell’host, o una macchina virtuale con un proprio kernel. Non un container, che esegue una sola applicazione. Vedi [Istanze](/docs/guides/instances/).

### Project (Incus’s) (it. _progetto_)

Su un host Incus, lo spazio in cui vivono le sue istanze, immagini, profili, volumi personalizzati e reti. Le pagine di un host Incus ne mostrano uno alla volta, scelto sotto il selettore degli host. Vedi [progetti](/docs/hosts/incus/#projects).

### Profile (it. _profilo_)

Su un host Incus, configurazione e dispositivi condivisi dalle istanze che lo elencano. Vedi [profili](/docs/guides/instances/#profiles).

### Trust token (it. _token di fiducia_)

Ciò che `incus config trust add` stampa su un server Incus, incollato una volta in Homeport per aggiungere quel server tramite HTTPS. Vedi [token di fiducia](/docs/hosts/incus/#over-https-with-a-trust-token).

### Pod (it. _pod Podman_)

Su un host Podman, un gruppo di container che condividono una rete e si avviano e si arrestano insieme, attorno a un container infra che tiene le porte pubblicate dal pod. I Pod di un cluster sono Pod di Kubernetes. Vedi [pod Podman](/docs/guides/podman-pods/).

### Quadlet unit (it. _unità Quadlet_)

Un file su un host Podman, come `web.container`, che systemd trasforma in un servizio, `web.service`, così i container di Podman si avviano all’accensione e systemd li mantiene in esecuzione. Vedi [unità Quadlet](/docs/guides/quadlet/).

### Cluster (it. _cluster_)

Un cluster Kubernetes o un cluster Nomad a cui Homeport si connette tramite la sua API. Uno Kubernetes si aggiunge da un contesto di un kubeconfig: un nome, l’indirizzo del server API e le credenziali fornite dal kubeconfig, memorizzate cifrate. Uno Nomad si aggiunge tramite l’indirizzo di uno dei suoi agenti, con un token ACL e il materiale TLS, memorizzati cifrati. Homeport non installa nulla su nessuno dei due e raggiunge solo ciò che quelle credenziali consentono. Non uno Swarm, che si trova tramite i suoi host e non si aggiunge mai. Gli oggetti propri di Kubernetes mantengono i loro nomi Kubernetes: Namespace, Pod, Deployment, Service, Node… Quelli di Nomad mantengono i nomi di Nomad: job, task group, task, allocation, client node, namespace. Vedi [Cluster](/docs/clusters/) e [cluster Nomad](/docs/clusters/nomad/).

### Allocation (it. _allocation_)

Su un cluster Nomad, un posizionamento del task group di un job su un client node, che esegue i suoi task. I suoi log e una shell al suo interno passano dai server di Nomad al nodo. Vedi [log e shell](/docs/clusters/nomad/#logs-and-shell).

### Swarm (it. _Swarm_)

Un cluster Docker Swarm, trovato tramite gli host che ne fanno parte e mai aggiunto da solo. Homeport lo raggiunge tramite qualsiasi host manager connesso, distingue gli Swarm tramite il loro ID swarm, e non memorizza nulla su di essi. Prende il nome dal suo primo host manager, poiché Docker dà lo stesso nome a ogni Swarm. Vedi [Swarm](/docs/guides/swarm/).

### Port forward (it. _inoltro di porta_)

Una porta sul loopback di questo computer che Homeport collega, tramite l’API di Kubernetes, a una porta di un Pod, o di un Service tramite uno dei suoi Pod pronti, come `kubectl port-forward`. Termina quando Homeport si chiude. Vedi [Inoltro di porte](/docs/clusters/port-forward/).

### Release (it. _release_)

Una release Helm: un’installazione di un chart su un cluster, con un nome nel suo Namespace, con una revisione numerata per ogni installazione, aggiornamento e rollback. Helm la conserva sul cluster; una release creata in un terminale e una creata in Homeport sono la stessa cosa. Vedi [Release Helm](/docs/clusters/helm/).

### Chart repository (it. _repository di chart_)

Un server da cui helm scarica i chart, nell’elenco di helm su questo computer — quello mostrato da `helm repo list`. I chart in un registro OCI si installano invece tramite il loro riferimento `oci://`. Vedi [repository di chart](/docs/clusters/helm/#chart-repositories).

### Node (it. _nodo_)

Un Engine in uno Swarm: un manager o un worker. I Node di un cluster sono Node di Kubernetes, le macchine su cui esegue i Pod. Un nodo è un host solo quando Homeport vi si connette, e la pagina di un host rimanda al suo nodo.

### Service (it. _servizio_)

Ciò che esegue uno Swarm: un’immagine, le sue impostazioni e un numero di repliche, o un task su ogni nodo per un servizio globale.

### Task (it. _task_)

Uno slot di un servizio su un nodo, eseguito lì come container. Quando un servizio non converge, i suoi task dicono perché. Su un host containerd, un task è ciò che viene eseguito per un container, che è solo metadati: avviare un container crea il suo task.

### Stack (it. _stack_)

Un progetto Compose in esecuzione su un host, o uno stack swarm in esecuzione su uno Swarm, identificato dal nome del suo progetto. Ogni stack è gestito, collegato, sincronizzato o esterno. Vedi [Stack](/docs/guides/stacks/) e [stack swarm](/docs/guides/swarm/#swarm-stacks).

### Managed Stack (it. _stack gestito_)

Uno stack la cui definizione compose e il cui ambiente sono conservati da Homeport, così può essere distribuito, ridistribuito e modificato da Homeport.

### Linked Stack (it. _stack collegato_)

Uno stack i cui file compose sono una cartella di questo computer. Homeport conserva dove si trovano la cartella e i suoi file compose, legge e scrive i file sul posto, e distribuisce con quella cartella come directory del progetto, così `build:`, `env_file:` e i percorsi relativi funzionano. Viene trovato dalle etichette compose dei container in esecuzione, o creato da una cartella. Diventa esterno finché i suoi file mancano. Vedi [Stack collegati](/docs/guides/stacks/#linked-stacks).

### Synced Stack (it. _stack sincronizzato_)

Uno stack la cui definizione compose proviene da un repository git. Homeport ne conserva un clone su questo computer, recupera un branch o un tag su richiesta o a intervalli mentre Homeport è aperto, mostra cosa è cambiato, e distribuisce un commit dalla cartella del file compose, così `build:` e i percorsi relativi funzionano. Non viene mai modificato in Homeport: i suoi file cambiano tramite git; solo il suo ambiente e le impostazioni di sincronizzazione appartengono a Homeport. Vedi [Stack sincronizzati](/docs/guides/synced-stacks/).

### Repository (it. _repository_)

Un repository git che Homeport può leggere per gli stack sincronizzati: il suo URL e il modo in cui Homeport si autentica — una chiave di deploy che ha generato, un nome utente e un token, o nulla per uno pubblico. Conservato una sola volta per tutti gli host, e condiviso da ogni stack sincronizzato creato da esso.

### Deploy key (it. _chiave di deploy_)

La chiave Ed25519 che Homeport genera per un repository, che aggiungi a esso sulla tua forge git con accesso in sola lettura. Mai la tua chiave SSH personale.

### External Stack (it. _stack esterno_)

Uno stack trovato in esecuzione su un host la cui definizione compose Homeport non ha, né memorizzata, né in una cartella di questo computer, né in un repository che segue. Può essere avviato, arrestato e rimosso, ma non ridistribuito né modificato.

### Adopt (it. _adottare_)

Trasformare uno stack esterno in uno stack gestito fornendone la definizione compose.

### Registry login (it. _accesso al registro_)

Il nome utente e la password o il token di accesso che Homeport conserva per un registro di container, come Docker Hub o GHCR, e usa su ogni host per scaricare, inviare e compilare. Vedi [Impostazioni](/docs/guides/settings/#registries).

### Helper container (it. _container ausiliario_)

Un container `busybox` di breve durata che Homeport crea su un host per aprire un volume — per sfogliarlo, eseguirne il backup, ripristinarlo o clonarlo — poiché la Engine API non può leggerne uno. Non ha rete, monta il volume in sola lettura a meno che non debba scriverci, e viene rimosso al termine dell’azione. Non compare mai nell’elenco dei container né nella pagina Eventi. Vedi [Sfogliare i file di un volume](/docs/guides/volumes-networks/#the-helper-container).

### Board (it. _Bacheca_)

La pagina che elenca i container di ogni host connesso in un’unica tabella, ciascuno etichettato con il suo host. Vedi [Bacheca](/docs/guides/overview/#board).

### Command palette (it. _riquadro comandi_)

La casella di ricerca che si apre con <kbd>Mod</kbd> <kbd>K</kbd>: vai a una pagina, cambia host, esegui un comando, o trova un container, un’immagine, un volume, una rete o uno stack su qualsiasi host connesso. Vedi [Tastiera](/docs/guides/keyboard/#command-palette).

### Engine (it. _Engine_)

Il Docker Engine (`dockerd`), o uno compatibile come Podman, che esegue i container su un computer. Homeport comunica con esso tramite la Docker Engine API.

### Socket (it. _socket_)

L’indirizzo su cui un Engine è in ascolto: un file socket Unix come `/var/run/docker.sock`, o una named pipe come `//./pipe/docker_engine` su Windows.

### Keychain (it. _portachiavi_)

L’archivio sicuro del tuo sistema operativo per password e chiavi — il Portachiavi di macOS, Gestione credenziali di Windows o un Secret Service di Linux — dove Homeport conserva la chiave che cifra i suoi segreti.

### Trust on first use (it. _fiducia al primo utilizzo_)

Il modo in cui Homeport verifica i server SSH: confermi l’impronta della chiave di un server alla prima connessione, e Homeport si rifiuta di connettersi se la chiave cambia in seguito, finché non confermi di nuovo.

## In italiano

L’app, la documentazione e il sito in italiano usano questi termini, così una schermata e una pagina dicono sempre la stessa cosa.

| Inglese        | Italiano              |
| -------------- | --------------------- |
| Host           | host (m.)             |
| Engine         | Engine (m.)           |
| Instance       | istanza (f.)          |
| Swarm          | Swarm (m.)            |
| Cluster        | cluster (m.)          |
| Stack          | stack (m.)            |
| Managed Stack  | stack gestito         |
| Linked Stack   | stack collegato       |
| Synced Stack   | stack sincronizzato   |
| External Stack | stack esterno         |
| Adopt          | adottare              |
| Board          | Bacheca               |
| Bastion        | bastion (m.)          |
| Deploy key     | chiave di deploy (f.) |
| Port forward   | inoltro di porta (m.) |
| Release (Helm) | release (f.)          |
