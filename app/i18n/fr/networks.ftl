### The `networks` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/networks.ts:
### change the text there, then run it again.

networks-nav = Réseaux
networks-title = Réseaux
networks-summary = { $count ->
        [one] { NUM($count) } réseau
       *[other] { NUM($count) } réseaux
    }
networks-cleanUp = Nettoyer
networks-create = Créer un réseau
networks-search = Rechercher des réseaux
networks-empty = Aucun réseau
networks-column-name = Nom
networks-column-stack = Stack
networks-column-driver = Pilote
networks-column-subnet = Sous-réseau
networks-column-containers = Conteneurs
networks-builtIn = intégré
networks-internal = interne
networks-inspectOne = Inspecter { $name }
networks-removeOne = Supprimer { $name }
networks-remove-title = Supprimer le réseau { $name } ?
networks-remove-attached = Il a encore { $count ->
        [one] { NUM($count) } conteneur attaché
       *[other] { NUM($count) } conteneurs attachés
    } ; déconnectez-les d’abord.
networks-remove-unused = Aucun conteneur ne l’utilise. Les conteneurs qui y font référence par son nom ne pourront pas démarrer.
networks-prune-title = Supprimer les réseaux inutilisés ?
networks-prune-description = Tous les réseaux personnalisés auxquels aucun conteneur n’est attaché sont supprimés.
networks-connected = Connecté
networks-disconnected = Déconnecté
networks-removed = { $name } supprimé
networks-pruned = { $count ->
        [one] { NUM($count) } réseau supprimé
       *[other] { NUM($count) } réseaux supprimés
    }
networks-createDialog-title = Créer un réseau
networks-createDialog-name = Nom
networks-createDialog-driver = Pilote
networks-createDialog-subnet = Sous-réseau
networks-createDialog-gateway = Passerelle
networks-createDialog-automatic = automatique
networks-createDialog-internal = Interne — pas d’accès aux réseaux extérieurs
networks-createDialog-attachable = Attachable
networks-dialog-containers = Conteneurs
networks-dialog-raw = JSON brut
networks-dialog-noContainer = Aucun conteneur n’est attaché.
networks-dialog-disconnect = Déconnecter
networks-dialog-containerToConnect = Conteneur à connecter
networks-dialog-connectPlaceholder = Connecter un conteneur…
networks-dialog-connect = Connecter
