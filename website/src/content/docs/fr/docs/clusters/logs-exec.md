---
title: Journaux et terminaux
description: Suivez les journaux d’un Pod ou de toute une charge de travail, voyez pourquoi un conteneur a planté, et ouvrez un terminal dans le conteneur d’un Pod.
---

La page d’un Pod a les onglets **Journaux** et **Terminal**, et la page d’une charge de travail un onglet **Journaux** pour tous ses Pods : les `kubectl logs` et `kubectl exec -it` du quotidien, par l’API Kubernetes, avec les identifiants propres au cluster.

## Journaux

**Journaux** diffuse les journaux d’un conteneur au fil de leur écriture. C’est le même lecteur que pour un conteneur sur un hôte :

- **Historique** : commencer par les 100, 1 000 ou 10 000 dernières lignes, ou par toutes.
- **Depuis** : seulement les lignes des dernières minutes, heures ou jours.
- **Horodatage** : l’heure à laquelle Kubernetes a enregistré chaque ligne.
- **Suivre** : garder la ligne la plus récente visible ; désactivez-le pour relire pendant que de nouvelles lignes continuent d’arriver.
- **Rechercher dans les journaux**, **Effacer** et **Télécharger les journaux** (dans un fichier texte, les lignes affichées jusque-là).

Un Pod à plusieurs conteneurs a un sélecteur **Conteneur**, où les conteneurs d’init sont signalés. Il s’ouvre sur le conteneur que choisirait `kubectl` : celui que nomme l’annotation `kubectl.kubernetes.io/default-container` du Pod, sinon son premier conteneur applicatif.

Kubernetes garde ensemble la sortie standard et la sortie d’erreur d’un conteneur : il n’y a donc pas de filtre stdout/stderr comme pour les conteneurs d’un hôte.

Le flux s’arrête quand vous quittez l’onglet, et l’indique quand le conteneur s’arrête.

### Pourquoi a-t-il planté ?

Quand un conteneur a redémarré, **Instance précédente** affiche les journaux de l’exécution d’avant le dernier redémarrage — pour un Pod en `CrashLoopBackOff`, l’exécution qui a planté —, comme `kubectl logs --previous`. Elle est désactivée pour un conteneur qui n’a jamais redémarré, puisqu’il n’y a rien à afficher.

Quand un conteneur n’a pas encore démarré (`ContainerCreating`, `CrashLoopBackOff` pendant qu’il attend de redémarrer), les journaux l’indiquent sur place, avec la raison. La page de son Pod indique pourquoi il attend.

### Les journaux d’une charge de travail

L’onglet **Journaux** d’un Deployment, d’un StatefulSet, d’un DaemonSet ou d’un Job entremêle les journaux de tous ses Pods au fil de leur écriture, chaque ligne étiquetée avec son Pod dans sa propre couleur, comme `kubectl logs -l app=api --prefix`. Choisissez un Pod pour le lire seul, et le conteneur quand son modèle en a plusieurs.

## Terminal

**Terminal** ouvre un shell dans l’un des conteneurs en cours d’exécution du Pod : `bash` quand l’image l’a, `sh` sinon. Saisissez plutôt une commande dans le champ (`psql -U app shop`, `top`) et **Connecter** l’exécute, découpée comme le ferait un shell.

C’est un vrai TTY : `vi`, `top`, les couleurs et les touches fléchées fonctionnent, et redimensionner la fenêtre redimensionne le terminal dans le conteneur. Quand le processus se termine, le terminal indique avec quel code ; **Reconnecter** en démarre un autre. Quitter l’onglet met fin au processus dans le conteneur.

### Quand il ne peut pas s’ouvrir

- **Pas de shell** : les images minimales (distroless) n’ont aucun shell. Saisissez une commande que l’image possède, ou attachez un conteneur de débogage depuis un terminal avec `kubectl debug -it <pod> --image=busybox --target=<container>` ; Homeport ne crée pas de conteneurs de débogage.
- **Pas en cours d’exécution** : un conteneur en attente ou terminé ne peut rien exécuter. Les Pods d’un Job terminé se sont arrêtés pour de bon.
- **Non autorisé** : ouvrir un terminal demande `create` sur `pods/exec`, et lire les journaux `get` sur `pods/log`, ce qui manque souvent aux rôles en lecture seule. Homeport indique lequel, avec la commande `kubectl auth can-i` pour vérifier.

Homeport ne copie pas de fichiers vers un Pod ni depuis un Pod (`kubectl cp`).
