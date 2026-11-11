### The `podmanpods` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/podmanpods.ts:
### change the text there, then run it again.

podmanpods-nav = Pods
podmanpods-title = Pods
podmanpods-summary = { $count ->
        [one] { NUM($count) } pod
       *[other] { NUM($count) } pods
    }
podmanpods-search = Rechercher des pods
podmanpods-empty = Aucun pod
podmanpods-emptyDescription = Un pod regroupe des conteneurs qui partagent un réseau et démarrent et s’arrêtent ensemble. Créez-en un, puis exécutez-y des conteneurs.
podmanpods-new = Nouveau pod
podmanpods-column-name = Nom
podmanpods-column-status = État
podmanpods-column-containers = Conteneurs
podmanpods-column-networks = Réseaux
podmanpods-column-created = Créé
podmanpods-running = { $running } sur { $count } en cours d’exécution
podmanpods-status-running = En cours d’exécution
podmanpods-status-degraded = Partiellement en cours d’exécution
podmanpods-status-paused = En pause
podmanpods-status-created = Créé
podmanpods-status-stopped = Arrêté
podmanpods-status-failed = En échec
podmanpods-action-start = Démarrer
podmanpods-action-stop = Arrêter
podmanpods-action-restart = Redémarrer
podmanpods-action-pause = Mettre en pause
podmanpods-action-unpause = Reprendre
podmanpods-actionOne-start = Démarrer { $name }
podmanpods-actionOne-stop = Arrêter { $name }
podmanpods-actionOne-restart = Redémarrer { $name }
podmanpods-actionOne-pause = Mettre { $name } en pause
podmanpods-actionOne-unpause = Reprendre { $name }
podmanpods-removeOne = Supprimer { $name }
podmanpods-done-start = { $name } démarré
podmanpods-done-stop = { $name } arrêté
podmanpods-done-restart = { $name } redémarré
podmanpods-done-pause = { $name } mis en pause
podmanpods-done-unpause = { $name } a repris
podmanpods-removed = { $name } supprimé, { $count ->
        [0] sans conteneur
        [one] avec { NUM($count) } conteneur
       *[other] avec { NUM($count) } conteneurs
    }
podmanpods-created = { $name } créé
podmanpods-remove-title = Supprimer { $name } ?
podmanpods-remove-stopped = { $count ->
        [0] Le pod et son conteneur infra sont supprimés.
        [one] Le pod, son conteneur infra et son { NUM($count) } conteneur sont supprimés.
       *[other] Le pod, son conteneur infra et ses { NUM($count) } conteneurs sont supprimés.
    } Leurs volumes sont conservés.
podmanpods-remove-running = { $count ->
        [0] Le pod est en cours d’exécution : il est arrêté, puis supprimé avec son conteneur infra.
        [one] Le pod est en cours d’exécution : son { NUM($count) } conteneur est arrêté, puis supprimé avec le pod et son conteneur infra.
       *[other] Le pod est en cours d’exécution : ses { NUM($count) } conteneurs sont arrêtés, puis supprimés avec le pod et son conteneur infra.
    } Leurs volumes sont conservés.
podmanpods-remove-stopAndRemove = Arrêter et supprimer
podmanpods-detail-containers = { $count ->
        [0] Conteneurs
        [one] { NUM($count) } conteneur
       *[other] { NUM($count) } conteneurs
    }
podmanpods-detail-noContainers = Rien ne s’exécute encore dans ce pod, à part son conteneur infra.
podmanpods-detail-infra = Infra
podmanpods-detail-settings = Paramètres
podmanpods-detail-ports = Ports publiés
podmanpods-detail-noPorts = Aucun
podmanpods-detail-shared = Namespaces partagés
podmanpods-detail-networks = Réseaux
podmanpods-detail-hostname = Nom d’hôte
podmanpods-detail-created = Créé
podmanpods-detail-labels = Labels
podmanpods-detail-command = Créé avec
podmanpods-detail-runContainer = Exécuter un conteneur
podmanpods-detail-runContainerIn = Exécuter un conteneur dans { $name }
podmanpods-newDialog-title = Nouveau pod
podmanpods-newDialog-description = Ses conteneurs partagent un même réseau : les ports sont donc publiés sur le pod, pour tous.
podmanpods-newDialog-name = Nom
podmanpods-newDialog-portsHint = Laissez le port de l’hôte vide pour que Podman en choisisse un libre. Les conteneurs exécutés dans le pod ne peuvent pas publier leurs propres ports.
podmanpods-newDialog-labels = Labels
podmanpods-newDialog-create = Créer le pod
podmanpods-form-nameInvalid = Commencez par une lettre ou un chiffre, puis utilisez des lettres, des chiffres, _, . ou -.
podmanpods-form-portInvalid = Les ports sont des nombres de 1 à 65535.
podmanpods-form-labelInvalid = Écrivez un label clé=valeur par ligne.
