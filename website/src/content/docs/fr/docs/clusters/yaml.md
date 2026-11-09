---
title: Appliquer et modifier du YAML
description: Appliquez des manifestes à un cluster ou modifiez le YAML d’un objet, en voyant ce qui change avant que quoi que ce soit ne change, avec le server-side apply.
---

Deux façons de changer un cluster avec du YAML, pour tout ce que les actions dédiées ne couvrent pas : **Appliquer du YAML** pour les manifestes, comme `kubectl apply -f`, et **Modifier** sur l’onglet **YAML** de n’importe quel objet, comme `kubectl edit`. Les deux montrent ce qui changerait avant que quoi que ce soit ne change.

## Appliquer du YAML

**Appliquer du YAML**, dans la barre latérale d’un cluster, prend un ou plusieurs manifestes : collez-les, ou choisissez **Ouvrir un fichier…** pour charger un fichier `.yaml`, `.yml` ou `.json` de cet ordinateur. Plusieurs documents séparés par `---` sont appliqués ensemble, dans l’ordre.

Un document sans namespace va dans le namespace choisi en haut à droite (ou, avec **Tous les namespaces**, dans le namespace par défaut du cluster). Les objets qui ne sont pas dans un namespace, comme un Namespace ou un ClusterRole, l’ignorent.

1. **Prévisualiser les changements** demande au cluster ce que ferait chaque document, par un dry run côté serveur : rien ne change encore. Chaque document s’affiche **Créé**, **Modifié** ou **Aucun changement**, avec les lignes qu’il changerait.
2. **Appliquer** vous demande de confirmer, en nommant le cluster, puis applique.

Appliquer deux fois les mêmes manifestes indique **Aucun changement** la seconde fois. Modifier le YAML après une prévisualisation en demande d’abord une nouvelle.

## Modifier un objet

L’onglet **YAML** de chaque objet propose **Modifier** : son YAML tel que le cluster le détient, sans ce que gère le serveur (`managedFields`, `status`, son `resourceVersion`, son uid et sa date de création). Changez-le, choisissez **Prévisualiser les changements** pour ne voir que les lignes que vous avez changées, puis **Appliquer**.

La prévisualisation compare votre YAML à l’objet tel qu’il est au moment de la prévisualisation : un changement fait entre-temps apparaît donc dans le diff. Si la modification change un champ qu’un autre outil gère, la prévisualisation le liste comme un conflit au lieu de l’écraser.

Les Secrets ne peuvent pas être modifiés dans Homeport, puisqu’il n’affiche jamais toutes leurs valeurs à la fois ; et les Events sont écrits par Kubernetes lui-même.

## Server-side apply

Homeport applique avec le server-side apply de Kubernetes, sous le field manager `homeport` : le cluster enregistre quels champs Homeport a définis, à côté de ceux définis par `kubectl`, Helm, des contrôleurs ou d’autres outils, ce que vous verrez dans les `managedFields` d’un objet.

Quand un document change un champ qu’un autre outil gère — `.spec.replicas` défini par `kubectl apply` ou géré par un autoscaler —, la prévisualisation indique quels champs et quels managers. Omettez ces champs, ou choisissez **Appliquer quand même** après une seconde confirmation : Homeport en prend le contrôle, et l’autre outil peut les redéfinir.

## Quand un document ne peut pas être appliqué

La prévisualisation affiche le problème de chaque document sur place, et **Appliquer** reste désactivé tant que chaque document ne peut pas être appliqué :

- **YAML non valide**, avec la ligne où il casse.
- **Kind inconnu** : le cluster ne sert pas cet `apiVersion` et ce `kind`. Une ressource personnalisée nécessite d’abord l’installation de sa CRD.
- **Refusé par le cluster** : la réponse de sa validation pour chaque champ, comme un champ obligatoire manquant ou une valeur du mauvais type.
- **Non autorisé** : appliquer demande `patch` sur la ressource (et `create` pour les nouveaux objets), avec la commande `kubectl auth can-i` pour vérifier.
