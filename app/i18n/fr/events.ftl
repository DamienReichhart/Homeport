### The `events` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/events.ts:
### change the text there, then run it again.

events-nav = Événements
events-title = Événements
events-description = Ce que le moteur a signalé, conservé entre les redémarrages.
events-descriptionKept = Ce que le moteur a signalé, conservé { $days ->
        [one] { NUM($days) } jour
       *[other] { NUM($days) } jours
    }, jusqu’à 10 000 événements.
events-clear = Effacer l’historique
events-clear-title = Effacer l’historique des événements ?
events-clear-description = Tous les événements conservés pour cet hôte sont oubliés. Les nouveaux continuent d’apparaître ; ceux du moteur lui-même ne sont pas touchés.
events-clear-confirm = Effacer l’historique
events-cleared = Historique des événements effacé
events-loadOlder = Charger des événements plus anciens
events-search = Rechercher des événements
events-type = Type
events-allTypes = Tous les types
events-empty-title = Aucun événement pour l’instant
events-empty-description = Démarrez ou arrêtez un conteneur et il apparaîtra ici.
events-column-time = Heure
events-column-type = Type
events-column-action = Action
events-column-object = Objet
events-column-stack = Stack
