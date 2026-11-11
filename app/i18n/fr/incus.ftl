### The `incus` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/incus.ts:
### change the text there, then run it again.

incus-delete = Supprimer
incus-status-running = En cours d’exécution
incus-status-stopped = Arrêtée
incus-status-frozen = Gelée
incus-status-starting = Démarrage
incus-status-stopping = Arrêt en cours
incus-status-freezing = Gel en cours
incus-status-error = Erreur
incus-status-unknown = Inconnu
incus-type-container = Conteneur
incus-type-virtualMachine = Machine virtuelle
incus-action-start = Démarrer
incus-action-stop = Arrêter
incus-action-restart = Redémarrer
incus-action-freeze = Geler
incus-action-unfreeze = Dégeler
incus-action-forceStop = Forcer l’arrêt
incus-actionOne-start = Démarrer { $name }
incus-actionOne-stop = Arrêter { $name }
incus-actionOne-restart = Redémarrer { $name }
incus-actionOne-freeze = Geler { $name }
incus-actionOne-unfreeze = Dégeler { $name }
incus-instances-nav = Instances
incus-instances-title = Instances
incus-instances-summary = { $count ->
        [one] { NUM($count) } instance
       *[other] { NUM($count) } instances
    }, dont { $running } en cours d’exécution, dans le projet { $project }.
incus-instances-search = Filtrer par nom ou description
incus-instances-filter-all = Toutes
incus-instances-empty = Aucune instance dans { $project }
incus-instances-emptyDescription = Les conteneurs système et les machines virtuelles de ce projet apparaissent ici. Choisissez un autre projet sous le sélecteur d’hôte pour en voir d’autres.
incus-instances-more = Plus d’actions pour { $name }
incus-instances-column-name = Nom
incus-instances-column-type = Type
incus-instances-column-status = État
incus-instances-column-image = Image
incus-instances-column-addresses = Adresses
incus-instances-column-memory = Mémoire
incus-instances-column-created = Créée
incus-instances-done-start = { $name } démarrée
incus-instances-done-stop = { $name } arrêtée
incus-instances-done-restart = { $name } redémarrée
incus-instances-done-freeze = { $name } gelée
incus-instances-done-unfreeze = { $name } dégelée
incus-instances-deleted = { $name } supprimée
incus-instances-delete-title = Supprimer { $name } ?
incus-instances-delete-description = L’instance et son disque sont supprimés du projet { $project }. Cette action est irréversible.
incus-instances-delete-snapshots = { $count ->
        [one] Son instantané est supprimé
       *[other] Ses { NUM($count) } instantanés sont supprimés
    } avec elle.
incus-instances-delete-snapshotsNamed = { $count ->
        [one] Son instantané { $names } est supprimé
       *[other] Ses instantanés { $names } sont supprimés
    } avec elle.
incus-instances-delete-stopFirst = Arrêtez-la d’abord : elle est en cours d’exécution
incus-instances-delete-stopAndDelete = Arrêter et supprimer
incus-instances-backToList = Retour aux instances
incus-instances-tab-overview = Vue d’ensemble
incus-instances-tab-console = Console
incus-instances-tab-exec = Exec
incus-instances-tab-snapshots = Instantanés
incus-instances-tab-config = Configuration
incus-instances-tab-devices = Périphériques
incus-instances-tab-raw = Brut
incus-instances-field-project = Projet
incus-instances-field-architecture = Architecture
incus-instances-field-addresses = Adresses
incus-instances-field-cpu = Temps CPU
incus-instances-field-memory = Mémoire
incus-instances-field-processes = Processus
incus-instances-field-created = Créée
incus-instances-field-lastUsed = Dernière utilisation
incus-instances-field-snapshots = Instantanés
incus-instances-field-location = Membre du cluster
incus-instances-field-description = Description
incus-instances-cpuSeconds = { $seconds } s
incus-instances-profiles = Profils
incus-instances-noConfig = Aucune configuration.
incus-instances-noDevices = Aucun périphérique.
incus-instances-config-key = Clé
incus-instances-config-value = Valeur
incus-instances-config-fromProfile = Issu d’un profil
incus-progress-waiting = En attente d’Incus…
incus-remote-server = Serveur d’images
incus-remote-search = Filtrer par système, version ou alias
incus-remote-list = Images du serveur
incus-remote-none = Aucune image ne correspond.
incus-remote-alias = Alias
incus-images-nav = Images
incus-images-title = Images
incus-images-summary = { $count ->
        [one] { NUM($count) } image
       *[other] { NUM($count) } images
    } dans le projet { $project }.
incus-images-search = Filtrer par alias, empreinte ou description
incus-images-empty = Aucune image dans { $project }
incus-images-emptyDescription = Les images sont téléchargées quand une instance est créée à partir de l’une d’elles, ou à l’avance avec Obtenir une image.
incus-images-get = Obtenir une image
incus-images-get-title = Obtenir une image
incus-images-get-description = Incus la télécharge dans le projet { $project } depuis le serveur d’images, pour que les instances démarrent rapidement à partir d’elle.
incus-images-get-autoUpdate = La maintenir à jour depuis le serveur
incus-images-get-keepAlias = Lui donner le même alias ici
incus-images-get-confirm = Obtenir l’image
incus-images-get-done = L’image est prête.
incus-images-copied = { $alias } copiée dans { $project }
incus-images-column-aliases = Alias
incus-images-column-description = Description
incus-images-column-type = Type
incus-images-column-arch = Architecture
incus-images-column-size = Taille
incus-images-column-uploaded = Ajoutée
incus-images-cached = en cache
incus-images-cachedHint = Téléchargée pour créer une instance ; Incus la supprime quand plus rien ne l’a utilisée depuis un moment.
incus-images-aliasAdd = Ajouter un alias à { $name }
incus-images-aliasRemove = Supprimer l’alias { $name }
incus-images-aliasRemoved = Alias { $name } supprimé
incus-images-alias-title = Ajouter un alias à { $name }
incus-images-alias-name = Alias
incus-images-alias-add = Ajouter l’alias
incus-images-refreshOne = Actualiser { $name } depuis son serveur
incus-images-refreshed = { $name } actualisée
incus-images-deleteOne = Supprimer { $name }
incus-images-deleted = { $name } supprimée
incus-images-delete-title = Supprimer l’image { $name } ?
incus-images-delete-description = Les instances créées à partir d’elle continuent de fonctionner : chacune a sa propre copie. Les nouvelles nécessiteront de la télécharger à nouveau.
incus-profiles-nav = Profils
incus-profiles-title = Profils
incus-profiles-summary = { $count ->
        [one] { NUM($count) } profil
       *[other] { NUM($count) } profils
    } dans le projet { $project }.
incus-profiles-search = Filtrer par nom ou description
incus-profiles-empty = Aucun profil dans { $project }
incus-profiles-new = Nouveau profil
incus-profiles-new-title = Nouveau profil
incus-profiles-new-description = Un profil vide dans le projet { $project }. Ajoutez sa configuration et ses périphériques une fois créé.
incus-profiles-new-name = Nom
incus-profiles-new-descriptionField = Description
incus-profiles-created = Profil { $name } créé
incus-profiles-column-name = Nom
incus-profiles-column-description = Description
incus-profiles-column-settings = Paramètres
incus-profiles-column-usedBy = Utilisé par
incus-profiles-settings = { $config ->
        [one] { NUM($config) } clé
       *[other] { NUM($config) } clés
    }, { $devices ->
        [one] { NUM($devices) } périphérique
       *[other] { NUM($devices) } périphériques
    }
incus-profiles-defaultKept = Incus conserve le profil default.
incus-profiles-inUse = Utilisé par { $names } : retirez-le d’abord de ces instances.
incus-profiles-deleteOne = Supprimer { $name }
incus-profiles-deleted = Profil { $name } supprimé
incus-profiles-delete-title = Supprimer le profil { $name } ?
incus-profiles-delete-description = Il est supprimé du projet { $project }. Aucune instance ne l’utilise.
incus-profiles-backToList = Retour aux profils
incus-profiles-noDescription = Aucune description
incus-profiles-reload = Recharger
incus-profiles-saved = Profil { $name } enregistré
incus-profiles-usedBy = Utilisé par
incus-profiles-editHint = Sa description, sa configuration et ses périphériques, tels que incus profile edit les affiche. L’enregistrement les applique aux instances qui l’utilisent, comme le fait Incus ; si quelqu’un l’a modifié depuis son chargement, Homeport refuse plutôt que d’écraser.
incus-profiles-yamlOf = YAML de { $name }
incus-new-action = Nouvelle instance
incus-new-title = Nouvelle instance
incus-new-description = Un conteneur système ou une machine virtuelle dans le projet { $project }, à partir d’une image.
incus-new-name = Nom
incus-new-image = Image
incus-new-origin-remote = Depuis un serveur d’images
incus-new-origin-local = Depuis les images de cet hôte
incus-new-localImage = Image sur cet hôte
incus-new-localImagePlaceholder = Choisir une image
incus-new-profiles = Profils
incus-new-cpu = CPU
incus-new-memory = Mémoire
incus-new-disk = Disque racine
incus-new-limitsHint = Facultatif : si vide, les profils décident. Les tailles s’écrivent comme 512MiB ou 20GiB.
incus-new-start = La démarrer une fois créée
incus-new-confirm = Créer
incus-new-open = L’ouvrir
incus-new-made = { $name } créée
incus-new-done = { $name } est prête.
incus-new-problem-name = Nommez-la avec des lettres, des chiffres et des tirets, en commençant par une lettre, jusqu’à 63 caractères.
incus-new-problem-image = Choisissez une image, ou saisissez son alias.
incus-new-problem-cpu = Le nombre de CPU est un nombre entier à partir de 1.
incus-new-problem-memory = La mémoire s’écrit comme 512MiB ou 2GiB.
incus-new-problem-disk = Le disque racine s’écrit comme 20GiB.
incus-exec-startFirst = Démarrez l’instance pour y exécuter des commandes.
incus-exec-command = Commande
incus-exec-commandPlaceholder = Un shell (bash, sinon sh)
incus-exec-user = ID utilisateur
incus-exec-cwd = Dossier
incus-exec-open = Ouvrir
incus-exec-exited = La commande s’est terminée.
incus-exec-exitedWithCode = La commande s’est terminée avec le code { $code }.
incus-console-startFirst = Démarrez l’instance pour vous attacher à sa console.
incus-console-hint = Sa console texte, telle que incus console l’affiche : l’invite de connexion, ou un démarrage en direct. Quitter l’onglet détache sans rien arrêter.
incus-console-reattach = Rattacher
incus-console-detached = Détaché de la console.
incus-snapshots-take = Prendre un instantané
incus-snapshots-take-title = Prendre un instantané de { $name }
incus-snapshots-take-description = Une copie de son disque dans son état actuel, pour pouvoir la restaurer plus tard : avant une mise à niveau ou un changement risqué.
incus-snapshots-take-name = Nom
incus-snapshots-take-namePlaceholder = snap0, snap1… si laissé vide
incus-snapshots-take-expiry = Le conserver
incus-snapshots-take-stateful = Inclure son état d’exécution (mémoire)
incus-snapshots-expiry-never = Jusqu’à sa suppression
incus-snapshots-expiry-1d = Un jour
incus-snapshots-expiry-7d = Une semaine
incus-snapshots-expiry-30d = 30 jours
incus-snapshots-taken = Instantané de { $name } pris
incus-snapshots-empty = Aucun instantané
incus-snapshots-emptyDescription = Prenez-en un avant de modifier quelque chose que vous pourriez vouloir annuler.
incus-snapshots-column-name = Nom
incus-snapshots-column-created = Pris
incus-snapshots-column-expires = Expire
incus-snapshots-column-size = Taille
incus-snapshots-stateful = avec état
incus-snapshots-restore = Restaurer
incus-snapshots-restore-title = Restaurer { $name } à { $snapshot } ?
incus-snapshots-restore-description = Tout ce qui a changé sur son disque depuis { $time }, quand { $snapshot } a été pris, est perdu. Une instance en cours d’exécution redémarre.
incus-snapshots-restored = { $name } restaurée à { $snapshot }
incus-snapshots-deleteOne = Supprimer { $snapshot }
incus-snapshots-delete-title = Supprimer l’instantané { $snapshot } ?
incus-snapshots-delete-description = L’instance reste telle quelle ; cet instantané ne pourra plus être restauré.
incus-snapshots-deleted = Instantané { $snapshot } supprimé
incus-storage-nav = Stockage
incus-storage-title = Stockage
incus-storage-summary = { $count ->
        [one] { NUM($count) } pool de stockage
       *[other] { NUM($count) } pools de stockage
    } ; volumes du projet { $project }.
incus-storage-empty = Aucun pool de stockage
incus-storage-emptyDescription = Créez-en un avec incus storage create, ou incus admin init.
incus-storage-usage = Espace utilisé dans { $pool }
incus-storage-space = { $used } utilisés sur { $total } ({ $percent })
incus-storage-spaceUnknown = Son pilote n’indique pas son taux de remplissage.
incus-storage-volumesOf = Volumes dans { $pool }
incus-storage-noVolumes = Aucun volume de ce projet dans le pool.
incus-storage-new = Nouveau volume
incus-storage-new-title = Nouveau volume dans { $pool }
incus-storage-new-description = Un volume personnalisé du projet { $project }, à attacher à des instances pour des données qui leur survivent.
incus-storage-new-name = Nom
incus-storage-new-size = Taille
incus-storage-new-content = Contenu
incus-storage-content-filesystem = Système de fichiers
incus-storage-content-block = Périphérique bloc
incus-storage-created = Volume { $name } créé
incus-storage-column-name = Nom
incus-storage-column-type = Type
incus-storage-column-content = Contenu
incus-storage-column-size = Taille
incus-storage-column-usedBy = Utilisé par
incus-storage-inUse = Utilisé par { $names } : détachez-le d’abord.
incus-storage-deleteOne = Supprimer { $name }
incus-storage-delete-title = Supprimer le volume { $name } ?
incus-storage-delete-description = Ses données sont supprimées de { $pool }. Rien ne l’utilise.
incus-storage-deleted = Volume { $name } supprimé
incus-networks-nav = Réseaux
incus-networks-title = Réseaux
incus-networks-summary = { $count ->
        [one] { NUM($count) } réseau visible
       *[other] { NUM($count) } réseaux visibles
    } par le projet { $project }.
incus-networks-search = Filtrer par nom ou type
incus-networks-empty = Aucun réseau
incus-networks-new = Nouveau bridge
incus-networks-new-title = Nouveau bridge
incus-networks-new-description = Un bridge géré : Incus lui fournit des adresses, le DHCP et le DNS, pour que les instances le rejoignent via un profil ou un périphérique.
incus-networks-new-name = Nom
incus-networks-new-ipv4 = IPv4
incus-networks-new-ipv6 = IPv6
incus-networks-new-addressHint = auto choisit un sous-réseau libre, none le désactive ; ou indiquez l’adresse du bridge avec son préfixe, comme 10.0.5.1/24.
incus-networks-new-nat = Masquer le trafic IPv4 sortant (NAT)
incus-networks-created = Bridge { $name } créé
incus-networks-column-name = Nom
incus-networks-column-type = Type
incus-networks-column-ipv4 = IPv4
incus-networks-column-ipv6 = IPv6
incus-networks-column-usedBy = Utilisé par
incus-networks-managed = Géré par Incus
incus-networks-unmanaged = Celui de cette machine
incus-networks-inUse = Utilisé par { $names } : retirez-le d’abord de ces éléments.
incus-networks-deleteOne = Supprimer { $name }
incus-networks-delete-title = Supprimer le réseau { $name } ?
incus-networks-delete-description = Incus supprime le bridge ainsi que son DHCP et son DNS. Rien ne l’utilise.
incus-networks-deleted = Réseau { $name } supprimé
incus-networks-gone = Le réseau { $name } n’existe plus.
incus-networks-backToList = Retour aux réseaux
incus-networks-field-managed = Créé par
incus-networks-field-state = État
incus-networks-field-addresses = Adresses sur l’hôte
incus-networks-field-ipv4 = Paramètre IPv4
incus-networks-field-ipv6 = Paramètre IPv6
incus-networks-field-nat = NAT
incus-networks-field-mtu = MTU
incus-networks-field-hwaddr = Adresse MAC
incus-networks-yes = Oui
incus-networks-no = Non
incus-networks-usedBy = Utilisé par
incus-networks-leases = Baux DHCP
incus-networks-noLeases = Aucun bail.
incus-networks-lease-hostname = Nom d’hôte
incus-networks-lease-address = Adresse
incus-networks-lease-hwaddr = Adresse MAC
incus-networks-lease-type = Type
