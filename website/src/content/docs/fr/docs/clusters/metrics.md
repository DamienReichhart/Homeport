---
title: CPU et mémoire
description: Voyez ce que les Pods, leurs conteneurs et les Nodes utilisent de CPU et de mémoire, par rapport à leurs demandes, limites et à l’allouable, grâce à metrics-server.
---

Quand un cluster exécute [metrics-server](https://github.com/kubernetes-sigs/metrics-server), Homeport affiche ce que chaque Pod et chaque Node utilise de CPU et de mémoire : dans les listes, et en graphiques en direct sur leurs pages. Il lit l’API Metrics (`metrics.k8s.io`) que sert metrics-server, comme le fait `kubectl top`.

## Ce qu’il faut

metrics-server doit déjà tourner sur le cluster : Homeport n’installe rien sur un cluster. Sa présence dépend de la façon dont le cluster a été créé :

| Cluster                    | metrics-server                                                                            |
| -------------------------- | ----------------------------------------------------------------------------------------- |
| GKE                        | Installé                                                                                  |
| AKS                        | Installé                                                                                  |
| k3s et k3d                 | Installé (sauf si k3s a été lancé avec `--disable metrics-server`)                        |
| Docker Desktop             | Installé avec son Kubernetes                                                              |
| kind                       | Non installé                                                                              |
| EKS                        | Non installé : ajoutez le module complémentaire metrics-server, ou installez-le vous-même |
| minikube                   | Non installé : `minikube addons enable metrics-server`                                    |
| Autres clusters (kubeadm…) | En général non installé                                                                   |

Pour l’installer vous-même, suivez [ses instructions](https://github.com/kubernetes-sigs/metrics-server#installation) ; sur un cluster local dont les kubelets ont des certificats autosignés (kind), metrics-server a aussi besoin de `--kubelet-insecure-tls`.

Sans metrics-server, rien ne casse : les colonnes CPU et mémoire ne sont pas affichées, et l’onglet **Métriques** d’un Pod ou d’un Node indique que metrics-server n’est pas installé, avec un lien vers lui. Juste après son installation, metrics-server a besoin d’environ une minute avant d’avoir des échantillons ; en attendant, l’onglet indique que l’API Metrics ne répond pas encore.

Vos identifiants doivent aussi pouvoir lire les métriques : `list` et `get` sur `pods.metrics.k8s.io` dans le Namespace (ou dans tous les Namespaces), et sur `nodes.metrics.k8s.io` pour les Nodes. Quand ils ne le peuvent pas, l’onglet **Métriques** indique quelle permission manque, et toutes les autres pages continuent de fonctionner.

## Dans les listes

Sur **Pods**, les colonnes **CPU** et **Mémoire** montrent ce que chaque Pod utilise, ses conteneurs additionnés, et la part que cela représente de sa limite, ou de sa demande s’il n’a pas de limite. Un Pod n’a de limite que si tous ses conteneurs en ont une : un conteneur sans limite peut utiliser tout le Node. Le CPU est en millicœurs, comme l’écrit `kubectl top` (`250m` est un quart de cœur), et la mémoire est le working set, ce sur quoi le kubelet se fonde pour évincer des Pods.

Sur **Nodes**, **CPU utilisé** et **Mémoire utilisée** montrent ce que chaque Node utilise, et sa part de ce qui est **allouable** aux Pods.

Cliquez sur l’en-tête d’une colonne pour trier par elle : les Pods ou les Nodes les plus chargés d’abord.

## Sur la page d’un Pod ou d’un Node

L’onglet **Métriques** d’un Pod trace le CPU et la mémoire de chaque conteneur, avec une ligne pointillée à sa demande et une autre à sa limite, quand elles sont définies. Celui d’un Node trace son CPU et sa mémoire avec une ligne à ce qui est allouable.

metrics-server lit les kubelets toutes les 15 secondes, et Homeport demande un nouvel échantillon au même rythme : un changement de charge apparaît sur le graphique en moins de 30 secondes. Les échantillons sont gardés tant que la page est ouverte, 15 minutes au plus, quel que soit l’onglet affiché, et seulement en mémoire : fermer la page arrête les lectures et les oublie. Homeport ne garde aucun historique des métriques ; pour cela, utilisez un système de supervision comme Prometheus.
