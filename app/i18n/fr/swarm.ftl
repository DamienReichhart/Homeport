### The `swarm` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/swarm.ts:
### change the text there, then run it again.

swarm-nav = Swarms
swarm-badge = Swarm
swarm-reachable = Un manager est connecté
swarm-unreachable = Aucun manager connecté
swarm-hostCounts = { $managers ->
        [one] { NUM($managers) } hôte manager
       *[other] { NUM($managers) } hôtes managers
    }, { $workers ->
        [one] { NUM($workers) } hôte worker
       *[other] { NUM($workers) } hôtes workers
    }
swarm-host-managerOf = Manager de { $swarm }
swarm-host-workerOf = Worker de { $swarm }
swarm-role-manager = Manager
swarm-role-worker = Worker
swarm-tab-summary = Résumé
swarm-tab-nodes = Nœuds
swarm-tab-services = Services
swarm-tab-tasks = Tâches
swarm-tab-stacks = Stacks
swarm-tab-secrets = Secrets
swarm-tab-configs = Configs
swarm-summary-title = Ce Swarm
swarm-summary-managerHosts = Hôtes managers
swarm-summary-workerHosts = Hôtes workers
swarm-summary-nodes = Nœuds
swarm-summary-nodeCounts = { $nodes ->
        [one] { NUM($nodes) } nœud
       *[other] { NUM($nodes) } nœuds
    }, { $managers ->
        [one] { NUM($managers) } manager
       *[other] { NUM($managers) } managers
    }
swarm-summary-services = Services
swarm-summary-created = Créé
swarm-summary-id = ID du Swarm
swarm-nodes-search = Rechercher des nœuds
swarm-nodes-column-hostname = Nœud
swarm-nodes-column-role = Rôle
swarm-nodes-column-availability = Disponibilité
swarm-nodes-column-state = État
swarm-nodes-column-engine = Moteur
swarm-nodes-column-address = Adresse
swarm-nodes-column-labels = Labels
swarm-nodes-isHost = hôte { $name }
swarm-nodes-availability-active = Actif
swarm-nodes-availability-pause = En pause
swarm-nodes-availability-drain = Drainé
swarm-nodes-manager-leader = leader
swarm-nodes-manager-reachable = joignable
swarm-nodes-manager-unreachable = injoignable
swarm-nodes-actionsFor = Actions pour { $name }
swarm-nodes-action-activate = Activer
swarm-nodes-action-pause = Mettre en pause
swarm-nodes-action-drain = Drainer
swarm-nodes-action-promote = Promouvoir en manager
swarm-nodes-action-demote = Rétrograder en worker
swarm-nodes-action-labels = Modifier les labels
swarm-nodes-action-tasks = Afficher ses tâches
swarm-nodes-updated = { $name } mis à jour
swarm-nodes-removed = { $name } retiré du Swarm
swarm-nodes-labels-title = Labels de { $name }
swarm-nodes-labels-description = Les contraintes de placement comme node.labels.zone==a choisissent les nœuds selon ces labels.
swarm-nodes-labels-key = Clé
swarm-nodes-labels-value = Valeur
swarm-nodes-labels-add = Ajouter un label
swarm-nodes-labels-removeOne = Supprimer le label { $key }
swarm-nodes-remove-title = Retirer { $name } du Swarm ?
swarm-nodes-remove-ready = Ce nœud est toujours actif. Le retirer de force laisse son moteur croire qu’il fait partie du Swarm jusqu’à ce qu’il le quitte de lui-même ; ses tâches sont replanifiées ailleurs.
swarm-nodes-remove-down = Ce nœud est hors service. Ses tâches sont replanifiées sur les autres nœuds.
swarm-nodes-remove-force = Forcer : le retirer bien qu’il soit actif
swarm-services-search = Rechercher des services
swarm-services-empty = Aucun service ne s’exécute sur ce Swarm
swarm-services-column-name = Nom
swarm-services-column-image = Image
swarm-services-column-mode = Mode
swarm-services-column-replicas = En cours d’exécution
swarm-services-column-ports = Ports
swarm-services-column-stack = Stack
swarm-services-column-update = Mise à jour
swarm-services-mode-replicated = Répliqué
swarm-services-mode-replicatedCount = Répliqué ({ $count })
swarm-services-mode-global = Global
swarm-services-mode-replicatedJob = Job
swarm-services-mode-globalJob = Job global
swarm-services-actionsFor = Actions pour { $name }
swarm-services-action-logs = Journaux
swarm-services-action-scale = Mettre à l’échelle
swarm-services-action-rollback = Revenir en arrière
swarm-services-scaled = { $name } mis à l’échelle à { $count ->
        [one] { NUM($count) } réplica
       *[other] { NUM($count) } réplicas
    }
swarm-services-rolledBack = { $name } revient en arrière
swarm-services-removed = { $name } supprimé
swarm-services-scale-title = Mettre { $name } à l’échelle
swarm-services-scale-description = Le Swarm démarre ou arrête des tâches jusqu’à ce que ce nombre s’exécute. 0 arrête le service sans le supprimer.
swarm-services-scale-replicas = Réplicas
swarm-services-scale-invalid = Saisissez un nombre entier de 0 à 10000.
swarm-services-rollback-title = Ramener { $name } en arrière ?
swarm-services-rollback-description = Ses paramètres précédents (image, environnement, réplicas…) sont rétablis, et ses tâches basculent vers eux comme lors d’une mise à jour.
swarm-services-remove-title = Supprimer le service { $name } ?
swarm-services-remove-description = { $count ->
        [0] Il n’exécute aucune tâche actuellement.
        [one] Sa tâche en cours d’exécution est arrêtée et supprimée.
       *[other] Ses { NUM($count) } tâches en cours d’exécution sont arrêtées et supprimées.
    } Ses volumes sont conservés.
swarm-service-tab-tasks = Tâches
swarm-service-tab-logs = Journaux
swarm-service-tab-inspect = Inspecter
swarm-service-gone = Ce service n’existe plus.
swarm-service-backToList = Retour aux services
swarm-tasks-column-task = Tâche
swarm-tasks-column-node = Nœud
swarm-tasks-column-state = État
swarm-tasks-column-error = Message
swarm-tasks-column-image = Image
swarm-tasks-desired = devrait être { $state }
swarm-tasks-openContainer = Ouvrir le conteneur
swarm-tasks-history = Afficher l’historique
swarm-tasks-onNode = Tâches sur { $name }
swarm-tasks-allNodes = Afficher tous les nœuds
swarm-tasks-empty = Aucune tâche
swarm-objects-searchSecrets = Rechercher des secrets
swarm-objects-searchConfigs = Rechercher des configs
swarm-objects-createSecret = Créer un secret
swarm-objects-createConfig = Créer une config
swarm-objects-secretNote = La valeur d’un secret est envoyée une seule fois au Swarm et ne peut pas être relue, ni par Homeport ni par personne : seuls les services auxquels il est donné la voient.
swarm-objects-noSecrets = Aucun secret
swarm-objects-noConfigs = Aucune config
swarm-objects-column-name = Nom
swarm-objects-column-services = Utilisé par
swarm-objects-column-labels = Labels
swarm-objects-column-created = Créé
swarm-objects-column-updated = Mis à jour
swarm-objects-unused = Inutilisé
swarm-objects-viewOne = Afficher { $name }
swarm-objects-removeOne = Supprimer { $name }
swarm-objects-removed = { $name } supprimé
swarm-objects-remove-title = Supprimer { $name } ?
swarm-objects-remove-used = Le Swarm refuse tant que des services l’utilisent : { $services }. Retirez-le d’abord de ces services.
swarm-objects-remove-unused = Aucun service ne l’utilise. Cette action est irréversible.
swarm-objects-binary = Contenu binaire, { $size ->
        [one] { NUM($size) } octet
       *[other] { NUM($size) } octets
    }.
swarm-objects-create-secretTitle = Créer un secret
swarm-objects-create-configTitle = Créer une config
swarm-objects-create-secretDescription = La valeur est envoyée au Swarm et jamais conservée par Homeport. Les services la lisent comme un fichier sous /run/secrets/.
swarm-objects-create-configDescription = Les services la lisent comme un fichier là où ils la montent. Les configs peuvent être affichées à nouveau plus tard.
swarm-objects-create-name = Nom
swarm-objects-create-secretPlaceholder = db_password
swarm-objects-create-configPlaceholder = nginx_conf
swarm-objects-create-value = Valeur
swarm-objects-create-content = Contenu
swarm-objects-create-show = Afficher
swarm-objects-create-hide = Masquer
swarm-objects-create-loadFile = Charger depuis un fichier…
swarm-objects-create-pickFile = Choisir le fichier à envoyer
swarm-objects-create-clearFile = Saisir plutôt la valeur
swarm-objects-create-fileNote = Le fichier est lu au moment de la création, envoyé au Swarm et non conservé.
swarm-services-created = { $name } créé
swarm-services-updating = { $name } bascule vers ses nouveaux paramètres
swarm-service-update = mise à jour { $state }
swarm-create-open = Créer un service
swarm-create-title = Créer un service
swarm-create-description = Le Swarm récupère l’image sur les nœuds qu’il choisit et maintient ce nombre de tâches en cours d’exécution. Le placement, les limites de ressources et les health checks se définissent avec le CLI docker et s’affichent dans Inspecter.
swarm-create-image = Image
swarm-create-imagePlaceholder = nginx:1.27
swarm-create-name = Nom
swarm-create-mode = Mode
swarm-create-restart = Redémarrage
swarm-create-restart-default = Toujours (par défaut)
swarm-create-restart-onFailure = En cas d’échec
swarm-create-restart-none = Jamais
swarm-create-command = Commande
swarm-create-commandPlaceholder = La commande de l’image
swarm-create-env = Environnement
swarm-create-hostMode = Publier uniquement sur le nœud qui exécute chaque tâche, plutôt que sur tous les nœuds
swarm-create-networks = Réseaux overlay
swarm-create-secrets = Secrets
swarm-create-configs = Configs
swarm-create-mountedAt = Emplacement de montage de { $name }
swarm-create-secretTarget = /run/secrets/{ $name }
swarm-create-configTarget = /{ $name }
swarm-create-submit = Créer
swarm-edit-open = Modifier
swarm-edit-title = Modifier { $name }
swarm-edit-description = Seul ce que vous modifiez ici est envoyé ; le reste du service est conservé. Le Swarm remplace ses tâches quelques-unes à la fois avec les nouveaux paramètres.
swarm-edit-redeploy = Redéployer toutes les tâches, en récupérant à nouveau l’image même si son tag est le même
swarm-edit-submit = Mettre à jour
swarm-membership-card-title = Swarm
swarm-membership-card-none = Cet hôte ne fait partie d’aucun Swarm.
swarm-membership-card-state = Son nœud est { $state }.
swarm-membership-card-ports = Démarrez un Swarm ici, ou rejoignez-en un. Les nœuds d’un Swarm doivent pouvoir se joindre sur 2377/tcp, 7946/tcp et udp, et 4789/udp ; Homeport ne peut pas ouvrir ces ports à votre place.
swarm-membership-created = Swarm créé
swarm-membership-joined = Swarm rejoint
swarm-membership-left = Swarm quitté
swarm-membership-advertise = Adresse annoncée
swarm-membership-advertisePlaceholder = Choisie par le moteur
swarm-membership-advertiseHelp = L’adresse à laquelle les autres nœuds joignent cet hôte. Laissez vide pour laisser le moteur choisir, ce qui fonctionne quand la machine n’a qu’une adresse.
swarm-membership-listen = Adresse d’écoute
swarm-membership-init-open = Initialiser un Swarm
swarm-membership-init-title = Démarrer un Swarm sur { $name }
swarm-membership-init-description = Cet hôte devient le premier manager d’un nouveau Swarm. D’autres hôtes peuvent ensuite le rejoindre comme workers ou managers.
swarm-membership-init-submit = Démarrer le Swarm
swarm-membership-join-open = Rejoindre un Swarm
swarm-membership-join-title = Faire rejoindre un Swarm à { $name }
swarm-membership-join-description = Choisissez un Swarm que Homeport connaît, et Homeport récupère son jeton d’adhésion auprès d’un manager connecté. Pour un autre Swarm, collez le jeton et l’adresse d’un manager obtenus avec docker swarm join-token.
swarm-membership-join-swarm = Swarm
swarm-membership-join-paste = Un autre Swarm, par son jeton
swarm-membership-join-token = Jeton d’adhésion
swarm-membership-join-address = Adresse du manager
swarm-membership-join-addressPlaceholder = 192.168.1.10:2377
swarm-membership-join-as = Rejoindre en tant que
swarm-membership-join-submit = Rejoindre
swarm-membership-leave-open = Quitter le Swarm
swarm-membership-leave-title = Faire quitter son Swarm à { $name } ?
swarm-membership-leave-worker = Ses tâches s’arrêtent ici et sont replanifiées sur les autres nœuds. Son nœud reste listé comme hors service jusqu’à ce qu’un manager le retire.
swarm-membership-leave-manager = Il cesse d’être un manager et un nœud. Les autres managers font tourner le Swarm ; le rétrograder d’abord est plus doux.
swarm-membership-leave-last = C’est le dernier manager du Swarm : le Swarm disparaît avec lui, avec tous ses services, secrets et configs. Les autres nœuds restent dans un Swarm sans manager.
swarm-membership-leave-force = Forcer : quitter bien qu’il soit manager
swarm-membership-leave-typeName = Saisissez { $name } pour confirmer
swarm-membership-leave-submit = Quitter
swarm-join-title = Commandes d’adhésion
swarm-join-description = Pour les machines que Homeport ne gère pas : exécutez-y l’une de ces commandes. Les hôtes que Homeport gère rejoignent plutôt depuis leur page, sans rien copier.
swarm-join-reveal = Afficher les commandes d’adhésion
swarm-join-hide = Les masquer
swarm-join-worker = En tant que worker
swarm-join-manager = En tant que manager
swarm-stacks-search = Rechercher des stacks
swarm-stacks-new = Nouvelle stack
swarm-stacks-deploy = Déployer
swarm-stacks-redeploy = Redéployer
swarm-stacks-adopt = Adopter
swarm-stacks-link = Lier à un dossier…
swarm-stacks-linkTitle = Choisir le dossier contenant les fichiers compose de { $name }
swarm-stacks-linked = { $name } est désormais une stack liée
swarm-stacks-empty = Aucune stack sur ce Swarm
swarm-stacks-emptyHint = Les stacks déployées avec docker stack deploy apparaissent ici, comme celles que vous déployez sur ce Swarm depuis Homeport.
swarm-stacks-backToList = Retour aux stacks
swarm-stacks-services = Services
swarm-stacks-noServices = Rien n’est encore déployé.
swarm-stacks-parts = Réseaux, secrets et configs
swarm-stacks-networks = Réseaux
swarm-stacks-noStartStop = Une stack swarm ne peut pas être arrêtée ni démarrée dans son ensemble : mettez plutôt ses services à l’échelle 0, depuis leurs pages.
swarm-stacks-removed = { $name } supprimée
swarm-stacks-removedPartly = { $name } supprimée, sauf :
swarm-stacks-remove-title = Supprimer la stack { $name } ?
swarm-stacks-remove-description = Ses services sont supprimés, puis ses réseaux, secrets et configs, comme avec docker stack rm. Ses volumes sont conservés sur chaque nœud.
