### The `workloads` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/workloads.ts:
### change the text there, then run it again.

workloads-nav = Workloads
workloads-title = Workloads
workloads-summary = { $count ->
        [one] { NUM($count) } Workload
       *[other] { NUM($count) } Workloads
    }
workloads-search = Workloads durchsuchen
workloads-empty = Hier ist nichts.
workloads-kind-deployment = Deployments
workloads-kind-statefulSet = StatefulSets
workloads-kind-daemonSet = DaemonSets
workloads-kind-job = Jobs
workloads-kind-cronJob = CronJobs
workloads-one-deployment = Deployment
workloads-one-statefulSet = StatefulSet
workloads-one-daemonSet = DaemonSet
workloads-one-job = Job
workloads-one-cronJob = CronJob
workloads-column-name = Name
workloads-column-namespace = Namespace
workloads-column-health = Zustand
workloads-column-ready = Bereit
workloads-column-completions = Abschlüsse
workloads-column-active = Aktiv
workloads-column-upToDate = Aktuell
workloads-column-schedule = Zeitplan
workloads-column-lastSchedule = Letzter Lauf
workloads-column-images = Images
workloads-column-age = Alter
workloads-tab-overview = Übersicht
workloads-tab-pods = Pods
workloads-tab-jobs = Jobs
workloads-tab-logs = Logs
workloads-tab-conditions = Bedingungen
workloads-tab-events = Events
workloads-tab-source = YAML
workloads-detail-namespace = Namespace
workloads-detail-ready = Bereit
workloads-detail-upToDate = Aktuell
workloads-detail-schedule = Zeitplan
workloads-detail-lastSchedule = Letzter Lauf
workloads-detail-strategy = Strategie
workloads-detail-selector = Wählt Pods aus nach
workloads-detail-owner = Gesteuert von
workloads-detail-age = Alter
workloads-detail-labels = Labels
workloads-detail-annotations = Annotationen
workloads-detail-containers = Container
workloads-detail-noJobs = Er hat keinen Job gestartet, der noch aufbewahrt wird.
workloads-container-name = Container
workloads-container-image = Image
workloads-container-ports = Ports
workloads-action-menu = Aktionen für { $name }
workloads-action-scale = Skalieren
workloads-action-restart = Neu starten
workloads-action-trigger = Jetzt ausführen
workloads-action-suspend = Aussetzen
workloads-action-resume = Fortsetzen
workloads-scale-title = { $name } skalieren
workloads-scale-description = Im Namespace { $namespace } auf { $cluster }. Derzeit { $current ->
        [one] läuft { NUM($current) } Pod
       *[other] laufen { NUM($current) } Pods
    }.
workloads-scale-replicas = Pods
workloads-scale-autoscaled = Der Autoscaler { $autoscaler } verwaltet seine Pods und setzt die Anzahl daher bei seiner nächsten Prüfung zurück.
workloads-scale-toZero = Mit 0 Pods bedient er nichts mehr, bis Sie ihn wieder hochskalieren.
workloads-scale-submit = Skalieren
workloads-scale-done = { $name } auf { $replicas ->
        [one] { NUM($replicas) } Pod
       *[other] { NUM($replicas) } Pods
    } skaliert
workloads-scale-failed = Homeport konnte { $name } nicht skalieren
workloads-restart-title = { $name } neu starten?
workloads-restart-description = Seine Pods im Namespace { $namespace } auf { $cluster } werden nacheinander ersetzt, wie seine Rollout-Strategie es vorgibt, wie kubectl rollout restart.
workloads-restart-confirm = Neu starten
workloads-restart-done = { $name } wird neu gestartet
workloads-restart-failed = Homeport konnte { $name } nicht neu starten
workloads-suspend-title = { $name } aussetzen?
workloads-suspend-description = Er startet im Namespace { $namespace } auf { $cluster } keine Jobs mehr, bis Sie ihn fortsetzen. Bereits laufende Jobs laufen weiter.
workloads-suspend-confirm = Aussetzen
workloads-suspend-done = { $name } ausgesetzt
workloads-suspend-failed = Homeport konnte { $name } nicht aussetzen
workloads-resume-done = { $name } fortgesetzt
workloads-resume-failed = Homeport konnte { $name } nicht fortsetzen
workloads-trigger-done = Job { $job } gestartet
workloads-trigger-failed = Homeport konnte keinen Job aus { $name } starten
