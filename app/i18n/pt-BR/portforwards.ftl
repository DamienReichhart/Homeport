### The `portforwards` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/portforwards.ts:
### change the text there, then run it again.

portforwards-nav = Encaminhamentos de porta
portforwards-title = Encaminhamentos de porta
portforwards-summary = { $count ->
        [one] { NUM($count) } encaminhamento
       *[other] { NUM($count) } encaminhamentos
    }
portforwards-empty-title = Nenhum encaminhamento de porta
portforwards-empty-description = Encaminhe uma porta na página de um Pod ou de um Service para acessá-lo a partir deste computador. Os encaminhamentos terminam quando o Homeport é encerrado.
portforwards-column-local = Endereço local
portforwards-column-target = Encaminha para
portforwards-column-namespace = Namespace
portforwards-column-cluster = Cluster
portforwards-column-connections = Conexões
portforwards-forward = Encaminhar…
portforwards-forwardPort = Encaminhar porta { $port }
portforwards-open = Abrir
portforwards-stop = Parar
portforwards-stopFailed = O Homeport não conseguiu parar o encaminhamento
portforwards-started = Encaminhando { $address } para { $target }:{ $port }
portforwards-failed = Um encaminhamento de porta falhou
portforwards-ended = Um encaminhamento de porta terminou
portforwards-dialog-title = Encaminhar { $target }:{ $port }
portforwards-dialog-description = No namespace { $namespace }. Acesse-o pela porta local enquanto o encaminhamento estiver ativo.
portforwards-dialog-localPort = Porta local
portforwards-dialog-anyPort = Qualquer porta livre
portforwards-dialog-loopback = Somente este computador pode se conectar: o encaminhamento escuta em 127.0.0.1 (e ::1).
portforwards-dialog-submit = Encaminhar
