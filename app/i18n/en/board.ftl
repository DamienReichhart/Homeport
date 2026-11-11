### The `board` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/board.ts:
### change the text there, then run it again.

board-nav = Board
board-title = Board
board-summary = { $running } of { $count ->
        [one] { NUM($count) } container
       *[other] { NUM($count) } containers
    } running on { $hosts ->
        [one] { NUM($hosts) } host
       *[other] { NUM($hosts) } hosts
    }
board-column-host = Host
board-column-stack = Stack
board-sortBy = Sort by { $column }
board-filter-hosts = Hosts
board-filter-allHosts = All hosts
board-filter-someHosts = { $count ->
        [one] { NUM($count) } host
       *[other] { NUM($count) } hosts
    }
board-filter-state = State
board-filter-stack = Stack
board-filter-anyStack = Any Stack
board-filter-clear = Clear filters
board-state-all = Any state
board-state-running = Running
board-state-stopped = Stopped
board-state-unhealthy = Unhealthy
board-search = Search every host
board-skipped = { $name } ({ $status }) isn’t shown.
board-openHost = Open host
board-listFailed = Couldn’t list the containers of { $name }.
board-noHosts-title = No host is connected
board-noHosts-description = The board lists the containers of every connected host.
board-empty-title = No containers yet
board-empty-description = None of your connected hosts runs a container.
board-noMatch = No container matches these filters.
board-selection = Selected containers
board-selectAll = Select every container shown
board-selectOne = Select { $name } on { $host }
board-remove-title = Remove { $count ->
        [one] { NUM($count) } container
       *[other] { NUM($count) } containers
    }?
board-remove-description = They’re deleted with their anonymous volumes; named volumes are kept. Running ones are stopped first.
board-remove-onHost = On { $host }: { $names }
