---
title: containerd
description: Se connecter à containerd sur cette machine ou via SSH, choisir un espace de noms, exécuter les tâches de ses conteneurs, lire leurs journaux, ouvrir un shell, et gérer images et instantanés.
---

**containerd** tourne sous k3s, nerdctl, BuildKit et de nombreux serveurs qui n’ont pas Docker. Homeport s’y connecte directement, via sa propre API gRPC, comme le décide l’[ADR 0020](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0020-containerd-over-grpc.md). Un hôte containerd a ses propres pages dans la barre latérale : **Conteneurs**, **Images**, **Instantanés** et **Espaces de noms**. Les pages de Docker, les stacks et le Tableau n’y sont pas proposés.

## Ajouter un hôte containerd

Choisissez **containerd** dans le champ **Moteur** du formulaire. Il joint containerd uniquement par son socket, donc la connexion est un **Socket local** ou **SSH** ; TCP et TLS sont grisés, parce que containerd ne sert pas son API sur le réseau.

| containerd                      | Socket                                                                         |
| ------------------------------- | ------------------------------------------------------------------------------ |
| Le service propre de containerd | `/run/containerd/containerd.sock`                                              |
| k3s                             | `/run/k3s/containerd/containerd.sock`                                          |
| Le containerd de Docker Engine  | `/run/containerd/containerd.sock`, ses conteneurs dans l’espace de noms `moby` |

Sous Linux, **Trouvés sur cette machine** liste le containerd de cet ordinateur et celui de k3s quand ils répondent. Un socket que vous ne pouvez pas encore utiliser est listé avec la façon d’y remédier.

### Permettre à votre utilisateur d’utiliser le socket

Le socket de containerd appartient à root. Pour l’utiliser en tant qu’autre utilisateur, en local ou via SSH, donnez à un groupe l’accès au socket dans `/etc/containerd/config.toml` et ajoutez l’utilisateur à ce groupe :

```bash
sudo groupadd -f containerd
sudo usermod -aG containerd "$USER"
getent group containerd | cut -d: -f3
```

Placez le nombre affiché sous `[grpc]` dans `/etc/containerd/config.toml`, redémarrez containerd, et reconnectez-vous :

```toml
[grpc]
  gid = 1001
```

```bash
sudo systemctl restart containerd
```

**containerd rootless** (`containerd-rootless.sh`) garde son socket à l’intérieur de l’espace de noms de RootlessKit, où rien d’extérieur ne peut le joindre, pas même Homeport. Homeport le signale quand il le trouve ; utilisez plutôt le service système de containerd.

### Ce que permet chaque version et chaque connexion

Homeport se connecte à containerd **1.7** et ultérieur. La récupération passe par le service Transfer de containerd, stable à partir de **2.0**.

| Action                                                     | containerd       | Connexion                          |
| ---------------------------------------------------------- | ---------------- | ---------------------------------- |
| Espaces de noms, conteneurs et tâches, images, instantanés | 1.7 et ultérieur | socket local ou SSH                |
| Récupérer une image                                        | 2.0 et ultérieur | socket local ou SSH                |
| Journaux et terminal                                       | 1.7 et ultérieur | SSH, ou un socket local sous Linux |

Un containerd trop ancien pour se connecter laisse l’hôte **Mal configuré**, en indiquant sa version ; un containerd trop ancien pour récupérer des images le signale quand vous en récupérez une.

## Espaces de noms

Chaque objet containerd vit dans un **espace de noms** : nerdctl utilise `default`, Kubernetes `k8s.io`, Docker `moby`, BuildKit `buildkit`. Les pages montrent un espace de noms à la fois, choisi sous le sélecteur d’hôte, qui liste chacun avec le nombre de conteneurs et d’images qu’il contient. Homeport retient ce choix pour chaque hôte d’un lancement à l’autre. Tant que vous n’en avez pas choisi, un hôte affiche `default`, ou le seul espace de noms qui contient quelque chose quand `default` est vide, donc un hôte k3s s’ouvre sur `k8s.io`.

La page **Espaces de noms** crée des espaces de noms vides et supprime ceux qui sont vides. containerd refuse de supprimer un espace de noms qui contient encore quelque chose, et Homeport indique ce qu’il contient encore : supprimez d’abord ses conteneurs, ses images et ses instantanés.

## Conteneurs et tâches

Dans containerd, un **conteneur** n’est que des métadonnées : son image, sa spécification, son instantané. Ce qui s’exécute, c’est sa **tâche**. La page **Conteneurs** liste les conteneurs de l’espace de noms avec leur tâche : **En cours d’exécution**, **En pause**, **Arrêtée** avec son code de sortie, ou **Aucune tâche**. Chacun indique l’outil qui l’a créé — **Kubernetes** (avec son Pod), **nerdctl**, **Docker** ou **ctr ou autre** — et les conteneurs pause des Pods sont masqués sauf si vous les demandez.

| Action                | Ce que fait Homeport                                                                                                                                                                                                                                                                                      |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Démarrer**          | Crée la tâche sur l’instantané du conteneur, en journalisant là où nerdctl journalise, et la démarre. Une tâche arrêtée est remplacée, comme `ctr task start` l’exige.                                                                                                                                    |
| **Arrêter**           | Envoie le signal d’arrêt de l’image (SIGTERM sauf si elle en nomme un autre), puis SIGKILL au bout de 10 secondes. Une tâche encore en vie après cela est signalée.                                                                                                                                       |
| **Mettre en pause**   | Fige la tâche ; **Reprendre** la libère.                                                                                                                                                                                                                                                                  |
| **Envoyer un signal** | Envoie n’importe quel signal, par son nom (`SIGHUP`) ou son numéro, au processus principal de la tâche.                                                                                                                                                                                                   |
| **Supprimer**         | Demande d’abord ; force l’arrêt d’une tâche en cours, supprime la tâche et le conteneur, et son instantané avec les fichiers qu’il a modifiés, sauf si vous décochez **Supprimer aussi** : un instantané conservé est marqué `containerd.io/gc.root`, pour que le ramasse-miettes de containerd le garde. |

La page d’un conteneur montre son environnement d’exécution, son instantané, ses labels et sa spécification OCI. Homeport ne crée pas de conteneurs sur containerd : containerd n’a pas de réseau propre, c’est nerdctl et CNI qui l’ajoutent.

Le **kubelet** gère les conteneurs de Kubernetes : il les redémarre après un arrêt et les recrée après une suppression. Leurs pages le signalent ; modifiez plutôt le Pod sur son cluster. Supprimer ici un conteneur nerdctl laisse les enregistrements propres à nerdctl (son nom et son réseau), comme le ferait `ctr` ; `nerdctl rm` est la façon la plus propre pour ceux-là.

## Journaux

containerd ne conserve aucun journal : l’outil qui démarre une tâche décide où va sa sortie. L’onglet **Journaux** la lit là où cet outil l’a écrite :

| Créé par                | Où va sa sortie                                                                            | Ce que Homeport affiche                                          |
| ----------------------- | ------------------------------------------------------------------------------------------ | ---------------------------------------------------------------- |
| nerdctl                 | un fichier journal JSON dans le dossier d’état de nerdctl, nommé par ses labels `nerdctl/` | ce fichier, suivi                                                |
| Kubernetes (`k8s.io`)   | `/var/log/pods/<pod>/<container>/N.log`, nommé dans l’enregistrement CRI                   | ce fichier, suivi                                                |
| `ctr` et autres clients | là où ils l’ont dirigée, en général nulle part                                             | **La sortie de ce conteneur n’est pas enregistrée**, et pourquoi |

Les conteneurs nerdctl qui journalisent vers un autre pilote (journald, fluentd) le nomment à la place. Homeport lit le fichier avec `tail` sur la machine de l’hôte (voir [ci-dessous](#ce-qui-sexécute-sur-lhôte)) ; les fichiers journaux appartiennent en général à root, donc l’utilisateur SSH doit avoir le droit de les lire.

## Un terminal dans un conteneur

L’onglet **Exec** ouvre un shell dans un conteneur en cours d’exécution — bash quand l’image l’a, sh sinon — ou la commande que vous indiquez, en tant qu’utilisateur numérique (`uid` ou `uid:gid`) et dans le dossier de votre choix. **Rouvrir** en démarre un nouveau avec ce que vous avez saisi.

containerd prend l’entrée et la sortie d’un processus sous forme de chemins sur sa machine. Homeport y exécute donc un court script qui crée deux FIFO dans un dossier temporaire privé, et y fait transiter le terminal ; containerd démarre le processus sur ces FIFO, à partir de la propre spécification de processus du conteneur. Fermer l’onglet force l’arrêt du processus et supprime le dossier.

## Images

La page **Images** liste les images de l’espace de noms avec leurs plateformes, les conteneurs créés à partir d’elles, et leur taille pour la plateforme de l’hôte, telle que la compte `ctr images ls`.

**Récupérer une image** récupère une image via containerd lui-même, puis la décompresse pour la plateforme de l’hôte dans son snapshotter par défaut, pour que `nerdctl run` puisse l’utiliser aussitôt. Donnez une référence complète (`ghcr.io/home/app:1.2`) ou courte (`nginx`, qui devient `docker.io/library/nginx:latest`). L’image d’un registre privé est récupérée avec ses identifiants de **Paramètres → Registres**, envoyés à containerd seulement quand le registre les demande, et jamais enregistrés sur l’hôte.

Une image à partir de laquelle un conteneur de l’espace de noms a été créé ne peut pas être supprimée ; supprimez d’abord les conteneurs. containerd libère le contenu d’une image dès que plus rien ne le retient.

## Instantanés

Les instantanés sont là où part le disque de containerd : les couches des images décompressées, et la couche modifiable de chaque conteneur. La page **Instantanés** montre ceux d’un snapshotter (`overlayfs` d’abord, ou tout autre que l’hôte a chargé) sous forme d’arbre par parent, les plus gros en premier, avec leur utilisation telle que la compte `ctr snapshot usage` et le total en haut.

Chacun indique ce qui l’utilise : un conteneur, par son instantané, ou une image, par ses couches. Les instantanés que rien n’utilise et sur lesquels rien n’est construit — comme celui conservé quand son conteneur a été supprimé ici — peuvent être affichés seuls avec **Inutilisés seulement**, et supprimés. Les autres ne peuvent pas être supprimés d’ici. Un instantané que rien ne référence et que rien ne marque comme racine, comme celui que laisse `ctr containers rm --keep-snapshot`, ne dure pas : le ramasse-miettes de containerd le supprime à son prochain passage.

## Ce qui s’exécute sur l’hôte

Tout ce qui précède passe par l’API de containerd, sauf les journaux et les terminaux, qui ont besoin des fichiers de la machine de containerd. Pour ceux-là, Homeport exécute ces commandes, sur la session SSH qu’il détient déjà, ou sur cet ordinateur pour un socket local sous Linux, avec les droits de l’utilisateur SSH et jamais avec `sudo` :

| Pour        | Commande                                                                                                                               |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| Journaux    | `tail -n <lines> [-F] -- '<log file>'` ; pour un conteneur Kubernetes sans enregistrement CRI, d’abord `ls` de son dossier de journaux |
| Un terminal | `mktemp -d`, `mkfifo -m 600` pour deux FIFO dans ce dossier, `cat` à travers elles, et `rm -rf` du dossier                             |

Un socket local sous macOS ou Windows ne peut pas les exécuter, et les journaux et terminaux indiquent alors qu’ils ont besoin de SSH. La [page de sécurité](/docs/reference/security/#ce-que-homeport-exécute-sur-vos-hôtes) liste chaque commande que Homeport exécute sur un hôte.

## Événements

Les pages se rafraîchissent à partir des propres événements de containerd, dans tous les espaces de noms : une tâche qui démarre ou se termine, un conteneur ou une image créé ou supprimé, un instantané validé. containerd n’en conserve aucun historique, donc ce qui s’est passé pendant que Homeport était fermé n’est pas complété.
