---
title: Hôtes et connexions
description: Comment Homeport se connecte aux Docker Engines, à containerd et à Incus, ce que signifie chaque état d’hôte, et comment fonctionne la reconnexion.
---

Un **hôte** est un Docker Engine, un containerd ou un Incus auquel Homeport se connecte. Il a un nom et une façon de le joindre. Chaque conteneur, instance, image, volume, réseau et stack que vous voyez appartient à exactement un hôte.

## Types de connexion

| Type             | Joindre un moteur…                                     | Identifiants                                                                  | Guide                                                           |
| ---------------- | ------------------------------------------------------ | ----------------------------------------------------------------------------- | --------------------------------------------------------------- |
| **Socket local** | sur cette machine, via un socket ou un pipe            | aucun                                                                         | [Moteurs locaux](/docs/hosts/local/)                            |
| **SSH**          | sur un serveur auquel vous pouvez vous connecter       | une clé Homeport, votre propre clé, ou ssh-agent                              | [SSH](/docs/hosts/ssh/)                                         |
| **TLS**          | exposé avec `dockerd --tlsverify`, ou le HTTPS d’Incus | certificat d’AC, certificat client et clé ; pour Incus, un jeton de confiance | [TLS et TCP](/docs/hosts/tls-tcp/), [Incus](/docs/hosts/incus/) |
| **TCP**          | exposé sans chiffrement, sur un réseau de confiance    | aucun                                                                         | [TLS et TCP](/docs/hosts/tls-tcp/)                              |

Pour un serveur distant, préférez **SSH** : il n’expose rien au-delà du port SSH et c’est le plus simple à configurer de façon sûre.

## Types de moteur

Outre la façon de le joindre, chaque hôte indique le type de moteur vers lequel il pointe, dans le champ **Moteur** du formulaire. Cela décide des pages que la barre latérale affiche pour cet hôte.

| Moteur                | Ce qu’il couvre                                                               | Pages                                                                                                         |
| --------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| **Compatible Docker** | Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop, Podman      | Conteneurs, Images, Volumes, Réseaux, Stacks, Événements ; et sur Podman, aussi ses propres pages             |
| **containerd**        | le service propre de containerd, celui de k3s, le containerd de Docker Engine | Conteneurs, Images, Instantanés et Espaces de noms de containerd ; voir [containerd](/docs/hosts/containerd/) |
| **Incus**             | Incus 6.0 et ultérieur : conteneurs système et machines virtuelles            | Instances, Images, Profils, Stockage et Réseaux ; voir [Incus](/docs/hosts/incus/)                            |

Le formulaire choisit le moteur d’après le chemin du socket quand celui-ci nomme le socket de containerd ou d’Incus. containerd ne se joint que par son socket, sur cet ordinateur ou via SSH, donc TCP et TLS sont grisés pour lui ; Incus par son socket ou via HTTPS (TLS), donc seul TCP est grisé. Les hôtes ajoutés avant Homeport 0.18 sont compatibles Docker, sans autre changement.

**Podman** est un moteur compatible Docker que Homeport reconnaît à chaque connexion, parce que le même socket peut changer de produit : l’état de l’hôte affiche alors **Podman** et sa version au lieu de celle du moteur, et la barre latérale ajoute les pages que seul Podman propose. Une page que l’hôte sélectionné ne peut pas afficher n’apparaît pas dans la barre latérale, et l’ouvrir depuis un lien explique pourquoi.

Un hôte qui fait partie d’un Docker Swarm affiche **Manager de** ou **Worker de** son Swarm, et ajouter un seul manager comme hôte suffit pour que le Swarm apparaisse dans la barre latérale. La page d’un hôte a aussi une carte **Swarm** pour démarrer un Swarm, en rejoindre un ou le quitter. Voir [Swarms](/docs/guides/swarm/).

## Ajouter, modifier et supprimer des hôtes

- **Ajoutez** un hôte avec **Ajouter un hôte…** dans le sélecteur d’hôte en haut de la barre latérale, ou **Ajouter un hôte** sur la vue d’ensemble.
- **Gérer les hôtes** dans le sélecteur d’hôte liste chaque hôte avec son type, son adresse et son état. Sélectionnez-en un pour ouvrir sa page.
- Sur la page d’un hôte, vous pouvez modifier ses détails de connexion et choisir **Enregistrer** ; Homeport se reconnecte avec les nouveaux paramètres. Le type de connexion d’un hôte ne peut pas être modifié — supprimez-le et ajoutez-le de nouveau à la place.
- L’interrupteur **Activé** déconnecte un hôte sans l’oublier. Un hôte désactivé ne réessaie pas tant que vous ne le réactivez pas.
- **Supprimer l’hôte** oublie l’hôte, ses identifiants enregistrés, les définitions compose de ses stacks gérées, ses stacks liées et l’historique de déploiement de ses stacks. Rien de ce qui s’exécute sur l’hôte n’est touché : les conteneurs, images et volumes restent tels quels, de même que les dossiers des stacks liées. Les stacks d’un Swarm que l’hôte gère appartiennent au Swarm, et sont conservées.

Les noms d’hôte doivent être uniques, sans tenir compte de la casse.

## Importer des hôtes

**Ajouter un hôte** offre deux façons d’ajouter plusieurs hôtes d’un coup, à partir de ce que cet ordinateur décrit déjà. Chacune ouvre un aperçu : choisissez les entrées à importer, renommez-les, et choisissez **Importer**.

- **Importer depuis ~/.ssh/config** ajoute les serveurs que nomme votre configuration SSH, avec leur utilisateur, leur port, leur clé et leurs bastions `ProxyJump`. Voir [Importer depuis ~/.ssh/config](/docs/hosts/ssh/#importer-depuis-sshconfig).
- **Importer les contextes Docker** ajoute les moteurs des contextes du CLI Docker (`docker context ls`), avec leurs certificats TLS.

Les clés et certificats sont lus une fois, au moment de l’import, et enregistrés chiffrés comme les autres ; ni `~/.ssh` ni `~/.docker` ne sont jamais modifiés.

### Contextes Docker

Homeport lit chaque contexte sous `~/.docker/contexts` (ou sous `$DOCKER_CONFIG` s’il est défini) et convertit son point d’accès Docker :

| Point d’accès                                            | Devient                                                            |
| -------------------------------------------------------- | ------------------------------------------------------------------ |
| `unix:///…`, `npipe:////./pipe/…`                        | un hôte **Socket local**                                           |
| `tcp://host:port` sans fichiers TLS                      | un hôte **TCP**                                                    |
| `tcp://host:port` avec `ca.pem`, `cert.pem` et `key.pem` | un hôte **TLS**, avec ses certificats                              |
| `ssh://user@host:port`                                   | un hôte **SSH** qui utilise ssh-agent, avec `/var/run/docker.sock` |

Le contexte intégré `default` n’est pas proposé : la [détection locale](/docs/hosts/local/) trouve déjà le moteur de cette machine. Un contexte est affiché mais ne peut pas être choisi quand :

- il définit `SkipTLSVerify`, puisque Homeport vérifie toujours le certificat du moteur ;
- son dossier TLS ne contient qu’une partie des trois fichiers ;
- son point d’accès est d’un autre type, comme un point d’accès Kubernetes ;
- un hôte joint déjà le même moteur. Son nom indique lequel.

Un contexte dont le nom est déjà celui d’un hôte est proposé sous la forme `name (2)`.

## Passer à un autre ordinateur

**Exporter** sur la page Hôtes enregistre les hôtes que vous choisissez dans un fichier protégé par une phrase secrète ; **Importer** sur la page Hôtes de l’autre ordinateur l’ouvre et ajoute les hôtes que vous choisissez, prêts à se connecter. Un hôte dont le nom y est déjà pris est proposé sous la forme `name (2)`, et vous pouvez le renommer ou l’ignorer. Voir [passer à un autre ordinateur](/docs/reference/security/#passer-à-un-autre-ordinateur) pour la façon dont le fichier est protégé.

## Sélectionner un hôte

Chaque page de ressources — Conteneurs, Images, Volumes, Réseaux, Stacks et Événements — montre l’hôte sélectionné dans le **sélecteur d’hôte**. Les pages d’un hôte containerd montrent aussi un seul espace de noms à la fois, choisi [sous le sélecteur](/docs/hosts/containerd/#espaces-de-noms), et celles d’un hôte Incus un seul [projet](/docs/hosts/incus/#projets). Changez d’hôte à cet endroit, ou sélectionnez la carte d’un hôte sur la [vue d’ensemble](/docs/guides/overview/). Homeport retient votre choix d’un lancement à l’autre.

## États

Chaque hôte affiche un état à côté de son nom. Quand l’hôte sélectionné n’est pas connecté, les pages de ressources affichent son état avec **Réessayer maintenant** et **Ouvrir les paramètres de l’hôte**.

| État                             | Signification                                                                               | Ce qui se passe ensuite                                                                  |
| -------------------------------- | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| **Connecté**                     | Le moteur a répondu. Sa version est affichée, ou celle de Podman, de containerd ou d’Incus. | Homeport continue de surveiller la connexion.                                            |
| **Connexion**                    | Homeport est en train de joindre le moteur.                                                 | —                                                                                        |
| **Désactivé**                    | L’interrupteur **Activé** de l’hôte est désactivé.                                          | Rien tant que vous ne l’activez pas.                                                     |
| **Injoignable**                  | Le réseau, le socket ou le moteur n’a pas répondu.                                          | Réessaie de lui-même, en attendant plus longtemps à chaque fois.                         |
| **Accès refusé**                 | Le serveur SSH a refusé la clé, ssh-agent a échoué, ou la négociation TLS a échoué.         | Corrigez la cause, puis **Reconnecter**.                                                 |
| **Clé du serveur non approuvée** | Première connexion SSH : la clé du serveur ou d’un bastion doit être approuvée par vous.    | [Approuvez la clé](/docs/hosts/ssh/#approuver-la-clé-du-serveur) sur la page de l’hôte.  |
| **Clé du serveur modifiée**      | Le serveur SSH a présenté une clé différente de celle que vous avez approuvée.              | [Enquêtez d’abord](/docs/hosts/ssh/#quand-la-clé-du-serveur-change).                     |
| **Identifiants perdus**          | Les identifiants enregistrés ne peuvent pas être déchiffrés.                                | [Saisissez-les de nouveau](/docs/reference/troubleshooting/#identifiants-perdus).        |
| **Trousseau indisponible**       | L’hôte a besoin de secrets enregistrés mais il n’y a pas de trousseau du système.           | Voir la [configuration requise](/docs/requirements/#facultatif-un-trousseau-du-système). |
| **Mal configuré**                | Un élément des paramètres de l’hôte manque ou n’est pas valide.                             | Le message dit lequel ; corrigez-le et **Enregistrer**.                                  |

## Reconnexion

Homeport se connecte à chaque hôte activé au démarrage, et chaque hôte a sa propre connexion, donc un serveur lent ne retarde jamais les autres.

Une fois connecté, Homeport écoute le flux d’événements du moteur — c’est ainsi que les pages se mettent à jour d’elles-mêmes — et vérifie le moteur toutes les 15 secondes. Si la connexion tombe, l’hôte devient **Injoignable** et Homeport réessaie après 1 seconde, puis 2, 4, 8 et ainsi de suite, jusqu’à une fois par minute. Une connexion réussie remet l’attente à zéro.

Seul **Injoignable** réessaie automatiquement. Tous les autres problèmes demandent une action de votre part, puis **Reconnecter** (sur la page de l’hôte) ou **Réessayer maintenant** réessaie aussitôt.

## Événements et mises à jour en direct

Homeport n’interroge pas le moteur à intervalles réguliers. Quand le moteur signale un changement — un conteneur qui démarre, une image en cours de récupération — les pages concernées se rafraîchissent en une fraction de seconde. La page [Événements](/docs/guides/overview/#événements) montre le flux brut.
