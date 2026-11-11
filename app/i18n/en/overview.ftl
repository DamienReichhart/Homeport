### The `overview` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/overview.ts:
### change the text there, then run it again.

overview-nav = Overview
overview-title = Overview
overview-addHost = Add host
overview-addCluster = Add cluster
overview-cluster-nodes = { $ready } of { $total } nodes ready
overview-cluster-nodesNotAllowed = Nodes: not allowed
overview-cluster-pods = { $running } of { $total } pods running
overview-cluster-podsNotAllowed = Pods: not allowed
overview-cluster-notAllowed = Not allowed.
overview-cluster-versionTitle = Cluster
overview-cluster-version = Version
overview-cluster-platform = Platform
overview-cluster-server = API server
overview-nomad-region = Region
overview-nomad-address = Address
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = All namespaces
overview-cluster-nodesTitle = Nodes
overview-cluster-nodesReady = ready
overview-cluster-allNodes = All nodes
overview-cluster-podsTitle = Pods
overview-cluster-running = { $count } running
overview-cluster-pending = { $count } pending
overview-cluster-crashing = { $count } failing
overview-cluster-done = { $count } done or other
overview-cluster-allPods = All pods
overview-cluster-unreadyTitle = Workloads not ready
overview-cluster-allReady = Every Deployment, StatefulSet and DaemonSet has its pods ready.
overview-cluster-warningsTitle = Recent warnings
overview-cluster-noWarnings = No warning events.
overview-cluster-allEvents = All events
overview-host-running = <strong>{ $running }</strong> / { $total } running
overview-host-images = { $count ->
        [one] { NUM($count) } image
       *[other] { NUM($count) } images
    }
overview-host-containerdContainers = { $count ->
        [one] { NUM($count) } container
       *[other] { NUM($count) } containers
    } in { $namespaces ->
        [one] { NUM($namespaces) } namespace
       *[other] { NUM($namespaces) } namespaces
    }
overview-host-incusInstances = { $count ->
        [one] { NUM($count) } instance
       *[other] { NUM($count) } instances
    } in { $projects ->
        [one] { NUM($projects) } project
       *[other] { NUM($projects) } projects
    }
overview-incus-title = Incus
overview-incus-version = Version
overview-incus-os = System
overview-incus-arch = Architecture
overview-incus-projects = Projects
overview-incus-shown = Shown
overview-incus-holds = { $instances ->
        [one] { NUM($instances) } instance
       *[other] { NUM($instances) } instances
    }, { $images ->
        [one] { NUM($images) } image
       *[other] { NUM($images) } images
    }
overview-incus-instances = Instances
overview-incus-images = Images
overview-incus-storage = Storage
overview-containerd-title = containerd
overview-containerd-version = Version
overview-containerd-revision = Revision
overview-containerd-platform = Platform
overview-containerd-namespaces = Namespaces
overview-containerd-shown = Shown
overview-containerd-holds = { $containers ->
        [one] { NUM($containers) } container
       *[other] { NUM($containers) } containers
    }, { $images ->
        [one] { NUM($images) } image
       *[other] { NUM($images) } images
    }
overview-containerd-containers = Containers
overview-containerd-images = Images
overview-containerd-snapshots = Snapshots
overview-host-cpu = { $percent } of { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPUs
    }
overview-host-cpuTitle = Container CPU use, 100% per CPU
overview-host-memoryTitle = Container memory use
overview-engine-title = Engine
overview-engine-name = Name
overview-engine-version = Version
overview-engine-os = OS
overview-engine-kernel = Kernel
overview-engine-architecture = Architecture
overview-engine-resources = Resources
overview-engine-resourcesValue = { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPUs
    } · { $memory }
overview-engine-storageDriver = Storage driver
overview-engine-rootDir = Root directory
overview-disk-title = Disk usage
overview-disk-cleanUp = Clean up…
overview-disk-column-type = Type
overview-disk-column-total = Total
overview-disk-column-active = In use
overview-disk-column-size = Size
overview-disk-column-reclaimable = Reclaimable
overview-disk-images = Images
overview-disk-containers = Containers
overview-disk-volumes = Volumes
overview-disk-buildCache = Build cache
overview-disk-partial = This Docker Engine is older than version 29 and reports less: image sizes are estimated, and volume and build cache sizes aren’t available.
overview-events-title = Recent events
overview-events-all = All events
overview-events-empty = Nothing has happened since Homeport connected.
overview-prune-title = Clean up
overview-prune-description = Choose what to remove from this host. This can't be undone.
overview-prune-confirm = Clean up
overview-prune-confirmWithVolumes = Delete, including volume data
overview-prune-containers = Stopped containers
overview-prune-containers-detail = Removes every container that is not running.
overview-prune-images = Dangling images
overview-prune-images-detail = Untagged layers left behind by builds and pulls.
overview-prune-allImages = All unused images
overview-prune-allImages-detail = Also removes tagged images no container uses.
overview-prune-networks = Unused networks
overview-prune-networks-detail = Custom networks no container is attached to.
overview-prune-buildCache = Build cache
overview-prune-buildCache-detail = Cached build layers.
overview-prune-volumes = Unused volumes
overview-prune-volumes-detail = Deletes the data in every volume no container uses, named volumes included.
overview-pruned = Removed { $containers ->
        [one] { NUM($containers) } container
       *[other] { NUM($containers) } containers
    }, { $images ->
        [one] { NUM($images) } image
       *[other] { NUM($images) } images
    }, { $networks ->
        [one] { NUM($networks) } network
       *[other] { NUM($networks) } networks
    } and { $volumes ->
        [one] { NUM($volumes) } volume
       *[other] { NUM($volumes) } volumes
    } — { $space } reclaimed.
overview-palette-prune = Clean up the selected host
overview-host-onBoard = Its containers on the board
