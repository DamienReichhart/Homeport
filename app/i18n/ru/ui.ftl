### The `ui` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/ui.ts:
### change the text there, then run it again.

ui-copy = Копировать
ui-copied = Скопировано
ui-search = Поиск
ui-search-clear = Очистить поиск
ui-shortcuts-focusSearch = Искать на этой странице
ui-shortcuts-thisPage = Эта страница
ui-table-selectAll = Выбрать все
ui-table-selectRow = Выбрать строку
ui-table-empty = Здесь ничего нет.
ui-table-page = Страница { $page } из { $pages } · { $rows ->
        [one] { NUM($rows) } строка
        [few] { NUM($rows) } строки
        [many] { NUM($rows) } строк
       *[other] { NUM($rows) } строки
    }
ui-table-previous = Предыдущая страница
ui-table-next = Следующая страница
ui-transfer-done = Готово.
ui-files-column-name = Имя
ui-files-column-size = Размер
ui-files-column-modified = Изменён
ui-files-column-mode = Права
ui-files-path = Путь
ui-files-downloadOne = Скачать { $name }
ui-files-empty = Эта папка пуста.
ui-files-upload = Загрузить
ui-json-key = Ключ
ui-json-value = Значение
ui-json-copyPath = Копировать путь
ui-json-copyValue = Копировать значение
ui-chart-time = Время
ui-chart-hide = Скрыть { $name }
ui-chart-show = Показать { $name }
ui-errorDetails-title = Подробности ошибки
ui-errorDetails-code = Код
ui-errorDetails-command = Команда
ui-errorDetails-when = Когда
ui-errorDetails-app = Приложение
ui-errorDetails-rawDetail = Исходный текст
ui-errorDetails-rawDetailNote = В том виде, в каком его выдали система, движок или библиотека, поэтому он не переведён.
ui-errorDetails-openLogFolder = Открыть папку журналов
ui-errorDetails-copyReport = Скопировать отчёт
ui-confirm-typeName = Введите { $name } для подтверждения
ui-select-noMatch = Ничего не найдено.
ui-password-show = Показать пароль
ui-password-hide = Скрыть пароль
ui-log-wrap = Переносить строки
ui-log-latest = К последним строкам
ui-log-matchCase = Учитывать регистр
ui-log-regex = Регулярное выражение
ui-log-filter = Фильтр
ui-log-filterHint = Показывать только подходящие строки
ui-log-matches = { $current } из { $count }
ui-log-noMatches = Нет совпадений
ui-log-invalidRegex = Недопустимое регулярное выражение
ui-log-skipped = { $count ->
        [one] — пропущена { NUM($count) } строка —
        [few] — пропущено { NUM($count) } строки —
        [many] — пропущено { NUM($count) } строк —
       *[other] — пропущено { NUM($count) } строки —
    }
ui-log-searchFailed = Поиск остановился, не прочитав все строки
