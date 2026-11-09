---
title: Clavier
description: Piloter Homeport sans la souris — la palette de commandes et la recherche sur tous les hôtes, des raccourcis pour chaque page et chaque hôte, et la fiche d’aide qui les liste.
---

Homeport peut s’utiliser au clavier. Appuyez sur <kbd>Mod</kbd> <kbd>K</kbd> pour ouvrir la [palette de commandes](#palette-de-commandes), et sur <kbd>?</kbd> sur n’importe quelle page pour voir tous les raccourcis qui y fonctionnent.

**Mod** désigne ci-dessous <kbd>⌘</kbd> sur macOS et <kbd>Ctrl</kbd> sur Windows et Linux ; sur macOS, <kbd>Alt</kbd> correspond à <kbd>⌥</kbd>.

## Palette de commandes

<kbd>Mod</kbd> <kbd>K</kbd> ouvre la palette de commandes depuis n’importe quelle page. Tapez ce que vous voulez, déplacez-vous avec <kbd>↑</kbd> et <kbd>↓</kbd>, puis appuyez sur <kbd>Enter</kbd> ; <kbd>Esc</kbd> la ferme et vous ramène là où vous étiez. Elle liste :

- **Aller à** — chaque page de la barre latérale : tapez `vol` et appuyez sur <kbd>Enter</kbd> pour ouvrir Volumes.
- **Hôtes** — chaque hôte avec son état : tapez une partie du nom d’un hôte pour passer dessus.
- **Actions** — **Exécuter un conteneur**, **Récupérer une image**, **Nouvelle stack**, **Nettoyer l’hôte sélectionné**, **Ajouter un hôte**, **Gérer les hôtes**, **Rechercher des mises à jour** et **Afficher les raccourcis clavier**.
- **Paramètres** — **Thème : clair**, **Thème : sombre** et **Thème : suivre le système**.

Les actions qui ont besoin d’un hôte — exécuter un conteneur, récupérer une image, créer une stack, nettoyer — ne sont listées que si l’hôte sélectionné est connecté.

### Rechercher sur tous les hôtes

Tapez au moins deux caractères et la palette cherche aussi parmi les **conteneurs**, les **stacks**, les **images**, les **volumes** et les **réseaux** de tous les hôtes connectés — « où tourne `postgres` ? » a une réponse sans changer d’hôte. Elle porte sur les noms, les ID courts et, pour les conteneurs, leur image et leur stack ; les noms qui commencent par ce que vous avez tapé viennent en premier.

- Les résultats sont regroupés par type, cinq par type jusqu’à **Afficher les …**. Chacun indique son hôte et, pour les conteneurs et les stacks, s’ils tournent : _prod-server · en cours d’exécution_.
- En choisir un fait passer sur son hôte et l’ouvre : la page d’un conteneur ou d’une stack, ou la page Images, Volumes ou Réseaux filtrée sur lui.
- Les hôtes qui ne sont pas connectés sont ignorés, et la palette indique combien : _2 hôtes non parcourus (non connecté)_.

Homeport ne charge les listes qu’une fois deux caractères tapés, puis les tient à jour grâce aux événements en direct de chaque hôte. Les journaux, les fichiers et les définitions Compose ne sont pas parcourus. Chaque élément affiche son raccourci, s’il en a un. Sans rien taper, les éléments utilisés en dernier viennent en premier, sous **Récents**, jusqu’à ce que Homeport quitte.

## Raccourcis

### Partout

| Touches                                    | Effet                                                                                       |
| ------------------------------------------ | ------------------------------------------------------------------------------------------- |
| <kbd>?</kbd>                               | Afficher les raccourcis clavier                                                             |
| <kbd>Mod</kbd> <kbd>1</kbd> … <kbd>9</kbd> | Ouvrir les pages de la barre latérale dans l’ordre : Vue d’ensemble, Conteneurs, Images…    |
| <kbd>Mod</kbd> <kbd>,</kbd>                | Ouvrir les Paramètres                                                                       |
| <kbd>Alt</kbd> <kbd>↑</kbd> / <kbd>↓</kbd> | Sélectionner l’hôte précédent ou suivant, dans l’ordre où la page Hôtes les liste           |
| <kbd>Mod</kbd> <kbd>R</kbd>                | Actualiser les listes de l’hôte sélectionné : conteneurs, images, volumes, réseaux, stacks… |

Survolez une page dans la barre latérale pour voir ses touches.

### Sur une page

| Touches      | Effet                                                                                                                                                                     |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <kbd>/</kbd> | Placer le curseur dans le champ de recherche de la page. <kbd>Esc</kbd> dans un champ vide le quitte.                                                                     |
| <kbd>N</kbd> | Déclencher l’action principale de la page : **Exécuter un conteneur**, **Récupérer**, **Créer un volume**, **Créer un réseau**, **Nouvelle stack** ou **Ajouter un hôte** |

Le bouton de l’action principale d’une page affiche <kbd>N</kbd> quand vous le survolez.

### Pendant la saisie

Les touches seules — <kbd>N</kbd>, <kbd>/</kbd>, <kbd>?</kbd> et les flèches avec <kbd>Alt</kbd> — ne se déclenchent jamais pendant que vous tapez dans un champ, dans l’éditeur d’une stack ou dans une boîte de dialogue : elles tapent le caractère à la place. Les raccourcis avec <kbd>Mod</kbd> y fonctionnent aussi. Dans le **Terminal** d’un conteneur, chaque touche va au conteneur et aucun raccourci ne se déclenche.
