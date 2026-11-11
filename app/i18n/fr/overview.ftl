### The `overview` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/overview.ts:
### change the text there, then run it again.

overview-nav = Vue d’ensemble
overview-title = Vue d’ensemble
overview-addHost = Ajouter un hôte
overview-addCluster = Ajouter un cluster
overview-cluster-nodes = { $ready } Nodes prêts sur { $total }
overview-cluster-nodesNotAllowed = Nodes : non autorisé
overview-cluster-pods = { $running } Pods en cours d’exécution sur { $total }
overview-cluster-podsNotAllowed = Pods : non autorisé
overview-cluster-notAllowed = Non autorisé.
overview-cluster-versionTitle = Cluster
overview-cluster-version = Version
overview-cluster-platform = Plateforme
overview-cluster-server = Serveur d’API
overview-nomad-region = Région
overview-nomad-address = Adresse
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = Tous les namespaces
overview-cluster-nodesTitle = Nodes
overview-cluster-nodesReady = prêts
overview-cluster-allNodes = Tous les Nodes
overview-cluster-podsTitle = Pods
overview-cluster-running = { $count } en cours d’exécution
overview-cluster-pending = { $count } en attente
overview-cluster-crashing = { $count } en échec
overview-cluster-done = { $count } terminés ou autres
overview-cluster-allPods = Tous les Pods
overview-cluster-unreadyTitle = Charges de travail non prêtes
overview-cluster-allReady = Chaque Deployment, StatefulSet et DaemonSet a ses Pods prêts.
overview-cluster-warningsTitle = Avertissements récents
overview-cluster-noWarnings = Aucun événement d’avertissement.
overview-cluster-allEvents = Tous les événements
overview-host-running = <strong>{ $running }</strong> / { $total } en cours d’exécution
overview-host-images = { $count ->
        [one] { NUM($count) } image
       *[other] { NUM($count) } images
    }
overview-host-containerdContainers = { $count ->
        [one] { NUM($count) } conteneur
       *[other] { NUM($count) } conteneurs
    } dans { $namespaces ->
        [one] { NUM($namespaces) } espace de noms
       *[other] { NUM($namespaces) } espaces de noms
    }
overview-host-incusInstances = { $count ->
        [one] { NUM($count) } instance
       *[other] { NUM($count) } instances
    } dans { $projects ->
        [one] { NUM($projects) } projet
       *[other] { NUM($projects) } projets
    }
overview-incus-title = Incus
overview-incus-version = Version
overview-incus-os = Système
overview-incus-arch = Architecture
overview-incus-projects = Projets
overview-incus-shown = Affiché
overview-incus-holds = { $instances ->
        [one] { NUM($instances) } instance
       *[other] { NUM($instances) } instances
    }, { $images ->
        [one] { NUM($images) } image
       *[other] { NUM($images) } images
    }
overview-incus-instances = Instances
overview-incus-images = Images
overview-incus-storage = Stockage
overview-containerd-title = containerd
overview-containerd-version = Version
overview-containerd-revision = Révision
overview-containerd-platform = Plateforme
overview-containerd-namespaces = Espaces de noms
overview-containerd-shown = Affiché
overview-containerd-holds = { $containers ->
        [one] { NUM($containers) } conteneur
       *[other] { NUM($containers) } conteneurs
    }, { $images ->
        [one] { NUM($images) } image
       *[other] { NUM($images) } images
    }
overview-containerd-containers = Conteneurs
overview-containerd-images = Images
overview-containerd-snapshots = Instantanés
overview-host-cpu = { $percent } de { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPU
    }
overview-host-cpuTitle = Utilisation CPU des conteneurs, 100 % par CPU
overview-host-memoryTitle = Utilisation mémoire des conteneurs
overview-engine-title = Moteur
overview-engine-name = Nom
overview-engine-version = Version
overview-engine-os = OS
overview-engine-kernel = Noyau
overview-engine-architecture = Architecture
overview-engine-resources = Ressources
overview-engine-resourcesValue = { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPU
    } · { $memory }
overview-engine-storageDriver = Pilote de stockage
overview-engine-rootDir = Répertoire racine
overview-disk-title = Utilisation du disque
overview-disk-cleanUp = Nettoyer…
overview-disk-column-type = Type
overview-disk-column-total = Total
overview-disk-column-active = Utilisé
overview-disk-column-size = Taille
overview-disk-column-reclaimable = Récupérable
overview-disk-images = Images
overview-disk-containers = Conteneurs
overview-disk-volumes = Volumes
overview-disk-buildCache = Cache de build
overview-disk-partial = Ce Docker Engine est antérieur à la version 29 et en indique moins : la taille des images est estimée, et celle des volumes et du cache de build n’est pas disponible.
overview-events-title = Événements récents
overview-events-all = Tous les événements
overview-events-empty = Rien ne s’est passé depuis la connexion de Homeport.
overview-prune-title = Nettoyer
overview-prune-description = Choisissez ce qu’il faut supprimer de cet hôte. Cette action est irréversible.
overview-prune-confirm = Nettoyer
overview-prune-confirmWithVolumes = Supprimer, données des volumes comprises
overview-prune-containers = Conteneurs arrêtés
overview-prune-containers-detail = Supprime tous les conteneurs qui ne sont pas en cours d’exécution.
overview-prune-images = Images orphelines
overview-prune-images-detail = Couches sans tag laissées par les builds et les récupérations.
overview-prune-allImages = Toutes les images inutilisées
overview-prune-allImages-detail = Supprime aussi les images avec tag qu’aucun conteneur n’utilise.
overview-prune-networks = Réseaux inutilisés
overview-prune-networks-detail = Réseaux personnalisés auxquels aucun conteneur n’est attaché.
overview-prune-buildCache = Cache de build
overview-prune-buildCache-detail = Couches de build en cache.
overview-prune-volumes = Volumes inutilisés
overview-prune-volumes-detail = Supprime les données de tous les volumes qu’aucun conteneur n’utilise, volumes nommés compris.
overview-pruned = { $containers ->
        [one] { NUM($containers) } conteneur
       *[other] { NUM($containers) } conteneurs
    }, { $images ->
        [one] { NUM($images) } image
       *[other] { NUM($images) } images
    }, { $networks ->
        [one] { NUM($networks) } réseau
       *[other] { NUM($networks) } réseaux
    } et { $volumes ->
        [one] { NUM($volumes) } volume
       *[other] { NUM($volumes) } volumes
    } supprimés — { $space } récupérés.
overview-palette-prune = Nettoyer l’hôte sélectionné
overview-host-onBoard = Ses conteneurs sur le tableau
