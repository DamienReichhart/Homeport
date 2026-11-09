---
title: Mise à l’échelle, redémarrage et suppression
description: Mettez à l’échelle des Deployments et des StatefulSets, redémarrez un déploiement, suspendez et exécutez des CronJobs, et supprimez n’importe quel objet d’un cluster, chaque fois après une confirmation qui le nomme.
---

Les changements courants d’un cluster sont à un clic sur ses pages, chacun envoyé à l’API Kubernetes avec les identifiants propres au cluster, comme le ferait `kubectl`. Les listes suivent le changement en direct par leur watch.

Chaque action qui supprime ou remplace quelque chose demande d’abord confirmation, et la confirmation nomme le cluster, le namespace, le kind et le nom : un changement n’atterrit ainsi jamais par accident sur le mauvais cluster ou le mauvais namespace.

## Charges de travail

La page d’une charge de travail porte ses actions dans l’en-tête ; sur **Charges de travail**, chaque ligne les a dans son menu **⋯**.

| Action                  | Sur                                   | Fait, comme                                                               |
| ----------------------- | ------------------------------------- | ------------------------------------------------------------------------- |
| **Mettre à l’échelle**  | Deployments, StatefulSets             | `kubectl scale --replicas=N`                                              |
| **Redémarrer**          | Deployments, StatefulSets, DaemonSets | `kubectl rollout restart`                                                 |
| **Suspendre**           | CronJobs                              | `kubectl patch cronjob … -p '{"spec":{"suspend":true}}'`                  |
| **Reprendre**           | CronJobs suspendues                   | la même chose, avec `false`                                               |
| **Exécuter maintenant** | CronJobs                              | `kubectl create job --from=cronjob/…`                                     |
| **Supprimer**           | Tous                                  | `kubectl delete` (ses Pods et ses Jobs partent avec lui, en arrière-plan) |

### Mettre à l’échelle

**Mettre à l’échelle** demande combien de Pods exécuter, en indiquant combien il en exécute actuellement. Mettre à l’échelle à `0` arrête la charge de travail jusqu’à ce que vous la remettiez à l’échelle. Si un HorizontalPodAutoscaler gère la charge de travail, la boîte de dialogue l’indique : l’autoscaler remettra le nombre à sa valeur lors de sa prochaine vérification ; modifiez plutôt le minimum et le maximum de l’autoscaler.

### Redémarrer

**Redémarrer** ajoute l’annotation `kubectl.kubernetes.io/restartedAt` au modèle de Pod, exactement comme le fait `kubectl rollout restart` : la charge de travail déploie progressivement un nouvel ensemble de Pods selon sa stratégie (un nouveau ReplicaSet pour un Deployment), et les anciens Pods partent une fois les nouveaux prêts.

### CronJobs

**Suspendre** empêche une CronJob de lancer de nouveaux Jobs (ceux déjà en cours continuent) et **Reprendre** la laisse à nouveau les lancer ; une CronJob suspendue s’affiche **En pause**. **Exécuter maintenant** lance immédiatement un Job à partir de son modèle, nommé `<cronjob>-manual-<id>`, possédé par la CronJob et marqué `cronjob.kubernetes.io/instantiate: manual` comme ceux de kubectl. Il apparaît sous l’onglet **Jobs** de la CronJob.

## Supprimer

Chaque page d’objet — une charge de travail, un Pod, un Service, un Ingress, une ConfigMap, un Secret, un Node, ou tout objet sous **Ressources de l’API** — a **Supprimer** dans son en-tête. Sur une liste, cochez des lignes et **Supprimer N** les supprime toutes après une seule confirmation qui les liste.

Les objets sont supprimés avec la propagation `Background`, comme le fait `kubectl delete` : l’objet part immédiatement et le ramasse-miettes supprime ce qu’il possède (les ReplicaSets et les Pods d’un Deployment, les Jobs d’une CronJob).

La confirmation indique ce qui se passe ensuite :

- **Un Pod possédé par un contrôleur** — un ReplicaSet, un StatefulSet, un DaemonSet ou un Job — revient : le contrôleur en lance un nouveau à sa place. Supprimez la charge de travail ou mettez-la à l’échelle pour vous en débarrasser.
- **Un Namespace** emporte tout ce qu’il contient : ses charges de travail, ses Pods, ses Services, ses ConfigMaps et ses Secrets.
- **Un Node** est retiré de la liste du cluster sans être drainé au préalable.

## Quand cela ne fonctionne pas

- **Non autorisé** : un changement que vos identifiants ne peuvent pas faire l’indique avec son verbe et sa ressource — « Vos identifiants n’autorisent pas “patch” sur deployments dans shop. » — et la commande `kubectl auth can-i` pour vérifier.
- **Modifié entre-temps** : quand quelqu’un d’autre a modifié l’objet entre votre consultation et votre action, Homeport l’indique ; regardez-le à nouveau et réessayez.
- **Déjà supprimé** : supprimer quelque chose de déjà supprimé indique qu’il n’existe plus.
