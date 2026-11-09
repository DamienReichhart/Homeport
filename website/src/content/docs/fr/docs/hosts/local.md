---
title: Moteurs locaux
description: Se connecter à Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop ou Podman sur cette machine.
---

Un hôte **Socket local** parle à un moteur de cette machine via son socket Unix, ou un pipe nommé sous Windows. Il n’a besoin d’aucun identifiant ni d’aucun trousseau.

## Détection automatique

Homeport cherche des moteurs en cours d’exécution aux emplacements habituels et liste ceux qui répondent sous **Trouvés sur cette machine**, sur l’écran d’accueil et sur la page **Ajouter un hôte**. Choisissez **Ajouter** à côté de l’un d’eux pour créer un hôte qui porte son nom.

| Environnement            | macOS et Linux                                                                                                        | Windows                           |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Docker Engine            | `/var/run/docker.sock`                                                                                                | —                                 |
| Docker Engine (rootless) | `$XDG_RUNTIME_DIR/docker.sock`                                                                                        | —                                 |
| Docker Desktop           | `~/.docker/desktop/docker.sock`, `~/.docker/run/docker.sock`                                                          | `//./pipe/docker_engine`          |
| Colima                   | `~/.colima/default/docker.sock`                                                                                       | —                                 |
| OrbStack                 | `~/.orbstack/run/docker.sock`                                                                                         | —                                 |
| Rancher Desktop          | `~/.rd/docker.sock`                                                                                                   | —                                 |
| Podman                   | `$XDG_RUNTIME_DIR/podman/podman.sock`, `/run/podman/podman.sock` ; [machines sous macOS](#machines-podman-sous-macos) | `//./pipe/podman-machine-default` |

Chaque candidat est vérifié en demandant sa version au moteur, donc seuls les moteurs en cours d’exécution sont listés. Quand plusieurs chemins mènent au même socket — sous macOS, `/var/run/docker.sock` pointe en général vers Docker Desktop, OrbStack ou Rancher Desktop — le moteur est listé une seule fois, sous le premier nom trouvé.

### Machines Podman sous macOS

Sous macOS, Podman exécute son moteur dans une VM (`podman machine`) dont le socket change avec chaque machine. Homeport demande au CLI `podman` le socket de chaque machine en cours d’exécution (`podman machine list` et `podman machine inspect`) et liste chacune sous la forme **Podman (_nom de la machine_)**, par exemple **Podman (podman-machine-default)**. Les machines arrêtées ne sont pas listées ; démarrez-en une avec `podman machine start` et ouvrez de nouveau **Ajouter un hôte**.

Si Homeport ne peut pas exécuter `podman` — il n’est ni dans son PATH, ni dans `/opt/homebrew/bin` ou `/usr/local/bin` — il regarde plutôt là où Podman place les sockets : `$TMPDIR/podman/<machine>-api.sock` pour Podman 5, et `~/.local/share/containers/podman/machine/…/podman.sock` pour Podman 4. Quand `podman-mac-helper` a relié `/var/run/docker.sock` à une machine, cette machine est listée une seule fois, sous son nom Podman.

## Ajouter un socket à la main

Si votre moteur n’est pas détecté — un profil Colima autre que `default`, une machine Podman sous Windows avec un autre nom que celui par défaut, ou un chemin personnalisé — ajoutez-le vous-même :

1. Choisissez **Ajouter un hôte…** dans le sélecteur d’hôte.
2. Sélectionnez **Socket local**.
3. Saisissez un **Nom** et le **Chemin du socket** : un socket Unix comme `/run/user/1000/docker.sock`, ou un pipe nommé comme `//./pipe/docker_engine` sous Windows.
4. Choisissez **Ajouter un hôte**.

Pour trouver le socket qu’utilise votre CLI Docker, exécutez :

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

Pour une machine Podman :

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Permissions sous Linux

Le socket par défaut `/var/run/docker.sock` appartient au groupe `docker`. Si votre utilisateur n’en fait pas partie, l’hôte affiche **Injoignable** avec _Permission denied_. Ajoutez-vous au groupe et reconnectez-vous :

```sh
sudo usermod -aG docker $USER
```

Faire partie du groupe `docker` équivaut à être root sur cette machine ; Docker rootless ou Podman l’évitent si cela compte pour vous.
