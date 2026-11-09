---
title: FAQ
description: Les questions fréquentes sur Homeport.
---

### Homeport est-il gratuit ?

Oui. Homeport est gratuit, pour un usage personnel comme professionnel, et open source sous la [GNU Affero General Public License, version 3](https://github.com/DamienReichhart/Homeport/blob/main/LICENSE) (AGPL).

### Que signifie l’AGPL pour moi ?

Rien ne change si vous utilisez Homeport : installez-le, utilisez-le chez vous ou au travail, sur autant d’ordinateurs que vous le souhaitez. L’AGPL ne demande quelque chose qu’à ceux qui **distribuent** Homeport, modifié ou non, ou qui proposent une version modifiée à d’autres via un réseau : ils doivent publier son code source complet sous la même licence. Chaque copie de Homeport renvoie au code source de sa version exacte dans **Paramètres → À propos → Licences**.

Les versions jusqu’à 0.11 ont été publiées sous licence MIT et restent disponibles sous cette licence.

### Homeport remplace-t-il Docker Desktop ?

Pas entièrement. Homeport est un gestionnaire : il n’exécute pas lui-même de Docker Engine. Sous Linux, installez Docker Engine et utilisez Homeport comme interface. Sous macOS et Windows, il vous faut quand même quelque chose qui exécute le moteur — Docker Desktop, Colima, OrbStack, Rancher Desktop ou Podman — et Homeport peut se placer au-dessus de n’importe lequel d’entre eux, en plus de vos serveurs distants.

### Fonctionne-t-il avec containerd ?

Oui, directement, sans Docker : le moteur d’un hôte peut être [containerd](/docs/hosts/containerd/), joint via son socket sur cette machine ou via SSH — celui de k3s, celui de Docker Engine, ou le service containerd d’un serveur. Homeport liste ses espaces de noms, ses conteneurs et leurs tâches (démarrer, arrêter, mettre en pause, envoyer un signal, supprimer), leurs journaux là où nerdctl ou Kubernetes les ont écrits, un terminal dans ces conteneurs, ses images (récupérer, supprimer) et ses instantanés. Il n’y crée pas de conteneurs et n’y déploie pas de stacks ; c’est le rôle de nerdctl.

### Fonctionne-t-il avec Incus ?

Oui : le moteur d’un hôte peut être [Incus](/docs/hosts/incus/) 6.0 ou ultérieur, joint via son socket sur cette machine ou via SSH, ou via HTTPS avec un jeton de confiance obtenu par `incus config trust add`. Homeport liste les [instances](/docs/guides/instances/) d’un projet, conteneurs système et machines virtuelles, les démarre, les arrête, les fige, les supprime et les crée, ouvre leur console et un terminal à l’intérieur, prend et restaure des instantanés, et gère images, profils, pools de stockage et réseaux. Il n’y déploie pas de stacks, et n’installe rien sur l’hôte.

### Fonctionne-t-il avec LXD ?

Pas tel quel : Homeport parle Incus, le fork communautaire de LXD, et refuse un serveur LXD, dont l’API a divergé. Migrez-le avec `lxd-to-incus`, puis ajoutez-le comme hôte Incus.

### Fonctionne-t-il avec Podman ?

Oui, via le socket de l’API compatible Docker de Podman. Homeport détecte les sockets Podman habituels sous Linux, chaque [machine Podman en cours d’exécution sous macOS](/docs/hosts/local/#machines-podman-sous-macos), et le pipe de la machine Podman par défaut sous Windows. Tout le reste peut être [ajouté à la main](/docs/hosts/local/#ajouter-un-socket-à-la-main).

Homeport reconnaît aussi Podman à la connexion, et propose alors ce que seul Podman offre, via l’API propre à Podman sur le même socket : les [pods](/docs/guides/podman-pods/), que vous pouvez lister, démarrer, arrêter, créer et supprimer, et dans lesquels vous pouvez exécuter des conteneurs depuis la boîte de dialogue d’exécution ; et les [unités Quadlet](/docs/guides/quadlet/), que vous pouvez lister, lire, installer, modifier et supprimer, et démarrer ou arrêter via SSH.

### Fonctionne-t-il avec Kubernetes, Nomad ou Docker Swarm ?

Docker Swarm, oui. Homeport trouve un Swarm via ses hôtes managers, dès que l’un d’eux est un hôte, et le gère entièrement : ses nœuds, services, tâches, secrets, configs et stacks swarm, y compris les stacks gérées, liées et synchronisées qui y sont déployées. Il peut aussi démarrer un Swarm sur un hôte, et en rejoindre ou en quitter un. Tout passe par l’API Engine d’un manager connecté ; rien n’est installé sur les nœuds. Voir [Swarms](/docs/guides/swarm/).

Kubernetes, oui. Ajoutez un cluster depuis votre kubeconfig — celui de cet ordinateur ou un que vous collez — et Homeport trouve pour vous les clusters locaux (Docker Desktop, OrbStack, Rancher Desktop, kind, k3d, minikube). EKS, GKE et AKS fonctionnent via le plugin exec que nomme leur kubeconfig. Vous pouvez parcourir en direct les Namespaces, charges de travail, Pods, Services, Ingress, ConfigMaps, Secrets, Nodes, Events et toute autre ressource d’API, personnalisées comprises ; suivre les journaux, ouvrir un terminal dans un Pod, rediriger des ports, mettre à l’échelle, redémarrer et supprimer, et appliquer ou modifier du YAML avec un aperçu. Rien n’est installé sur le cluster : Homeport ne parle qu’à l’API Kubernetes que votre kubeconfig atteint déjà, avec les droits de ses identifiants. Avec le CLI `helm` sur cet ordinateur, Homeport liste, installe, met à niveau, fait revenir en arrière et désinstalle aussi des [releases Helm](/docs/clusters/helm/), et gère les dépôts de charts de helm. Voir [Clusters](/docs/clusters/).

Nomad, oui. Ajoutez un cluster par l’adresse de son agent, avec un jeton ACL et des éléments TLS s’il en utilise. Vous pouvez lister les jobs et leurs allocations, en direct ; exécuter un job depuis du HCL ou du JSON après avoir vu le plan de Nomad ; en modifier un et le réexécuter, l’arrêter ou le purger, et mettre à l’échelle ses groupes de tâches ; suivre les journaux d’une tâche et y ouvrir un shell ; drainer des nœuds et changer leur éligibilité ; et choisir un namespace. Rien n’est installé sur le cluster et aucun CLI `nomad` n’est nécessaire : Homeport parle à l’API HTTP de Nomad avec les droits du jeton. Voir [Clusters Nomad](/docs/clusters/nomad/).

### Ai-je besoin de kubectl ?

Non. Homeport lit votre kubeconfig et parle lui-même à l’API Kubernetes. Les seuls programmes qu’il exécute sont les [plugins exec](/docs/clusters/exec-plugins/) que nomme un kubeconfig, comme l’AWS CLI pour EKS, `gke-gcloud-auth-plugin` pour GKE ou `kubelogin` pour AKS — les mêmes que lancerait `kubectl`. Avoir `kubectl` sous la main reste utile pour vérifier ce que vos identifiants permettent (`kubectl auth can-i`).

### Dois-je installer quelque chose sur mes serveurs ?

Non — seulement un Docker Engine et un serveur SSH, que vous avez très probablement déjà. Homeport n’installe pas d’agents.

Un Swarm n’a besoin de rien de plus que le moteur non plus. Ses nœuds doivent toutefois pouvoir se joindre, ce que Homeport ne peut pas configurer pour vous : ouvrez 2377/tcp (vers les managers), 7946/tcp et 7946/udp, et 4789/udp entre eux. Voir [ports entre les nœuds](/docs/guides/swarm/#ports-entre-les-nœuds).

### Ai-je besoin de Docker sur mon ordinateur pour gérer des serveurs distants ?

Non. Homeport parle lui-même l’API Engine et SSH. Les seules exceptions sont le déploiement de stacks gérées, liées et synchronisées, qui utilise la commande `docker compose` sur votre ordinateur, et leur déploiement sur un Swarm, qui utilise la commande `docker` (`docker stack deploy`). Ni l’un ni l’autre n’a besoin d’un moteur sur votre ordinateur.

### Homeport garde-t-il mes stacks synchronisées quand il est fermé ?

Non. Les [stacks synchronisées](/docs/guides/synced-stacks/) ne sont vérifiées pour de nouveaux commits, et déployées automatiquement si vous l’activez, que pendant que Homeport est ouvert. Homeport n’installe rien sur vos serveurs — ni agent, ni récepteur de webhooks — donc quand il est fermé, rien ne surveille vos dépôts. Quand vous le rouvrez, il attend un intervalle, puis les vérifie toutes. Pour des déploiements qui doivent suivre chaque push à toute heure, lancez-les depuis votre CI.

Pour garder Homeport en marche sans sa fenêtre, activez **Fermer dans la zone de notification**, et **Démarrer à l’ouverture de session** avec **Démarrer masqué dans la zone de notification**, dans [Paramètres → Bureau](/docs/guides/settings/#bureau).

### Homeport peut-il me prévenir quand un conteneur plante ?

Oui, tant qu’il s’exécute — même masqué dans la zone de notification. Il affiche une notification du système quand un conteneur s’arrête avec une erreur, manque de mémoire ou devient en mauvaise santé, et quand un hôte devient injoignable ou revient. Chaque type peut être désactivé dans [Paramètres → Notifications](/docs/guides/settings/#notifications). Il l’apprend du flux d’événements qu’il maintient déjà avec chaque hôte, donc rien n’est installé sur vos serveurs ; quand Homeport ne s’exécute pas, personne n’est prévenu.

### Homeport a-t-il besoin de git ?

Seulement pour les [stacks synchronisées](/docs/guides/synced-stacks/), qui exécutent le `git` installé sur votre ordinateur, avec la clé de déploiement ou le jeton propre à chaque dépôt. **Paramètres → Cette machine** indique si Homeport l’a trouvé. Votre propre configuration git, vos assistants d’identification et vos clés SSH ne sont jamais utilisés.

### Plusieurs personnes peuvent-elles partager les hôtes de Homeport ?

Chaque personne exécute son propre Homeport avec ses propres hôtes et clés. Avec une clé créée par Homeport par personne et par serveur, vous pouvez voir et révoquer l’accès de chacun dans `authorized_keys`.

### Puis-je déplacer Homeport vers un autre ordinateur ?

Oui, de deux façons :

- **Tout** : **Paramètres → Données → Sauvegarder…** enregistre chaque hôte, cluster, stack, identifiant de registre, dépôt et paramètre, avec leurs secrets, dans un fichier protégé par une phrase secrète d’au moins 12 caractères. Sur le nouvel ordinateur, **Restaurer…** au même endroit l’ouvre, liste ce qu’il contient, et remplace ce qui s’y trouve. Tout se connecte et se déploie sans vous redemander de clé ni de mot de passe. Voir [sauvegarder et restaurer](/docs/guides/settings/#sauvegarder-et-restaurer).
- **Certains hôtes** : sur la page **Hôtes**, **Exportez** les hôtes que vous choisissez, et **Importez**-les sur l’autre ordinateur, à côté des hôtes qu’il a déjà. Voir [passer à un autre ordinateur](/docs/reference/security/#passer-à-un-autre-ordinateur).

Copier `homeport.db` ne fonctionne pas pour les secrets : ils sont chiffrés avec une clé conservée dans le trousseau de cet ordinateur, et apparaissent comme **Identifiants perdus** ailleurs.

### Une sauvegarde inclut-elle mes conteneurs et mes volumes ?

Non. Une sauvegarde contient ce que Homeport conserve — comment joindre vos hôtes, les définitions et environnements de vos stacks, les identifiants et les paramètres — pas ce qui s’exécute sur vos hôtes. Vos conteneurs, images et volumes restent sur les hôtes. Pour copier les données d’un volume, [sauvegardez-le dans un fichier](/docs/guides/volumes-networks/#sauvegarder-et-restaurer) depuis la page Volumes.

### Homeport peut-il construire des images ?

Oui, à deux endroits. La page **Images** construit une image à partir d’un dossier de cet ordinateur. Et une [stack liée](/docs/guides/stacks/#stacks-liées) — une stack dont les fichiers compose sont dans un dossier de cet ordinateur — peut utiliser `build:` : Homeport exécute `docker compose` dans ce dossier, donc le contexte de build est envoyé depuis cet endroit au moteur de l’hôte, qui le construit. **Reconstruire et déployer** reconstruit après une modification du code. Une stack gérée ne peut pas construire, puisque Homeport ne détient que son YAML et qu’il n’y a aucun dossier à partir duquel construire : liez plutôt le dossier, ou poussez l’image vers un registre.

### Pourquoi ma stack apparaît-elle comme externe sur mon autre ordinateur ?

Une stack démarrée avec `docker compose up` dans un dossier n’apparaît comme [liée](/docs/guides/stacks/#comment-les-stacks-liées-sont-trouvées) que sur un ordinateur où ce dossier et ses fichiers compose existent, au même chemin, et définissent les services qui s’exécutent. Sur un autre ordinateur, le chemin n’existe pas, donc Homeport n’a pas de fichier compose pour elle et l’affiche comme externe. Pour la gérer aussi depuis cet ordinateur, clonez le projet sur celui-ci et liez-le avec **Nouvelle stack → Utiliser un dossier de cet ordinateur**, ou adoptez-la comme stack gérée.

### Homeport collecte-t-il des données sur moi ?

Non. Il n’y a ni compte, ni télémétrie, ni analyse d’usage. La seule requête que Homeport fait de lui-même depuis votre ordinateur est la recherche de mises à jour, qui télécharge un petit fichier depuis ce site web et n’envoie rien sur vous ; vous pouvez la désactiver dans **Paramètres → Mises à jour**. Par ailleurs, la recherche d’images plus récentes demande aux registres de vos images, via le moteur de chaque hôte, vers quelle version pointent leurs tags ; pour un hôte local, cette requête part aussi de votre ordinateur. Voir [sécurité et vos données](/docs/reference/security/#ce-à-quoi-homeport-se-connecte).

### Homeport peut-il utiliser mes registres privés ?

Oui. Ajoutez des identifiants pour chaque registre dans **Paramètres → Registres** — Docker Hub, GHCR ou le vôtre. Homeport les transmet au moteur pour récupérer, pousser et construire sur n’importe quel hôte, et récupère avec eux les images d’une stack gérée, liée ou synchronisée avant de la déployer. Les mots de passe et les jetons sont chiffrés avec votre trousseau du système. Homeport ne lit pas les identifiants de `docker login` sur cet ordinateur.

### Comment Homeport sait-il qu’une image plus récente existe ?

Il demande au registre de l’image, via le moteur de l’hôte, vers quel digest pointe maintenant le tag, et le compare à celui que l’hôte a récupéré. Rien n’est téléchargé, mais Docker Hub décompte chaque question de sa limite de récupérations, comme un `docker pull` qui ne trouve rien de nouveau. Homeport conserve donc chaque réponse 6 heures, interroge au plus quatre registres à la fois, et ne redemande plus tôt que si vous choisissez **Rechercher des images plus récentes**. Des identifiants Docker Hub dans **Paramètres → Registres** relèvent la limite. Si un registre est injoignable ou limite le débit, Homeport n’affiche aucune marque pour ses images et écrit une ligne dans son journal.

### Comment mettre à jour ?

Sous Windows, sous macOS et avec l’AppImage, Homeport vous indique quand une nouvelle version est sortie : choisissez **Installer et redémarrer**. Sous Linux, les paquets `.deb` et `.rpm` se mettent à jour avec le reste de votre système depuis le dépôt de paquets Homeport. Voir [mise à jour](/docs/reference/updating/).

### Homeport est-il sur Homebrew, winget, Flathub ou l’AUR ?

Pas encore. C’est prévu ; la [page d’installation](/docs/installation/#prochainement) renvoie vers l’issue de chacun.

### Homeport est-il disponible dans ma langue ?

Homeport parle anglais, français, espagnol, allemand, italien, portugais du Brésil, russe, chinois simplifié, japonais, coréen, hindi et arabe, et suit la langue de votre système d’exploitation sauf si vous en choisissez une dans [Paramètres → Apparence](/docs/guides/settings/#apparence). Le site web existe aussi dans toutes ces langues ; la documentation est entièrement traduite en français, et les autres langues affichent la page anglaise là où une traduction n’existe pas encore. Pour ajouter une langue, voir [Contribuer](/docs/project/contributing/#traduire-homeport).

### Homeport est-il accessible ?

Il peut être utilisé au clavier seul, suit le réglage de réduction des animations de votre système, respecte le contraste WCAG 2.2 AA, et chaque page est vérifiée avec des outils d’accessibilité automatisés. Un audit manuel avec des lecteurs d’écran est en cours. Voir [Clavier et accessibilité](/docs/reference/accessibility/) pour ce qui fonctionne, les limites connues et comment signaler un obstacle.

### Y a-t-il une interface en ligne de commande ?

Non — Homeport est une application de bureau. Votre CLI `docker` existant fonctionne à côté, et Homeport se met à jour en direct quand vous modifiez des choses depuis le terminal.

### Comment puis-je aider ?

Signalez des bugs et proposez des fonctionnalités dans les [issues GitHub](https://github.com/DamienReichhart/Homeport/issues), ou consultez [contribuer](/docs/project/contributing/).
