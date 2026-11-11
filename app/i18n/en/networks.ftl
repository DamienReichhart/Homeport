### The `networks` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/networks.ts:
### change the text there, then run it again.

networks-nav = Networks
networks-title = Networks
networks-summary = { $count ->
        [one] { NUM($count) } network
       *[other] { NUM($count) } networks
    }
networks-cleanUp = Clean up
networks-create = Create network
networks-search = Search networks
networks-empty = No networks
networks-column-name = Name
networks-column-stack = Stack
networks-column-driver = Driver
networks-column-subnet = Subnet
networks-column-containers = Containers
networks-builtIn = built-in
networks-internal = internal
networks-inspectOne = Inspect { $name }
networks-removeOne = Remove { $name }
networks-remove-title = Remove network { $name }?
networks-remove-attached = It still has { $count ->
        [one] { NUM($count) } attached container
       *[other] { NUM($count) } attached containers
    }; disconnect them first.
networks-remove-unused = No container uses it. Containers that reference it by name will fail to start.
networks-prune-title = Remove unused networks?
networks-prune-description = Every custom network no container is attached to is removed.
networks-connected = Connected
networks-disconnected = Disconnected
networks-removed = Removed { $name }
networks-pruned = Removed { $count ->
        [one] { NUM($count) } network
       *[other] { NUM($count) } networks
    }
networks-createDialog-title = Create a network
networks-createDialog-name = Name
networks-createDialog-driver = Driver
networks-createDialog-subnet = Subnet
networks-createDialog-gateway = Gateway
networks-createDialog-automatic = automatic
networks-createDialog-internal = Internal — no access to outside networks
networks-createDialog-attachable = Attachable
networks-dialog-containers = Containers
networks-dialog-raw = Raw JSON
networks-dialog-noContainer = No container is attached.
networks-dialog-disconnect = Disconnect
networks-dialog-containerToConnect = Container to connect
networks-dialog-connectPlaceholder = Connect a container…
networks-dialog-connect = Connect
