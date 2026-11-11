### The `search` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/search.ts:
### change the text there, then run it again.

search-kind-container = Conteneurs
search-kind-stack = Stacks
search-kind-image = Images
search-kind-volume = Volumes
search-kind-network = Réseaux
search-onHostState = { $host } · { $state ->
        [running] en cours d’exécution
       *[other] arrêté
    }
search-showAll = Afficher les { $count }
search-skipped = { $count ->
        [one] { NUM($count) } hôte non parcouru
       *[other] { NUM($count) } hôtes non parcourus
    } (non connecté)
