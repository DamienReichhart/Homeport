---
title: Requisiti di sistema
description: Ciò che serve a Homeport per funzionare, e le parti facoltative che abilitano SSH, TLS, le distribuzioni di stack, i cluster Kubernetes e Nomad, e Helm.
---

## Sistemi operativi

| Sistema | Versioni                                                                  | Architetture         |
| ------- | ------------------------------------------------------------------------- | -------------------- |
| Windows | Windows 10 e 11                                                           | x64, ARM64           |
| macOS   | macOS 10.15 Catalina o successivo                                         | Apple silicon, Intel |
| Linux   | glibc 2.35 o successiva: Ubuntu 22.04, Debian 12, Fedora 36 e più recenti | x64, ARM64           |

Homeport usa la web view del sistema per disegnare la sua finestra: WebView2 su Windows (inclusa in Windows 10 e 11), WebKit su macOS e WebKitGTK su Linux (installata come dipendenza dei pacchetti `.deb` e `.rpm`, e inclusa nell’AppImage).

## Un Docker Engine da gestire

Homeport non include un runtime di container. Gestisce Docker Engine già in esecuzione da qualche parte:

- **Su questo computer** — Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop o Podman. Homeport li trova per te; vedi [Engine locali](/docs/hosts/local/).
- **Su un server** — raggiungibile tramite [SSH](/docs/hosts/ssh/), o tramite [TLS o TCP](/docs/hosts/tls-tcp/).

Funziona qualsiasi Engine che parli la Docker Engine API, compreso il socket compatibile con Docker di Podman.

Gestisce anche [containerd](/docs/hosts/containerd/) 1.7 e successivi direttamente, tramite il suo socket su questo computer o tramite SSH: quello di k3s, quello del Docker Engine stesso, o il servizio containerd su un server. Scaricare immagini lì richiede containerd 2.0 o successivo, e log e terminali richiedono che l’host sia raggiunto tramite SSH, o un containerd su questo computer (Linux).

Gestisce anche [Incus](/docs/hosts/incus/) 6.0 e successivi, tramite il suo socket su questo computer o tramite SSH, o tramite HTTPS con un token di fiducia. Le macchine virtuali richiedono la virtualizzazione hardware (`/dev/kvm`) sul server Incus, e un terminale in una di esse richiede l’`incus-agent` eseguito dalla sua immagine.

Su un host Podman, Homeport usa anche l’API propria di Podman: i [pod](/docs/guides/podman-pods/) richiedono Podman 4.0 o successivo, e le [unità Quadlet](/docs/guides/quadlet/) sono elencate da Podman 5.7 e installate, modificate e rimosse dalla 5.8. Avviare e arrestare un’unità Quadlet richiede che l’host sia raggiunto tramite SSH, o un Podman su questo computer (Linux).

## Facoltativo: cluster Kubernetes

Per esplorare i [cluster](/docs/clusters/), Homeport ha bisogno di un kubeconfig che li raggiunga (quello usato da `kubectl`, o uno che incolli) e dell’accesso di rete ai loro server API. Parla da sé l’API di Kubernetes, quindi `kubectl` non è necessario.

- **Versioni di Kubernetes**: cluster con Kubernetes 1.24 o successivo. Homeport usa solo API stabili (`v1`, `apps/v1`, `batch/v1`, `networking.k8s.io/v1`), ed è compilato con l’API 1.33.
- **Plugin exec**: un kubeconfig cloud indica un programma da eseguire su questo computer per ottenere le credenziali — l’[AWS CLI](https://aws.amazon.com/cli/) (versione 2) per EKS, `gke-gcloud-auth-plugin` della Google Cloud CLI per GKE, `kubelogin` per AKS. Installa quello indicato dal tuo kubeconfig; vedi [plugin exec](/docs/clusters/exec-plugins/).

## Facoltativo: cluster Nomad

Per gestire i [cluster Nomad](/docs/clusters/nomad/), Homeport ha bisogno dell’accesso di rete all’API HTTP di un agente Nomad (porta 4646 per impostazione predefinita) e, quando il cluster ha le ACL attivate, di un token; vedi [la policy ACL](/docs/clusters/nomad/#acl-policies) di cui ha bisogno. Parla da sé l’API di Nomad, quindi la CLI `nomad` non è necessaria, e nulla viene installato sul cluster.

- **Versioni di Nomad**: Nomad 1.6 o successivo. **Modifica ed esegui** parte dalla specifica inviata di un job, che Nomad conserva dalla 1.6 in poi.

## Facoltativo: un portachiavi del sistema operativo

Homeport cifra le chiavi SSH, i certificati TLS, le credenziali kubeconfig dei cluster, gli ambienti degli stack, gli accessi ai registri e le chiavi di deploy e i token dei repository con una chiave che conserva nel portachiavi del tuo sistema operativo:

- **macOS** — Portachiavi
- **Windows** — Gestione credenziali
- **Linux** — un provider Secret Service, come GNOME Keyring o KWallet

Senza portachiavi, Homeport mostra un banner e funziona comunque con socket locali, TCP semplice e SSH tramite **ssh-agent**. Gli host TLS, gli host SSH con chiavi memorizzate, i cluster, gli ambienti degli stack, gli accessi ai registri e i repository privati richiedono il portachiavi; senza, le immagini vengono scaricate in modo anonimo e si possono aggiungere solo repository pubblici. Vedi [la sicurezza e i tuoi dati](/docs/reference/security/).

## Facoltativo: Docker Compose

Per distribuire [stack gestiti, collegati e sincronizzati](/docs/guides/stacks/), Homeport esegue il comando `docker compose` su **questo** computer — non sull’host. Installa la Docker CLI con il plugin Compose (Docker Desktop lo include), o `docker-compose` v2 autonomo.

Senza di esso, gli stack compaiono comunque — compresi gli stack collegati — e possono essere avviati, arrestati e rimossi; solo la distribuzione richiede Compose. **Impostazioni → Questo computer** mostra se Homeport l’ha trovato.

Per distribuire uno stack su uno [Swarm](/docs/guides/swarm/), Homeport esegue invece `docker stack deploy` su questo computer, quindi ha bisogno del comando `docker` stesso (il plugin Compose non viene usato). Docker Desktop lo include; su Linux, installa il pacchetto Docker CLI della tua distribuzione (`docker-ce-cli`, `docker.io` o `docker`). Non è necessario che un Engine sia in esecuzione qui.

## Facoltativo: git

Gli [stack sincronizzati](/docs/guides/synced-stacks/) eseguono il `git` installato su **questo** computer per clonare e aggiornare i loro repository: Git for Windows, gli strumenti da riga di comando di Apple (`xcode-select --install`), o il pacchetto `git` della tua distribuzione. Su Windows, Homeport lo indirizza all’OpenSSH di sistema per i repository SSH. Tutte le altre funzioni funzionano senza git; **Impostazioni → Questo computer** mostra se Homeport l’ha trovato.

## Facoltativo: Helm

Per gestire le [release Helm](/docs/clusters/helm/) su un cluster, Homeport esegue il comando `helm`, versione 3 o successiva, su **questo** computer. Indirizza helm al cluster tramite una connessione privata, quindi nessuna credenziale del cluster viene scritta su disco, e helm usa i propri repository di chart, come in un terminale. Installalo con `brew install helm` (macOS), `winget install Helm.Helm`, Chocolatey o Scoop (Windows), oppure `snap install helm --classic`, il pacchetto della tua distribuzione o lo script di [helm.sh](https://helm.sh/docs/intro/install/) (Linux). Tutte le altre pagine dei cluster funzionano senza; **Impostazioni → Questo computer** mostra se Homeport l’ha trovato.

## Facoltativo: un’area di notifica

Homeport mette un’icona nell’area di notifica del sistema (la barra dei menu su macOS, l’area di notifica su Windows) che elenca ogni host con il suo stato. Windows e macOS ne hanno sempre una. Su Linux richiede la libreria **appindicator**: l’AppImage la include, e i pacchetti `.deb` e `.rpm` la raccomandano, quindi `apt`, `dnf` e `zypper` la installano a meno che tu non abbia disattivato i pacchetti raccomandati. Altrimenti installa `libayatana-appindicator3-1` (Debian, Ubuntu, openSUSE) o `libayatana-appindicator-gtk3` (Fedora). Alcuni desktop nascondono le icone dell’area di notifica — GNOME le mostra solo con un’estensione come AppIndicator Support. Senza area di notifica, Homeport funziona dalla sua finestra, e chiudere la finestra lo chiude.

## Cosa richiede ogni funzione

| Funzione                                         | Portachiavi | CLI Compose | git | helm |
| ------------------------------------------------ | :---------: | :---------: | :-: | :--: |
| Distribuire stack su uno Swarm                   |      —      | docker CLI  |  —  |  —   |
| Host con socket locale e TCP                     |      —      |      —      |  —  |  —   |
| Host SSH con ssh-agent                           |      —      |      —      |  —  |  —   |
| Host SSH con una chiave di Homeport o importata  |     Sì      |      —      |  —  |  —   |
| Host TLS                                         |     Sì      |      —      |  —  |  —   |
| Container, immagini, volumi, reti                |      —      |      —      |  —  |  —   |
| Avviare, arrestare, rimuovere stack              |      —      |      —      |  —  |  —   |
| Distribuire stack gestiti e collegati            |      —      |     Sì      |  —  |  —   |
| L’ambiente di uno stack gestito o sincronizzato  |     Sì      |      —      |  —  |  —   |
| Accessi ai registri (immagini private, invii)    |     Sì      |      —      |  —  |  —   |
| Stack sincronizzati da un repository pubblico    |      —      |     Sì      | Sì  |  —   |
| Stack sincronizzati con chiave di deploy o token |     Sì      |     Sì      | Sì  |  —   |
| Compilare un’immagine da una cartella            |      —      |      —      |  —  |  —   |
| Cluster Kubernetes                               |     Sì      |      —      |  —  |  —   |
| Cluster Nomad                                    |     Sì      |      —      |  —  |  —   |
| Release Helm                                     |     Sì      |      —      |  —  |  Sì  |
