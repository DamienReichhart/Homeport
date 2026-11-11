### The `clusterevents` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/clusterevents.ts:
### change the text there, then run it again.

clusterevents-nav = Events
clusterevents-title = Events
clusterevents-summary = { $count ->
        [one] { NUM($count) } event
       *[other] { NUM($count) } events
    }
clusterevents-search = Search by reason, object or message
clusterevents-empty = No events. Kubernetes keeps them for about an hour.
clusterevents-filter = Type
clusterevents-all = All
clusterevents-warnings = Warnings
clusterevents-normal = Normal
clusterevents-column-type = Type
clusterevents-column-reason = Reason
clusterevents-column-object = Object
clusterevents-column-namespace = Namespace
clusterevents-column-message = Message
clusterevents-column-count = Count
clusterevents-column-lastSeen = Last seen
