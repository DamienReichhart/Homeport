### The `board` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/board.ts:
### change the text there, then run it again.

board-nav = Quadro
board-title = Quadro
board-summary = { $running } de { $count ->
        [one] { NUM($count) } contêiner
       *[other] { NUM($count) } contêineres
    } em execução em { $hosts ->
        [one] { NUM($hosts) } host
       *[other] { NUM($hosts) } hosts
    }
board-column-host = Host
board-column-stack = Stack
board-sortBy = Ordenar por { $column }
board-filter-hosts = Hosts
board-filter-allHosts = Todos os hosts
board-filter-someHosts = { $count ->
        [one] { NUM($count) } host
       *[other] { NUM($count) } hosts
    }
board-filter-state = Estado
board-filter-stack = Stack
board-filter-anyStack = Qualquer stack
board-filter-clear = Limpar filtros
board-state-all = Qualquer estado
board-state-running = Em execução
board-state-stopped = Parado
board-state-unhealthy = Não íntegro
board-search = Pesquisar em todos os hosts
board-skipped = { $name } ({ $status }) não é mostrado.
board-openHost = Abrir host
board-listFailed = Não foi possível listar os contêineres de { $name }.
board-noHosts-title = Nenhum host está conectado
board-noHosts-description = O Quadro lista os contêineres de todos os hosts conectados.
board-empty-title = Nenhum contêiner ainda
board-empty-description = Nenhum dos seus hosts conectados executa um contêiner.
board-noMatch = Nenhum contêiner corresponde a esses filtros.
board-selection = Contêineres selecionados
board-selectAll = Selecionar todos os contêineres mostrados
board-selectOne = Selecionar { $name } em { $host }
board-remove-title = Remover { $count ->
        [one] { NUM($count) } contêiner
       *[other] { NUM($count) } contêineres
    }?
board-remove-description = Eles são excluídos com seus volumes anônimos; os volumes nomeados são mantidos. Os que estão em execução são parados antes.
board-remove-onHost = Em { $host }: { $names }
