### The `podmanpods` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/podmanpods.ts:
### change the text there, then run it again.

podmanpods-nav = Pods
podmanpods-title = Pods
podmanpods-summary = { $count ->
        [one] { NUM($count) } pod
       *[other] { NUM($count) } pods
    }
podmanpods-search = Search pods
podmanpods-empty = No pods
podmanpods-emptyDescription = A pod groups containers that share a network and start and stop together. Create one, then run containers in it.
podmanpods-new = New pod
podmanpods-column-name = Name
podmanpods-column-status = Status
podmanpods-column-containers = Containers
podmanpods-column-networks = Networks
podmanpods-column-created = Created
podmanpods-running = { $running } of { $count } running
podmanpods-status-running = Running
podmanpods-status-degraded = Partly running
podmanpods-status-paused = Paused
podmanpods-status-created = Created
podmanpods-status-stopped = Stopped
podmanpods-status-failed = Failed
podmanpods-action-start = Start
podmanpods-action-stop = Stop
podmanpods-action-restart = Restart
podmanpods-action-pause = Pause
podmanpods-action-unpause = Resume
podmanpods-actionOne-start = Start { $name }
podmanpods-actionOne-stop = Stop { $name }
podmanpods-actionOne-restart = Restart { $name }
podmanpods-actionOne-pause = Pause { $name }
podmanpods-actionOne-unpause = Resume { $name }
podmanpods-removeOne = Remove { $name }
podmanpods-done-start = { $name } started
podmanpods-done-stop = { $name } stopped
podmanpods-done-restart = { $name } restarted
podmanpods-done-pause = { $name } paused
podmanpods-done-unpause = { $name } resumed
podmanpods-removed = { $name } removed, { $count ->
        [0] with no containers
        [one] with { NUM($count) } container
       *[other] with { NUM($count) } containers
    }
podmanpods-created = { $name } created
podmanpods-remove-title = Remove { $name }?
podmanpods-remove-stopped = { $count ->
        [0] The pod and its infra container are removed.
        [one] The pod, its infra container and its { NUM($count) } container are removed.
       *[other] The pod, its infra container and its { NUM($count) } containers are removed.
    } Their volumes are kept.
podmanpods-remove-running = { $count ->
        [0] The pod is running: it is stopped, then removed with its infra container.
        [one] The pod is running: its { NUM($count) } container is stopped, then removed with the pod and its infra container.
       *[other] The pod is running: its { NUM($count) } containers are stopped, then removed with the pod and its infra container.
    } Their volumes are kept.
podmanpods-remove-stopAndRemove = Stop and remove
podmanpods-detail-containers = { $count ->
        [0] Containers
        [one] { NUM($count) } container
       *[other] { NUM($count) } containers
    }
podmanpods-detail-noContainers = Nothing runs in this pod yet besides its infra container.
podmanpods-detail-infra = Infra
podmanpods-detail-settings = Settings
podmanpods-detail-ports = Published ports
podmanpods-detail-noPorts = None
podmanpods-detail-shared = Shared namespaces
podmanpods-detail-networks = Networks
podmanpods-detail-hostname = Hostname
podmanpods-detail-created = Created
podmanpods-detail-labels = Labels
podmanpods-detail-command = Created with
podmanpods-detail-runContainer = Run container
podmanpods-detail-runContainerIn = Run a container in { $name }
podmanpods-newDialog-title = New pod
podmanpods-newDialog-description = Its containers share one network, so ports are published on the pod, for all of them.
podmanpods-newDialog-name = Name
podmanpods-newDialog-portsHint = Leave the host port empty to let Podman pick a free one. Containers run in the pod can’t publish ports of their own.
podmanpods-newDialog-labels = Labels
podmanpods-newDialog-create = Create pod
podmanpods-form-nameInvalid = Start with a letter or digit, then use letters, digits, _, . or -.
podmanpods-form-portInvalid = Ports are numbers from 1 to 65535.
podmanpods-form-labelInvalid = Write one key=value label per line.
