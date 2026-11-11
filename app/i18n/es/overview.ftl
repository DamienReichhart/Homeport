### The `overview` namespace, in Español (es).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/es/overview.ts:
### change the text there, then run it again.

overview-nav = Resumen
overview-title = Resumen
overview-addHost = Añadir host
overview-addCluster = Añadir clúster
overview-cluster-nodes = { $ready } de { $total } Nodes listos
overview-cluster-nodesNotAllowed = Nodes: no permitido
overview-cluster-pods = { $running } de { $total } Pods en ejecución
overview-cluster-podsNotAllowed = Pods: no permitido
overview-cluster-notAllowed = No permitido.
overview-cluster-versionTitle = Clúster
overview-cluster-version = Versión
overview-cluster-platform = Plataforma
overview-cluster-server = Servidor de la API
overview-nomad-region = Región
overview-nomad-address = Dirección
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = Todos los namespaces
overview-cluster-nodesTitle = Nodes
overview-cluster-nodesReady = listos
overview-cluster-allNodes = Todos los Nodes
overview-cluster-podsTitle = Pods
overview-cluster-running = { $count } en ejecución
overview-cluster-pending = { $count } pendientes
overview-cluster-crashing = { $count } con fallos
overview-cluster-done = { $count } terminados u otros
overview-cluster-allPods = Todos los Pods
overview-cluster-unreadyTitle = Cargas de trabajo no listas
overview-cluster-allReady = Todos los Deployments, StatefulSets y DaemonSets tienen sus Pods listos.
overview-cluster-warningsTitle = Advertencias recientes
overview-cluster-noWarnings = No hay eventos de advertencia.
overview-cluster-allEvents = Todos los eventos
overview-host-running = <strong>{ $running }</strong> / { $total } en ejecución
overview-host-images = { $count ->
        [one] { NUM($count) } imagen
       *[other] { NUM($count) } imágenes
    }
overview-host-containerdContainers = { $count ->
        [one] { NUM($count) } contenedor
       *[other] { NUM($count) } contenedores
    } en { $namespaces ->
        [one] { NUM($namespaces) } namespace
       *[other] { NUM($namespaces) } namespaces
    }
overview-host-incusInstances = { $count ->
        [one] { NUM($count) } instancia
       *[other] { NUM($count) } instancias
    } en { $projects ->
        [one] { NUM($projects) } proyecto
       *[other] { NUM($projects) } proyectos
    }
overview-incus-title = Incus
overview-incus-version = Versión
overview-incus-os = Sistema
overview-incus-arch = Arquitectura
overview-incus-projects = Proyectos
overview-incus-shown = Mostrado
overview-incus-holds = { $instances ->
        [one] { NUM($instances) } instancia
       *[other] { NUM($instances) } instancias
    }, { $images ->
        [one] { NUM($images) } imagen
       *[other] { NUM($images) } imágenes
    }
overview-incus-instances = Instancias
overview-incus-images = Imágenes
overview-incus-storage = Almacenamiento
overview-containerd-title = containerd
overview-containerd-version = Versión
overview-containerd-revision = Revisión
overview-containerd-platform = Plataforma
overview-containerd-namespaces = Namespaces
overview-containerd-shown = Mostrado
overview-containerd-holds = { $containers ->
        [one] { NUM($containers) } contenedor
       *[other] { NUM($containers) } contenedores
    }, { $images ->
        [one] { NUM($images) } imagen
       *[other] { NUM($images) } imágenes
    }
overview-containerd-containers = Contenedores
overview-containerd-images = Imágenes
overview-containerd-snapshots = Instantáneas
overview-host-cpu = { $percent } de { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPU
    }
overview-host-cpuTitle = Uso de CPU de los contenedores, 100 % por CPU
overview-host-memoryTitle = Uso de memoria de los contenedores
overview-engine-title = Engine
overview-engine-name = Nombre
overview-engine-version = Versión
overview-engine-os = Sistema operativo
overview-engine-kernel = Kernel
overview-engine-architecture = Arquitectura
overview-engine-resources = Recursos
overview-engine-resourcesValue = { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPU
    } · { $memory }
overview-engine-storageDriver = Driver de almacenamiento
overview-engine-rootDir = Directorio raíz
overview-disk-title = Uso del disco
overview-disk-cleanUp = Limpiar…
overview-disk-column-type = Tipo
overview-disk-column-total = Total
overview-disk-column-active = En uso
overview-disk-column-size = Tamaño
overview-disk-column-reclaimable = Recuperable
overview-disk-images = Imágenes
overview-disk-containers = Contenedores
overview-disk-volumes = Volúmenes
overview-disk-buildCache = Caché de compilación
overview-disk-partial = Este Docker Engine es anterior a la versión 29 e informa de menos datos: los tamaños de las imágenes son estimados y los tamaños de los volúmenes y de la caché de compilación no están disponibles.
overview-events-title = Eventos recientes
overview-events-all = Todos los eventos
overview-events-empty = No ha pasado nada desde que Homeport se conectó.
overview-prune-title = Limpiar
overview-prune-description = Elige qué eliminar de este host. Esta acción no se puede deshacer.
overview-prune-confirm = Limpiar
overview-prune-confirmWithVolumes = Eliminar, incluidos los datos de los volúmenes
overview-prune-containers = Contenedores detenidos
overview-prune-containers-detail = Elimina todos los contenedores que no están en ejecución.
overview-prune-images = Imágenes huérfanas
overview-prune-images-detail = Capas sin etiqueta que dejan las compilaciones y las descargas.
overview-prune-allImages = Todas las imágenes sin usar
overview-prune-allImages-detail = También elimina las imágenes etiquetadas que no usa ningún contenedor.
overview-prune-networks = Redes sin usar
overview-prune-networks-detail = Redes personalizadas a las que no está conectado ningún contenedor.
overview-prune-buildCache = Caché de compilación
overview-prune-buildCache-detail = Capas de compilación en caché.
overview-prune-volumes = Volúmenes sin usar
overview-prune-volumes-detail = Elimina los datos de todos los volúmenes que no usa ningún contenedor, incluidos los volúmenes con nombre.
overview-pruned = Se eliminaron { $containers ->
        [one] { NUM($containers) } contenedor
       *[other] { NUM($containers) } contenedores
    }, { $images ->
        [one] { NUM($images) } imagen
       *[other] { NUM($images) } imágenes
    }, { $networks ->
        [one] { NUM($networks) } red
       *[other] { NUM($networks) } redes
    } y { $volumes ->
        [one] { NUM($volumes) } volumen
       *[other] { NUM($volumes) } volúmenes
    }: { $space } recuperados.
overview-palette-prune = Limpiar el host seleccionado
overview-host-onBoard = Sus contenedores en el Tablero
