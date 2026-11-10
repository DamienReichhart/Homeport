---
title: Unités Quadlet
description: Lister les unités Quadlet d’un hôte Podman avec leur état systemd, lire leurs fichiers, les installer, les modifier et les supprimer, et démarrer ou arrêter leurs services.
---

**Quadlet** est la façon dont Podman exécute des services de longue durée sous systemd. Une unité Quadlet est un fichier — `web.container`, `shop.pod`, `cache.volume`, `lan.network`, `app.kube`, `base.image`, `api.build` — que le générateur de systemd transforme en service, `web.service`, qui démarre le conteneur au démarrage de la machine et le redémarre quand il s’arrête. Podman rootful garde ses unités dans `/etc/containers/systemd/` ; Podman rootless garde celles de chaque utilisateur dans `~/.config/containers/systemd/`.

La page **Quadlet** apparaît dans la barre latérale quand un hôte Podman est sélectionné. Elle passe par l’API propre à Podman sur le socket que l’hôte utilise déjà, et par `systemctl` pour démarrer et arrêter, comme le décide lâ[ADR 0019](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0019-quadlet-units-through-podmans-api-and-systemctl.md).

## Ce que permet chaque version de Podman et chaque connexion

L’API de Podman a acquis la prise en charge de Quadlet en deux versions, et elle ne démarre ni n’arrête jamais le service d’une unité. Ce que Homeport peut faire dépend donc de la version de Podman de l’hôte et de la façon dont Homeport le joint :

| Action                                         | Podman                     | Connexion                          |
| ---------------------------------------------- | -------------------------- | ---------------------------------- |
| Lister les unités avec leur état               | 5.7 et ultérieures         | toutes                             |
| Lister les unités à partir de leurs conteneurs | 4.0 à 5.6                  | toutes                             |
| Lire le fichier d’une unité                    | 5.8 et ultérieures         | toutes                             |
| Installer, modifier et supprimer des unités    | 5.8 et ultérieures         | toutes                             |
| Démarrer, arrêter et redémarrer un service     | toutes celles avec Quadlet | SSH, ou un socket local sous Linux |

Avec Podman antérieur à 5.7, les unités sont trouvées grâce au label `PODMAN_SYSTEMD_UNIT` que Podman pose sur les conteneurs qu’elles créent, si bien qu’une unité n’apparaît qu’une fois qu’elle a créé un conteneur, nommée d’après son service. La page le signale au-dessus de la liste.

Démarrer et arrêter demandent `systemctl`, que Homeport exécute là où Podman tourne : par la session SSH d’un hôte SSH, ou sur cet ordinateur pour un socket local sous Linux. Un hôte TCP ou TLS, ou un socket local sous macOS ou Windows (où Podman tourne dans une machine virtuelle), peut faire tout le reste ; les boutons sont désactivés, avec la raison.

## La page Quadlet

| Colonne        | Affiche                                                                                                                                                      |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Unité**      | Le nom de fichier de l’unité, et l’application avec laquelle elle a été installée.                                                                           |
| **Type**       | `container`, `pod`, `volume`, `network`, `kube`, `image`, `build` ou `artifact`.                                                                             |
| **Service**    | Le service systemd qu’elle génère, ou **pas encore générée** tant que systemd ne l’a pas chargée.                                                            |
| **État**       | **En cours d’exécution**, **Démarrage**, **Arrêt en cours**, **Arrêtée**, **En échec**, **Non chargée par systemd** ou **Modèle**, selon ce que dit systemd. |
| **Conteneurs** | Combien de conteneurs elle a créés.                                                                                                                          |

Chaque ligne propose **Démarrer** pour une unité arrêtée ou **Arrêter** pour une unité en cours d’exécution. Sélectionnez une unité pour ouvrir sa page. La liste se met à jour d’elle-même quand ses conteneurs démarrent et s’arrêtent.

Une unité **Non chargée par systemd** est installée, mais le générateur de systemd l’a ignorée : en général à cause d’une erreur dans le fichier. Sur l’hôte, `systemctl --user daemon-reload` (ou `systemctl daemon-reload` pour Podman rootful) puis `journalctl` indiquent pourquoi.

Sur la page **Conteneurs**, un conteneur créé par une unité affiche le service de l’unité à côté de son nom ; sélectionnez-le pour ouvrir l’unité.

## La page d’une unité

- L’en-tête propose **Démarrer**, **Arrêter** et **Redémarrer**, **Modifier** et **Supprimer**.
- **Service** montre l’unité systemd, s’il s’agit d’une unité système ou d’une unité utilisateur (`systemctl --user`), l’emplacement de son fichier sur l’hôte, et son application et son pod quand elle en a.
- **Fichier de l’unité** montre le fichier tel que Podman le lit, avec coloration syntaxique.
- **Conteneurs** liste les conteneurs qu’elle a créés, chacun ouvrant sa page de conteneur.

Arrêter le conteneur d’une unité depuis la page Conteneurs va à l’encontre de systemd, qui le redémarre en vertu de `Restart=`. Arrêtez plutôt l’unité.

## Installer une unité

**Nouvelle unité** (ou `n` sur la page Quadlet, ou **Nouvelle unité** dans la palette de commandes) ouvre un éditeur avec un point de départ pour le type que vous choisissez, ou un fichier que vous ouvrez depuis cet ordinateur avec **Ouvrir un fichier…**. Nommez-le avec son extension, comme `web.container` ; le fichier a besoin de la section de son type, `[Container]` pour un `.container`.

**Ajouter des fichiers…** place d’autres fichiers à côté de l’unité — un `Containerfile` pour un `.build`, le YAML d’un `.kube`, un fichier d’environnement — dans un dossier d’application nommé d’après l’unité, comme le fait `podman quadlet install`.

Podman écrit les fichiers là où vont les unités de son utilisateur et recharge systemd, si bien que le service existe tout de suite. L’installation ne le démarre pas : choisissez **Démarrer** une fois qu’il est là. Une unité dont le nom est déjà pris est refusée ; modifiez plutôt celle-là.

## Modifier et supprimer

**Modifier** ouvre le fichier de l’unité. **Enregistrer** l’écrit sur place et recharge systemd ; les fichiers installés à côté restent. Un service en cours d’exécution garde ses anciens réglages jusqu’à ce que vous le **Redémarriez**. Si le fichier a changé sur l’hôte depuis que vous l’avez ouvert, **Enregistrer** est refusé et le dit, plutôt que d’écraser cette modification : rouvrez l’unité pour la voir, ou choisissez **Écraser**.

**Supprimer** efface le fichier de l’unité et recharge systemd. **Arrêter d’abord son service en cours d’exécution** est activé par défaut ; sans cela, Podman refuse de supprimer une unité en cours d’exécution. Les conteneurs, volumes et images créés par l’unité sont conservés.

## Ce que Homeport exécute sur l’hôte

Pour **Démarrer**, **Arrêter** et **Redémarrer**, et rien d’autre, Homeport exécute une commande sur le serveur de l’hôte en SSH (ou sur cet ordinateur pour un socket local), avec les droits de l’utilisateur SSH :

```bash
systemctl --user start 'web.service'
```

`--user` est présent pour les unités d’un Podman rootless et omis pour celles d’un Podman rootful ; `start` devient `stop` ou `restart` pour ces actions. Homeport vérifie le nom du service et le met entre guillemets, et n’utilise jamais `sudo`. Ainsi :

- Les **unités système** (Podman rootful) demandent un utilisateur SSH autorisé à les gérer, comme `root`.
- Les **unités utilisateur** (Podman rootless) demandent l’instance systemd de l’utilisateur. Pour qu’elle tourne sans session ouverte, et pour que les unités démarrent au démarrage de la machine, activez le lingering une fois, en tant que cet utilisateur sur l’hôte :

```bash
loginctl enable-linger "$USER"
```

La [page sur la sécurité](/docs/reference/security/#ce-que-homeport-exécute-sur-vos-hôtes) liste chaque commande que Homeport exécute sur un hôte.
