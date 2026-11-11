### The `search` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/search.ts:
### change the text there, then run it again.

search-kind-container = Contêineres
search-kind-stack = Stacks
search-kind-image = Imagens
search-kind-volume = Volumes
search-kind-network = Redes
search-onHostState = { $host } · { $state ->
        [running] em execução
       *[other] parado
    }
search-showAll = Mostrar todos os { $count }
search-skipped = { $count ->
        [one] { NUM($count) } host não pesquisado
       *[other] { NUM($count) } hosts não pesquisados
    } (não conectado)
