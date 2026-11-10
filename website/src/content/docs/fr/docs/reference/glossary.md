---
title: Glossaire
description: Les mots qu’emploie Homeport, et ce qu’ils signifient.
---

Homeport nomme les choses avec précision. Ces termes apparaissent dans toute l’application et dans cette documentation. Chaque entrée porte le terme anglais du produit, suivi du mot qu’emploie l’application en français.

### Host (fr. _hôte_)

Un moteur auquel Homeport se connecte, défini par un nom et une façon de le joindre : un socket local, TCP, TLS ou SSH. Chaque conteneur, image, volume, réseau et stack appartient à exactement un hôte. Voir [Hôtes et connexions](/docs/hosts/).

### Engine kind (fr. _type de moteur_)

Le type de moteur vers lequel pointe un hôte : compatible Docker (Docker Engine, ou Podman via son API compatible), containerd ou Incus. Il décide des pages qu’affiche un hôte : les hôtes containerd et Incus ont les leurs. Podman n’est pas un type à part : Homeport le reconnaît à chaque connexion. Voir [Types de moteur](/docs/hosts/#types-de-moteur).

### Namespace (containerd’s) (fr. _espace de noms containerd_)

Sur un hôte containerd, l’espace dans lequel vit chaque conteneur, image et instantané : nerdctl utilise `default`, Kubernetes `k8s.io`, Docker `moby`. Les pages d’un hôte containerd en montrent un à la fois. Les Namespaces d’un cluster sont ceux de Kubernetes. Voir [espaces de noms containerd](/docs/hosts/containerd/#espaces-de-noms).

### Snapshot (fr. _instantané_)

Sur un hôte containerd, une couche de fichiers sur le disque : une couche d’image décompressée, ou la couche modifiable d’un conteneur. Voir [instantanés](/docs/hosts/containerd/#instantanés). Sur un hôte Incus, un état enregistré d’une instance, auquel elle peut être restaurée ; voir [instantanés d’instance](/docs/guides/instances/#instantanés).

### Instance (fr. _instance_)

Ce qu’exécute un hôte Incus : un conteneur système, qui démarre une distribution Linux complète sur le noyau de l’hôte, ou une machine virtuelle avec son propre noyau. Pas un conteneur, qui exécute une seule application. Voir [Instances](/docs/guides/instances/).

### Project (Incus’s) (fr. _projet_)

Sur un hôte Incus, l’espace dans lequel vivent ses instances, images, profils, volumes personnalisés et réseaux. Les pages d’un hôte Incus en montrent un à la fois, choisi sous le sélecteur d’hôte. Voir [projets](/docs/hosts/incus/#projets).

### Profile (fr. _profil_)

Sur un hôte Incus, une configuration et des périphériques partagés par les instances qui le listent. Voir [profils](/docs/guides/instances/#profils).

### Trust token (fr. _jeton de confiance_)

Ce qu’affiche `incus config trust add` sur un serveur Incus, collé une fois dans Homeport pour ajouter ce serveur via HTTPS. Voir [jetons de confiance](/docs/hosts/incus/#via-https-avec-un-jeton-de-confiance).

### Pod (fr. _pod Podman_)

Sur un hôte Podman, un groupe de conteneurs qui partagent un même réseau et démarrent et s’arrêtent ensemble, autour d’un conteneur infra qui détient les ports que publie le pod. Les Pods d’un cluster sont des Pods Kubernetes. Voir [Pods Podman](/docs/guides/podman-pods/).

### Quadlet unit (fr. _unité Quadlet_)

Un fichier sur un hôte Podman, comme `web.container`, que systemd transforme en service, `web.service`, pour que les conteneurs de Podman démarrent au boot et que systemd les maintienne en marche. Voir [Unités Quadlet](/docs/guides/quadlet/).

### Bastion (fr. _bastion_)

Un serveur SSH par lequel on joint un hôte SSH, quand le propre serveur SSH de l’hôte n’accepte que les connexions venant de lui : ce qu’OpenSSH appelle `ProxyJump`. Un hôte peut en avoir plusieurs, dans l’ordre, chacun avec sa propre clé de serveur approuvée. Un bastion n’est pas un hôte : il n’exécute aucun moteur auquel Homeport parle. Voir [Via un bastion](/docs/hosts/ssh/#through-a-bastion).

### Cluster (fr. _cluster_)

Un cluster Kubernetes ou un cluster Nomad auquel Homeport se connecte via son API. Un cluster Kubernetes s’ajoute à partir d’un contexte d’un kubeconfig : un nom, l’adresse du serveur d’API, et les identifiants que donne le kubeconfig, enregistrés chiffrés. Un cluster Nomad s’ajoute par l’adresse de l’un de ses agents, avec un jeton ACL et des éléments TLS, enregistrés chiffrés. Homeport n’installe rien sur l’un ni sur l’autre et n’atteint que ce que ces identifiants permettent. Pas un Swarm, qui se trouve via ses hôtes et ne s’ajoute jamais. Les objets propres à Kubernetes gardent leurs noms Kubernetes : Namespace, Pod, Deployment, Service, Node… Ceux de Nomad gardent les noms de Nomad : job, groupe de tâches, tâche, allocation, nœud client, namespace. Voir [Clusters](/docs/clusters/) et [Clusters Nomad](/docs/clusters/nomad/).

### Allocation (fr. _allocation_)

Sur un cluster Nomad, un placement du groupe de tâches d’un job sur un nœud client, qui exécute ses tâches. Ses journaux et un shell à l’intérieur passent par les serveurs de Nomad jusqu’au nœud. Voir [journaux et shell](/docs/clusters/nomad/#journaux-et-shell).

### Swarm (fr. _Swarm_)

Un cluster Docker Swarm, trouvé via les hôtes qui en font partie et jamais ajouté à part. Homeport le joint via n’importe quel hôte manager connecté, distingue les Swarms par leur ID de swarm, et n’enregistre rien sur aucun. Il porte le nom de son premier hôte manager, puisque Docker donne le même nom à tous les Swarms. Voir [Swarms](/docs/guides/swarm/).

### Port forward (fr. _redirection de port_)

Un port sur l’interface de bouclage de cet ordinateur que Homeport relie, via l’API Kubernetes, à un port d’un Pod, ou d’un Service via l’un de ses Pods prêts, comme `kubectl port-forward`. Elle prend fin quand Homeport quitte. Voir [Redirection de ports](/docs/clusters/port-forward/).

### Release (fr. _release_)

Une release Helm : une installation d’un chart sur un cluster, nommée dans son Namespace, avec une révision numérotée pour chaque installation, mise à niveau et retour arrière. Helm la conserve sur le cluster ; une release créée dans un terminal et une release créée dans Homeport sont les mêmes. Voir [Releases Helm](/docs/clusters/helm/).

### Chart repository (fr. _dépôt de charts_)

Un serveur depuis lequel helm télécharge des charts, dans la liste propre à helm sur cet ordinateur — celle qu’affiche `helm repo list`. Les charts d’un registre OCI s’installent plutôt par leur référence `oci://`. Voir [dépôts de charts](/docs/clusters/helm/#dépôts-de-charts).

### Node (fr. _nœud_)

Un moteur dans un Swarm : un manager ou un worker. Les Nodes d’un cluster sont des Nodes Kubernetes, les machines sur lesquelles il exécute des Pods. Un nœud n’est un hôte que quand Homeport s’y connecte, et la page d’un hôte renvoie vers son nœud.

### Service (fr. _service_)

Ce qu’exécute un Swarm : une image, ses paramètres et un nombre de réplicas, ou une tâche sur chaque nœud pour un service global.

### Task (fr. _tâche_)

Un emplacement d’un service sur un nœud, exécuté là sous forme de conteneur. Quand un service ne converge pas, ses tâches disent pourquoi. Sur un hôte containerd, une tâche est ce qui s’exécute pour un conteneur, lequel n’est que des métadonnées : démarrer un conteneur crée sa tâche.

### Stack (fr. _stack_)

Un projet Compose qui s’exécute sur un hôte, ou une stack swarm qui s’exécute sur un Swarm, identifié par son nom de projet. Chaque stack est gérée, liée, synchronisée ou externe. Voir [Stacks](/docs/guides/stacks/) et [stacks swarm](/docs/guides/swarm/#stacks-swarm).

### Managed Stack (fr. _stack gérée_)

Une stack dont Homeport détient la définition compose et l’environnement, pour qu’elle puisse être déployée, redéployée et modifiée depuis Homeport.

### Linked Stack (fr. _stack liée_)

Une stack dont les fichiers compose sont un dossier de cet ordinateur. Homeport conserve l’emplacement du dossier et de ses fichiers compose, lit et écrit les fichiers sur place, et déploie avec ce dossier comme répertoire du projet, pour que `build:`, `env_file:` et les chemins relatifs fonctionnent. Trouvée à partir des labels compose des conteneurs en cours d’exécution, ou créée à partir d’un dossier. Elle devient externe tant que ses fichiers sont absents. Voir [Stacks liées](/docs/guides/stacks/#stacks-liées).

### Synced Stack (fr. _stack synchronisée_)

Une stack dont la définition compose provient d’un dépôt git. Homeport en conserve un clone sur cet ordinateur, récupère une branche ou un tag à la demande ou à intervalle régulier tant que Homeport est ouvert, montre ce qui a changé, et déploie un commit depuis le dossier du fichier compose, pour que `build:` et les chemins relatifs fonctionnent. Elle n’est jamais modifiée dans Homeport : ses fichiers changent via git ; seuls son environnement et ses paramètres de synchronisation appartiennent à Homeport. Voir [Stacks synchronisées](/docs/guides/synced-stacks/).

### Repository (fr. _dépôt_)

Un dépôt git que Homeport peut lire pour les stacks synchronisées : son URL et la façon dont Homeport s’y authentifie — une clé de déploiement qu’il a générée, un nom d’utilisateur et un jeton, ou rien pour un dépôt public. Conservé une seule fois pour tous les hôtes, et partagé par chaque stack synchronisée créée à partir de lui.

### Deploy key (fr. _clé de déploiement_)

La clé Ed25519 que Homeport génère pour un dépôt, que vous y ajoutez sur votre forge git avec un accès en lecture seule. Jamais votre propre clé SSH.

### External Stack (fr. _stack externe_)

Une stack trouvée en cours d’exécution sur un hôte et dont Homeport n’a pas la définition compose, ni enregistrée, ni dans un dossier de cet ordinateur, ni dans un dépôt qu’il suit. Elle peut être démarrée, arrêtée et supprimée, mais ni redéployée ni modifiée.

### Adopt (fr. _adopter_)

Transformer une stack externe en stack gérée en fournissant sa définition compose.

### Registry login (fr. _identifiants de registre_)

Le nom d’utilisateur et le mot de passe ou le jeton d’accès que Homeport conserve pour un registre de conteneurs, comme Docker Hub ou GHCR, et qu’il utilise sur chaque hôte pour récupérer, pousser et construire. Voir [Paramètres](/docs/guides/settings/#registres).

### Helper container (fr. _conteneur auxiliaire_)

Un conteneur `busybox` éphémère que Homeport crée sur un hôte pour ouvrir un volume — pour le parcourir, le sauvegarder, le restaurer ou le cloner — puisque l’API Engine ne peut pas en lire un. Il n’a pas de réseau, monte le volume en lecture seule sauf s’il doit y écrire, et est supprimé quand l’action se termine. Il n’apparaît jamais dans la liste des conteneurs ni sur la page Événements. Voir [Parcourir les fichiers d’un volume](/docs/guides/volumes-networks/#le-conteneur-auxiliaire).

### Board (fr. _Tableau_)

La page qui liste les conteneurs de chaque hôte connecté dans un seul tableau, chacun étiqueté avec son hôte. Voir [Tableau](/docs/guides/overview/#tableau).

### Command palette (fr. _palette de commandes_)

Le champ de recherche ouvert avec <kbd>Mod</kbd> <kbd>K</kbd> : aller à une page, changer d’hôte, exécuter une commande, ou trouver un conteneur, une image, un volume, un réseau ou une stack sur n’importe quel hôte connecté. Voir [Clavier](/docs/guides/keyboard/#palette-de-commandes).

### Engine (fr. _moteur_)

Le logiciel vers lequel pointe un hôte, celui qu’on installe, démarre et arrête : le Docker Engine (`dockerd`) ou un moteur compatible comme Docker Desktop, Podman, Colima ou OrbStack, auquel Homeport parle via l’API Docker Engine ; ou containerd ou Incus, via leur propre API. Pas un synonyme d’hôte : « démarrer le Docker Engine » parle du moteur, « Homeport ne peut pas joindre prod-server » parle de l’hôte.

### Socket (fr. _socket_)

L’adresse sur laquelle écoute un moteur : un fichier de socket Unix comme `/var/run/docker.sock`, ou un pipe nommé comme `//./pipe/docker_engine` sous Windows.

### Keychain (fr. _trousseau du système_)

Le stockage sécurisé de mots de passe et de clés de votre système d’exploitation — le Trousseau de macOS, le Gestionnaire d’identification de Windows ou un Secret Service sous Linux — où Homeport conserve la clé qui chiffre ses secrets.

### Trust on first use (fr. _confiance au premier usage_)

La façon dont Homeport vérifie les serveurs SSH : vous confirmez l’empreinte de la clé d’un serveur à la première connexion, et Homeport refuse de se connecter si la clé change ensuite, jusqu’à ce que vous confirmiez de nouveau.

## En français

L’application, la documentation et le site web en français emploient ces termes, pour qu’un écran et une page disent toujours la même chose. Chaque autre langue a sa propre table dans [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n).

| Anglais        | Français                 |
| -------------- | ------------------------ |
| Host           | hôte (m.)                |
| Engine         | moteur (m.)              |
| Instance       | instance (f.)            |
| Swarm          | Swarm (m.)               |
| Cluster        | cluster (m.)             |
| Stack          | stack (f.)               |
| Managed Stack  | stack gérée              |
| Linked Stack   | stack liée               |
| Synced Stack   | stack synchronisée       |
| External Stack | stack externe            |
| Adopt          | adopter                  |
| Board          | Tableau                  |
| Bastion        | bastion (m.)             |
| Deploy key     | clé de déploiement (f.)  |
| Port forward   | redirection de port (f.) |
| Release (Helm) | release (f.)             |
