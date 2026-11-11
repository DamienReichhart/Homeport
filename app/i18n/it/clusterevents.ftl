### The `clusterevents` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/clusterevents.ts:
### change the text there, then run it again.

clusterevents-nav = Eventi
clusterevents-title = Eventi
clusterevents-summary = { $count ->
        [one] { NUM($count) } evento
       *[other] { NUM($count) } eventi
    }
clusterevents-search = Cerca per motivo, oggetto o messaggio
clusterevents-empty = Nessun evento. Kubernetes li conserva per circa un’ora.
clusterevents-filter = Tipo
clusterevents-all = Tutti
clusterevents-warnings = Avvisi
clusterevents-normal = Normali
clusterevents-column-type = Tipo
clusterevents-column-reason = Motivo
clusterevents-column-object = Oggetto
clusterevents-column-namespace = Namespace
clusterevents-column-message = Messaggio
clusterevents-column-count = Conteggio
clusterevents-column-lastSeen = Ultima occorrenza
