### The `search` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/search.ts:
### change the text there, then run it again.

search-kind-container = Container
search-kind-stack = Stacks
search-kind-image = Images
search-kind-volume = Volumes
search-kind-network = Netzwerke
search-onHostState = { $host } · { $state ->
        [running] läuft
       *[other] gestoppt
    }
search-showAll = Alle { $count } anzeigen
search-skipped = { $count ->
        [one] { NUM($count) } Host
       *[other] { NUM($count) } Hosts
    } nicht durchsucht (nicht verbunden)
