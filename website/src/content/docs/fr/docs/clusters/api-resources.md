---
title: Ressources de l’API
description: Parcourez chaque type d’objet que sert un cluster, ressources personnalisées comprises, avec les colonnes qu’affiche kubectl get.
---

Homeport a ses propres pages pour les charges de travail, les Pods, les Services, les Ingress, les ConfigMaps, les Secrets, les Nodes et les Events. **Ressources de l’API**, dans la barre latérale d’un cluster, couvre tout le reste : les PersistentVolumes et leurs claims, les NetworkPolicies, les ServiceAccounts et le RBAC, et les ressources qu’ajoutent les opérateurs avec leurs CRD — les Certificates de cert-manager, les Applications d’Argo CD, les ServiceMonitors de Prometheus.

## Trouver un type

La colonne de gauche liste chaque type que sert le cluster, regroupé par groupe d’API (`core` d’abord), dans la version préférée de chaque groupe, comme le fait `kubectl api-resources`. Recherchez par kind, par pluriel ou par groupe, ou saisissez un nom court : `pvc`, `netpol`, `sa`, `cert`.

Un cadenas marque les types que vous ne pouvez pas lister : ceux que vos identifiants n’ont pas le droit de lister dans le namespace choisi (comme le cluster l’indique à Homeport par une `SelfSubjectRulesReview`), et les quelques types dont l’API ne permet aucune liste. En ouvrir un indique pourquoi. Certains clusters, comme ceux dont l’autorisation passe par un webhook, ne peuvent pas le dire ; rien n’est alors marqué, et une liste refusée l’indique.

Installer ou supprimer une CRD met la liste à jour d’elle-même ; **Actualiser** la relit.

## Lister des objets

Choisissez un type et ses objets sont listés avec les mêmes colonnes que `kubectl get` : le serveur d’API indique lesquelles, y compris les `additionalPrinterColumns` d’une CRD ; les Certificates de cert-manager affichent ainsi **Ready**, **Secret** et **Age** comme dans un terminal. La liste suit les changements en direct, par un watch.

Les types à namespace ont le sélecteur de namespace, comme les autres pages de Homeport ; avec **Tous les namespaces**, une colonne **Namespace** est ajoutée.

## La page d’un objet

Cliquez sur un objet pour ouvrir sa page :

- **Vue d’ensemble** : son namespace, les objets qui le possèdent — liés à leur page —, son âge, ses labels et ses annotations.
- **Conditions**, quand son `status` en a, comme pour les pages propres à Homeport.
- **Événements** le concernant, et son **YAML**, sans `managedFields`.

Quand son type le permet, **Modifier** ouvre son YAML pour le changer par une prévisualisation et un server-side apply (voir [Appliquer et modifier du YAML](/docs/clusters/yaml/)), et **Supprimer** le supprime après une confirmation qui le nomme, ainsi que le cluster (voir [Supprimer](/docs/clusters/actions/#supprimer)). Sur la liste, cochez plusieurs objets pour les supprimer ensemble.
