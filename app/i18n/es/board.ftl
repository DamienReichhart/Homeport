### The `board` namespace, in Español (es).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/es/board.ts:
### change the text there, then run it again.

board-nav = Tablero
board-title = Tablero
board-summary = { $running } de { $count ->
        [one] { NUM($count) } contenedor
       *[other] { NUM($count) } contenedores
    } en ejecución en { $hosts ->
        [one] { NUM($hosts) } host
       *[other] { NUM($hosts) } hosts
    }
board-column-host = Host
board-column-stack = Stack
board-sortBy = Ordenar por { $column }
board-filter-hosts = Hosts
board-filter-allHosts = Todos los hosts
board-filter-someHosts = { $count ->
        [one] { NUM($count) } host
       *[other] { NUM($count) } hosts
    }
board-filter-state = Estado
board-filter-stack = Stack
board-filter-anyStack = Cualquier stack
board-filter-clear = Borrar filtros
board-state-all = Cualquier estado
board-state-running = En ejecución
board-state-stopped = Detenidos
board-state-unhealthy = No saludables
board-search = Buscar en todos los hosts
board-skipped = { $name } ({ $status }) no se muestra.
board-openHost = Abrir host
board-listFailed = No se pudieron listar los contenedores de { $name }.
board-noHosts-title = No hay ningún host conectado
board-noHosts-description = El Tablero lista los contenedores de todos los hosts conectados.
board-empty-title = Aún no hay contenedores
board-empty-description = Ninguno de tus hosts conectados ejecuta un contenedor.
board-noMatch = Ningún contenedor coincide con estos filtros.
board-selection = Contenedores seleccionados
board-selectAll = Seleccionar todos los contenedores mostrados
board-selectOne = Seleccionar { $name } en { $host }
board-remove-title = ¿Eliminar { $count ->
        [one] { NUM($count) } contenedor
       *[other] { NUM($count) } contenedores
    }?
board-remove-description = Se eliminan junto con sus volúmenes anónimos; los volúmenes con nombre se conservan. Los que están en ejecución se detienen antes.
board-remove-onHost = En { $host }: { $names }
