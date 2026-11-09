---
title: Configuration requise
description: Ce dont Homeport a besoin pour fonctionner, et les éléments facultatifs qui permettent SSH, TLS, le déploiement de stacks, les clusters Kubernetes et Nomad, et Helm.
---

## Systèmes d’exploitation

| Système | Versions                                                                      | Architectures     |
| ------- | ----------------------------------------------------------------------------- | ----------------- |
| Windows | Windows 10 et 11                                                              | x64, ARM64        |
| macOS   | macOS 10.15 Catalina ou ultérieur                                             | Puce Apple, Intel |
| Linux   | glibc 2.35 ou ultérieure : Ubuntu 22.04, Debian 12, Fedora 36 et plus récents | x64, ARM64        |

Homeport utilise la vue web du système pour dessiner sa fenêtre : WebView2 sous Windows (incluse avec Windows 10 et 11), WebKit sous macOS, et WebKitGTK sous Linux (installée comme dépendance des paquets `.deb` et `.rpm`, et intégrée à l’AppImage).

## Un Docker Engine à gérer

Homeport n’inclut pas d’environnement d’exécution de conteneurs. Il gère des Docker Engines qui s’exécutent déjà quelque part :

- **Sur cette machine** — Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop ou Podman. Homeport les trouve pour vous ; voir [moteurs locaux](/docs/hosts/local/).
- **Sur un serveur** — joignable via [SSH](/docs/hosts/ssh/), ou via [TLS ou TCP](/docs/hosts/tls-tcp/).

Tout moteur qui parle l’API Docker Engine fonctionne, y compris le socket compatible Docker de Podman.

Il gère aussi directement [containerd](/docs/hosts/containerd/) 1.7 et ultérieur, via son socket sur cette machine ou via SSH : celui de k3s, celui de Docker Engine, ou le service containerd d’un serveur. Y récupérer des images nécessite containerd 2.0 ou ultérieur, et les journaux et terminaux nécessitent un hôte joint via SSH, ou un containerd sur cet ordinateur (Linux).

Il gère aussi [Incus](/docs/hosts/incus/) 6.0 et ultérieur, via son socket sur cette machine ou via SSH, ou via HTTPS avec un jeton de confiance. Les machines virtuelles nécessitent la virtualisation matérielle (`/dev/kvm`) sur le serveur Incus, et un terminal dans l’une d’elles nécessite l’`incus-agent` qu’exécute son image.

Sur un hôte Podman, Homeport utilise aussi l’API propre à Podman : les [pods](/docs/guides/podman-pods/) nécessitent Podman 4.0 ou ultérieur, et les [unités Quadlet](/docs/guides/quadlet/) sont listées à partir de Podman 5.7 et installées, modifiées et supprimées à partir de 5.8. Démarrer et arrêter une unité Quadlet nécessite un hôte joint via SSH, ou un Podman sur cet ordinateur (Linux).

## Facultatif : clusters Kubernetes

Pour parcourir des [clusters](/docs/clusters/), Homeport a besoin d’un kubeconfig qui les atteint (celui qu’utilise `kubectl`, ou un que vous collez) et d’un accès réseau à leurs serveurs d’API. Il parle lui-même l’API Kubernetes, donc `kubectl` n’est pas nécessaire.

- **Versions de Kubernetes** : des clusters sous Kubernetes 1.24 ou ultérieur. Homeport n’utilise que des API stables (`v1`, `apps/v1`, `batch/v1`, `networking.k8s.io/v1`), et est construit sur l’API 1.33.
- **Plugins exec** : un kubeconfig de cloud nomme un programme à exécuter sur cet ordinateur pour obtenir ses identifiants — l’[AWS CLI](https://aws.amazon.com/cli/) (version 2) pour EKS, `gke-gcloud-auth-plugin` du Google Cloud CLI pour GKE, `kubelogin` pour AKS. Installez celui que nomme votre kubeconfig ; voir [plugins exec](/docs/clusters/exec-plugins/).

## Facultatif : clusters Nomad

Pour gérer des [clusters Nomad](/docs/clusters/nomad/), Homeport a besoin d’un accès réseau à l’API HTTP d’un agent Nomad (port 4646 par défaut) et, quand le cluster a les ACL activées, d’un jeton ; voir [la politique ACL](/docs/clusters/nomad/#politiques-acl) dont il a besoin. Il parle lui-même l’API de Nomad, donc le CLI `nomad` n’est pas nécessaire, et rien n’est installé sur le cluster.

- **Versions de Nomad** : Nomad 1.6 ou ultérieur. **Modifier et exécuter** part de la spécification soumise d’un job, que Nomad conserve à partir de 1.6.

## Facultatif : un trousseau du système

Homeport chiffre les clés SSH, les certificats TLS, les identifiants kubeconfig des clusters, les environnements des stacks, les identifiants de registre et les clés de déploiement et jetons des dépôts avec une clé qu’il conserve dans le trousseau de votre système d’exploitation :

- **macOS** — Trousseau
- **Windows** — Gestionnaire d’identification
- **Linux** — un fournisseur Secret Service, comme GNOME Keyring ou KWallet

Sans trousseau, Homeport affiche un bandeau et fonctionne quand même avec les sockets locaux, le TCP simple et SSH via **ssh-agent**. Les hôtes TLS, les hôtes SSH avec clés enregistrées, les clusters, les environnements des stacks, les identifiants de registre et les dépôts privés ont besoin du trousseau ; sans lui, les images sont récupérées de façon anonyme et seuls des dépôts publics peuvent être ajoutés. Voir [sécurité et vos données](/docs/reference/security/).

## Facultatif : Docker Compose

Pour déployer des [stacks gérées, liées et synchronisées](/docs/guides/stacks/), Homeport exécute la commande `docker compose` sur **cette** machine — pas sur l’hôte. Installez le CLI Docker avec le plugin Compose (Docker Desktop l’inclut), ou le `docker-compose` v2 autonome.

Sans lui, les stacks apparaissent quand même — stacks liées comprises — et peuvent être démarrées, arrêtées et supprimées ; seul le déploiement nécessite Compose. **Paramètres → Cette machine** indique si Homeport l’a trouvé.

Pour déployer une stack sur un [Swarm](/docs/guides/swarm/), Homeport exécute plutôt `docker stack deploy` sur cette machine, il a donc besoin de la commande `docker` elle-même (le plugin Compose n’est pas utilisé). Docker Desktop l’inclut ; sous Linux, installez le paquet du CLI Docker de votre distribution (`docker-ce-cli`, `docker.io` ou `docker`). Aucun moteur n’a besoin de s’exécuter ici.

## Facultatif : git

Les [stacks synchronisées](/docs/guides/synced-stacks/) exécutent le `git` installé sur **cette** machine pour cloner et récupérer leurs dépôts : Git for Windows, les outils en ligne de commande d’Apple (`xcode-select --install`), ou le paquet `git` de votre distribution. Sous Windows, Homeport l’oriente vers l’OpenSSH du système pour les dépôts SSH. Toutes les autres fonctions marchent sans git ; **Paramètres → Cette machine** indique si Homeport l’a trouvé.

## Facultatif : Helm

Pour gérer des [releases Helm](/docs/clusters/helm/) sur un cluster, Homeport exécute la commande `helm`, version 3 ou ultérieure, sur **cette** machine. Il oriente helm vers le cluster via une connexion privée, donc aucun identifiant de cluster n’est écrit sur le disque, et helm utilise ses propres dépôts de charts, comme dans un terminal. Installez-le avec `brew install helm` (macOS), `winget install Helm.Helm`, Chocolatey ou Scoop (Windows), ou `snap install helm --classic`, le paquet de votre distribution ou le script de [helm.sh](https://helm.sh/docs/intro/install/) (Linux). Toutes les autres pages de cluster marchent sans lui ; **Paramètres → Cette machine** indique si Homeport l’a trouvé.

## Facultatif : une zone de notification

Homeport place une icône dans la zone de notification (la barre des menus sous macOS, la zone de notification sous Windows) qui liste chaque hôte avec son état. Windows et macOS en ont toujours une. Sous Linux, elle nécessite la bibliothèque **appindicator** : l’AppImage l’inclut, et les paquets `.deb` et `.rpm` la recommandent, donc `apt`, `dnf` et `zypper` l’installent sauf si vous avez désactivé les paquets recommandés. Sinon, installez `libayatana-appindicator3-1` (Debian, Ubuntu, openSUSE) ou `libayatana-appindicator-gtk3` (Fedora). Certains bureaux masquent les icônes de la zone de notification — GNOME ne les affiche qu’avec une extension comme AppIndicator Support. Sans zone de notification, Homeport fonctionne depuis sa fenêtre, et fermer la fenêtre le quitte.

## Ce dont chaque fonction a besoin

| Fonction                                                      | Trousseau | CLI Compose | git | helm |
| ------------------------------------------------------------- | :-------: | :---------: | :-: | :--: |
| Déployer des stacks sur un Swarm                              |     —     | CLI docker  |  —  |  —   |
| Hôtes socket local et TCP                                     |     —     |      —      |  —  |  —   |
| Hôtes SSH avec ssh-agent                                      |     —     |      —      |  —  |  —   |
| Hôtes SSH avec une clé Homeport ou importée                   |    Oui    |      —      |  —  |  —   |
| Hôtes TLS                                                     |    Oui    |      —      |  —  |  —   |
| Conteneurs, images, volumes, réseaux                          |     —     |      —      |  —  |  —   |
| Démarrer, arrêter, supprimer des stacks                       |     —     |      —      |  —  |  —   |
| Déployer des stacks gérées et liées                           |     —     |     Oui     |  —  |  —   |
| L’environnement d’une stack gérée ou synchronisée             |    Oui    |      —      |  —  |  —   |
| Identifiants de registre (images privées, pousser des images) |    Oui    |      —      |  —  |  —   |
| Stacks synchronisées depuis un dépôt public                   |     —     |     Oui     | Oui |  —   |
| Stacks synchronisées avec clé de déploiement ou jeton         |    Oui    |     Oui     | Oui |  —   |
| Construire une image depuis un dossier                        |     —     |      —      |  —  |  —   |
| Clusters Kubernetes                                           |    Oui    |      —      |  —  |  —   |
| Clusters Nomad                                                |    Oui    |      —      |  —  |  —   |
| Releases Helm                                                 |    Oui    |      —      |  —  | Oui  |
