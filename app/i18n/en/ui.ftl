### The `ui` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/ui.ts:
### change the text there, then run it again.

ui-copy = Copy
ui-copied = Copied
ui-search = Search
ui-search-clear = Clear search
ui-shortcuts-focusSearch = Search this page
ui-shortcuts-thisPage = This page
ui-table-selectAll = Select all
ui-table-selectRow = Select row
ui-table-empty = Nothing here.
ui-table-page = Page { $page } of { $pages } · { $rows ->
        [one] { NUM($rows) } row
       *[other] { NUM($rows) } rows
    }
ui-table-previous = Previous page
ui-table-next = Next page
ui-transfer-done = Done.
ui-files-column-name = Name
ui-files-column-size = Size
ui-files-column-modified = Modified
ui-files-column-mode = Mode
ui-files-path = Path
ui-files-downloadOne = Download { $name }
ui-files-empty = This folder is empty.
ui-chart-time = Time
ui-chart-hide = Hide { $name }
ui-chart-show = Show { $name }
ui-errorDetails-title = Error details
ui-errorDetails-code = Code
ui-errorDetails-command = Command
ui-errorDetails-when = When
ui-errorDetails-app = App
ui-errorDetails-rawDetail = Raw detail
ui-errorDetails-rawDetailNote = As the system, the Engine or the library wrote it, so it isn’t translated.
ui-errorDetails-openLogFolder = Open log folder
ui-errorDetails-copyReport = Copy report
ui-confirm-typeName = Type { $name } to confirm
ui-select-noMatch = Nothing matches.
ui-password-show = Show password
ui-password-hide = Hide password
ui-log-wrap = Wrap lines
ui-log-latest = Jump to latest
ui-log-matchCase = Match case
ui-log-regex = Regular expression
ui-log-filter = Filter
ui-log-filterHint = Show only the lines that match
ui-log-matches = { $current } of { $count }
ui-log-noMatches = No matches
ui-log-invalidRegex = Not a valid regular expression
ui-log-skipped = { $count ->
        [one] — { NUM($count) } line skipped —
       *[other] — { NUM($count) } lines skipped —
    }
ui-log-searchFailed = The search stopped before reading every line
