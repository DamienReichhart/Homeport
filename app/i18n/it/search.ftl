### The `search` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/search.ts:
### change the text there, then run it again.

search-kind-container = Container
search-kind-stack = Stack
search-kind-image = Immagini
search-kind-volume = Volumi
search-kind-network = Reti
search-onHostState = { $host } · { $state ->
        [running] in esecuzione
       *[other] arrestato
    }
search-showAll = Mostra tutti ({ $count })
search-skipped = { $count ->
        [one] { NUM($count) } host non incluso
       *[other] { NUM($count) } host non inclusi
    } nella ricerca (non connessi)
