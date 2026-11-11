### The `overview` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/overview.ts:
### change the text there, then run it again.

overview-nav = Panoramica
overview-title = Panoramica
overview-addHost = Aggiungi host
overview-addCluster = Aggiungi cluster
overview-cluster-nodes = { $ready } di { $total } Node pronti
overview-cluster-nodesNotAllowed = Node: non consentito
overview-cluster-pods = { $running } di { $total } Pod in esecuzione
overview-cluster-podsNotAllowed = Pod: non consentito
overview-cluster-notAllowed = Non consentito.
overview-cluster-versionTitle = Cluster
overview-cluster-version = Versione
overview-cluster-platform = Piattaforma
overview-cluster-server = Server API
overview-nomad-region = Regione
overview-nomad-address = Indirizzo
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = Tutti i namespace
overview-cluster-nodesTitle = Node
overview-cluster-nodesReady = pronti
overview-cluster-allNodes = Tutti i Node
overview-cluster-podsTitle = Pod
overview-cluster-running = { $count } in esecuzione
overview-cluster-pending = { $count } in attesa
overview-cluster-crashing = { $count } in errore
overview-cluster-done = { $count } completati o altro
overview-cluster-allPods = Tutti i Pod
overview-cluster-unreadyTitle = Carichi di lavoro non pronti
overview-cluster-allReady = Ogni Deployment, StatefulSet e DaemonSet ha i suoi Pod pronti.
overview-cluster-warningsTitle = Avvisi recenti
overview-cluster-noWarnings = Nessun evento di avviso.
overview-cluster-allEvents = Tutti gli eventi
overview-host-running = <strong>{ $running }</strong> / { $total } in esecuzione
overview-host-images = { $count ->
        [one] { NUM($count) } immagine
       *[other] { NUM($count) } immagini
    }
overview-host-containerdContainers = { $count ->
        [one] { NUM($count) } container
       *[other] { NUM($count) } container
    } in { $namespaces ->
        [one] { NUM($namespaces) } namespace
       *[other] { NUM($namespaces) } namespace
    }
overview-host-incusInstances = { $count ->
        [one] { NUM($count) } istanza
       *[other] { NUM($count) } istanze
    } in { $projects ->
        [one] { NUM($projects) } progetto
       *[other] { NUM($projects) } progetti
    }
overview-incus-title = Incus
overview-incus-version = Versione
overview-incus-os = Sistema
overview-incus-arch = Architettura
overview-incus-projects = Progetti
overview-incus-shown = Mostrato
overview-incus-holds = { $instances ->
        [one] { NUM($instances) } istanza
       *[other] { NUM($instances) } istanze
    }, { $images ->
        [one] { NUM($images) } immagine
       *[other] { NUM($images) } immagini
    }
overview-incus-instances = Istanze
overview-incus-images = Immagini
overview-incus-storage = Archiviazione
overview-containerd-title = containerd
overview-containerd-version = Versione
overview-containerd-revision = Revisione
overview-containerd-platform = Piattaforma
overview-containerd-namespaces = Namespace
overview-containerd-shown = Mostrato
overview-containerd-holds = { $containers ->
        [one] { NUM($containers) } container
       *[other] { NUM($containers) } container
    }, { $images ->
        [one] { NUM($images) } immagine
       *[other] { NUM($images) } immagini
    }
overview-containerd-containers = Container
overview-containerd-images = Immagini
overview-containerd-snapshots = Snapshot
overview-host-cpu = { $percent } di { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPU
    }
overview-host-cpuTitle = Uso della CPU dei container, 100% per CPU
overview-host-memoryTitle = Uso della memoria dei container
overview-engine-title = Engine
overview-engine-name = Nome
overview-engine-version = Versione
overview-engine-os = Sistema operativo
overview-engine-kernel = Kernel
overview-engine-architecture = Architettura
overview-engine-resources = Risorse
overview-engine-resourcesValue = { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPU
    } · { $memory }
overview-engine-storageDriver = Driver di archiviazione
overview-engine-rootDir = Directory radice
overview-disk-title = Uso del disco
overview-disk-cleanUp = Ripulisci…
overview-disk-column-type = Tipo
overview-disk-column-total = Totale
overview-disk-column-active = In uso
overview-disk-column-size = Dimensione
overview-disk-column-reclaimable = Recuperabile
overview-disk-images = Immagini
overview-disk-containers = Container
overview-disk-volumes = Volumi
overview-disk-buildCache = Cache di build
overview-disk-partial = Questo Docker Engine è precedente alla versione 29 e fornisce meno informazioni: le dimensioni delle immagini sono stimate, e quelle dei volumi e della cache di build non sono disponibili.
overview-events-title = Eventi recenti
overview-events-all = Tutti gli eventi
overview-events-empty = Non è successo nulla da quando Homeport si è connesso.
overview-prune-title = Ripulisci
overview-prune-description = Scegli cosa rimuovere da questo host. L’operazione non può essere annullata.
overview-prune-confirm = Ripulisci
overview-prune-confirmWithVolumes = Elimina, compresi i dati dei volumi
overview-prune-containers = Container arrestati
overview-prune-containers-detail = Rimuove ogni container non in esecuzione.
overview-prune-images = Immagini orfane
overview-prune-images-detail = Layer senza tag lasciati da build e download.
overview-prune-allImages = Tutte le immagini inutilizzate
overview-prune-allImages-detail = Rimuove anche le immagini con tag non usate da alcun container.
overview-prune-networks = Reti inutilizzate
overview-prune-networks-detail = Reti personalizzate a cui non è collegato alcun container.
overview-prune-buildCache = Cache di build
overview-prune-buildCache-detail = Layer di build in cache.
overview-prune-volumes = Volumi inutilizzati
overview-prune-volumes-detail = Elimina i dati di ogni volume non usato da alcun container, compresi i volumi con nome.
overview-pruned = Rimossi { $containers ->
        [one] { NUM($containers) } container
       *[other] { NUM($containers) } container
    }, { $images ->
        [one] { NUM($images) } immagine
       *[other] { NUM($images) } immagini
    }, { $networks ->
        [one] { NUM($networks) } rete
       *[other] { NUM($networks) } reti
    } e { $volumes ->
        [one] { NUM($volumes) } volume
       *[other] { NUM($volumes) } volumi
    } — { $space } recuperati.
overview-palette-prune = Ripulisci l’host selezionato
overview-host-onBoard = I suoi container nella bacheca
