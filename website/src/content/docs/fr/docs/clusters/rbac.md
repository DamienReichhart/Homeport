---
title: RBAC
description: Voyez les Roles, les bindings et les sujets d’un cluster, ce que chaque sujet peut faire et où, et ce que vos propres identifiants peuvent faire.
---

**RBAC**, dans la barre latérale d’un cluster Kubernetes, montre qui peut faire quoi sur le cluster. La page est en lecture seule : Homeport ne crée, ne modifie ni ne supprime d’objet RBAC (vous pouvez toujours les modifier en YAML depuis [Ressources de l’API](/docs/clusters/api-resources/)).

Comme les autres pages, elle suit le sélecteur de namespace : avec un namespace choisi, elle montre les Roles et les RoleBindings de ce namespace, ainsi que les ClusterRoles et les ClusterRoleBindings, qui s’appliquent partout.

## Roles

**Roles** liste les Roles (chacun dans son namespace) et les ClusterRoles, avec leur nombre de règles. Ouvrez-en un pour sa page :

- ses **règles**, telles que Kubernetes les écrit : ressources, groupes d’API (`core` pour le groupe core), noms de ressources et verbes, ou URL hors ressources comme `/healthz` ;
- les **bindings qui l’utilisent**, chacun lié à ses sujets.

Un ClusterRole marqué **agrégé** a une `aggregationRule` : sa page montre ses sélecteurs de labels, les ClusterRoles qu’ils sélectionnent, et ses règles comprennent les leurs. Les `admin`, `edit` et `view` de Kubernetes sont construits ainsi : le ClusterRole d’une CRD portant le label `rbac.authorization.k8s.io/aggregate-to-edit: "true"` apparaît dans `edit`.

## Bindings

**Bindings** liste les RoleBindings et les ClusterRoleBindings, chacun avec le rôle qu’il accorde, lié, et les sujets qu’il nomme : Users, Groups et ServiceAccounts. Un binding dont le rôle n’existe pas est marqué **introuvable** ; il n’accorde rien.

## Sujets

**Sujets** liste tous ceux que nomme un binding. Ouvrez un User, un Group ou un ServiceAccount pour voir ce qu’il peut faire, sous forme de matrice des ressources et des verbes, regroupée par portée : **Tout le cluster** d’abord, puis chaque namespace. En dessous, les bindings qui le lui accordent, et comment : en le nommant directement, ou par un groupe.

Homeport fusionne les règles de chaque binding qui s’applique :

- Un ClusterRoleBinding accorde son ClusterRole partout : dans chaque namespace, sur les ressources de portée cluster comme les Nodes, et sur les URL hors ressources.
- Un RoleBinding accorde son rôle dans son propre namespace seulement, même quand il lie un ClusterRole. Un ServiceAccount lié à `edit` par un RoleBinding dans `prod` peut modifier dans `prod`, et nulle part ailleurs.
- Les bindings des groupes où Kubernetes place chaque sujet comptent aussi : un ServiceAccount est dans `system:serviceaccounts`, `system:serviceaccounts:<son namespace>` et `system:authenticated`, et il est aussi l’utilisateur `system:serviceaccount:<namespace>:<nom>` ; un User est dans `system:authenticated`.
- `*` désigne tous les verbes, groupes d’API ou ressources, `*/scale` cette sous-ressource de toutes les ressources, et une règle avec des noms de ressources ne s’applique qu’à ces objets (la matrice les montre sous **Seulement**). Une ligne déjà couverte par une plus large est omise.

**C’est une vue au mieux.** Les permissions d’un sujet sont calculées à partir des bindings que vos identifiants peuvent lire, et non demandées au serveur d’API : elles ne demandent donc aucun droit d’emprunter l’identité de quiconque. Elles peuvent manquer ce qu’autorisent d’autres autorisateurs, comme un webhook (fréquent sur les clusters managés) ou ABAC, et les groupes qu’un authentificateur donne à un utilisateur (depuis les claims OIDC ou l’organisation d’un certificat client), que seul le serveur d’API connaît. Pour être sûr de vos propres identifiants, utilisez **Que puis-je faire ici ?**.

Avec un namespace choisi, seuls les RoleBindings de ce namespace sont lus ; choisissez **Tous les namespaces** pour voir partout les permissions d’un sujet.

## Que puis-je faire ici ?

**Que puis-je faire ici ?** est la réponse du serveur d’API lui-même pour les identifiants qu’utilise Homeport, dans le namespace choisi (celui par défaut du cluster avec **Tous les namespaces**), comme l’affiche `kubectl auth can-i --list`. La page indique sous quelle identité ils se connectent et dans quels groupes, sur les clusters qui le disent (Kubernetes 1.28 et suivants), avec un lien vers la page de ce sujet.

Quand le serveur d’API indique que sa liste peut être incomplète — un autorisateur comme un webhook peut autoriser plus qu’il ne sait lister —, la page le dit.

**Puis-je… ?** pose une question, comme `kubectl auth can-i` : un verbe, une ressource et un namespace (vide pour tous les namespaces), et en option un groupe d’API, une sous-ressource et le nom d’un objet. La ressource peut être son pluriel (`secrets`), un nom court (`deploy`) ou son pluriel avec son groupe (`deployments.apps`) ; Homeport la résout comme kubectl. La réponse est accompagnée de la même question sous forme de commande à copier, comme `kubectl auth can-i delete secrets --namespace prod`, qui donne la même réponse.

Quand vos identifiants ne peuvent lister aucun objet RBAC, la page ne montre que **Que puis-je faire ici ?**, sous un message qui nomme ce qui a été refusé. Quand seuls certains peuvent être listés, par exemple les RoleBindings mais pas les ClusterRoles, la page indique lesquels, et ce qu’elle montre les laisse de côté.
