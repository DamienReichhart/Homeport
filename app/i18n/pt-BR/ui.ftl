### The `ui` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/ui.ts:
### change the text there, then run it again.

ui-copy = Copiar
ui-copied = Copiado
ui-search = Pesquisar
ui-shortcuts-focusSearch = Pesquisar nesta página
ui-shortcuts-thisPage = Esta página
ui-table-selectAll = Selecionar tudo
ui-table-selectRow = Selecionar linha
ui-table-empty = Nada aqui.
ui-table-page = Página { $page } de { $pages } · { $rows ->
        [one] { NUM($rows) } linha
       *[other] { NUM($rows) } linhas
    }
ui-table-previous = Página anterior
ui-table-next = Próxima página
ui-transfer-done = Concluído.
ui-files-column-name = Nome
ui-files-column-size = Tamanho
ui-files-column-modified = Modificado
ui-files-column-mode = Modo
ui-files-path = Caminho
ui-files-downloadOne = Baixar { $name }
ui-files-empty = Esta pasta está vazia.
ui-chart-time = Hora
ui-errorDetails-title = Detalhes do erro
ui-errorDetails-code = Código
ui-errorDetails-command = Comando
ui-errorDetails-when = Quando
ui-errorDetails-app = App
ui-errorDetails-rawDetail = Detalhe bruto
ui-errorDetails-rawDetailNote = Como o sistema, o Engine ou a biblioteca o escreveu, por isso não é traduzido.
ui-errorDetails-openLogFolder = Abrir pasta de logs
ui-errorDetails-copyReport = Copiar relatório
ui-confirm-typeName = Digite { $name } para confirmar
ui-select-noMatch = Nada corresponde.
ui-password-show = Mostrar a senha
ui-password-hide = Ocultar a senha
ui-log-wrap = Quebrar linhas
ui-log-latest = Ir para o mais recente
ui-log-matchCase = Diferenciar maiúsculas
ui-log-regex = Expressão regular
ui-log-filter = Filtrar
ui-log-filterHint = Mostrar só as linhas correspondentes
ui-log-matches = { $current } de { $count }
ui-log-noMatches = Nenhum resultado
ui-log-invalidRegex = Expressão regular inválida
ui-log-skipped = { $count ->
        [one] — { NUM($count) } linha ignorada —
       *[other] — { NUM($count) } linhas ignoradas —
    }
