### The `ui` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/ui.ts:
### change the text there, then run it again.

ui-copy = Kopieren
ui-copied = Kopiert
ui-search = Suchen
ui-search-clear = Suche löschen
ui-shortcuts-focusSearch = Diese Seite durchsuchen
ui-shortcuts-thisPage = Diese Seite
ui-table-selectAll = Alle auswählen
ui-table-selectRow = Zeile auswählen
ui-table-empty = Hier ist nichts.
ui-table-page = Seite { $page } von { $pages } · { $rows ->
        [one] { NUM($rows) } Zeile
       *[other] { NUM($rows) } Zeilen
    }
ui-table-previous = Vorherige Seite
ui-table-next = Nächste Seite
ui-transfer-done = Fertig.
ui-files-column-name = Name
ui-files-column-size = Größe
ui-files-column-modified = Geändert
ui-files-column-mode = Modus
ui-files-path = Pfad
ui-files-downloadOne = { $name } herunterladen
ui-files-empty = Dieser Ordner ist leer.
ui-files-upload = Hochladen
ui-json-key = Schlüssel
ui-json-value = Wert
ui-json-copyPath = Pfad kopieren
ui-json-copyValue = Wert kopieren
ui-chart-time = Zeit
ui-chart-hide = { $name } ausblenden
ui-chart-show = { $name } einblenden
ui-chart-more = +{ $count }
ui-errorDetails-title = Fehlerdetails
ui-errorDetails-code = Code
ui-errorDetails-command = Befehl
ui-errorDetails-when = Zeitpunkt
ui-errorDetails-app = App
ui-errorDetails-rawDetail = Rohdetails
ui-errorDetails-rawDetailNote = So, wie das System, die Engine oder die Bibliothek es geschrieben hat, daher nicht übersetzt.
ui-errorDetails-openLogFolder = Protokollordner öffnen
ui-errorDetails-copyReport = Bericht kopieren
ui-confirm-typeName = Geben Sie zur Bestätigung { $name } ein
ui-select-noMatch = Keine Treffer.
ui-password-show = Passwort anzeigen
ui-password-hide = Passwort verbergen
ui-log-wrap = Zeilen umbrechen
ui-log-latest = Zum Neuesten springen
ui-log-matchCase = Groß-/Kleinschreibung beachten
ui-log-regex = Regulärer Ausdruck
ui-log-filter = Filtern
ui-log-filterHint = Nur passende Zeilen anzeigen
ui-log-matches = { $current } von { $count }
ui-log-noMatches = Keine Treffer
ui-log-invalidRegex = Kein gültiger regulärer Ausdruck
ui-log-skipped = { $count ->
        [one] — { NUM($count) } Zeile übersprungen —
       *[other] — { NUM($count) } Zeilen übersprungen —
    }
ui-log-searchFailed = Die Suche wurde abgebrochen, bevor alle Zeilen gelesen waren
ui-terminal-find = Im Terminal suchen
ui-terminal-closeSearch = Suche schließen
ui-terminal-atLeast = { $count }+
ui-terminal-noneNear = Keine Treffer in der Nähe
