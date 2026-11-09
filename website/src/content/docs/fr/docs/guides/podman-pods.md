---
title: Pods Podman
description: Lister, démarrer, arrêter et supprimer des pods Podman, en créer un avec ses ports publiés, et y exécuter des conteneurs.
---

Un **pod** est la façon dont Podman regroupe des conteneurs : ils partagent un même réseau (ainsi que leur nom d’hôte et l’IPC), si bien qu’ils se joignent les uns les autres sur `localhost`, et ils démarrent et s’arrêtent ensemble. Chaque pod a un **conteneur infra** qui détient ce qu’ils partagent et les ports que le pod publie. Homeport affiche les pods sur les hôtes dont le moteur est Podman, par l’API propre à Podman sur le socket que l’hôte utilise déjà ; rien n’est installé sur l’hôte.

La page **Pods** n’apparaît dans la barre latérale que lorsqu’un hôte Podman est sélectionné. Sur tout autre moteur compatible Docker, elle n’est pas là, et un lien vers elle explique pourquoi. Voir [Types de moteur](/docs/hosts/#types-de-moteur) pour savoir comment Homeport reconnaît Podman.

## La page Pods

| Colonne        | Affiche                                                                                                                                                  |
| -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Nom**        | Le nom du pod, avec son état sous forme de pastille.                                                                                                     |
| **État**       | **En cours d’exécution**, **Partiellement en cours d’exécution** (certains conteneurs sont arrêtés), **En pause**, **Arrêté**, **Créé** ou **En échec**. |
| **Conteneurs** | Combien de ses propres conteneurs tournent, sans compter le conteneur infra.                                                                             |
| **Réseaux**    | Les réseaux auxquels le pod est rattaché.                                                                                                                |
| **Créé**       | Quand il a été créé.                                                                                                                                     |

Chaque ligne a l’action qui correspond à son état — **Démarrer** pour un pod arrêté, **Arrêter** pour un pod en cours d’exécution — et **Supprimer**. Sélectionnez un pod pour ouvrir sa page. La liste se met à jour d’elle-même quand les conteneurs du pod démarrent et s’arrêtent, que ce soit depuis Homeport ou depuis un terminal.

Sur la page **Conteneurs**, un conteneur qui tourne dans un pod affiche le nom du pod à côté du sien ; sélectionnez ce nom pour ouvrir le pod.

## La page d’un pod

L’en-tête propose **Exécuter un conteneur** et les actions du pod : **Démarrer**, **Arrêter**, **Redémarrer**, **Mettre en pause**, **Reprendre** et **Supprimer**, selon ce que son état permet. Chacune agit à la fois sur tous les conteneurs du pod, comme le font `podman pod start` et ses semblables.

- **Conteneurs** liste les conteneurs du pod, son conteneur infra en dernier. Sélectionnez-en un pour ouvrir sa page de conteneur, avec ses journaux, son terminal et ses fichiers.
- **Paramètres** montre les ports que publie le pod, les espaces de noms que partagent ses conteneurs, ses réseaux, son nom d’hôte et ses labels, ainsi que la commande `podman pod create` avec laquelle il a été créé, quand Podman l’a conservée.

## Supprimer un pod

**Supprimer** demande d’abord confirmation et indique combien de conteneurs partent avec le pod. Un pod en cours d’exécution est d’abord arrêté : le bouton indique **Arrêter et supprimer**. Les volumes du pod sont conservés.

## Créer un pod

**Nouveau pod** (ou `n` sur la page Pods, ou **Nouveau pod** dans la palette de commandes) demande :

- un **Nom** : des lettres, des chiffres, `_`, `.` et `-`, en commençant par une lettre ou un chiffre ;
- les **Ports** à publier, port de l’hôte vers port du conteneur, comme dans la boîte de dialogue Exécuter. Laissez le port de l’hôte vide pour laisser Podman en choisir un libre ;
- des **Labels**, un `key=value` par ligne.

Les ports appartiennent au pod : ils sont publiés sur son conteneur infra et atteignent celui des conteneurs du pod qui écoute sur ce port. La page du nouveau pod s’ouvre, prête à y exécuter des conteneurs. Un pod n’a pas de conteneurs à lui tant que vous n’en exécutez pas un ; Podman le démarre avec le premier.

## Exécuter un conteneur dans un pod

Sur un hôte Podman, la boîte de dialogue **Exécuter** a un champ **Pod**, et le bouton **Exécuter un conteneur** d’un pod ouvre la boîte de dialogue avec ce pod déjà choisi. Un conteneur dans un pod :

- partage le réseau du pod, si bien qu’il ne peut ni publier de ports ni rejoindre un autre réseau : la boîte de dialogue masque ces champs dès qu’un pod est choisi ;
- utilise le nom d’hôte et les réglages DNS du pod, si bien que Homeport refuse les siens ;
- garde tout le reste de ce que propose la boîte de dialogue Exécuter : commande, environnement, montages, politique de redémarrage, limites, capacités, périphériques, health check et journalisation.

Dupliquer et Modifier ne déplacent jamais un conteneur dans un pod : Podman ne peut pas déplacer un conteneur existant dans un pod.

## Ce dont Homeport a besoin

Les pods fonctionnent avec Podman 4.0 et versions ultérieures, sur tous les types de connexion — socket local, SSH, TLS ou TCP —, puisqu’ils passent par le même socket que tout le reste.
