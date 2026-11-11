### The `containerd` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/containerd.ts:
### change the text there, then run it again.

containerd-state-none = No task
containerd-state-created = Created
containerd-state-running = Running
containerd-state-paused = Paused
containerd-state-pausing = Pausing
containerd-state-stopped = Stopped
containerd-state-unknown = Unknown
containerd-state-exited = Exited ({ $code })
containerd-createdBy-kubernetes = Kubernetes
containerd-createdBy-nerdctl = nerdctl
containerd-createdBy-docker = Docker
containerd-createdBy-other = ctr or other
containerd-containers-nav = Containers
containerd-containers-title = Containers
containerd-containers-summary = { $count ->
        [one] { NUM($count) } container
       *[other] { NUM($count) } containers
    }, { $running } running, in the namespace { $namespace }.
containerd-containers-search = Filter by name, id or image
containerd-containers-showSandboxes = Show Pods’ pause containers
containerd-containers-sandbox = pause
containerd-containers-empty = No containers in { $namespace }
containerd-containers-emptyDescription = Containers made with nerdctl, ctr or Kubernetes show here. Pick another namespace under the host switcher to see others.
containerd-containers-column-name = Name
containerd-containers-column-image = Image
containerd-containers-column-state = Task
containerd-containers-column-createdBy = Made by
containerd-containers-column-created = Created
containerd-containers-action-start = Start
containerd-containers-action-stop = Stop
containerd-containers-action-pause = Pause
containerd-containers-action-resume = Resume
containerd-containers-action-kill = Send signal
containerd-containers-actionOne-start = Start { $name }
containerd-containers-actionOne-stop = Stop { $name }
containerd-containers-actionOne-pause = Pause { $name }
containerd-containers-actionOne-resume = Resume { $name }
containerd-containers-removeOne = Remove { $name }
containerd-containers-done-start = { $name } started
containerd-containers-done-stop = { $name } stopped
containerd-containers-done-pause = { $name } paused
containerd-containers-done-resume = { $name } resumed
containerd-containers-done-kill = { $signal } sent to { $name }
containerd-containers-removed = { $name } removed
containerd-containers-kubelet = Kubernetes manages this container: the kubelet starts it again after a stop, and makes it again after a removal. Change its Pod on the Cluster instead.
containerd-containers-remove-title = Remove { $name }?
containerd-containers-remove-running = Its task is running: Homeport kills it, then removes the task and the container.
containerd-containers-remove-stopped = Homeport removes its task and the container.
containerd-containers-remove-snapshot = Also remove its snapshot ({ $key }), with the files it changed
containerd-containers-remove-killAndRemove = Kill and remove
containerd-containers-kill-title = Send a signal to { $name }
containerd-containers-kill-description = To its task’s main process, as ctr task kill -s does. A name like SIGHUP, or a number.
containerd-containers-kill-signal = Signal
containerd-containers-kill-send = Send
containerd-containers-backToList = Back to containers
containerd-containers-tab-overview = Overview
containerd-containers-tab-logs = Logs
containerd-containers-tab-exec = Exec
containerd-containers-tab-spec = Spec
containerd-logs-from = From { $path }
containerd-logs-none-title = This container’s output isn’t recorded
containerd-logs-none-description = containerd keeps no logs: the tool that starts a task decides where its output goes. Containers made with ctr and most other clients send it nowhere; nerdctl and Kubernetes write it to a file, which Homeport reads.
containerd-logs-driver-title = nerdctl sends this container’s output to { $driver }
containerd-logs-driver-description = Homeport reads nerdctl’s default json-file logs only. Read these where { $driver } keeps them, or run the container again with the json-file driver.
containerd-exec-startFirst = Start the container to open a terminal in it.
containerd-exec-command = Command
containerd-exec-commandPlaceholder = A shell, unless you give a command
containerd-exec-user = User ID
containerd-exec-workingDir = Folder
containerd-exec-open = Open again
containerd-exec-exited = The process exited.
containerd-exec-exitedWithCode = The process exited with code { $code }.
containerd-containers-field-id = ID
containerd-containers-field-namespace = Namespace
containerd-containers-field-image = Image
containerd-containers-field-runtime = Runtime
containerd-containers-field-snapshot = Snapshot
containerd-containers-field-pid = PID
containerd-containers-field-exit = Last exit
containerd-containers-field-pod = Kubernetes Pod
containerd-containers-field-created = Created
containerd-containers-field-updated = Updated
containerd-containers-field-extensions = Extensions
containerd-containers-labels = Labels
containerd-containers-noSpec = containerd holds no runtime spec for this container.
containerd-images-nav = Images
containerd-images-title = Images
containerd-images-summary = { $count ->
        [one] { NUM($count) } image
       *[other] { NUM($count) } images
    } in the namespace { $namespace }.
containerd-images-pull = Pull image
containerd-images-pulled = { $reference } pulled into { $namespace }
containerd-images-removed = { $name } removed
containerd-images-search = Filter by name
containerd-images-empty = No images in { $namespace }
containerd-images-emptyDescription = Pull one: containerd fetches it, then unpacks it for this host’s platform so its containers can run.
containerd-images-column-name = Name
containerd-images-column-platforms = Platforms
containerd-images-column-size = Size
containerd-images-column-containers = Containers
containerd-images-column-created = Created
containerd-images-sizeUnknown = Its content for this host’s platform isn’t all here.
containerd-images-removeOne = Remove { $name }
containerd-images-inUse = Containers of this namespace were made from it. Remove them first.
containerd-images-remove-title = Remove { $name }?
containerd-images-remove-description = containerd removes the name, and frees its content and snapshots once nothing else holds them.
containerd-snapshots-nav = Snapshots
containerd-snapshots-title = Snapshots
containerd-snapshots-summary = { $count ->
        [one] { NUM($count) } snapshot
       *[other] { NUM($count) } snapshots
    } in { $namespace } use { $total }; { $unused ->
        [0] none is unused
        [one] { NUM($unused) } is unused
       *[other] { NUM($unused) } are unused
    } ({ $unusedSize }).
containerd-snapshots-snapshotter = Snapshotter
containerd-snapshots-unusedOnly = Only unused
containerd-snapshots-unused = Unused
containerd-snapshots-usedByContainer = container { $id }
containerd-snapshots-usedByImage = image { $name }
containerd-snapshots-empty = No snapshots in { $snapshotter }
containerd-snapshots-emptyDescription = Snapshots are the layers of unpacked images and the writable layers of containers.
containerd-snapshots-column-key = Key
containerd-snapshots-column-kind = Kind
containerd-snapshots-column-size = Size
containerd-snapshots-column-usedBy = Used by
containerd-snapshots-kind-committed = Layer
containerd-snapshots-kind-active = Writable
containerd-snapshots-kind-view = View
containerd-snapshots-kind-unknown = Unknown
containerd-snapshots-removeOne = Remove { $key }
containerd-snapshots-removed = Snapshot { $key } removed
containerd-snapshots-inUse = Something uses it or is built on it. Remove the container or image first.
containerd-snapshots-remove-title = Remove the snapshot { $key }?
containerd-snapshots-remove-description = Nothing uses it. Its files, { $size }, are deleted from the host.
containerd-namespaces-nav = Namespaces
containerd-namespaces-title = Namespaces
containerd-namespaces-summary = { $count ->
        [one] { NUM($count) } namespace
       *[other] { NUM($count) } namespaces
    }. Every containerd object lives in one: nerdctl uses default, Kubernetes k8s.io, Docker moby.
containerd-namespaces-new = New namespace
containerd-namespaces-created = Namespace { $name } created
containerd-namespaces-removed = Namespace { $name } removed
containerd-namespaces-shown = Shown
containerd-namespaces-show = Show
containerd-namespaces-removeOne = Remove { $name }
containerd-namespaces-notEmpty = Only empty namespaces can be removed. Remove its containers and images first.
containerd-namespaces-remove-title = Remove the namespace { $name }?
containerd-namespaces-remove-description = containerd refuses if it still holds snapshots or content that the counts don’t show.
containerd-namespaces-column-name = Name
containerd-namespaces-column-containers = Containers
containerd-namespaces-column-images = Images
containerd-namespaces-column-labels = Labels
containerd-namespaces-empty = No namespaces
containerd-namespaces-emptyDescription = containerd makes default the first time something is created in it.
containerd-namespaces-newDialog-title = New namespace
containerd-namespaces-newDialog-description = An empty namespace, to keep containers and images apart from the others.
containerd-namespaces-newDialog-name = Name
containerd-namespaces-newDialog-invalid = Use letters and digits, separated by single dots, dashes or underscores.
containerd-namespaces-newDialog-create = Create
