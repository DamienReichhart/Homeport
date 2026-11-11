### The `search` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/search.ts:
### change the text there, then run it again.

search-kind-container = Containers
search-kind-stack = Stacks
search-kind-image = Images
search-kind-volume = Volumes
search-kind-network = Networks
search-onHostState = { $host } · { $state ->
        [running] running
       *[other] stopped
    }
search-showAll = Show all { $count }
search-skipped = { $count ->
        [one] { NUM($count) } host
       *[other] { NUM($count) } hosts
    } not searched (not connected)
