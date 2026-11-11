### The `portforwards` namespace, in Русский (ru).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ru/portforwards.ts:
### change the text there, then run it again.

portforwards-nav = Пробросы портов
portforwards-title = Пробросы портов
portforwards-summary = { $count ->
        [one] { NUM($count) } проброс
        [few] { NUM($count) } проброса
        [many] { NUM($count) } пробросов
       *[other] { NUM($count) } проброса
    }
portforwards-empty-title = Нет пробросов портов
portforwards-empty-description = Пробросьте порт со страницы Pod или Service, чтобы обращаться к нему с этого компьютера. Пробросы завершаются, когда Homeport закрывается.
portforwards-column-local = Локальный адрес
portforwards-column-target = Куда
portforwards-column-namespace = Пространство имён
portforwards-column-cluster = Кластер
portforwards-column-connections = Соединения
portforwards-forward = Пробросить…
portforwards-forwardPort = Пробросить порт { $port }
portforwards-open = Открыть
portforwards-stop = Остановить
portforwards-stopFailed = Homeport не удалось остановить проброс
portforwards-started = { $address } проброшен на { $target }:{ $port }
portforwards-failed = Проброс порта завершился ошибкой
portforwards-ended = Проброс порта завершён
portforwards-dialog-title = Пробросить { $target }:{ $port }
portforwards-dialog-description = В пространстве имён { $namespace }. Обращайтесь к нему через локальный порт, пока работает проброс.
portforwards-dialog-localPort = Локальный порт
portforwards-dialog-anyPort = Любой свободный порт
portforwards-dialog-loopback = Подключиться может только этот компьютер: проброс слушает 127.0.0.1 (и ::1).
portforwards-dialog-submit = Пробросить
