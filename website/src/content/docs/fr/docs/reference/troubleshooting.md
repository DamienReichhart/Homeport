---
title: Dépannage
description: Les solutions aux problèmes que vous rencontrerez le plus probablement, classées selon le message qu’affiche Homeport.
---

Si votre problème n’est pas ici, consultez le [fichier journal](/docs/reference/security/#journaux) et [ouvrez une issue](https://github.com/DamienReichhart/Homeport/issues) en indiquant votre version de Homeport, visible dans **Paramètres**.

## Installation et démarrage

### Windows affiche « Windows a protégé votre ordinateur »

Le programme d’installation n’est pas encore signé avec un certificat commercial. Choisissez **Informations complémentaires**, puis **Exécuter quand même**. Vous pouvez d’abord [vérifier le téléchargement](/docs/reference/verifying/).

### macOS indique que Homeport « ne peut pas être ouvert »

Homeport n’est pas encore notarié par Apple. Ouvrez **Réglages Système → Confidentialité et sécurité** et choisissez **Ouvrir quand même** à côté de Homeport, ou exécutez :

```sh
xattr -dr com.apple.quarantine /Applications/Homeport.app
```

### L’AppImage ne démarre pas

Si elle mentionne FUSE, installez `libfuse2` (`libfuse2t64` sur Ubuntu 24.04 et ultérieur). Vérifiez que le fichier est exécutable : `chmod +x Homeport*.AppImage`.

### Homeport s’ouvre au mauvais endroit

Homeport se rouvre avec la taille, la position et l’état agrandi qu’avait sa fenêtre quand vous l’avez fermée. Si l’écran sur lequel elle se trouvait n’est plus là, elle s’ouvre centrée sur un écran restant. Pour oublier l’emplacement enregistré, quittez Homeport et supprimez `.window-state.json` de son dossier de paramètres :

| Système | Fichier                                                             |
| ------- | ------------------------------------------------------------------- |
| Linux   | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%APPDATA%\com.homeport.app\.window-state.json`                     |

### Rouvrir Homeport n’ouvre pas de seconde fenêtre

Un seul Homeport s’exécute à la fois : l’ouvrir alors qu’il s’exécute déjà amène sa fenêtre au premier plan, même quand elle est masquée dans la [zone de notification](/docs/guides/overview/#zone-de-notification). Deux copies se connecteraient deux fois à chaque hôte et écriraient dans les mêmes données.

### Il n’y a pas d’icône dans la zone de notification

Sous Linux, la zone de notification nécessite la bibliothèque appindicator, et certains bureaux masquent ces icônes : sous GNOME, installez l’extension AppIndicator Support. Si Homeport ne trouve aucune bibliothèque appindicator, **Paramètres → Bureau** le signale et fermer la fenêtre quitte l’application. Voir la [configuration requise](/docs/requirements/#facultatif-une-zone-de-notification).

### Un bandeau rouge indique qu’aucun trousseau du système n’est disponible

Homeport n’a pas pu joindre votre trousseau. Sous Linux, cela signifie en général qu’aucun Secret Service ne s’exécute — fréquent sur les bureaux minimaux et les gestionnaires de fenêtres. Installez et démarrez GNOME Keyring (`gnome-keyring`) ou activez le Secret Service de KWallet, puis redémarrez Homeport. En attendant, utilisez des hôtes socket local, TCP ou ssh-agent.

## Mises à jour et dépôt de paquets

### « Impossible de rechercher les mises à jour »

Homeport n’a pas pu télécharger les informations de mise à jour depuis ce site web : vous êtes peut-être hors ligne, ou derrière un proxy ou un pare-feu qui bloque `damienreichhart.github.io`. Homeport continue de fonctionner ; il vérifie de nouveau plus tard, ou choisissez **Paramètres → Mises à jour → Vérifier maintenant**. Vous pouvez toujours télécharger la dernière version depuis la [page de téléchargement](/download/).

### Une mise à jour ne s’installe pas

Homeport n’installe une mise à jour que si sa signature correspond à sa clé de mise à jour, et s’arrête si le téléchargement est incomplet ou altéré. Réessayez plus tard ; si l’échec persiste, téléchargez la nouvelle version depuis la [page de téléchargement](/download/) et installez-la par-dessus l’ancienne.

### apt affiche « NO_PUBKEY » ou « The repository is not signed »

La clé Homeport manque dans `/etc/apt/keyrings/homeport.asc`. Relancez l’[installation rapide](/docs/installation/#installation-rapide), ou les deux premières commandes de la [configuration manuelle](/docs/installation/#configuration-manuelle).

### dnf ou zypper demande s’il faut importer une clé

C’est la clé de paquets Homeport, la première fois que votre gestionnaire de paquets utilise le dépôt. Vérifiez que l’empreinte affichée correspond à celle de [vérifier les téléchargements](/docs/reference/verifying/#signatures-des-paquets), puis acceptez.

### apt affiche « Hash Sum mismatch » juste après une release

Le dépôt était en cours de mise à jour pendant qu’apt le lisait. Relancez `sudo apt update` quelques minutes plus tard.

### Mon gestionnaire de paquets ne voit pas la nouvelle version

Vérifiez que le dépôt est toujours configuré : `/etc/apt/sources.list.d/homeport.sources`, `/etc/yum.repos.d/homeport.repo` ou `/etc/zypp/repos.d/homeport.repo`. Si vous l’avez supprimé, lancez l’[installation rapide](/docs/installation/#installation-rapide) pour l’ajouter de nouveau. Si vous avez installé 0.2.0 ou antérieur depuis un fichier téléchargé, le dépôt n’a jamais été ajouté : voir [mettre à jour depuis 0.2.0](/docs/reference/updating/#mettre-à-jour-depuis-020-ou-antérieur).

## Connexion

### Injoignable : Permission denied

Votre utilisateur ne peut pas ouvrir le socket Docker. Sous Linux, ajoutez-le au groupe `docker` et reconnectez-vous :

```sh
sudo usermod -aG docker $USER
```

### Injoignable : No such file or directory

Rien n’écoute au chemin du socket. Démarrez votre moteur — Docker Desktop, Colima, OrbStack, Rancher Desktop — ou vérifiez le chemin. Voir [moteurs locaux](/docs/hosts/local/#ajouter-un-socket-à-la-main).

### « Le Docker Engine n’a pas répondu à temps » ou « Délai de connexion SSH dépassé »

Homeport attend une connexion pendant 10 secondes. Vérifiez l’adresse et le port, que le serveur est démarré, et qu’aucun pare-feu ne bloque. Homeport continue de réessayer de lui-même.

### Accès refusé : « Le serveur a refusé la clé pour … »

Le serveur n’accepte pas encore la clé de Homeport. Exécutez sur le serveur la commande **Ajouter la clé publique** de la page de l’hôte, puis choisissez **Reconnecter**. Voir [Hôtes SSH](/docs/hosts/ssh/#autoriser-la-clé-de-homeport).

### « Le serveur SSH n’a pas pu ouvrir /var/run/docker.sock »

Homeport s’est connecté, mais n’a pas pu joindre Docker. Vérifiez :

- le **Chemin du socket distant** — Docker rootless utilise `/run/user/<uid>/docker.sock` ;
- que l’utilisateur peut utiliser Docker : `sudo usermod -aG docker <user>` sur le serveur ;
- que le serveur autorise la redirection de sockets (`AllowStreamLocalForwarding` dans `sshd_config`, activé par défaut).

### « Aucun ssh-agent n’est en cours d’exécution »

Démarrez votre agent SSH et ajoutez votre clé avec `ssh-add`. Si vous avez ouvert Homeport depuis un menu du bureau, il ne voit peut-être pas l’agent qu’utilise votre terminal ; déconnectez-vous et reconnectez-vous, ou lancez Homeport depuis un terminal où `ssh-add -l` liste votre clé.

Sous Windows, Homeport utilise soit le service OpenSSH Authentication Agent, soit Pageant de PuTTY. Ouvrez Pageant et chargez votre clé, ou démarrez le service dans un PowerShell administrateur :

```powershell
Get-Service ssh-agent | Set-Service -StartupType Automatic
Start-Service ssh-agent
ssh-add
```

### Accès refusé : « Échec de la négociation TLS »

- Le certificat d’AC doit être celui qui a signé le certificat du serveur.
- L’adresse que vous avez saisie doit correspondre à un nom ou une adresse IP du certificat du serveur.
- Le serveur doit s’exécuter avec `--tlsverify` et faire confiance à l’AC de votre certificat client.

### Clé du serveur modifiée

Le serveur SSH a présenté une clé différente de celle que vous avez approuvée. Si vous avez réinstallé le serveur ou changé ses clés, choisissez **Approuver la nouvelle clé**. Si vous ne savez pas pourquoi elle a changé, ne vous connectez pas : demandez à la personne qui gère le serveur. Voir [Hôtes SSH](/docs/hosts/ssh/#quand-la-clé-du-serveur-change).

### Identifiants perdus

La clé qui chiffre vos secrets manque dans le trousseau ou a changé — après une réinitialisation du trousseau, un nouveau profil utilisateur, ou la copie des données de Homeport vers un autre ordinateur (utilisez plutôt [Exporter et Importer](/docs/reference/security/#passer-à-un-autre-ordinateur) pour cela). Homeport conserve l’hôte ; saisissez de nouveau ses identifiants sur la page de l’hôte :

- **TLS** — chargez de nouveau les trois fichiers de certificat et choisissez **Enregistrer**.
- **SSH avec une clé importée** — collez de nouveau la clé privée et la phrase secrète et choisissez **Enregistrer**.
- **SSH avec une clé Homeport** — choisissez **Régénérer la clé**, puis autorisez la nouvelle clé sur le serveur.

Les environnements des stacks gérées scellés avec l’ancienne clé doivent aussi être saisis de nouveau.

## Clusters

### Accès refusé : « … a refusé les identifiants de son kubeconfig »

Le serveur d’API a répondu 401 : le certificat ou le jeton a expiré ou a été révoqué. Vérifiez que `kubectl --context <context> get namespaces` fonctionne toujours, puis utilisez **Actualiser depuis le kubeconfig** sur la page du cluster. Pour un plugin exec, Homeport lui a déjà demandé une fois un nouveau jeton.

### « … n’est pas installé sur cet ordinateur »

Le [plugin exec](/docs/clusters/exec-plugins/) du cluster (`aws`, `gke-gcloud-auth-plugin`, `kubelogin`…) n’a été trouvé ni dans le `PATH` ni à ses emplacements d’installation habituels. Installez-le, vérifiez qu’il s’exécute dans un terminal, puis **Reconnecter**.

### « … vous demande de vous connecter, ce qu’il ne peut pas faire dans Homeport »

Le plugin exec veut un navigateur ou un code d’appareil. Exécutez une fois `kubectl --context <context> get namespaces` dans un terminal, connectez-vous là, puis **Reconnecter**.

### « … n’a pas pu obtenir d’identifiants pour … »

Le plugin exec s’est exécuté et a échoué. **Détails** montre la fin de ce qu’il a affiché, souvent un profil expiré ou une mauvaise région. Homeport réessaie de lui-même.

### « … se connecte avec le fournisseur d’authentification gcp, que Kubernetes ne prend plus en charge »

Les fournisseurs d’authentification `gcp` et `azure` ont été retirés de Kubernetes. Récupérez de nouveau les identifiants avec `gke-gcloud-auth-plugin` installé (GKE), ou exécutez `kubelogin convert-kubeconfig` (AKS), puis ajoutez de nouveau le contexte.

### Vos identifiants n’autorisent pas « … » sur … dans …

Le RBAC du cluster ne permet pas à l’utilisateur de votre kubeconfig de faire cela. Choisissez un namespace que vous avez le droit de voir dans le sélecteur en haut à droite, ou demandez à l’administrateur du cluster. Le message donne la commande `kubectl auth can-i` qui le vérifie. Les autres pages continuent de fonctionner.

### Injoignable : « … a refusé la connexion » ou « … n’a pas répondu à temps »

Le serveur d’API n’est pas joignable depuis cet ordinateur : le cluster est arrêté (pour un cluster local), derrière un VPN, ou l’adresse du serveur dans le kubeconfig a changé. Homeport continue de réessayer de lui-même ; vérifiez `kubectl cluster-info` avec le même contexte.

### « Le certificat de … n’est pas signé par l’AC de son kubeconfig »

kind, k3d et minikube créent de nouveaux certificats chaque fois qu’ils recréent un cluster. Utilisez **Actualiser depuis le kubeconfig** sur la page du cluster.

### Nomad : « … ne connaît pas le jeton ACL envoyé par Homeport »

Le jeton a été mal saisi, a expiré, ou a été supprimé. Créez-en un nouveau avec la politique de [Politiques ACL](/docs/clusters/nomad/#politiques-acl) (`nomad acl token create -name=homeport -policy=homeport`) et collez son secret ID dans les **Paramètres** du cluster. Le cluster reste en **Accès refusé** jusqu’à ce que vous le fassiez.

### Nomad : « … a refusé : il manque au jeton ACL une permission nécessaire »

Les politiques du jeton n’accordent pas ce dont cette page ou cette action a besoin ; le message dit lequel (lire les jobs ou les journaux, exécuter ou arrêter des jobs, mettre à l’échelle, ouvrir un shell, modifier des nœuds). Ajoutez la capacité à la politique du jeton ; [Politiques ACL](/docs/clusters/nomad/#politiques-acl) liste ce dont chacune a besoin. Les autres pages continuent de fonctionner.

### Nomad : « … n’a pas de leader : ses serveurs n’arrivent pas à s’accorder sur l’un d’eux »

Moins d’une majorité des serveurs du cluster s’exécutent ou peuvent se joindre, donc Nomad ne peut pas répondre. Vérifiez-les avec `nomad server members` sur un serveur, et remettez-en suffisamment en service. Homeport continue de réessayer.

### Nomad : « … n’a pas de job … »

Le job a été arrêté et purgé, ou supprimé par le ramasse-miettes, ou il se trouve dans un autre namespace que celui choisi. Rouvrez la page Jobs, avec **Tous les namespaces** si besoin.

### Nomad : « Le groupe de tâches … ne peut pas en exécuter plus de … » ou « moins de … »

Le bloc `scaling` du groupe de tâches définit un `min` et un `max`, et le nombre demandé est en dehors. Modifiez les bornes dans la spécification du job et exécutez-le de nouveau pour aller plus loin.

### Nomad : « Nomad ne met pas … à l’échelle pendant qu’un de ses déploiements est en cours »

Exécuter une nouvelle version d’un job de service démarre un déploiement, et Nomad refuse de modifier son nombre d’instances tant que chaque allocation de la nouvelle version n’est pas en bonne santé. Attendez que les allocations du job se stabilisent, puis mettez de nouveau à l’échelle.

### Nomad : « La spécification du job ne peut pas être analysée, à la ligne …, colonne … »

L’analyseur de Nomad a trouvé un problème à cet endroit ; **Détails** contient son message. Les blocs `variable` ont besoin d’une valeur par défaut, puisque Homeport ne transmet pas de fichiers de variables. Une spécification JSON doit être un job au format de l’API de Nomad, tel que l’affiche `nomad job inspect`.

### Nomad : « Nomad ne peut pas placer toutes les allocations de … »

Le plan n’a trouvé aucun nœud éligible disposant des ressources que demande le groupe de tâches, ou correspondant à ses contraintes ; le plan dit lequel. **Exécuter quand même** met en file d’attente ce qui ne peut pas être placé jusqu’à ce qu’un nœud puisse le prendre ; sinon, réduisez les ressources, modifiez les contraintes, ou ajoutez des nœuds.

### Nomad : « … a changé depuis sa planification : il n’a donc pas été exécuté »

Quelqu’un d’autre a exécuté ou arrêté le job entre votre plan et **Exécuter**, et Homeport n’écrase pas sa modification. Revenez en arrière, planifiez de nouveau, et vérifiez le nouveau diff avant d’exécuter.

### Nomad : « La tâche … de l’allocation … n’est pas en cours d’exécution »

Un shell a besoin d’une tâche en cours d’exécution : celle-ci n’a pas démarré, s’est terminée, ou a échoué. Ses journaux restent lisibles jusqu’à ce que l’allocation soit supprimée par le ramasse-miettes.

### Nomad : « … ne peut pas joindre le nœud sur lequel s’exécute l’allocation … »

Les journaux et les shells passent par les serveurs jusqu’au nœud client, et les serveurs ne peuvent pas le joindre : le nœud est arrêté, déconnecté, ou son agent s’est arrêté. Vérifiez-le sur la page Nœuds ou avec `nomad node status`.

### Helm : « Le CLI helm n’est pas installé sur cet ordinateur »

Les [releases Helm](/docs/clusters/helm/) nécessitent `helm` 3 ou ultérieur sur cet ordinateur. Installez-le (`brew install helm`, `winget install Helm.Helm`, ou `snap install helm --classic`), puis rouvrez la page Helm ; **Paramètres → Cette machine** montre celui que Homeport a trouvé. Un `helm` 2 est signalé comme non pris en charge.

### Helm : « Une autre installation, mise à niveau ou un autre retour arrière de cette release est encore en cours »

La release est `pending-…` : une installation, une mise à niveau ou un retour arrière ne s’est pas terminé, souvent parce qu’il a été interrompu. helm refuse de la modifier tant que cela n’est pas terminé. Attendez, ou faites revenir la release à sa dernière révision déployée depuis son **Historique**.

## Utiliser Homeport

### Déployer est grisé : « Le CLI docker compose n’est pas installé »

Homeport a besoin de `docker compose` sur cet ordinateur pour déployer des stacks. Installez Docker Desktop ou le CLI Docker avec le plugin Compose, puis redémarrez Homeport. **Paramètres → Cette machine** indique s’il est trouvé.

S’il est installé mais introuvable, Homeport a peut-être un `PATH` différent de celui de votre terminal — fréquent quand il est lancé depuis un menu du bureau sous macOS. Assurez-vous que `docker` se trouve à un emplacement standard comme `/usr/local/bin`.

### Le déploiement échoue : « utilise build: »

Une stack gérée n’a pas de dossier à partir duquel construire. [Liez plutôt le dossier du projet](/docs/guides/stacks/#créer-une-stack-liée) — une stack liée peut construire — ou construisez l’image et poussez-la vers un registre, puis utilisez `image:`.

### Ma stack est devenue externe

Une [stack liée](/docs/guides/stacks/#quand-le-dossier-est-déplacé-ou-disparaît) apparaît comme externe, marquée ⚠ **Ses fichiers compose ne sont plus sur cet ordinateur**, quand son dossier a été déplacé, renommé ou supprimé, ou qu’un fichier compose qu’il contient l’a été. Rien n’est perdu : remettez le dossier là où il était et la stack redevient liée au prochain rafraîchissement. Si le dossier a été déplacé définitivement, choisissez **Oublier** dans l’onglet **Définition** de la stack, puis liez-la de nouveau depuis son nouvel emplacement avec **Nouvelle stack → Utiliser un dossier de cet ordinateur**.

Une stack démarrée depuis un dossier d’un autre ordinateur est toujours externe ici, puisque cet ordinateur n’a pas ce chemin.

### « Modifiés en dehors de Homeport »

Les fichiers compose d’une stack liée ont été modifiés en dehors de Homeport — dans votre éditeur, par `git pull` — depuis la dernière lecture de Homeport. Choisissez **Recharger** pour les voir tels qu’ils sont. Si vous les modifiiez dans Homeport au même moment, l’enregistrement est refusé avec **… a été modifié en dehors de Homeport depuis que vous l’avez ouvert**, pour qu’aucune des deux modifications ne soit perdue sans bruit : rechargez et refaites votre modification, ou choisissez **Écraser** pour garder la vôtre. Voir [modifier une stack liée](/docs/guides/stacks/#modifier-une-stack-liée).

### La récupération d’une image privée échoue

Ajoutez des identifiants pour le registre de l’image dans **Paramètres → Registres**, et utilisez **Vérifier les identifiants** pour confirmer que le registre les accepte. GHCR, et les comptes Docker Hub avec authentification à deux facteurs, nécessitent un jeton d’accès plutôt que le mot de passe. La boîte de dialogue de récupération indique quels identifiants elle utilisera ; si elle indique **de façon anonyme**, le registre de l’image ne correspond à aucun identifiant enregistré — `ghcr.io/org/app` a besoin d’identifiants pour `ghcr.io`, et `org/app` d’identifiants pour Docker Hub.

### Un build échoue

Le message nomme l’étape qui a échoué, comme **Le build a échoué à l’étape 3/5 (RUN make)** ; **Détails** contient la fin de la sortie du build. Homeport utilise le builder classique du moteur, donc les Dockerfiles qui ont besoin de BuildKit (`RUN --mount`, `--secret`, heredocs) échouent ; construisez-les depuis un terminal. Une image `FROM` privée a besoin d’identifiants pour son registre dans **Paramètres → Registres**.

### Aucune marque d’image plus récente n’apparaît

La marque ne s’affiche que lorsque le registre a été interrogé et a répondu. Les images construites localement ou récupérées par digest n’en affichent jamais. Si un registre est injoignable ou limite les requêtes (Docker Hub décompte ces vérifications de sa limite de récupérations), Homeport écrit une ligne dans son [journal](/docs/reference/security/#journaux) et n’affiche rien ; les réponses sont aussi conservées 6 heures, donc choisissez **Rechercher des images plus récentes** pour interroger de nouveau tout de suite. Des identifiants Docker Hub dans **Paramètres → Registres** relèvent la limite.

### La recherche sur Docker Hub affiche une erreur

La recherche s’exécute sur l’hôte sélectionné, donc elle a besoin de l’accès à Internet de cet hôte, et les tags sont lus depuis `hub.docker.com` par cet ordinateur. L’un comme l’autre peut être limité en débit ou bloqué par un proxy. La boîte de dialogue récupère quand même toute référence complète que vous saisissez, comme `postgres:17-alpine`.

### L’onglet Fichiers indique que le conteneur n’a pas d’outils find/stat

Les images minimales et distroless n’incluent pas `find` et `stat`, que Homeport utilise pour lister les fichiers. Utilisez `docker cp` pour ces conteneurs.

### Le terminal ne s’ouvre pas

Le conteneur doit être en cours d’exécution, et avoir `/bin/sh` ou `/bin/bash`. Utilisez **Nouveau terminal** pour exécuter un autre programme.

### Un volume ou un réseau refuse d’être supprimé

Le moteur refuse tant qu’un conteneur — même arrêté — l’utilise encore. Supprimez d’abord ces conteneurs ; la confirmation les liste.

### « Ceci nécessite Podman 5.8 ou ultérieur »

L’API de Podman lit, installe, modifie et supprime les [unités Quadlet](/docs/guides/quadlet/) à partir de Podman 5.8, et les liste à partir de 5.7. Mettez à jour Podman sur l’hôte, ou utilisez `podman quadlet` dans un terminal sur celui-ci. Démarrer et arrêter des unités fonctionne avec tous les Podman.

### Quadlet : Démarrer et Arrêter sont grisés

Homeport démarre et arrête le service d’une unité avec `systemctl`, qu’il ne peut exécuter que via SSH ou pour un Podman sur cet ordinateur (sous Linux). Joignez l’hôte via SSH plutôt que via TCP ou TLS. Une unité dont le service affiche **pas encore générée** n’a rien à démarrer : le générateur de systemd a ignoré son fichier.

### Quadlet : « Le systemd de l’utilisateur ne s’exécute pas sur l’hôte »

Les unités utilisateur (Podman rootless) ont besoin de l’instance systemd de l’utilisateur, qui ne s’exécute que pendant que l’utilisateur est connecté, sauf si le lingering est activé. En tant que cet utilisateur sur l’hôte, exécutez `loginctl enable-linger "$USER"`, puis réessayez.

### Quadlet : « systemd n’a pas autorisé cet utilisateur à gérer … »

Les unités système (Podman rootful) ne peuvent être démarrées et arrêtées que par un utilisateur autorisé à les gérer, comme `root`, et Homeport n’utilise jamais `sudo`. Connectez-vous en tant qu’un tel utilisateur, ou utilisez Podman rootless, dont les unités appartiennent à l’utilisateur.

### containerd : « n’est pas autorisé à utiliser containerd à /run/containerd/containerd.sock »

Le socket de containerd appartient à root. Donnez à un groupe l’accès au socket avec `gid` sous `[grpc]` dans `/etc/containerd/config.toml`, ajoutez votre utilisateur (ou l’utilisateur SSH) à ce groupe, redémarrez containerd et reconnectez-vous ; [la page containerd](/docs/hosts/containerd/#permettre-à-votre-utilisateur-dutiliser-le-socket) donne les commandes. containerd rootless ne peut pas du tout être joint : son socket n’existe qu’à l’intérieur de l’espace de noms de RootlessKit.

### containerd : « La sortie de ce conteneur n’est pas enregistrée »

containerd ne conserve aucun journal ; l’outil qui a démarré la tâche décide où va sa sortie. Les conteneurs créés avec `ctr` ne l’envoient en général nulle part. Les conteneurs de nerdctl et de Kubernetes journalisent dans des fichiers que Homeport lit ; voir [Journaux](/docs/hosts/containerd/#journaux).

### containerd : les journaux ou le terminal indiquent qu’ils ont besoin de SSH

Ils ont besoin des fichiers de la machine de containerd, que Homeport atteint via SSH, ou sur cet ordinateur pour un socket local sous Linux. Ajoutez l’hôte via SSH. Si le fichier journal ne peut pas être lu, il appartient en général à root : connectez-vous en tant qu’utilisateur autorisé à le lire.

### containerd : « ne peut pas récupérer d’images pour Homeport »

La récupération passe par le service Transfer de containerd, stable à partir de containerd 2.0. Mettez à jour containerd, ou récupérez l’image avec `ctr images pull` ou `nerdctl pull` sur l’hôte ; l’image apparaît ensuite sur la page Images.

### Incus : « n’est pas autorisé à utiliser Incus à /var/lib/incus/unix.socket »

Le socket d’administration d’Incus est destiné au groupe `incus-admin`, et le `unix.socket.user` restreint au groupe `incus`. Ajoutez votre utilisateur (ou l’utilisateur SSH) avec `sudo usermod -aG incus-admin "$USER"`, puis déconnectez-vous et reconnectez-vous, ou reconnectez-vous via SSH. Voir [les sockets d’Incus](/docs/hosts/incus/#ses-sockets).

### Incus : « Homeport n’a pas pu joindre Incus à … » lors d’un ajout avec un jeton de confiance

Incus n’écoute pas sur le réseau tant que `core.https_address` n’est pas défini. Sur le serveur, exécutez `incus config set core.https_address :8443`, vérifiez qu’un pare-feu laisse passer le port 8443, et créez un nouveau jeton : le jeton liste les adresses sur lesquelles Incus écoutait au moment de sa création.

### Incus : « Incus a refusé le jeton de confiance »

Un jeton ne sert qu’une fois, et expire. Créez-en un nouveau sur le serveur avec `incus config trust add homeport` et collez-le avant son expiration.

### Incus : « … a présenté un certificat d’empreinte …, et non …, celle approuvée »

Le certificat du serveur n’est pas celui que Homeport a approuvé : Incus a été réinstallé, son certificat a été renouvelé, ou autre chose répond à cette adresse. Vérifiez l’empreinte sur le serveur avec `incus info` ; si c’est celle qui est affichée, supprimez l’hôte et ajoutez-le de nouveau avec un nouveau jeton. Voir [certificats épinglés](/docs/hosts/incus/#le-certificat-du-serveur-est-épinglé).

### Incus : « Cet hôte ne peut pas exécuter de machines virtuelles »

Incus a besoin de KVM (`/dev/kvm`) et de QEMU pour les machines virtuelles. Activez la virtualisation dans le firmware du serveur (ou la virtualisation imbriquée pour un serveur virtuel), ou créez plutôt un conteneur système.

### Incus : « L’agent Incus de la machine virtuelle n’est pas en cours d’exécution »

Exec dans une machine virtuelle passe par l’`incus-agent` qu’elle contient, que les propres images d’Incus démarrent au boot. Attendez la fin du démarrage, ou utilisez l’onglet **Console**, qui n’a besoin d’aucun agent.

### « … n’est pas connecté »

L’action avait besoin de l’hôte, mais il s’est déconnecté. Attendez qu’il se reconnecte, ou choisissez **Réessayer maintenant**.
