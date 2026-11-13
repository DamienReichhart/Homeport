### The `ui` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/ui.ts:
### change the text there, then run it again.

ui-copy = Copia
ui-copied = Copiato
ui-search = Cerca
ui-search-clear = Cancella ricerca
ui-shortcuts-focusSearch = Cerca in questa pagina
ui-shortcuts-thisPage = Questa pagina
ui-table-selectAll = Seleziona tutto
ui-table-selectRow = Seleziona riga
ui-table-empty = Niente da mostrare.
ui-table-page = Pagina { $page } di { $pages } · { $rows ->
        [one] { NUM($rows) } riga
       *[other] { NUM($rows) } righe
    }
ui-table-previous = Pagina precedente
ui-table-next = Pagina successiva
ui-transfer-done = Fatto.
ui-files-column-name = Nome
ui-files-column-size = Dimensione
ui-files-column-modified = Modificato
ui-files-column-mode = Permessi
ui-files-path = Percorso
ui-files-downloadOne = Scarica { $name }
ui-files-empty = Questa cartella è vuota.
ui-files-upload = Carica
ui-json-key = Chiave
ui-json-value = Valore
ui-json-copyPath = Copia percorso
ui-json-copyValue = Copia valore
ui-chart-time = Ora
ui-chart-hide = Nascondi { $name }
ui-chart-show = Mostra { $name }
ui-chart-more = +{ $count }
ui-errorDetails-title = Dettagli dell’errore
ui-errorDetails-code = Codice
ui-errorDetails-command = Comando
ui-errorDetails-when = Quando
ui-errorDetails-app = App
ui-errorDetails-rawDetail = Dettaglio originale
ui-errorDetails-rawDetailNote = Così come l’ha scritto il sistema, l’Engine o la libreria, quindi non è tradotto.
ui-errorDetails-openLogFolder = Apri la cartella dei log
ui-errorDetails-copyReport = Copia il rapporto
ui-confirm-typeName = Digita { $name } per confermare
ui-select-noMatch = Nessun risultato.
ui-password-show = Mostra la password
ui-password-hide = Nascondi la password
ui-log-wrap = A capo automatico
ui-log-latest = Vai alle ultime righe
ui-log-matchCase = Maiuscole/minuscole
ui-log-regex = Espressione regolare
ui-log-filter = Filtra
ui-log-filterHint = Mostra solo le righe corrispondenti
ui-log-matches = { $current } di { $count }
ui-log-noMatches = Nessun risultato
ui-log-invalidRegex = Espressione regolare non valida
ui-log-skipped = { $count ->
        [one] — { NUM($count) } riga saltata —
       *[other] — { NUM($count) } righe saltate —
    }
ui-log-searchFailed = La ricerca si è interrotta prima di leggere tutte le righe
