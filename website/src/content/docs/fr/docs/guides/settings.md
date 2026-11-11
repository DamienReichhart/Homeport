---
title: Paramètres
description: Thème et langue, la zone de notification, le démarrage à l’ouverture de session, les notifications, les mises à jour, les identifiants de registre, les dépôts, la fréquence de vérification des stacks synchronisées, ce que cette machine prend en charge, la version et les licences.
---

**Paramètres** est la dernière page de la barre latérale, au-dessus des liens **Documentation** et **GitHub**, qui s’ouvrent dans votre navigateur.

## Apparence

**Thème** vaut **Suivre le système** par défaut, et suit le mode clair ou sombre de votre système d’exploitation quand il change. Choisissez **Clair** ou **Sombre** pour le fixer.

**Langue** vaut **Suivre le système** par défaut : Homeport s’affiche dans la langue de votre système d’exploitation quand il la connaît, et en anglais sinon. Le choix nomme la langue trouvée, comme **Suivre le système (Français)**. Choisissez une langue pour utiliser celle-là à la place ; toute la fenêtre change aussitôt, sans redémarrage, tout comme le menu de la zone de notification et les notifications.

Homeport parle anglais (English), français, espagnol (Español), allemand (Deutsch), italien (Italiano), portugais du Brésil (Português), russe (Русский), chinois simplifié (简体中文), japonais (日本語), coréen (한국어), hindi (हिन्दी) et arabe (العربية), qu’il affiche de droite à gauche. Les variantes régionales suivent leur langue : le français canadien affiche le français, le portugais européen affiche le portugais du Brésil. Le chinois traditionnel n’est pas encore disponible et affiche l’anglais.

Les dates, les nombres et les tailles suivent aussi la langue, dans le style régional de votre système quand il en a un pour elle : _1.5 GB_ et _4 Nov 2026_ en anglais britannique, _1,5 Go_ et _4 nov. 2026_ en français. Les noms, les ID, les commandes, les journaux et ce que disent les moteurs et les clusters restent tels quels, et les **Détails** d’une erreur gardent le texte d’origine du système ou de la bibliothèque qui a échoué, pour qu’un rapport de bug se lise de la même façon dans toutes les langues.

## Bureau

Comment Homeport fonctionne sur cet ordinateur.

- **Fermer dans la zone de notification** — désactivé par défaut, si bien que fermer la fenêtre quitte Homeport. Activez-le pour masquer plutôt la fenêtre dans la [zone de notification](/docs/guides/overview/#zone-de-notification) : Homeport continue de tourner avec ses connexions aux hôtes ouvertes. Faites-le revenir depuis l’icône de la zone de notification, ou en ouvrant à nouveau Homeport ; quittez avec **Quitter** dans la zone de notification. L’interrupteur est grisé, avec une explication, sur un bureau sans zone de notification.
- **Démarrer à l’ouverture de session** — désactivé par défaut. Activez-le et Homeport démarre quand vous ouvrez votre session, pour surveiller vos hôtes dès le départ. L’interrupteur reflète ce que contient votre système : si vous retirez vous-même Homeport de vos éléments d’ouverture, il apparaît désactivé. Sur Windows, il figure dans **Gestionnaire des tâches → Applications de démarrage**, sur macOS dans **Réglages Système → Général → Ouverture**, sur Linux dans `~/.config/autostart/`. L’entrée d’une AppImage suit le fichier quand vous le déplacez, à partir de la prochaine fois que vous l’ouvrez.
- **Démarrer masqué dans la zone de notification** — désactivé par défaut, et disponible uniquement quand **Démarrer à l’ouverture de session** est activé. Quand Homeport démarre à l’ouverture de session, il reste dans la [zone de notification](/docs/guides/overview/#zone-de-notification) sans ouvrir sa fenêtre. Ouvrir Homeport vous-même affiche toujours la fenêtre, et sans zone de notification il s’ouvre toujours.

## Notifications

Homeport affiche une notification de votre système d’exploitation quand quelque chose se produit sur un hôte, que sa fenêtre soit ouverte, réduite ou [fermée dans la zone de notification](#bureau). Chaque type a son propre interrupteur, tous activés par défaut, et un interrupteur prend effet aussitôt :

| Interrupteur                                                  | Exemple                                                                                   |
| ------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| **Un conteneur s’arrête sur une erreur ou manque de mémoire** | _web-1 s’est arrêté sur prod-server (code de sortie 137)_, _db a manqué de mémoire sur …_ |
| **Un conteneur passe en mauvaise santé**                      | _api est en mauvaise santé sur prod-server_                                               |
| **Un hôte connecté devient injoignable**                      | _prod-server est injoignable : …_                                                         |
| **Un hôte injoignable est de retour**                         | _prod-server est de nouveau connecté._                                                    |

Arrêter un conteneur — avec **Arrêter** dans Homeport, `docker stop`, `docker kill` ou `docker compose down` — n’est pas un plantage, donc rien ne s’affiche ; pas plus que pour un conteneur qui se termine avec le code 0. Un hôte qui ne parvient pas à se connecter au démarrage de Homeport n’est pas non plus une panne : seul compte un hôte qui était connecté. Un conteneur bloqué en redémarrage, ou un hôte qui décroche sans cesse, notifie au plus une fois toutes les 5 minutes.

Les notifications viennent des connexions que Homeport tient déjà, si bien que rien n’est installé sur vos hôtes, et elles s’arrêtent quand Homeport quitte. Votre système d’exploitation décide de leur apparence et de leur affichage : si aucune n’apparaît, vérifiez ses paramètres de notification pour Homeport.

## Mises à jour

- **Rechercher automatiquement les mises à jour** — activé par défaut : Homeport cherche une nouvelle version à son démarrage et toutes les 6 heures, et affiche une barre en haut de la fenêtre quand il y en a une.
- **Vérifier maintenant** — vérifie aussitôt, même si la recherche automatique est désactivée, et indique si vous avez la dernière version.

Quand une mise à jour est disponible, la carte montre ce que vous pouvez faire, ce qui dépend de la façon dont vous avez installé Homeport :

- **Windows, macOS et l’AppImage** — **Installer et redémarrer** télécharge la mise à jour, vérifie sa signature, l’installe et rouvre Homeport.
- **Paquets `.deb` et `.rpm`** — votre gestionnaire de paquets installe les mises à jour, donc la carte montre la commande à exécuter, avec un bouton **Copier la commande**.

**Nouveautés** ouvre le [journal des modifications](/changelog/). Voir [mettre à jour](/docs/reference/updating/) pour en savoir plus.

## Registres

Les identifiants des registres de conteneurs — Docker Hub, GitHub Container Registry (GHCR) ou le vôtre —, enregistrés une fois dans Homeport et utilisés sur tous les hôtes : quand vous récupérez une image, exécutez un conteneur à partir d’une image qui n’est pas encore là, poussez, construisez, ou déployez une stack gérée, liée ou synchronisée. Il y a un seul jeu d’identifiants par adresse de registre.

- **Ajouter des identifiants** demande le **Registre** (`docker.io`, `ghcr.io`, `registry.example.com:5000`), le **Nom d’utilisateur** et le **Mot de passe ou jeton d’accès**. GHCR, ainsi que les comptes Docker Hub avec authentification à deux facteurs, demandent un jeton d’accès plutôt que le mot de passe.
- **Vérifier les identifiants** demande au registre, depuis cet ordinateur, s’il accepte les identifiants, comme le fait `docker login`, sans rien enregistrer. Des identifiants refusés le disent avec une piste ; un registre que Homeport ne peut pas joindre le dit à la place.
- **Modifier** change l’adresse ou le nom d’utilisateur ; laissez le mot de passe vide pour garder celui enregistré. **Supprimer** oublie les identifiants : les images de ce registre sont de nouveau récupérées anonymement.

Docker Hub peut être saisi sous la forme `docker.io`, `index.docker.io` ou `https://index.docker.io/v1/` ; c’est un seul et même registre. Une image sans registre dans son nom, comme `nginx` ou `bitnami/redis`, utilise les identifiants de Docker Hub.

Les mots de passe et les jetons sont chiffrés avec la clé du trousseau du système et ne sont plus jamais affichés, journalisés ni placés dans les détails d’une erreur. Sans trousseau, la carte explique que les identifiants ne peuvent pas être enregistrés, et les images sont récupérées anonymement. Voir [sécurité](/docs/reference/security/).

Homeport ne lit pas `~/.docker/config.json` ni ses assistants d’identification.

## Dépôts

Les dépôts git à partir desquels les [stacks synchronisées](/docs/guides/synced-stacks/) sont déployées, enregistrés une fois et utilisables sur tous les hôtes.

- **Ajouter un dépôt** demande l’URL du dépôt et la façon dont Homeport le joint : une **Clé de déploiement** qu’il crée pour ce dépôt (URL SSH), un **Jeton** avec son nom d’utilisateur (URL HTTPS), ou **Public** pour un dépôt que tout le monde peut lire.
- Le bouton en forme d’engrenage montre la clé de déploiement d’un dépôt, avec l’endroit où l’ajouter sur GitHub, GitLab, Gitea et Forgejo, et **Nouvelle clé** pour la remplacer ; ou **Remplacer le jeton**. **Tester l’accès** liste ses branches et ses tags depuis cet ordinateur. Une fois que Homeport a approuvé la clé d’un serveur SSH, son empreinte s’y affiche, avec **Réinitialiser** pour le cas où la forge l’a vraiment changée.
- **Supprimer** oublie le dépôt et supprime son clone sur cet ordinateur ; c’est refusé tant qu’une stack synchronisée l’utilise.

Les clés et les jetons sont chiffrés avec la clé du trousseau du système et ne sont plus jamais affichés, journalisés ni placés dans les détails d’une erreur. Votre propre configuration git, vos assistants d’identification et vos clés SSH ne sont jamais utilisés. Voir [dépôts](/docs/guides/synced-stacks/#dépôts).

## Stacks synchronisées

**Vérifier les dépôts toutes les** fixe le nombre de minutes que Homeport attend entre deux vérifications des dépôts des stacks synchronisées à la recherche de nouveaux commits : 5 par défaut, au moins 1. Homeport ne vérifie que lorsqu’il est ouvert ; rien ne se synchronise quand il est fermé. Voir [interrogation et déploiement automatique](/docs/guides/synced-stacks/#interrogation-et-déploiement-automatique).

## Données

Ce que Homeport conserve sur cet ordinateur.

### Sauvegarder et restaurer

**Sauvegarder…** enregistre chaque hôte, cluster, stack, identifiant de registre, dépôt et paramètre, avec leurs clés, certificats, jetons, mots de passe et environnements, dans un seul fichier `.homeport`. Tapez deux fois une phrase secrète d’au moins 12 caractères : tout le fichier est chiffré avec elle, et elle ne peut pas être récupérée. Les événements des moteurs et l’historique des déploiements ne sont pas inclus, ni rien de ce qui se trouve sur vos hôtes ou vos clusters — les conteneurs, les images, les volumes et les ressources Kubernetes restent où ils sont.

**Restaurer…** ouvre un tel fichier avec sa phrase secrète et liste ce qu’il contient : ses hôtes et ses clusters par leur nom, et le nombre de stacks, d’identifiants de registre et de dépôts. **Tout remplacer** remplace alors toutes les données de Homeport par celles de la sauvegarde, sur cet ordinateur ou sur un autre :

- Tout se connecte et se déploie aussitôt : les hôtes SSH et TLS, les registres et les dépôts, sans redemander de clé ni de mot de passe, ni d’approuver la clé d’un serveur.
- Une [stack liée](/docs/guides/stacks/#stacks-liées) dont le dossier n’est pas sur cet ordinateur apparaît comme externe jusqu’à ce que le dossier soit là, au même chemin.
- Les [stacks synchronisées](/docs/guides/synced-stacks/) clonent à nouveau leur dépôt à leur prochaine synchronisation.
- Les données actuelles sont d’abord copiées dans `homeport.db.before-restore-<date>`, dans le dossier de données de Homeport. Restaurer un mauvais fichier, celui d’un Homeport plus récent, ou un fichier qui échoue à mi-chemin ne change rien.

Pour ajouter plutôt quelques hôtes à ceux que vous avez, utilisez **Exporter** et **Importer** sur la page Hôtes. [Vos données et sauvegardes](/docs/reference/data-and-backups/) indique où se trouvent les fichiers de Homeport, ce que contient une sauvegarde, et comment copier vous-même la base de données.

### Conserver les événements des moteurs

**Conserver les événements des moteurs pendant** fixe le nombre de jours pendant lesquels les [événements du moteur](/docs/guides/overview/#événements) de chaque hôte sont conservés : 30 par défaut, de 1 à 365. Chaque hôte conserve au plus 10 000 événements, quelle que soit cette valeur. Les plus anciens sont supprimés au démarrage de Homeport et toutes les heures, et aussitôt quand vous baissez le nombre.

## Cette machine

Ce que Homeport peut utiliser sur cet ordinateur :

- **Trousseau du système** — **Disponible** quand les clés SSH, les certificats, les environnements des stacks et les identifiants de registre peuvent être enregistrés chiffrés ; **Indisponible** sinon. Voir la [configuration requise](/docs/requirements/#facultatif-un-trousseau-du-système).
- **CLI Compose** — la version de Compose trouvée, comme `docker compose v2.29.1`, ou **Introuvable** si les stacks peuvent être démarrées et arrêtées mais pas déployées. Voir [Stacks](/docs/guides/stacks/#ce-dont-le-déploiement-a-besoin).
- **git** — le git trouvé, comme `/usr/bin/git 2.43.0`, ou ce qu’il faut installer si les stacks synchronisées ne peuvent pas être utilisées. Voir la [configuration requise](/docs/requirements/#facultatif-git).
- **CLI Helm** — le helm trouvé, comme `/usr/local/bin/helm v3.16.2`, ou ce qu’il faut installer pour ce système si les [releases Helm](/docs/clusters/helm/) ne peuvent pas être listées ni installées. Helm 2 est signalé comme non pris en charge. Voir la [configuration requise](/docs/requirements/#facultatif-helm).

Homeport vérifie les CLI à chaque ouverture de cette page, si bien qu’une CLI installée depuis le lancement apparaît ; après avoir installé un trousseau, quittez et rouvrez Homeport.

## À propos

Affiche la version de Homeport et la plateforme pour laquelle il a été compilé, par exemple `v1.0.0 · macos/aarch64`.

**Ouvrir le dossier des journaux** montre où Homeport écrit son [journal](/docs/reference/security/#journaux).

**Exporter le diagnostic…** enregistre un zip à joindre quand vous [signalez un problème](https://github.com/DamienReichhart/Homeport/issues/new/choose) : un résumé des versions et des types d’hôtes et de clusters que vous utilisez, et les journaux sans leurs secrets. La boîte de dialogue montre d’abord ce qu’il contient, et rien n’est envoyé nulle part. Voir [Exporter le diagnostic](/docs/reference/troubleshooting/#exporter-le-diagnostic-pour-un-rapport-de-bug).

**Licences** montre la licence de Homeport (la GNU AGPL, version 3), un lien vers le code source de la version que vous utilisez, et les mentions de chaque paquet open source avec lequel il est construit.
