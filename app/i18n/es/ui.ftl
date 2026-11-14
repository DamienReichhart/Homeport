### The `ui` namespace, in Español (es).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/es/ui.ts:
### change the text there, then run it again.

ui-copy = Copiar
ui-copied = Copiado
ui-search = Buscar
ui-search-clear = Borrar búsqueda
ui-shortcuts-focusSearch = Buscar en esta página
ui-shortcuts-thisPage = Esta página
ui-table-selectAll = Seleccionar todo
ui-table-selectRow = Seleccionar fila
ui-table-empty = No hay nada aquí.
ui-table-page = Página { $page } de { $pages } · { $rows ->
        [one] { NUM($rows) } fila
       *[other] { NUM($rows) } filas
    }
ui-table-previous = Página anterior
ui-table-next = Página siguiente
ui-transfer-done = Listo.
ui-files-column-name = Nombre
ui-files-column-size = Tamaño
ui-files-column-modified = Modificado
ui-files-column-mode = Modo
ui-files-path = Ruta
ui-files-downloadOne = Descargar { $name }
ui-files-empty = Esta carpeta está vacía.
ui-files-upload = Subir
ui-json-key = Clave
ui-json-value = Valor
ui-json-copyPath = Copiar ruta
ui-json-copyValue = Copiar valor
ui-chart-time = Hora
ui-chart-hide = Ocultar { $name }
ui-chart-show = Mostrar { $name }
ui-chart-more = +{ $count }
ui-errorDetails-title = Detalles del error
ui-errorDetails-code = Código
ui-errorDetails-command = Comando
ui-errorDetails-when = Cuándo
ui-errorDetails-app = App
ui-errorDetails-rawDetail = Detalle original
ui-errorDetails-rawDetailNote = Tal como lo escribió el sistema, el Engine o la biblioteca, por eso no está traducido.
ui-errorDetails-openLogFolder = Abrir carpeta de logs
ui-errorDetails-copyReport = Copiar informe
ui-confirm-typeName = Escribe { $name } para confirmar
ui-select-noMatch = No hay coincidencias.
ui-password-show = Mostrar la contraseña
ui-password-hide = Ocultar la contraseña
ui-log-wrap = Ajustar líneas
ui-log-latest = Ir a lo más reciente
ui-log-matchCase = Distinguir mayúsculas
ui-log-regex = Expresión regular
ui-log-filter = Filtrar
ui-log-filterHint = Mostrar solo las líneas que coinciden
ui-log-matches = { $current } de { $count }
ui-log-noMatches = Sin coincidencias
ui-log-invalidRegex = No es una expresión regular válida
ui-log-skipped = { $count ->
        [one] — { NUM($count) } línea omitida —
       *[other] — { NUM($count) } líneas omitidas —
    }
ui-log-searchFailed = La búsqueda se detuvo antes de leer todas las líneas
ui-terminal-find = Buscar en el terminal
ui-terminal-closeSearch = Cerrar búsqueda
ui-terminal-atLeast = { $count }+
