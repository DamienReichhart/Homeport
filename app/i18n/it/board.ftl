### The `board` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/board.ts:
### change the text there, then run it again.

board-nav = Bacheca
board-title = Bacheca
board-summary = { $running } di { $count ->
        [one] { NUM($count) } container
       *[other] { NUM($count) } container
    } in esecuzione su { $hosts ->
        [one] { NUM($hosts) } host
       *[other] { NUM($hosts) } host
    }
board-column-host = Host
board-column-stack = Stack
board-sortBy = Ordina per { $column }
board-filter-hosts = Host
board-filter-allHosts = Tutti gli host
board-filter-someHosts = { $count ->
        [one] { NUM($count) } host
       *[other] { NUM($count) } host
    }
board-filter-state = Stato
board-filter-stack = Stack
board-filter-anyStack = Qualsiasi stack
board-filter-clear = Cancella i filtri
board-state-all = Qualsiasi stato
board-state-running = In esecuzione
board-state-stopped = Arrestati
board-state-unhealthy = Non integri
board-search = Cerca in tutti gli host
board-skipped = { $name } ({ $status }) non è mostrato.
board-openHost = Apri l’host
board-listFailed = Impossibile elencare i container di { $name }.
board-noHosts-title = Nessun host connesso
board-noHosts-description = La bacheca elenca i container di tutti gli host connessi.
board-empty-title = Ancora nessun container
board-empty-description = Nessuno dei tuoi host connessi esegue un container.
board-noMatch = Nessun container corrisponde a questi filtri.
board-selection = Container selezionati
board-selectAll = Seleziona tutti i container mostrati
board-selectOne = Seleziona { $name } su { $host }
board-remove-title = Rimuovere { $count ->
        [one] { NUM($count) } container
       *[other] { NUM($count) } container
    }?
board-remove-description = Vengono eliminati con i loro volumi anonimi; i volumi con nome vengono conservati. Quelli in esecuzione vengono prima arrestati.
board-remove-onHost = Su { $host }: { $names }
