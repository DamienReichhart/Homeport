---
title: Sécurité et vos données
description: Ce que Homeport enregistre, où, comment les secrets sont chiffrés, et ce qu’il ne fait jamais.
---

Homeport détient des identifiants qui ouvrent vos serveurs. Cette page explique exactement ce qu’il conserve, et comment c’est protégé. [Vos données et sauvegardes](/docs/reference/data-and-backups/) indique ce qu’il faut copier pour le sauvegarder, et comment le restaurer.

## Ce que Homeport enregistre

Tout se trouve dans une base de données SQLite, `homeport.db`, dans le dossier des données d’application de votre utilisateur :

| Système | Dossier                                           |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

Elle contient :

| Données                                                                 | Enregistrement |
| ----------------------------------------------------------------------- | -------------- |
| Noms, adresses, utilisateurs SSH et chemins de socket des hôtes         | En clair       |
| Clés publiques SSH et clés de serveur que vous avez approuvées          | En clair       |
| Clés privées SSH et leurs phrases secrètes                              | **Chiffré**    |
| Certificats d’AC TLS, certificats clients et clés                       | **Chiffré**    |
| Clés clientes et certificats épinglés des hôtes Incus                   | **Chiffré**    |
| Fichiers compose des stacks gérées                                      | En clair       |
| Environnements des stacks gérées                                        | **Chiffré**    |
| Dossiers et chemins des fichiers compose des stacks liées               | En clair       |
| Historique de déploiement : fichiers compose et profils                 | En clair       |
| Historique de déploiement : environnements                              | **Chiffré**    |
| Adresses de registre et noms d’utilisateur                              | En clair       |
| Mots de passe et jetons d’accès de registre                             | **Chiffré**    |
| URL des dépôts, moitiés publiques des clés de déploiement               | En clair       |
| Clés de serveur des dépôts que vous avez approuvées                     | En clair       |
| Clés de déploiement et jetons d’accès des dépôts                        | **Chiffré**    |
| Noms des clusters, URL des serveurs d’API et namespaces                 | En clair       |
| Façon dont chaque cluster se connecte, son contexte kubeconfig          | En clair       |
| AC, certificats clients et clés des clusters                            | **Chiffré**    |
| Jetons, mots de passe et jetons OIDC des clusters                       | **Chiffré**    |
| Commandes, arguments et environnement des plugins exec                  | **Chiffré**    |
| Votre thème, votre dernière sélection et le namespace de chaque cluster | En clair       |
| Les événements du moteur de chaque hôte, pour la page Événements        | En clair       |

Homeport ne copie rien d’autre depuis vos moteurs : les conteneurs, images, volumes et journaux sont lus en direct et jamais enregistrés, sauf quand vous téléchargez vous-même des journaux ou des fichiers, ou sauvegardez un volume. Les événements du moteur qu’il conserve — ce qui est arrivé à quel conteneur, image, volume ou réseau, avec leurs labels — sont gardés 30 jours par défaut ([Paramètres → Données](/docs/guides/settings/#données)), au plus 10 000 par hôte, et supprimés avec leur hôte ou avec **Effacer l’historique**.

### Dossiers des stacks liées

Pour une [stack liée](/docs/guides/stacks/#stacks-liées), Homeport enregistre seulement l’emplacement de son dossier et de ses fichiers compose, et une empreinte des fichiers pour savoir quand ils changent — jamais une copie de ceux-ci. Il lit les fichiers compose quand vous ouvrez la stack, et ne les écrit que quand vous choisissez **Enregistrer** dans son onglet **Définition**, en remplaçant chaque fichier modifié en une seule étape et en conservant ses permissions ; il n’écrase pas un fichier qui a changé depuis que vous l’avez ouvert, sauf si vous le demandez. Homeport ne lit ni n’écrit jamais le `.env` du dossier : Compose le lit quand il s’exécute. Oublier une stack liée ne touche jamais au dossier.

### Dépôts

Pour les [stacks synchronisées](/docs/guides/synced-stacks/), Homeport conserve l’URL de chaque dépôt, et soit une clé de déploiement qu’il a générée pour ce dépôt (sa moitié privée chiffrée), soit votre nom d’utilisateur et un jeton d’accès (chiffré). Le contenu d’un dépôt n’est pas un secret : son clone se trouve dans un dossier `repositories` à côté de `homeport.db`.

### Swarms

Homeport n’enregistre rien sur un [Swarm](/docs/guides/swarm/) : il le trouve via ses hôtes managers à chaque connexion. Les valeurs des secrets Swarm vont au Swarm et ne sont pas conservées par Homeport : une valeur que vous saisissez ou chargez depuis un fichier est envoyée une fois au moteur, jamais écrite dans `homeport.db`, dans un fichier ou dans le journal, et effacée de la mémoire quand la boîte de dialogue se ferme. Le moteur ne renvoie jamais la valeur d’un secret, donc Homeport ne peut pas non plus en afficher une. Les jetons de jonction sont récupérés auprès d’un manager quand vous en avez besoin, et ne sont jamais enregistrés ni journalisés.

### Clusters

Les identifiants d’un [cluster](/docs/clusters/) proviennent de votre kubeconfig et ne sont enregistrés que chiffrés, comme les clés SSH ; les fichiers auxquels le kubeconfig fait référence sont lus une fois et intégrés, un kubeconfig collé n’est jamais écrit dans un fichier, et votre kubeconfig n’est jamais modifié. Un [plugin exec](/docs/clusters/exec-plugins/) s’exécute sur cet ordinateur, en votre nom, et le jeton ou le certificat qu’il affiche n’est conservé qu’en mémoire, jusqu’à son expiration. Homeport n’enregistre rien d’autre sur ce qui s’exécute sur un cluster : les Pods, les charges de travail et le reste sont lus en direct. Les [redirections de port](/docs/clusters/port-forward/) n’écoutent que sur l’interface de bouclage de cet ordinateur (`127.0.0.1` et `::1`), donc rien d’autre sur votre réseau ne peut les utiliser, et elles prennent fin quand Homeport quitte. Les modifications faites avec [Appliquer du YAML ou Modifier](/docs/clusters/yaml/) sont enregistrées sur le cluster sous le gestionnaire de champs `homeport`.

[Helm](/docs/clusters/helm/) exécute votre CLI `helm` sur un cluster sans votre kubeconfig et sans écrire aucun identifiant de cluster sur le disque : pour chaque commande, Homeport ouvre une connexion privée sur l’interface de bouclage de cet ordinateur (`127.0.0.1`, un port aléatoire) qui n’accepte que les requêtes portant un jeton aléatoire, et donne à helm un kubeconfig temporaire, lisible par vous seul, qui contient ce jeton et rien d’autre. Homeport ajoute au passage les propres identifiants du cluster à chaque requête, et supprime le kubeconfig et ferme la connexion quand la commande se termine. Les valeurs de chart sont transmises à helm sur son entrée standard, jamais dans un fichier. Les dépôts de charts, identifiants de registre et caches propres à helm restent ceux de helm, comme dans un terminal ; Homeport n’ajoute que des dépôts publics et n’enregistre aucun mot de passe de dépôt.

Le jeton ACL, le certificat d’AC, le certificat client et la clé d’un [cluster Nomad](/docs/clusters/nomad/) ne sont enregistrés que chiffrés, comme les clés SSH ; son adresse, sa région et son nom de serveur TLS sont conservés en clair. Le jeton circule dans l’en-tête `X-Nomad-Token` de chaque requête, jamais dans une URL, y compris le WebSocket d’un shell. Homeport ne conserve aucune copie de la spécification d’un job : Nomad l’enregistre avec le job.

**Les valeurs des Secrets Kubernetes** ne sont récupérées que quand vous choisissez **Afficher** pour une clé, une à la fois : les listes, la page d’un Secret et son YAML portent ses clés et leurs tailles, jamais ses valeurs. Une valeur affichée reste dans la mémoire de cette page — pas dans un cache, pas dans `homeport.db`, pas dans le journal — et est masquée de nouveau quand vous quittez la page.

## Comment les secrets sont chiffrés

- Chaque secret est scellé séparément avec **XChaCha20-Poly1305**, un chiffrement authentifié, donc toute altération est détectée en plus de la lecture empêchée.
- La clé de 256 bits est aléatoire, créée au premier lancement, et conservée dans votre **trousseau du système** — le Trousseau de macOS, le Gestionnaire d’identification de Windows, ou le Secret Service sous Linux — sous le service `com.homeport.app`. Elle n’est jamais enregistrée à côté des données.
- Il n’y a pas de mot de passe principal : le trousseau lie déjà la clé à votre compte utilisateur.

Quelqu’un qui possède seulement une copie de `homeport.db` peut lire les noms et adresses des hôtes et des clusters, mais pas vos clés, certificats, jetons, environnements de stacks ou mots de passe de registre.

### Sans trousseau

Homeport affiche un bandeau et n’enregistre aucun secret. Les hôtes socket local, TCP et ssh-agent fonctionnent toujours. Voir la [configuration requise](/docs/requirements/#facultatif-un-trousseau-du-système).

### Si la clé du trousseau est perdue

Réinitialiser le trousseau, ou déplacer `homeport.db` vers un autre ordinateur ou un autre utilisateur, laisse des secrets qui ne peuvent plus être déchiffrés. Homeport ne supprime rien : les hôtes et les clusters concernés affichent **Identifiants perdus** jusqu’à ce que vous saisissiez de nouveau leurs identifiants ; [Si les secrets sont perdus](/docs/reference/data-and-backups/#si-les-secrets-sont-perdus) liste ce qu’il faut saisir de nouveau, et où. Pour déplacer vos données, [sauvegardez-les](#sauvegardes) ou [exportez plutôt vos hôtes](#passer-à-un-autre-ordinateur).

Homeport ne crée jamais de nouvelle clé par-dessus des secrets qu’il détient déjà. Quand le trousseau répond mais n’a pas d’entrée `com.homeport.app` alors que la base de données contient des secrets chiffrés — un autre trousseau répond cette fois, comme le Secret Service de KeePassXC au lieu de GNOME Keyring —, Homeport n’enregistre aucun secret et indique que **l’entrée du trousseau avec laquelle les secrets de Homeport ont été scellés est manquante**. Revenez au trousseau qui contient l’entrée et redémarrez Homeport. Si l’entrée a disparu pour de bon, choisissez **Repartir avec une nouvelle clé** dans **Paramètres → Cette machine** : les secrets scellés avec l’ancienne clé apparaissent alors comme **Identifiants perdus**, à saisir à nouveau ou à restaurer depuis une sauvegarde.

### Passer à un autre ordinateur

**Exporter** sur la page Hôtes écrit les hôtes que vous choisissez dans un fichier, avec leurs clés, leurs certificats et les clés de serveur approuvées, chiffré avec une phrase secrète que vous choisissez :

- Le fichier entier est chiffré (XChaCha20-Poly1305, avec la clé dérivée de la phrase secrète par Argon2id), donc sans la phrase secrète il ne révèle rien — pas même les noms ou adresses des hôtes. Toute modification du fichier le rend illisible.
- La phrase secrète doit comporter au moins 12 caractères. Homeport ne l’enregistre ni ne la journalise jamais ; si vous la perdez, le fichier ne peut pas être ouvert.
- **Importer** sur l’autre ordinateur demande la phrase secrète, puis enregistre les secrets de chaque hôte chiffrés avec la propre clé de trousseau de cet ordinateur, comme les autres.
- Un hôte dont les identifiants sont déjà perdus sur cet ordinateur est exporté sans eux, et l’export indique lesquels.

Traitez le fichier comme les clés qu’il contient : supprimez-le une fois importé. Le format est décrit dans l’[ADR 0013](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0013-passphrase-protected-export-bundles.md).

### Sauvegardes

**Paramètres → Données → Sauvegarder…** écrit tout ce qui précède — hôtes, clusters, stacks, identifiants de registre, dépôts et paramètres, avec leurs secrets déchiffrés — dans un fichier au même format chiffré, avec une phrase secrète que vous saisissez deux fois. Les événements du moteur et l’historique de déploiement ne sont pas inclus. Un secret qui ne peut pas être déchiffré sur cet ordinateur est laissé de côté, et la sauvegarde indique lequel.

**Restaurer…** demande la phrase secrète et liste ce que contient le fichier avant de modifier quoi que ce soit. La restauration ensuite :

1. copie la base de données actuelle dans `homeport.db.before-restore-<date and time>`, à côté de `homeport.db` ;
2. remplace chaque hôte, cluster, stack, identifiant, dépôt et paramètre en une seule étape, donc un échec laisse vos données telles qu’elles étaient ;
3. chiffre de nouveau chaque secret avec la clé de trousseau de cet ordinateur.

Une sauvegarde faite par un Homeport plus récent est refusée ; une sauvegarde faite par un Homeport plus ancien se restaure. La sauvegarde contient tous les secrets de Homeport : gardez-la, ainsi que sa phrase secrète, aussi précieusement que les clés elles-mêmes. [Vos données et sauvegardes](/docs/reference/data-and-backups/) donne les étapes, ce qu’une sauvegarde inclut et laisse de côté, et comment copier vous-même la base de données.

## Les clés ne touchent jamais le disque en clair

Homeport a son propre client SSH, donc il n’écrit jamais de clé privée dans un fichier pour que `ssh` la lise, et ne modifie jamais `~/.ssh`. Les clés déchiffrées restent en mémoire.

Quand `docker compose` déploie une stack sur un hôte SSH ou TLS, Homeport ouvre un socket privé éphémère sur cet ordinateur — lisible seulement par votre utilisateur, ou un pipe nommé qui refuse les clients distants sous Windows — et le redirige sur sa propre connexion, pour cette seule commande.

Les clés et jetons des dépôts n’atteignent jamais le disque non plus. Pour chaque commande `git`, Homeport sert un agent SSH privé qui ne détient que la clé de déploiement de ce dépôt — sur un socket que seul votre utilisateur peut ouvrir, ou un pipe nommé qui refuse les clients distants sous Windows — ou transmet le jeton dans un en-tête HTTPS via l’environnement de la commande. Votre propre configuration git, vos assistants d’identification et vos clés SSH sont laissés de côté, donc rien n’est mis en cache dans votre trousseau ou dans `~/.git-credentials`.

Les identifiants de registre n’atteignent jamais `docker compose` non plus : Homeport récupère lui-même les images d’une stack, via l’API du moteur, avant d’exécuter `docker compose up`, donc l’environnement de la commande et les fichiers qui lui sont donnés ne contiennent aucun mot de passe ni jeton.

## Faire confiance aux serveurs

La première fois que Homeport se connecte à un serveur SSH, il vous demande de confirmer l’empreinte de la clé du serveur, et il refuse de se connecter si cette clé change ensuite, jusqu’à ce que vous confirmiez de nouveau. Les hôtes TLS vérifient le certificat du serveur par rapport à votre AC et au nom du serveur ; les hôtes Incus via HTTPS n’acceptent que le certificat avec lequel ils ont été ajoutés. Voir [Hôtes SSH](/docs/hosts/ssh/#première-connexion).

### Certificats Incus

Ajouter un [hôte Incus via HTTPS](/docs/hosts/incus/#via-https-avec-un-jeton-de-confiance) crée en mémoire un certificat client et sa clé privée, une clé ECDSA P-384, et les enregistre chiffrés avec les autres secrets de l’hôte, jamais en clair sur le disque. Le propre certificat d’Incus est enregistré de la même façon et épinglé : Homeport n’accepte du serveur que ce certificat exact, comparé par son empreinte SHA-256, et en refuse un différent, en affichant les deux empreintes. Le jeton de confiance est utilisé une fois et n’est pas conservé. Retirez le certificat de Homeport sur le serveur avec `incus config trust remove` pour le révoquer.

## Ce à quoi Homeport se connecte

- **Les hôtes que vous ajoutez.** Quand vous récupérez, poussez ou construisez une image, c’est le moteur qui contacte le registre, pas Homeport. Homeport lui transmet l’identifiant enregistré pour ce registre avec la requête, sur la connexion de l’hôte ; le moteur ne le conserve pas.
- **Les registres de vos images, via le moteur de chaque hôte**, pour savoir si une [image plus récente](/docs/guides/containers/#images-plus-récentes) est publiée : quand la page Conteneurs, Images ou Stacks se charge (chaque réponse est conservée 6 heures) et quand vous choisissez **Rechercher des images plus récentes**. Le moteur demande au registre le digest vers lequel pointe un tag, avec l’identifiant enregistré pour les registres privés, et ne télécharge rien. Pour un hôte local, le moteur s’exécute sur votre ordinateur, donc la requête part de celui-ci.
- **Docker Hub, quand vous parcourez les tags** dans la boîte de dialogue de récupération : Homeport liste les tags d’un dépôt depuis `hub.docker.com`, depuis cet ordinateur, avec votre identifiant Docker Hub s’il y en a un d’enregistré. La recherche de dépôts passe plutôt par le moteur de l’hôte.
- **Un registre, quand vous vérifiez ses identifiants** dans **Paramètres → Registres** : Homeport interroge le point d’accès `/v2/` du registre, et son service de jetons, depuis cet ordinateur, en envoyant le nom d’utilisateur et le mot de passe ou le jeton via HTTPS (HTTP simple seulement pour un registre sur cet ordinateur, comme `localhost:5000`).
- **Les serveurs de vos dépôts**, pour les [stacks synchronisées](/docs/guides/synced-stacks/) : quand vous testez l’accès, créez ou synchronisez une stack synchronisée, et à intervalle régulier tant que Homeport est ouvert si l’interrogation périodique est activée. Homeport exécute `git` depuis cet ordinateur avec la clé de déploiement ou le jeton de ce dépôt.
- **Les clusters que vous ajoutez**, via le serveur d’API Kubernetes que nomme leur kubeconfig (et son proxy, s’il en a un), avec les identifiants qu’il donne, en vérifiant le certificat du serveur par rapport à l’AC du kubeconfig. Les plugins exec contactent leur propre cloud (AWS, Google Cloud, Azure) depuis cet ordinateur, comme ils le font pour `kubectl`.
- **Les clusters Nomad que vous ajoutez**, via l’adresse d’agent que vous donnez, et rien d’autre : les journaux et shells d’un job passent par cet agent, qui les relaie au nœud client, donc Homeport ne se connecte jamais aux nœuds eux-mêmes. Le certificat de l’agent est vérifié par rapport à l’AC que vous donnez, ou aux AC de cet ordinateur.
- **Les serveurs d’images Incus**, pour les sélecteurs d’image d’un [hôte Incus](/docs/guides/instances/#créer-une-instance) : Homeport télécharge la liste des images d’un serveur (`streams/v1/images.json`, depuis `images.linuxcontainers.org` sauf si vous en choisissez un autre) depuis cet ordinateur, et la conserve dix minutes. Incus télécharge lui-même les images.
- **Les dépôts de charts et les registres OCI, via helm**, quand vous ajoutez ou mettez à jour un dépôt, cherchez des charts, ou installez ou mettez à niveau un chart : `helm` télécharge les index et les charts depuis cet ordinateur, comme dans un terminal.
- **Ce site web, pour rechercher des mises à jour** : au lancement et toutes les 6 heures, Homeport télécharge `updates/latest.json`, un petit fichier qui indique la dernière version. La requête ne contient rien sur vous, vos hôtes ou vos conteneurs, mais comme toute requête web elle atteint GitHub Pages, qui héberge ce site, depuis votre adresse IP. Désactivez-la dans **Paramètres → Mises à jour**. Les mises à jour elles-mêmes se téléchargent depuis GitHub Releases, et Homeport n’en installe une que si elle est signée avec la clé de mise à jour de Homeport. Voir [mise à jour](/docs/reference/updating/).

## Ce que Homeport exécute sur vos hôtes

Homeport parle au moteur d’un hôte via son API, et n’exécute des commandes sur le serveur d’un hôte que pour ce que cette API ne permet pas :

| Quand                                                                                             | Où                                                                | Commande                                                                                                                              |
| ------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Le serveur d’un hôte SSH refuse d’ouvrir le socket de son moteur                                  | le serveur SSH                                                    | une vérification en shell POSIX : le socket existe-t-il et est-il lisible, et un moteur est-il installé                               |
| **Démarrer**, **Arrêter** ou **Redémarrer** une [unité Quadlet](/docs/guides/quadlet/)            | le serveur SSH, ou cet ordinateur pour un socket local sous Linux | `systemctl [--user] start\|stop\|restart '<unit>.service'`                                                                            |
| Les [journaux d’un conteneur containerd](/docs/hosts/containerd/#journaux)                        | le serveur SSH, ou cet ordinateur pour un socket local sous Linux | `tail -n <lines> [-F] -- '<log file>'`, après `ls` du dossier de journaux d’un conteneur Kubernetes quand CRI ne nomme pas le fichier |
| Un [terminal dans un conteneur containerd](/docs/hosts/containerd/#un-terminal-dans-un-conteneur) | le serveur SSH, ou cet ordinateur pour un socket local sous Linux | `mktemp -d`, `mkfifo -m 600` pour deux FIFO dans ce dossier, `cat` à travers elles, puis `rm -rf` du dossier                          |

Elles s’exécutent avec les droits de l’utilisateur SSH (ou les vôtres, sur cet ordinateur) et jamais avec `sudo`. Les valeurs sont vérifiées et mises entre guillemets avant d’entrer dans une commande. L’installation, la modification et la suppression des unités Quadlet passent par l’API de Podman, qui écrit elle-même les fichiers. Rien ne s’exécute sur un hôte Incus : sa console et ses terminaux passent par la propre API d’Incus. Les seuls fichiers que Homeport crée sur un hôte sont les deux FIFO d’un terminal containerd, dans un dossier temporaire privé supprimé quand le terminal se ferme.

## Ce que Homeport ne fait jamais

- **Aucun compte.** Il n’y a rien à quoi s’inscrire.
- **Aucune télémétrie, aucune analyse d’usage, aucun rapport de plantage.** Quand vous signalez un bug, l’[archive de diagnostic](/docs/reference/troubleshooting/#exporter-le-diagnostic-pour-un-rapport-de-bug) est un fichier que vous exportez, lisez et joignez vous-même.
- **Rien d’installé sur vos clusters.** Homeport n’utilise que l’API Kubernetes, avec les droits de vos identifiants, et ne modifie rien sans que vous le demandiez : tout ce qui supprime ou remplace quelque chose demande d’abord, en nommant le cluster.
- **Rien d’installé sur vos hôtes.** Les seules commandes que Homeport exécute sur un hôte sont [listées ci-dessus](#ce-que-homeport-exécute-sur-vos-hôtes). Le seul conteneur que Homeport démarre de lui-même est un [conteneur auxiliaire](/docs/guides/volumes-networks/#le-conteneur-auxiliaire) éphémère quand vous ouvrez, sauvegardez, restaurez ou clonez un volume ; il n’a pas de réseau et est supprimé une fois terminé. Son image n’est récupérée qu’une fois que vous avez accepté.
- **Aucune installation silencieuse.** Une mise à jour n’est installée que quand vous choisissez **Installer et redémarrer** ; les paquets Linux ne sont jamais mis à jour que par votre gestionnaire de paquets.

La [politique de confidentialité](/privacy/) expose ce que cela signifie en droit : qui est responsable de vos données, sur quelle base, et comment exercer vos droits.

## Journaux

Homeport écrit un petit fichier journal — quelques dizaines de kilo-octets, remplacé quand il est plein — pour diagnostiquer les problèmes. Il contient des événements de connexion et des erreurs, pas vos identifiants ni la sortie de vos conteneurs.

| Système | Dossier                                 |
| ------- | --------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` |
| macOS   | `~/Library/Logs/com.homeport.app/`      |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` |

Homeport n’a ni télémétrie ni rapport de plantage : un journal ne quitte jamais votre ordinateur sauf si vous l’envoyez. Pour un rapport de bug, **Paramètres → À propos → Exporter le diagnostic…** enregistre un zip avec un résumé des versions et ces journaux, après avoir remplacé tout ce qui a la forme d’un secret (clés, mots de passe, jetons, identifiants dans les URL) et, sauf si vous demandez le contraire, les noms et adresses de vos hôtes et de vos clusters. Voir [Exporter le diagnostic](/docs/reference/troubleshooting/#exporter-le-diagnostic-pour-un-rapport-de-bug).

## Faire confiance à un téléchargement

Les releases sont construites par GitHub Actions à partir du code source public. Chaque fichier est accompagné d’une somme de contrôle SHA-256 et d’une attestation de provenance de build signée qui le relie au commit et au workflow exacts qui l’ont construit. Voir [vérifier les téléchargements](/docs/reference/verifying/).

## Signaler une vulnérabilité

Merci de signaler les problèmes de sécurité en privé via le [signalement privé de vulnérabilités de GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new) plutôt que dans une issue publique.
