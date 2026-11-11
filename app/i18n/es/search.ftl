### The `search` namespace, in Español (es).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/es/search.ts:
### change the text there, then run it again.

search-kind-container = Contenedores
search-kind-stack = Stacks
search-kind-image = Imágenes
search-kind-volume = Volúmenes
search-kind-network = Redes
search-onHostState = { $host } · { $state ->
        [running] en ejecución
       *[other] detenido
    }
search-showAll = Mostrar los { $count }
search-skipped = { $count ->
        [one] { NUM($count) } host sin buscar (no conectado)
       *[other] { NUM($count) } hosts sin buscar (no conectados)
    }
