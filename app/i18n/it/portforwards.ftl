### The `portforwards` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/portforwards.ts:
### change the text there, then run it again.

portforwards-nav = Inoltri di porta
portforwards-title = Inoltri di porta
portforwards-summary = { $count ->
        [one] { NUM($count) } inoltro
       *[other] { NUM($count) } inoltri
    }
portforwards-empty-title = Nessun inoltro di porta
portforwards-empty-description = Inoltra una porta dalla pagina di un Pod o di un Service per raggiungerla da questo computer. Gli inoltri terminano quando chiudi Homeport.
portforwards-column-local = Indirizzo locale
portforwards-column-target = Inoltra a
portforwards-column-namespace = Namespace
portforwards-column-cluster = Cluster
portforwards-column-connections = Connessioni
portforwards-forward = Inoltra…
portforwards-forwardPort = Inoltra la porta { $port }
portforwards-open = Apri
portforwards-stop = Arresta
portforwards-stopFailed = Homeport non è riuscito ad arrestare l’inoltro
portforwards-started = Inoltro di { $address } a { $target }:{ $port }
portforwards-failed = Un inoltro di porta non è riuscito
portforwards-ended = Un inoltro di porta è terminato
portforwards-dialog-title = Inoltra { $target }:{ $port }
portforwards-dialog-description = Nel namespace { $namespace }. Raggiungilo alla porta locale finché l’inoltro è attivo.
portforwards-dialog-localPort = Porta locale
portforwards-dialog-anyPort = Qualsiasi porta libera
portforwards-dialog-loopback = Solo questo computer può connettersi: l’inoltro è in ascolto su 127.0.0.1 (e ::1).
portforwards-dialog-submit = Inoltra
