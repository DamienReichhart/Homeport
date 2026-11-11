### The `board` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/board.ts:
### change the text there, then run it again.

board-nav = Board
board-title = Board
board-summary = { $running } von { $count ->
        [one] { NUM($count) } Container
       *[other] { NUM($count) } Containern
    } laufen auf { $hosts ->
        [one] { NUM($hosts) } Host
       *[other] { NUM($hosts) } Hosts
    }
board-column-host = Host
board-column-stack = Stack
board-sortBy = Nach { $column } sortieren
board-filter-hosts = Hosts
board-filter-allHosts = Alle Hosts
board-filter-someHosts = { $count ->
        [one] { NUM($count) } Host
       *[other] { NUM($count) } Hosts
    }
board-filter-state = Zustand
board-filter-stack = Stack
board-filter-anyStack = Beliebiger Stack
board-filter-clear = Filter zurücksetzen
board-state-all = Beliebiger Zustand
board-state-running = Läuft
board-state-stopped = Gestoppt
board-state-unhealthy = Fehlerhaft
board-search = Alle Hosts durchsuchen
board-skipped = { $name } ({ $status }) wird nicht angezeigt.
board-openHost = Host öffnen
board-listFailed = Die Container von { $name } konnten nicht aufgelistet werden.
board-noHosts-title = Kein Host ist verbunden
board-noHosts-description = Das Board listet die Container aller verbundenen Hosts auf.
board-empty-title = Noch keine Container
board-empty-description = Auf keinem Ihrer verbundenen Hosts läuft ein Container.
board-noMatch = Kein Container passt zu diesen Filtern.
board-selection = Ausgewählte Container
board-selectAll = Alle angezeigten Container auswählen
board-selectOne = { $name } auf { $host } auswählen
board-remove-title = { $count ->
        [one] { NUM($count) } Container
       *[other] { NUM($count) } Container
    } entfernen?
board-remove-description = Sie werden mit ihren anonymen Volumes gelöscht; benannte Volumes bleiben erhalten. Laufende werden zuerst gestoppt.
board-remove-onHost = Auf { $host }: { $names }
