### The `overview` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/overview.ts:
### change the text there, then run it again.

overview-nav = Übersicht
overview-title = Übersicht
overview-addHost = Host hinzufügen
overview-addCluster = Cluster hinzufügen
overview-cluster-nodes = { $ready } von { $total } Nodes bereit
overview-cluster-nodesNotAllowed = Nodes: nicht erlaubt
overview-cluster-pods = { $running } von { $total } Pods laufen
overview-cluster-podsNotAllowed = Pods: nicht erlaubt
overview-cluster-notAllowed = Nicht erlaubt.
overview-cluster-versionTitle = Cluster
overview-cluster-version = Version
overview-cluster-platform = Plattform
overview-cluster-server = API-Server
overview-nomad-region = Region
overview-nomad-address = Adresse
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = Alle Namespaces
overview-cluster-nodesTitle = Nodes
overview-cluster-nodesReady = bereit
overview-cluster-allNodes = Alle Nodes
overview-cluster-podsTitle = Pods
overview-cluster-running = { $count } laufen
overview-cluster-pending = { $count } ausstehend
overview-cluster-crashing = { $count } fehlerhaft
overview-cluster-done = { $count } fertig oder sonstige
overview-cluster-allPods = Alle Pods
overview-cluster-unreadyTitle = Nicht bereite Workloads
overview-cluster-allReady = Jedes Deployment, jedes StatefulSet und jedes DaemonSet hat seine Pods bereit.
overview-cluster-warningsTitle = Aktuelle Warnungen
overview-cluster-noWarnings = Keine Warn-Events.
overview-cluster-allEvents = Alle Events
overview-host-running = <strong>{ $running }</strong> / { $total } laufen
overview-host-images = { $count ->
        [one] { NUM($count) } Image
       *[other] { NUM($count) } Images
    }
overview-host-containerdContainers = { $count ->
        [one] { NUM($count) } Container
       *[other] { NUM($count) } Container
    } in { $namespaces ->
        [one] { NUM($namespaces) } Namespace
       *[other] { NUM($namespaces) } Namespaces
    }
overview-host-incusInstances = { $count ->
        [one] { NUM($count) } Instanz
       *[other] { NUM($count) } Instanzen
    } in { $projects ->
        [one] { NUM($projects) } Projekt
       *[other] { NUM($projects) } Projekten
    }
overview-incus-title = Incus
overview-incus-version = Version
overview-incus-os = System
overview-incus-arch = Architektur
overview-incus-projects = Projekte
overview-incus-shown = Angezeigt
overview-incus-holds = { $instances ->
        [one] { NUM($instances) } Instanz
       *[other] { NUM($instances) } Instanzen
    }, { $images ->
        [one] { NUM($images) } Image
       *[other] { NUM($images) } Images
    }
overview-incus-instances = Instanzen
overview-incus-images = Images
overview-incus-storage = Speicher
overview-containerd-title = containerd
overview-containerd-version = Version
overview-containerd-revision = Revision
overview-containerd-platform = Plattform
overview-containerd-namespaces = Namespaces
overview-containerd-shown = Angezeigt
overview-containerd-holds = { $containers ->
        [one] { NUM($containers) } Container
       *[other] { NUM($containers) } Container
    }, { $images ->
        [one] { NUM($images) } Image
       *[other] { NUM($images) } Images
    }
overview-containerd-containers = Container
overview-containerd-images = Images
overview-containerd-snapshots = Snapshots
overview-host-cpu = { $percent } von { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPUs
    }
overview-host-cpuTitle = CPU-Nutzung der Container, 100 % pro CPU
overview-host-memoryTitle = Speichernutzung der Container
overview-engine-title = Engine
overview-engine-name = Name
overview-engine-version = Version
overview-engine-os = Betriebssystem
overview-engine-kernel = Kernel
overview-engine-architecture = Architektur
overview-engine-resources = Ressourcen
overview-engine-resourcesValue = { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPUs
    } · { $memory }
overview-engine-storageDriver = Speichertreiber
overview-engine-rootDir = Stammverzeichnis
overview-disk-title = Speicherplatz
overview-disk-cleanUp = Aufräumen…
overview-disk-column-type = Typ
overview-disk-column-total = Gesamt
overview-disk-column-active = In Verwendung
overview-disk-column-size = Größe
overview-disk-column-reclaimable = Freigebbar
overview-disk-images = Images
overview-disk-containers = Container
overview-disk-volumes = Volumes
overview-disk-buildCache = Build-Cache
overview-disk-partial = Diese Docker Engine ist älter als Version 29 und meldet weniger: Image-Größen werden geschätzt, und Größen von Volumes und Build-Cache sind nicht verfügbar.
overview-events-title = Aktuelle Ereignisse
overview-events-all = Alle Ereignisse
overview-events-empty = Seit Homeport verbunden ist, ist nichts passiert.
overview-prune-title = Aufräumen
overview-prune-description = Wählen Sie, was von diesem Host entfernt werden soll. Dies lässt sich nicht rückgängig machen.
overview-prune-confirm = Aufräumen
overview-prune-confirmWithVolumes = Löschen, einschließlich Volume-Daten
overview-prune-containers = Gestoppte Container
overview-prune-containers-detail = Entfernt alle Container, die nicht laufen.
overview-prune-images = Verwaiste Images
overview-prune-images-detail = Ungetaggte Layer, die Builds und Pulls hinterlassen haben.
overview-prune-allImages = Alle unbenutzten Images
overview-prune-allImages-detail = Entfernt auch getaggte Images, die kein Container verwendet.
overview-prune-networks = Unbenutzte Netzwerke
overview-prune-networks-detail = Benutzerdefinierte Netzwerke, mit denen kein Container verbunden ist.
overview-prune-buildCache = Build-Cache
overview-prune-buildCache-detail = Zwischengespeicherte Build-Layer.
overview-prune-volumes = Unbenutzte Volumes
overview-prune-volumes-detail = Löscht die Daten in allen Volumes, die kein Container verwendet, benannte Volumes eingeschlossen.
overview-pruned = { $containers ->
        [one] { NUM($containers) } Container
       *[other] { NUM($containers) } Container
    }, { $images ->
        [one] { NUM($images) } Image
       *[other] { NUM($images) } Images
    }, { $networks ->
        [one] { NUM($networks) } Netzwerk
       *[other] { NUM($networks) } Netzwerke
    } und { $volumes ->
        [one] { NUM($volumes) } Volume
       *[other] { NUM($volumes) } Volumes
    } entfernt — { $space } freigegeben.
overview-palette-prune = Ausgewählten Host aufräumen
overview-host-onBoard = Seine Container auf dem Board
