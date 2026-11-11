---
title: Vos données et sauvegardes
description: Où Homeport conserve ses données sur chaque système, comment les sauvegarder et les restaurer, et ce qu’il faut saisir de nouveau quand les secrets sont perdus.
---

Homeport conserve ce qu’il sait sur votre ordinateur : comment joindre vos hôtes et vos clusters, les définitions et les environnements de vos stacks, vos identifiants de registre, vos dépôts et vos paramètres. Cette page indique où cela se trouve, ce qu’il faut copier, et comment le récupérer. Pour la façon dont c’est protégé, voir [Sécurité et vos données](/docs/reference/security/).

Ce qui s’exécute sur vos hôtes et vos clusters ne fait pas partie des données de Homeport : les conteneurs, les images, les volumes et les objets Kubernetes ou Nomad restent où ils sont, et aucune sauvegarde de Homeport ne les contient. Pour copier le contenu d’un volume, [sauvegardez-le dans un fichier](/docs/guides/volumes-networks/#sauvegarder-et-restaurer) depuis la page Volumes.

## Ce qui se trouve où

### Le dossier de données

| Système | Dossier                                           |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

Sous Linux, le dossier se trouve sous `$XDG_DATA_HOME` quand cette variable est définie. Si Homeport ne peut pas ouvrir sa base de données, sa fenêtre affiche le chemin complet du fichier, avec **Ouvrir le dossier**.

| Dans le dossier                      | Ce que c’est                                                                                                                                                                                                  |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `homeport.db`                        | La base de données : chaque hôte, cluster, stack, identifiant de registre, dépôt et paramètre, l’historique des déploiements et les événements des moteurs. Les secrets qu’elle contient sont chiffrés.       |
| `homeport.db-wal`, `homeport.db-shm` | Le journal d’écriture anticipée de SQLite et son index, présents pendant que Homeport s’exécute. Les dernières modifications peuvent ne se trouver que dans le fichier `-wal` jusqu’à ce que Homeport quitte. |
| `homeport.db.before-restore-<date>`  | Une copie de la base de données faite juste avant chaque [restauration](#restaurer-une-sauvegarde). Supprimez celles dont vous ne voulez plus.                                                                |
| `repositories/` et `synced-stacks/`  | Les clones et les copies de travail des dépôts des [stacks synchronisées](/docs/guides/synced-stacks/). Homeport les recrée à partir des dépôts eux-mêmes.                                                    |

[Ce que Homeport enregistre](/docs/reference/security/#ce-que-homeport-enregistre) liste ce que contient la base de données, et quelles parties sont chiffrées.

### L’entrée du trousseau

La clé qui chiffre les secrets n’est pas dans le dossier. C’est une entrée du trousseau de votre système d’exploitation, avec le service `com.homeport.app` et le compte `secrets-key` : dans **Trousseaux d’accès** sous macOS, **Gestionnaire d’identification → Informations d’identification Windows** sous Windows, et **Mots de passe et clés** (Seahorse) ou KWallet sous Linux. Elle appartient à votre compte utilisateur sur cet ordinateur.

La base de données et cette entrée vont ensemble. Une base de données sans sa clé s’ouvre quand même, avec chaque nom, adresse et fichier compose, mais aucun de ses secrets ne peut être déchiffré : les hôtes et les clusters affichent **Identifiants perdus**. C’est pourquoi une copie du dossier ne suffit pas pour passer à un autre ordinateur, et pourquoi la [sauvegarde dans l’application](#sauvegarder-dans-homeport) est celle qui est recommandée.

### Les journaux et l’emplacement de la fenêtre

| Système | Journaux                                | Taille et position de la fenêtre                                    |
| ------- | --------------------------------------- | ------------------------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Logs/com.homeport.app/`      | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` | `%APPDATA%\com.homeport.app\.window-state.json`                     |

Ni les uns ni l’autre ne valent la peine d’être sauvegardés. **Paramètres → À propos → Ouvrir le dossier des journaux** ouvre les journaux.

### Ce qui reste à vous

- **Les dossiers des stacks liées.** Homeport ne retient que l’emplacement du dossier et des fichiers compose d’une [stack liée](/docs/guides/stacks/#stacks-liées). Le dossier, ses fichiers compose et son `.env` sont à vous : sauvegardez-les comme vos autres projets, en général avec git.
- **Votre kubeconfig, `~/.ssh` et `~/.docker`.** Homeport les lit quand vous importez depuis eux et ne les modifie jamais.
- **Les dépôts de charts et les identifiants de helm.** Ils appartiennent à helm, et sont partagés avec votre terminal.

## Sauvegarder dans Homeport

**Paramètres → Données → Sauvegarder…** écrit tout dans un seul fichier `.homeport`. C’est la sauvegarde à préférer, car elle emporte les secrets eux-mêmes, protégés par une phrase secrète plutôt que par le trousseau de cet ordinateur : elle se restaure sur n’importe quel ordinateur.

1. Ouvrez **Paramètres**, et sous **Données** choisissez **Sauvegarder…**.
2. Tapez deux fois une phrase secrète d’au moins 12 caractères. Homeport ne l’enregistre jamais, et rien ne peut ouvrir le fichier sans elle.
3. Choisissez où enregistrer le fichier.

Homeport indique ensuite ce qu’il a enregistré, et nomme tout ce qu’il a dû enregistrer sans son secret parce que ce secret ne pouvait pas être déchiffré sur cet ordinateur.

| Dans la sauvegarde                                                                                                  | Hors de la sauvegarde                                                             |
| ------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| Les hôtes, avec leurs clés, phrases secrètes, certificats, bastions et les clés de serveur que vous avez approuvées | Les événements des moteurs                                                        |
| Les clusters, avec leurs certificats, jetons et réglages de plugin exec                                             | L’historique des déploiements                                                     |
| Les fichiers compose et les environnements des stacks gérées                                                        | Les dossiers et les fichiers des stacks liées                                     |
| Les chemins des dossiers et des fichiers des stacks liées                                                           | Les clones des stacks synchronisées (recréés à la prochaine synchronisation)      |
| Le dépôt, la branche ou le tag, les réglages de synchronisation et l’environnement des stacks synchronisées         | Les journaux, et la taille et la position de la fenêtre                           |
| Les stacks déployées sur un Swarm                                                                                   | Tout ce qui se trouve sur vos hôtes et vos clusters : conteneurs, images, volumes |
| Les identifiants de registre, avec leurs mots de passe et leurs jetons                                              | Les dépôts de charts de helm                                                      |
| Les dépôts, avec leurs clés de déploiement et leurs jetons                                                          | Les redirections de port, qui prennent fin quand Homeport quitte                  |
| Vos paramètres                                                                                                      |                                                                                   |

Le fichier entier est chiffré avec la phrase secrète, donc sans elle il ne révèle rien, pas même le nom d’un hôte. Il contient tous les secrets de Homeport : gardez-le, ainsi que sa phrase secrète, aussi précieusement que les clés elles-mêmes. [Sauvegardes](/docs/reference/security/#sauvegardes) décrit le chiffrement.

Sauvegardez avant d’essayer une préversion, avant de réinstaller votre système, et chaque fois que vous avez ajouté des hôtes que vous ne voudriez pas configurer de nouveau.

### Restaurer une sauvegarde

1. Sur cet ordinateur ou sur un autre, ouvrez **Paramètres**, et sous **Données** choisissez **Restaurer…**.
2. Choisissez le fichier `.homeport` et tapez sa phrase secrète. Homeport liste ce que contient le fichier, ainsi que le Homeport qui l’a créé et quand, avant de modifier quoi que ce soit.
3. Choisissez **Tout remplacer**.

La restauration remplace toutes les données de Homeport par celles de la sauvegarde ; elle ne fusionne pas. Homeport copie d’abord la base de données actuelle dans `homeport.db.before-restore-<date>`, dans le dossier de données, puis remplace tout en une seule étape, donc un échec laisse vos données telles qu’elles étaient. Chaque secret est chiffré de nouveau avec la clé du trousseau de cet ordinateur.

Ensuite :

- Les hôtes et les clusters se reconnectent aussitôt, sans demander de clé, de mot de passe ni de clé de serveur à approuver.
- Une stack liée dont le dossier n’est pas sur cet ordinateur, au même chemin, apparaît comme externe jusqu’à ce qu’il y soit.
- Les stacks synchronisées clonent de nouveau leur dépôt à leur prochaine synchronisation.
- La page Événements et l’historique des déploiements repartent vides.

Une sauvegarde faite par un Homeport plus ancien se restaure dans un plus récent. Une sauvegarde faite par un Homeport plus récent est refusée : mettez d’abord Homeport à jour.

Pour revenir à ce que vous aviez avant une restauration, quittez Homeport, renommez `homeport.db` autrement, renommez en `homeport.db` le `homeport.db.before-restore-<date>` que vous voulez, supprimez les `homeport.db-wal` et `homeport.db-shm` restés à côté, et rouvrez Homeport.

### Ne déplacer que certains hôtes

**Exporter** et **Importer** sur la page Hôtes déplacent les hôtes que vous choisissez, avec leurs secrets, dans un fichier du même genre, et les ajoutent aux hôtes que l’autre ordinateur a déjà. Voir [passer à un autre ordinateur](/docs/reference/security/#passer-à-un-autre-ordinateur).

## Copier vous-même les fichiers

Une copie de la base de données convient à une sauvegarde système de cet ordinateur, comme Time Machine, l’Historique des fichiers ou votre propre script. Elle conserve ce que la sauvegarde dans l’application laisse de côté, les événements des moteurs et l’historique des déploiements, mais **pas la clé du trousseau** : elle ne se restaure avec ses secrets que pour le même utilisateur sur le même ordinateur, ou sur un ordinateur dont le trousseau a été restauré avec elle. Partout ailleurs, chaque nom, adresse et fichier compose revient, et les secrets doivent être [saisis de nouveau](#si-les-secrets-sont-perdus).

### Pendant que Homeport est fermé

Quittez Homeport, depuis **Quitter** sur son icône s’il se [ferme dans la zone de notification](/docs/guides/settings/#bureau), puis copiez tout le dossier de données. Une fois Homeport quitté, `homeport.db` contient tout et les fichiers `-wal` et `-shm` ont disparu.

### Pendant que Homeport s’exécute

Ne copiez pas `homeport.db` seul pendant que Homeport s’exécute : les dernières modifications peuvent encore se trouver dans `homeport.db-wal`, et une copie faite en pleine écriture peut être incohérente. Demandez plutôt une copie cohérente à SQLite, avec l’outil en ligne de commande `sqlite3` (inclus dans macOS ; sous Linux et Windows, installez-le depuis le paquet `sqlite3` de votre distribution ou depuis [sqlite.org](https://sqlite.org/download.html)) :

```sh
# Linux
sqlite3 ~/.local/share/com.homeport.app/homeport.db ".backup '$HOME/homeport-backup.db'"

# macOS
sqlite3 "$HOME/Library/Application Support/com.homeport.app/homeport.db" ".backup '$HOME/homeport-backup.db'"
```

```powershell
# Windows
sqlite3 "$env:APPDATA\com.homeport.app\homeport.db" ".backup '$HOME\homeport-backup.db'"
```

Le résultat est un seul fichier complet, sans `-wal` à côté. Pour le vérifier :

```sh
sqlite3 ~/homeport-backup.db "PRAGMA integrity_check;"
```

La commande affiche `ok`.

### Restaurer une copie

1. Quittez Homeport.
2. Dans le dossier de données, mettez `homeport.db` de côté, et supprimez `homeport.db-wal` et `homeport.db-shm` s’ils sont là : ils appartiennent à la base de données que vous remplacez.
3. Placez-y votre copie, nommée `homeport.db`. Si vous avez copié tout le dossier, remettez plutôt le dossier en place.
4. Ouvrez Homeport.

Homeport met à niveau une copie faite par une version plus ancienne quand il l’ouvre. Une copie utilisée en dernier par un Homeport plus récent est refusée, sans être modifiée, avec [un message qui nomme la version à installer](/docs/reference/troubleshooting/#cette-base-de-données-a-été-utilisée-en-dernier-par-homeport--ou-une-version-plus-récente).

## Si les secrets sont perdus

Après une réinitialisation du trousseau, un nouveau compte utilisateur, ou une base de données copiée ouverte sur un autre ordinateur, Homeport conserve tout ce qu’il peut encore lire et signale ce qu’il ne peut pas lire. Rien n’est supprimé, et ce qui n’est pas un secret (noms, adresses, fichiers compose, chemins de dossiers) est intact. Saisissez de nouveau :

| Quoi                                | Comment                                                                                                                                                                          |
| ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Hôtes TLS**                       | Sur la page de l’hôte, chargez de nouveau le certificat d’AC, le certificat client et la clé client, et choisissez **Enregistrer**.                                              |
| **Hôtes SSH avec une clé importée** | Collez de nouveau la clé privée et sa phrase secrète, et choisissez **Enregistrer**.                                                                                             |
| **Hôtes SSH avec une clé Homeport** | Choisissez **Régénérer la clé**, puis [autorisez la nouvelle clé](/docs/hosts/ssh/#autoriser-la-clé-de-homeport) sur le serveur. Retirez-y l’ancienne ligne `homeport@`.         |
| **Hôtes Incus via HTTPS**           | Supprimez l’hôte et [ajoutez-le de nouveau avec un nouveau jeton de confiance](/docs/hosts/incus/#via-https-avec-un-jeton-de-confiance).                                         |
| **Clusters Kubernetes**             | **Actualiser depuis le kubeconfig** sur la page du cluster.                                                                                                                      |
| **Clusters Nomad**                  | Collez le jeton ACL, et les certificats s’il en utilise, dans les **Paramètres** du cluster.                                                                                     |
| **Environnements des stacks**       | Tapez de nouveau l’environnement d’une stack gérée ou synchronisée sur sa page. Les stacks liées gardent le leur dans le `.env` de leur dossier, que Homeport n’a jamais détenu. |
| **Identifiants de registre**        | Dans **Paramètres → Registres**, modifiez chaque identifiant et tapez son mot de passe ou son jeton.                                                                             |
| **Dépôts**                          | Dans **Paramètres → Dépôts**, créez une **Nouvelle clé** et ajoutez-la au dépôt sur votre forge, ou utilisez **Remplacer le jeton**.                                             |

Les hôtes socket local, TCP et ssh-agent ne détiennent aucun secret et continuent de fonctionner. Les environnements enregistrés dans l’historique des déploiements ne peuvent pas être saisis de nouveau ; les fichiers compose de l’historique restent lisibles.

Si Homeport indique plutôt que l’entrée du trousseau avec laquelle ses secrets ont été scellés est manquante, il n’y a pas encore renoncé : revenez au trousseau qui contient l’entrée, ou choisissez **Repartir avec une nouvelle clé** dans **Paramètres → Cette machine** avant de les saisir à nouveau.

Voir [Identifiants perdus](/docs/reference/troubleshooting/#identifiants-perdus) dans le dépannage.

## Supprimer vos données

[Supprimer vos données](/docs/reference/updating/#supprimer-vos-données) liste les dossiers et l’entrée du trousseau à supprimer, et les clés à retirer de vos serveurs.
