---
title: Releases Helm
description: Listez, installez, mettez à niveau, ramenez à une révision antérieure et désinstallez les releases Helm d’un cluster, et gérez les dépôts de charts, avec le CLI helm de cet ordinateur.
---

**Helm**, dans la barre latérale d’un cluster, affiche les releases [Helm](https://helm.sh) qui s’y trouvent : les mêmes releases que `helm list` affiche dans un terminal, qu’elles aient été installées depuis Homeport ou non.

## Ce dont Homeport a besoin

Homeport exécute la commande `helm`, en version 3 ou ultérieure, sur **cet** ordinateur. Installez-la avec `brew install helm` sur macOS, `winget install Helm.Helm` (ou Chocolatey, ou Scoop) sur Windows, et `snap install helm --classic`, le paquet de votre distribution ou le script de helm.sh sur Linux. **Paramètres → Cette machine** affiche le helm que Homeport a trouvé ; sans helm, la page Helm indique quoi installer pour votre système, et toutes les autres pages du cluster fonctionnent.

Homeport ne donne pas votre kubeconfig à helm. Pour chaque commande helm, il ouvre une connexion privée au cluster sur l’adresse de loopback de cet ordinateur et donne à helm un kubeconfig temporaire qui pointe vers elle, contenant un jeton à usage unique et aucun identifiant du cluster. Homeport ajoute au passage les identifiants propres au cluster — certificats, jetons, jetons des plugins exec — à chaque requête, comme pour toutes les autres pages, et supprime le kubeconfig quand la commande se termine. helm joint ainsi chaque cluster que Homeport joint, clusters cloud avec plugins exec compris, et aucun identifiant n’est écrit sur le disque.

Helm conserve ses releases sous forme de Secrets dans le Namespace de chaque release : Homeport n’installe rien sur le cluster.

## Releases

La liste affiche pour chaque release son nom, son Namespace, son chart et sa version de chart, sa version d’application, sa révision, son état et la date de sa dernière modification. Choisissez un Namespace en haut à droite, ou tous. helm n’a pas de mises à jour en direct : la liste redemande donc toutes les 30 secondes tant qu’elle est ouverte, et **Actualiser** redemande immédiatement.

Une release **failed** est marquée en rouge, et une release **pending** (en attente d’une installation, d’une mise à niveau ou d’un retour arrière : `pending-install`, `pending-upgrade`, `pending-rollback`) en orange : une release en attente est verrouillée jusqu’à la fin de cette opération.

Si vos identifiants ne peuvent pas lister les Secrets dans tous les Namespaces, Homeport liste à la place les releases du Namespace par défaut du cluster et l’indique ; choisissez un autre Namespace pour voir ses releases.

## Installer un chart

**Installer un chart** (ou <kbd>n</kbd>), sur l’onglet Releases, installe un chart sur le cluster sélectionné :

1. **Choisir le chart.** Recherchez parmi les charts des [dépôts](#dépôts-de-charts) de helm et cliquez sur **Utiliser**, ou, sous **Depuis un registre OCI**, donnez une référence comme `oci://registry-1.docker.io/bitnamicharts/nginx`, qui s’installe sans ajouter de dépôt.
2. **Choisir sa version, son nom et son Namespace.** Les versions d’un chart de dépôt sont listées, la plus récente d’abord ; la version d’un chart OCI est saisie, ou laissée vide pour la plus récente. La release est d’abord nommée d’après le chart : au plus 53 lettres minuscules, chiffres, tirets et points. Le Namespace est celui qu’affiche la page (ou celui par défaut du cluster), et il est créé s’il n’existe pas, sauf si vous décochez cette option.
3. **Modifier ses valeurs.** L’éditeur commence avec les valeurs par défaut du chart (`helm show values`) ; changez ce dont vous avez besoin, et **Rétablir les valeurs par défaut du chart** les remet. Les valeurs sont vérifiées comme YAML avant que helm ne s’exécute, et sont transmises à helm sur son entrée standard — jamais dans un fichier.
4. **Prévisualiser**, si vous le souhaitez : helm rend le chart avec vos valeurs et l’aide du cluster (`--dry-run=server`), et Homeport affiche le manifeste qu’il appliquerait, hooks compris, ainsi que les notes du chart. Rien ne change sur le cluster.
5. **Installer.** La sortie de helm s’affiche au fur et à mesure, comme `helm install` l’écrit. Avec **Attendre que ses ressources soient prêtes**, helm attend que les Deployments, Pods et Services de la release soient prêts, pendant au plus le nombre de minutes indiqué (5 par défaut), et marque la release en échec s’ils ne le sont pas. **Ouvrir la release** ouvre ensuite sa page.

Les charts d’un dossier local ou d’un fichier `.tgz` ne s’installent pas depuis Homeport ; les charts de registres OCI qui demandent une connexion non plus, sauf si vous vous êtes connecté avec `helm registry login` dans un terminal.

## La page d’une release

Cliquez sur une release pour l’ouvrir :

- **Historique** — chaque révision, la plus récente d’abord, avec sa date, son état, son chart, sa version d’application et la description par helm de ce qui s’est passé (`Install complete`, `Upgrade "web" failed: …`).
- **Valeurs** — les valeurs données à l’installation ou à la mise à niveau (**Fournies par l’utilisateur**), ou toutes les valeurs avec lesquelles le chart a été rendu, valeurs par défaut comprises (**Calculées, avec les valeurs par défaut du chart**).
- **Notes** — ce que le chart affiche après une installation (son `NOTES.txt`).
- **Manifeste** — les objets Kubernetes rendus par la révision, en YAML.
- **Ressources** — ces objets par kind, nom et Namespace, chacun ouvrant sa page dans Homeport : sa propre page pour les Deployments, Pods, Services et autres, sa page sous [Ressources de l’API](/docs/clusters/api-resources/) pour le reste.

Valeurs, Notes, Manifeste et Ressources affichent la révision courante ; choisissez-en une plus ancienne au-dessus d’eux pour voir avec quoi elle a été installée.

## Mettre à niveau une release

**Mettre à niveau**, sur la page d’une release, change sa version de chart, ses valeurs, ou les deux :

- **Chart** — le chart à partir duquel mettre à niveau. helm n’enregistre pas d’où vient un chart : Homeport cherche donc un chart de ce nom dans vos dépôts, ceux qui ont la version courante de la release d’abord. Choisissez-en un autre avec **Un autre chart…**, sous la forme `repository/chart` ou d’une référence `oci://` — ce dont a besoin une release installée depuis un registre OCI.
- **Version du chart** — les versions de ce chart, la plus récente d’abord, ou saisie pour un chart OCI.
- **Valeurs** — l’éditeur commence avec les valeurs fournies par l’utilisateur de la release. Elles remplacent celles de la release : les valeurs par défaut du chart complètent le reste. **Changements des valeurs** les affiche côte à côte avec les valeurs actuelles, et **Rétablir les valeurs actuelles** les remet.
- **Attendre que ses ressources soient prêtes**, pendant au plus le nombre de minutes indiqué.

**Prévisualiser** rend la mise à niveau avec l’aide du cluster sans rien changer, et affiche le nouveau manifeste et ce qui y change par rapport au manifeste actuel. **Mettre à niveau** exécute `helm upgrade`, avec la sortie de helm en direct. Chaque mise à niveau est une nouvelle révision dans **Historique**.

## Revenir en arrière

Dans **Historique**, **Revenir à la révision N** sur n’importe quelle révision antérieure demande d’abord confirmation, en nommant les versions de chart et d’application actuelles et à venir, puis exécute `helm rollback` : helm remet le chart et les valeurs de cette révision sous forme d’une nouvelle révision ; un retour arrière peut donc lui-même être annulé par un retour arrière. Cochez **Attendre que ses ressources soient prêtes** pour que helm les attende.

Une release bloquée en **pending** après une installation, une mise à niveau ou un retour arrière interrompu est verrouillée : helm refuse de la changer tant que l’opération n’est pas terminée. La ramener à sa dernière révision déployée la déverrouille.

## Désinstaller

**Désinstaller**, sur la page d’une release, nomme la release, son Namespace et son cluster et liste les objets qui seront supprimés avec elle, avant que `helm uninstall` ne s’exécute. Les objets qu’un chart marque comme à conserver (`helm.sh/resource-policy: keep`) et les PersistentVolumeClaims des StatefulSets restent sur le cluster. **Conserver son historique, pour pouvoir revenir en arrière** laisse la release listée comme `uninstalled`, pour qu’elle puisse être ramenée à une révision ; sans cela, la release disparaît de la liste. **Attendre que ses objets soient supprimés** fait attendre helm jusqu’à leur suppression.

## Dépôts de charts

**Dépôts de charts**, le second onglet de la page Helm, liste les dépôts que helm connaît sur cet ordinateur : la même liste que `helm repo list` dans un terminal, partagée par tous les clusters. Homeport ne tient aucune liste propre : un dépôt ajouté dans un terminal apparaît ici, et un dépôt ajouté ici apparaît dans `helm repo list`.

- **Ajouter un dépôt** demande un nom (lettres, chiffres, points, tirets et tirets bas) et l’URL `https://` du dépôt, comme `https://charts.bitnami.com/bitnami`, et télécharge son index.
- **Tout mettre à jour**, ou **Mettre à jour** sur un dépôt, télécharge les index les plus récents, comme le fait `helm repo update`, avec la sortie de helm affichée en direct. Les charts publiés depuis la dernière mise à jour n’apparaissent qu’après une mise à jour.
- **Supprimer** oublie un dépôt, après votre confirmation. Les releases installées depuis ce dépôt continuent de s’exécuter.

**Trouver des charts** recherche parmi les charts de ces dépôts par nom ou par description, comme le fait `helm search repo` : saisissez `postgres` après avoir ajouté le dépôt de Bitnami pour voir ses charts PostgreSQL, avec leurs dernières versions de chart et d’application.

Homeport n’ajoute que des dépôts publics. Un dépôt qui demande une connexion s’ajoute dans un terminal, avec `helm repo add --username …`, où helm conserve son mot de passe comme il le fait toujours ; il apparaît alors ici et fonctionne comme les autres. Un chart dans un registre OCI (`oci://…`) n’a besoin d’aucun dépôt.

## Quand cela ne fonctionne pas

- **Le CLI helm n’est pas installé** : installez helm 3 comme indiqué plus haut, puis rouvrez la page.
- **Helm 2** : Homeport ne pilote que Helm 3 et ultérieur. Installez un helm récent.
- **Non autorisé** : helm lit les releases dans des Secrets ; les lister demande `list` sur `secrets` dans le Namespace. Détails indique ce qui a été refusé.
- **Une autre installation, mise à niveau ou un autre retour arrière est en cours** : la release est verrouillée par une opération en attente. Attendez qu’elle se termine, ou ramenez la release à sa dernière révision déployée.
- **Une release de ce nom existe déjà** : les noms de releases sont uniques dans un Namespace. Choisissez-en un autre, ou mettez à niveau la release existante.
- **Chart ou version introuvable** : vérifiez le nom et la version ; un chart de dépôt a besoin que son dépôt soit ajouté et [mis à jour](#dépôts-de-charts). Pour mettre à niveau une release dont le chart n’est dans aucun de vos dépôts, ajoutez d’abord ce dépôt.
- **Les valeurs ne sont pas du YAML valide** : le message nomme la ligne ; corrigez-la et réessayez.
- **Attente abandonnée** : avec **Attendre**, une ressource n’a pas été prête à temps. Regardez les Pods et les Events de la release pour voir ce qui bloque.
- **Pas un dépôt de charts, ou injoignable** : l’URL ne sert aucun `index.yaml`, ou cet ordinateur ne peut pas la joindre. Détails affiche ce qu’a dit helm.
- **Non connecté** : helm passe par la connexion du cluster dans Homeport ; la page du cluster indique pourquoi il n’est pas connecté.
