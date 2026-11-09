---
title: Plugins exec
description: Comment Homeport se connecte aux clusters EKS, GKE et AKS avec le plugin d’identifiants exec que nomme leur kubeconfig, et ce qui s’exécute où.
---

Les kubeconfigs cloud ne contiennent ni mot de passe ni certificat : leur utilisateur nomme un **plugin exec**, un programme de votre ordinateur qui affiche un jeton de courte durée quand on le lui demande. Homeport exécute le même programme que `kubectl`, sur cet ordinateur, selon le protocole `ExecCredential` de Kubernetes (`client.authentication.k8s.io/v1` et `v1beta1`).

## Fournisseurs courants

| Fournisseur | L’utilisateur du kubeconfig exécute  | Installation                                                                 |
| ----------- | ------------------------------------ | ---------------------------------------------------------------------------- |
| EKS         | `aws eks get-token --cluster-name …` | l’AWS CLI, version 2                                                         |
| GKE         | `gke-gcloud-auth-plugin`             | la Google Cloud CLI, puis `gcloud components install gke-gcloud-auth-plugin` |
| AKS         | `kubelogin get-token …`              | `az aks install-cli`, ou kubelogin depuis ses releases                       |

Écrivez le kubeconfig comme d’habitude — `aws eks update-kubeconfig`, `gcloud container clusters get-credentials`, `az aks get-credentials` suivi de `kubelogin convert-kubeconfig` —, puis [ajoutez ses contextes](/docs/clusters/).

## Ce qui s’exécute où

- Le plugin s’exécute **sur cet ordinateur**, sous votre compte, avec les arguments et les variables d’environnement que lui donne le kubeconfig (`AWS_PROFILE`, par exemple). Homeport ne lui demande jamais d’être interactif et ne lui fournit aucune entrée.
- Un lanceur de bureau démarre Homeport avec un `PATH` réduit : Homeport cherche donc aussi dans les emplacements d’installation habituels des CLI AWS, Google Cloud et Azure et de kubelogin (Homebrew, le dossier propre au SDK, `~/.local/bin`, `~/.krew/bin`, Program Files…). Un plugin indiqué par son chemin complet dans le kubeconfig s’exécute depuis cet emplacement.
- Le jeton ou le certificat qu’il affiche est conservé **en mémoire uniquement**, jusqu’à peu avant son expiration ; le plugin s’exécute alors à nouveau. Il s’exécute une fois par durée de vie de jeton, et non une fois par requête. Quand l’API refuse un jeton, Homeport demande une fois un nouveau jeton au plugin avant d’afficher **Accès refusé**.
- Les caches propres au plugin (`~/.kube/cache`, `~/.aws/sso/cache`, la configuration de gcloud) le regardent ; Homeport n’y écrit rien.

## Quand il ne peut pas se connecter

- **« … n’est pas installé sur cet ordinateur »** — le programme est introuvable. Installez-le (le message indique comment, avec l’`installHint` du kubeconfig quand il en a un), vérifiez qu’il fonctionne dans un terminal, puis choisissez **Reconnecter**.
- **« … vous demande de vous connecter, ce qu’il ne peut pas faire dans Homeport »** — le plugin veut un navigateur ou un code d’appareil (AWS SSO, connexion par appareil Azure, une session `gcloud` expirée). Lancez `kubectl` une fois dans un terminal avec ce contexte, connectez-vous-y, puis choisissez **Reconnecter** : le plugin trouve alors sa connexion en cache.
- **« … n’a pas pu obtenir d’identifiants pour … »** — il s’est exécuté mais a échoué ; **Détails** affiche la fin de ce qu’il a écrit. Homeport réessaie de lui-même.

Voir [Dépannage](/docs/reference/troubleshooting/#clusters) pour les autres messages.
