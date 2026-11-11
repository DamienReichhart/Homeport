### The `nodes` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/nodes.ts:
### change the text there, then run it again.

nodes-nav = Nodes
nodes-title = Nodes
nodes-summary = { $count ->
        [one] { NUM($count) } node
       *[other] { NUM($count) } nodes
    }
nodes-search = Search nodes
nodes-ready = Ready
nodes-notReady = Not ready
nodes-cordoned = scheduling disabled
nodes-column-name = Name
nodes-column-status = Status
nodes-column-roles = Roles
nodes-column-version = Version
nodes-column-ip = Internal IP
nodes-column-platform = Platform
nodes-column-cpu = CPUs
nodes-column-memory = Memory
nodes-column-age = Age
nodes-tab-overview = Overview
nodes-tab-pods = Pods
nodes-tab-conditions = Conditions
nodes-tab-events = Events
nodes-tab-source = YAML
nodes-detail-roles = Roles
nodes-detail-version = Kubelet
nodes-detail-os = OS image
nodes-detail-kernel = Kernel
nodes-detail-runtime = Container runtime
nodes-detail-addresses = Addresses
nodes-detail-taints = Taints
nodes-detail-noTaints = None
nodes-detail-age = Age
nodes-detail-labels = Labels
nodes-detail-resources = Resources
nodes-resource-name = Resource
nodes-resource-capacity = Capacity
nodes-resource-allocatable = Allocatable
nodes-column-cpuUsed = CPU used
nodes-column-memoryUsed = Memory used
nodes-tab-metrics = Metrics
