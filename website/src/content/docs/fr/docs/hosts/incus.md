---
title: Incus
description: Se connecter à Incus sur cette machine, via SSH ou via HTTPS avec un jeton de confiance, et choisir le projet que montrent ses pages.
---

**Incus** exécute des conteneurs système et des machines virtuelles, les **instances** du [guide](/docs/guides/instances/). Homeport s’y connecte via sa propre API REST, comme le décide l’[ADR 0021](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0021-incus-over-its-rest-api.md). Un hôte Incus a ses propres pages dans la barre latérale : **Instances**, **Images**, **Profils**, **Stockage** et **Réseaux**. Les pages de Docker, les stacks et le Tableau n’y sont pas proposés.

## Ajouter un hôte Incus

Choisissez **Incus** dans le champ **Moteur** du formulaire, puis la façon de le joindre :

| Connexion        | Quand                                                            | Ce qu’il faut                                                                   |
| ---------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| **Socket local** | Incus s’exécute sur cet ordinateur (Linux)                       | votre utilisateur dans le groupe `incus-admin`, ou `incus` (voir ci-dessous)    |
| **SSH**          | Incus s’exécute sur un serveur auquel vous pouvez vous connecter | l’utilisateur SSH dans le groupe `incus-admin`, ou `incus`                      |
| **TLS** (HTTPS)  | Incus écoute sur le réseau, ou SSH n’est pas autorisé            | un jeton de confiance, ou un certificat client auquel Incus fait déjà confiance |

**TCP** simple est grisé : Incus ne sert pas son API sans TLS. Homeport se connecte à **Incus 6.0** (la version à support long terme) et ultérieur.

### Ses sockets

| Socket                            | Qui peut l’utiliser              | Ce qu’il atteint                                                             |
| --------------------------------- | -------------------------------- | ---------------------------------------------------------------------------- |
| `/var/lib/incus/unix.socket`      | le groupe `incus-admin`          | tout : chaque projet, pool de stockage et réseau                             |
| `/var/lib/incus/unix.socket.user` | le groupe `incus` (`incus-user`) | uniquement le projet propre de l’utilisateur, créé à la première utilisation |

Sous Linux, **Trouvés sur cette machine** liste les deux quand ils répondent, le socket restreint sous le nom **Incus (restricted to your project)**. Un socket que vous ne pouvez pas encore utiliser est listé avec la commande qui y remédie :

```bash
sudo usermod -aG incus-admin "$USER"
```

Les changements de groupe s’appliquent aux nouvelles connexions : déconnectez-vous et reconnectez-vous (ou reconnectez-vous via SSH) avant d’ajouter l’hôte. Un Incus arrêté indique les unités qui le démarrent :

```bash
sudo systemctl enable --now incus.socket incus.service
```

### Via HTTPS avec un jeton de confiance

Incus écoute sur le réseau dès que `core.https_address` est défini. Sur le serveur Incus :

```bash
incus config set core.https_address :8443
incus config trust add homeport
```

La seconde commande affiche un **jeton de confiance** : une ligne de lettres et de chiffres. Dans Homeport, choisissez **Incus** et **TLS**, gardez **Jeton de confiance**, et collez-le. Homeport affiche ce qu’il y lit (le nom auquel il est destiné, les adresses du serveur et l’empreinte de son certificat, et sa date d’expiration), puis, sur **Ajouter** :

1. crée son propre certificat client, avec une clé ECDSA P-384 créée en mémoire ;
2. essaie les adresses du jeton dans l’ordre, et garde la première dont le certificat a l’empreinte que nomme le jeton ; toute autre est refusée, avec les deux empreintes ;
3. envoie le jeton avec son certificat, et vérifie qu’Incus lui fait désormais confiance.

L’hôte apparaît alors dans `incus config trust list` sous le nom du jeton. Un jeton ne sert qu’une fois et expire ; un jeton utilisé, révoqué ou expiré est refusé avec la commande qui en crée un nouveau. Pour révoquer Homeport plus tard, retirez son certificat à cet endroit :

```bash
incus config trust list
incus config trust remove <fingerprint>
```

**Certificat existant** prend un certificat client et une clé auxquels Incus fait déjà confiance (`incus config trust add-certificate`), avec le propre certificat d’Incus à approuver (`/var/lib/incus/server.crt`, ou `cluster.crt` dans un cluster).

### Le certificat du serveur est épinglé

Le certificat d’Incus est auto-signé, donc il n’y a pas d’AC par rapport à laquelle le vérifier. Homeport conserve avec l’hôte le certificat qu’il a approuvé et n’accepte ensuite que celui-là, comparé par son empreinte SHA-256, quelle que soit l’adresse à laquelle il est joint. Si Incus est réinstallé ou son certificat renouvelé, l’hôte est refusé avec l’empreinte attendue et celle reçue : vérifiez la nouvelle sur le serveur avec `incus info`, puis ajoutez de nouveau l’hôte avec un nouveau jeton. La clé client et les deux certificats sont enregistrés chiffrés comme tous les autres secrets ; voir [sécurité](/docs/reference/security/#certificats-incus).

## Projets

Incus conserve les instances, images, profils, volumes personnalisés et réseaux dans des **projets**. Les pages montrent un projet à la fois, choisi sous le sélecteur d’hôte, qui liste chacun avec le nombre d’instances et d’images qu’il contient. Homeport retient ce choix pour chaque hôte d’un lancement à l’autre. Tant que vous n’en avez pas choisi, un hôte affiche `default`, ou son seul projet sur le socket restreint. Les pools de stockage appartiennent à tout le serveur ; leurs volumes sont affichés pour le projet sélectionné.

## Événements

Les pages se rafraîchissent à partir des événements de cycle de vie d’Incus, dans tous les projets : une instance qui démarre ou s’arrête, un instantané pris, une image ajoutée, un profil ou un réseau modifié. Ils sont aussi conservés sur la page **Événements**. Incus n’en conserve aucun historique, donc ce qui s’est passé pendant que Homeport était fermé n’est pas complété.

Une fois connecté, Homeport interroge Incus avec `GET /1.0` toutes les 15 secondes. Un Incus arrêté rend l’hôte **Injoignable**, et Homeport se reconnecte de lui-même quand il répond de nouveau.

## Ce que Homeport fait sur l’hôte

Rien ne s’exécute sur un hôte Incus à part Incus lui-même : les consoles et les terminaux passent par les propres websockets d’Incus, et les images distantes sont téléchargées par Incus. Homeport lit depuis cet ordinateur la liste des images d’un serveur d’images distant, pour les sélecteurs d’image.
