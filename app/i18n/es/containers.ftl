### The `containers` namespace, in Español (es).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/es/containers.ts:
### change the text there, then run it again.

containers-nav = Contenedores
containers-title = Contenedores
containers-summary = { $running } en ejecución de { $count }
containers-runContainer = Ejecutar contenedor
containers-search = Buscar contenedores
containers-onlyRunning = Solo en ejecución
containers-empty-title = No hay contenedores
containers-empty-description = Ejecuta uno a partir de una imagen o despliega un stack.
containers-noMatch = Ningún contenedor coincide.
containers-column-name = Nombre
containers-column-image = Imagen
containers-column-status = Estado
containers-column-ports = Puertos
containers-column-cpu = CPU
containers-column-memory = Memoria
containers-column-created = Creado
containers-selectOne = Seleccionar { $name }
containers-selectStack = Seleccionar el stack { $stack }
containers-stack-running = { $running }/{ $count } en ejecución
containers-stack-swarm = Swarm
containers-stack-open = Abrir stack
containers-inPod = En el pod de Podman { $pod }
containers-fromUnit = Creado por la unidad Quadlet { $unit }
containers-selected = { $count } seleccionados
containers-action-start = Iniciar
containers-action-stop = Detener
containers-action-restart = Reiniciar
containers-action-pause = Pausar
containers-action-resume = Reanudar
containers-action-kill = Matar
containers-action-rename = Cambiar nombre
containers-action-edit = Editar
containers-action-duplicate = Duplicar
containers-action-logs = Logs
containers-action-terminal = Abrir terminal
containers-action-commit = Hacer commit en una imagen…
containers-action-export = Exportar sistema de archivos…
containers-kill-title = ¿Matar { $name }?
containers-kill-description = El proceso principal recibe SIGKILL de inmediato, sin oportunidad de cerrarse limpiamente.
containers-done-start = { $count ->
        [one] Se inició el contenedor
       *[other] Se iniciaron { NUM($count) } contenedores
    }
containers-done-stop = { $count ->
        [one] Se detuvo el contenedor
       *[other] Se detuvieron { NUM($count) } contenedores
    }
containers-done-restart = { $count ->
        [one] Se reinició el contenedor
       *[other] Se reiniciaron { NUM($count) } contenedores
    }
containers-done-pause = { $count ->
        [one] Se pausó el contenedor
       *[other] Se pausaron { NUM($count) } contenedores
    }
containers-done-unpause = { $count ->
        [one] Se reanudó el contenedor
       *[other] Se reanudaron { NUM($count) } contenedores
    }
containers-done-kill = { $count ->
        [one] Se mató el contenedor
       *[other] Se mataron { NUM($count) } contenedores
    }
containers-done-remove = { $count ->
        [one] Se eliminó el contenedor
       *[other] Se eliminaron { NUM($count) } contenedores
    }
containers-remove-title = ¿Eliminar { $name }?
containers-remove-titleMany = ¿Eliminar { $count ->
        [one] { NUM($count) } contenedor
       *[other] { NUM($count) } contenedores
    }?
containers-remove-deleted = { $count ->
        [one] Se eliminan el contenedor, su capa de escritura y sus volúmenes anónimos.
       *[other] Se eliminan los contenedores, su capa de escritura y sus volúmenes anónimos.
    }
containers-remove-running = Los { $count } en ejecución se detendrán antes.
containers-remove-namedKept = Los volúmenes con nombre se conservan.
containers-detail-gone = Este contenedor ya no existe.
containers-detail-backToList = Volver a los contenedores
containers-tab-logs = Logs
containers-tab-inspect = Inspeccionar
containers-tab-health = Salud
containers-tab-exec = Exec
containers-tab-files = Archivos
containers-tab-stats = Estadísticas
containers-tab-processes = Procesos
containers-notRunning = El contenedor no se está ejecutando.
containers-logs-tail-100 = Últimas 100
containers-logs-tail-1000 = Últimas 1000
containers-logs-tail-10000 = Últimas 10 000
containers-logs-tail-all = Todo
containers-logs-since-all = Cualquier momento
containers-logs-since-300 = Últimos 5 min
containers-logs-since-3600 = Última hora
containers-logs-since-86400 = Últimas 24 h
containers-logs-history = Historial
containers-logs-sinceLabel = Desde
containers-logs-timestamps = Marcas de tiempo
containers-logs-follow = Seguir
containers-logs-find = Buscar en los logs
containers-logs-previous = Coincidencia anterior
containers-logs-next = Coincidencia siguiente
containers-logs-clear = Borrar
containers-logs-download = Descargar logs
containers-logs-saveTitle = Guardar logs
containers-logs-saved = Se guardaron los logs
containers-logs-saving = Guardando logs… { $percent }
containers-logs-stdout = stdout
containers-logs-stderr = stderr
containers-logs-end = — fin de los logs —
containers-inspect-raw = JSON sin procesar
containers-inspect-container = Contenedor
containers-inspect-health = Salud
containers-inspect-environment = Entorno
containers-inspect-mounts = Montajes
containers-inspect-ports = Puertos
containers-inspect-networks = Redes
containers-inspect-labels = Etiquetas
containers-inspect-id = ID
containers-inspect-image = Imagen
containers-inspect-created = Creado
containers-inspect-state = Estado
containers-inspect-started = Iniciado
containers-inspect-entrypoint = Entrypoint
containers-inspect-command = Comando
containers-inspect-workingDir = Directorio de trabajo
containers-inspect-user = Usuario
containers-inspect-restartPolicy = Política de reinicio
containers-inspect-networkMode = Modo de red
containers-inspect-exited = { $status } (salida { $code })
containers-inspect-outOfMemory = { $state }, sin memoria
containers-inspect-restarted = { $policy } · reiniciado { $count }×
containers-inspect-restartedMax = { $policy } (máx. { $max }) · reiniciado { $count }×
containers-inspect-imageDefault = predeterminado de la imagen
containers-inspect-notPublished = no publicado
containers-inspect-readWrite = lectura y escritura
containers-inspect-readOnly = solo lectura
containers-inspect-failingStreak = { $count } fallos seguidos
containers-inspect-noHealthcheck = La imagen no define ninguna comprobación de salud.
containers-inspect-column-started = Iniciado
containers-inspect-column-exit = Salida
containers-inspect-column-output = Resultado
containers-inspect-column-variable = Variable
containers-inspect-column-value = Valor
containers-inspect-column-type = Tipo
containers-inspect-column-source = Origen
containers-inspect-column-destination = Destino
containers-inspect-column-mode = Modo
containers-inspect-column-container = Contenedor
containers-inspect-column-publishedOn = Publicado en
containers-inspect-column-network = Red
containers-inspect-column-ip = IP
containers-inspect-column-gateway = Puerta de enlace
containers-inspect-column-aliases = Alias
containers-inspect-column-label = Etiqueta
containers-health-none = Este contenedor no tiene comprobación de salud.
containers-health-title = Comprobación de salud
containers-health-command = Comando
containers-health-timingLabel = Tiempos
containers-health-timing = cada { $interval }, tiempo de espera { $timeout }, { $retries ->
        [one] { NUM($retries) } reintento
       *[other] { NUM($retries) } reintentos
    }, periodo inicial { $start }
containers-health-failingStreak = Fallos seguidos
containers-health-lastResults = Últimos resultados
containers-health-noRuns = Aún no se ha ejecutado ninguna comprobación.
containers-health-noOutput = Sin salida.
containers-health-showLess = Mostrar menos
containers-health-showMore = Mostrar { $count ->
        [one] { NUM($count) } línea más
       *[other] { NUM($count) } líneas más
    }
containers-health-exit = salida { $code }
containers-health-took = · tardó { $duration }
containers-exec-startFirst = Inicia el contenedor para abrir un terminal.
containers-exec-shell = Shell
containers-exec-shellNumber = Shell { $number }
containers-exec-closeOne = Cerrar { $title }
containers-exec-new = Nuevo terminal
containers-exec-command = Comando
containers-exec-commandPlaceholder = bash, o sh si no está
containers-exec-user = Usuario
containers-exec-directory = Directorio
containers-exec-default = predeterminado
containers-exec-open = Abrir
containers-exec-none = No hay ningún terminal abierto.
containers-exec-exited = [el proceso terminó]
containers-exec-exitedWithCode = [el proceso terminó con el código { $code }]
containers-files-startFirst = Inicia el contenedor para explorar sus archivos.
containers-files-saveOne = Guardar { $name }
containers-files-uploadTo = Subir a { $path }
containers-files-uploadHere = Subir aquí
containers-files-saved = Se guardó { $name }
containers-files-savedArchive = Se guardó { $name } como archivo .tar
containers-files-downloading = Guardando { $name }…
containers-files-uploading = Subiendo { $done } de { $count } a { $path }…
containers-files-uploaded = { $count ->
        [one] Se subió { NUM($count) } archivo
       *[other] Se subieron { NUM($count) } archivos
    } a { $path }
containers-stats-startFirst = Inicia el contenedor para ver su uso de recursos.
containers-stats-cpu = CPU
containers-stats-memory = Memoria
containers-stats-memoryNow = · { $used } de { $limit }
containers-stats-used = Usado
containers-stats-network = Red
containers-stats-networkNow = · { $received } de entrada, { $sent } de salida
containers-stats-received = Recibido
containers-stats-sent = Enviado
containers-stats-disk = E/S de disco
containers-stats-processes = · { $count ->
        [one] { NUM($count) } proceso
       *[other] { NUM($count) } procesos
    }
containers-stats-read = Lectura
containers-stats-written = Escritura
containers-run-title = Ejecutar un contenedor
containers-run-description = Primero se descarga la imagen si este host no la tiene.
containers-run-submit = Ejecutar
containers-run-starting = Iniciando…
containers-run-started = Contenedor iniciado
containers-run-image = Imagen
containers-run-name = Nombre
containers-run-namePlaceholder = Se genera si se deja vacío
containers-run-nameKept = Se conserva. Usa Cambiar nombre para modificarlo.
containers-run-command = Comando
containers-run-commandPlaceholder = El predeterminado de la imagen
containers-run-environment = Entorno
containers-run-network = Red
containers-run-pod = Pod
containers-run-noPod = Ninguno
containers-run-podOwnsNetwork = Los contenedores de { $pod } comparten su red: los puertos, la red, el nombre de host y el DNS son los del pod.
containers-run-defaultNetwork = Predeterminada (bridge)
containers-run-restart = Reinicio
containers-run-restartPolicy = Política de reinicio
containers-run-restart-no = Nunca
containers-run-restart-onFailure = Si falla
containers-run-restart-unlessStopped = Salvo si se detiene
containers-run-restart-always = Siempre
containers-run-autoRemove = Eliminar el contenedor cuando termine
containers-run-ports = Puertos
containers-run-hostPort = Puerto del host
containers-run-hostPortPlaceholder = Puerto del host (cualquiera)
containers-run-containerPort = Puerto del contenedor
containers-run-protocol = Protocolo
containers-run-removePort = Quitar puerto
containers-run-addPort = Añadir puerto
containers-run-volumes = Volúmenes
containers-run-source = Origen
containers-run-sourcePlaceholder = Nombre de volumen o /ruta/del/host
containers-run-containerPath = Ruta en el contenedor
containers-run-readOnly = solo lectura
containers-run-removeVolume = Quitar volumen
containers-run-addVolume = Añadir volumen
containers-run-removeValue = Quitar { $value }
containers-run-error-size = Escribe un tamaño como 512m o 2g.
containers-run-error-cpus = Escribe un número de CPU como 0.5 o 2.
containers-run-error-cpuShares = Escribe un número entero como 512.
containers-run-error-memorySwap = Escribe un tamaño como 1g, o -1 para swap ilimitado.
containers-run-error-gpuCount = Escribe cuántas GPU, como 1.
containers-run-error-duration = Escribe una duración como 30s o 1m30s.
containers-run-error-retries = Escribe un número entero como 3.
containers-advanced-title = Opciones avanzadas
containers-advanced-hasErrors = Tiene errores
containers-advanced-resources = Recursos
containers-advanced-labels = Etiquetas
containers-advanced-security = Seguridad
containers-advanced-devices = Dispositivos
containers-advanced-healthcheck = Comprobación de salud
containers-advanced-network = Identidad de red
containers-advanced-logging = Logs
containers-advanced-cpus = Límite de CPU
containers-advanced-cpusHint = CPU, como 0.5 o 2.
containers-advanced-cpusPlaceholder = Todas las CPU
containers-advanced-cpuShares = CPU shares
containers-advanced-cpuSharesHint = Peso frente a otros contenedores; 1024 es el valor predeterminado.
containers-advanced-memory = Límite de memoria
containers-advanced-memoryHint = Como 512m o 2g.
containers-advanced-memoryPlaceholder = Sin límite
containers-advanced-memoryReservation = Reserva de memoria
containers-advanced-memoryReservationHint = Un límite flexible, que se respeta cuando al host le falta memoria.
containers-advanced-memorySwap = Límite de memoria y swap
containers-advanced-memorySwapHint = Memoria más swap; -1 para swap ilimitado.
containers-advanced-memorySwapPlaceholder = El doble del límite de memoria
containers-advanced-none = Ninguno
containers-advanced-labelsHint = Una clave=valor por línea, tal como las leen los proxies inversos y las herramientas de copia de seguridad.
containers-advanced-capAdd = Añadir capabilities
containers-advanced-capAddHint = Como NET_ADMIN para una VPN.
containers-advanced-capDrop = Quitar capabilities
containers-advanced-capDropHint = ALL las quita todas.
containers-advanced-privileged = Privilegiado
containers-advanced-privilegedTitle = El contenedor obtiene acceso completo al host
containers-advanced-privilegedWarning = Puede usar todos los dispositivos y capabilities, y cualquier cosa que se ejecute en él puede tomar el control del host. Es preferible añadir las capabilities y los dispositivos que necesite.
containers-advanced-hostDevice = Dispositivo del host
containers-advanced-containerDevice = Dispositivo del contenedor
containers-advanced-samePath = La misma ruta
containers-advanced-permissions = Permisos
containers-advanced-removeDevice = Quitar dispositivo
containers-advanced-addDevice = Añadir dispositivo
containers-advanced-gpus = GPU
containers-advanced-gpusHint = Requiere que el container toolkit del fabricante de la GPU esté instalado en el host.
containers-advanced-gpus-all = Todas las GPU
containers-advanced-gpus-count = Un número de GPU
containers-advanced-gpus-ids = GPU elegidas
containers-advanced-gpuCount = Cuántas
containers-advanced-gpuIds = Índices o UUID de GPU
containers-advanced-gpuIdsHint = Separados por comas.
containers-advanced-health-image = La de la propia imagen, si tiene
containers-advanced-health-custom = Un comando
containers-advanced-health-disabled = Ninguna (desactivar la de la imagen)
containers-advanced-healthCommand = Comando de comprobación
containers-advanced-healthCommandHint = Se ejecuta con /bin/sh -c; el código de salida 0 significa saludable.
containers-advanced-interval = Intervalo
containers-advanced-timeout = Tiempo de espera
containers-advanced-retries = Reintentos
containers-advanced-startPeriod = Periodo inicial
containers-advanced-hostname = Nombre de host
containers-advanced-hostnamePlaceholder = El ID del contenedor
containers-advanced-domainname = Nombre de dominio
containers-advanced-dns = Servidores DNS
containers-advanced-dnsHint = Una dirección por línea.
containers-advanced-dnsSearch = Dominios de búsqueda DNS
containers-advanced-dnsSearchHint = Uno por línea.
containers-advanced-extraHosts = Hosts adicionales
containers-advanced-extraHostsHint = Un nombre:dirección por línea; host-gateway representa la dirección del host.
containers-advanced-logDriver = Driver de logs
containers-advanced-logDriverHint = Con none, la pestaña Logs queda vacía.
containers-advanced-logDriverPlaceholder = El predeterminado del Engine
containers-advanced-logOptions = Opciones del driver
containers-advanced-logOptionsHint = Una clave=valor por línea; max-size y max-file rotan los logs json-file.
containers-rename-title = Cambiar el nombre de { $name }
containers-rename-newName = Nombre nuevo
containers-duplicate-title = Duplicar { $name }
containers-duplicate-sharedPorts = { $name } ya publica { $count ->
        [one] el puerto del host
       *[other] los puertos del host
    } { $ports }.
containers-duplicate-sharedPortsFix = { $count ->
        [one] La copia no puede iniciarse mientras ambos lo usen
       *[other] La copia no puede iniciarse mientras ambos los usen
    }: cambia el puerto del host o déjalo vacío para que el Engine elija uno.
containers-edit-title = Editar { $name }
containers-edit-description = Al aplicar se recrea el contenedor con estos ajustes, conservando su nombre, sus redes y sus volúmenes con nombre.
containers-edit-apply = Aplicar
containers-edit-recreating = Recreando…
containers-edit-recreated = Contenedor recreado
containers-edit-confirmTitle = ¿Recrear { $name }?
containers-edit-nothingLost = No se pierde nada más: se conservan su nombre, sus redes y sus volúmenes con nombre.
containers-edit-keptBut = Se conservan su nombre, sus redes y sus volúmenes con nombre, pero no:
containers-edit-commitFirst = Hacer antes commit en una imagen…
containers-edit-recreate = Recrear
containers-loss-anonymousVolume = El volumen anónimo en { $path }: el nuevo contenedor empieza con uno vacío. El anterior se conserva como volumen sin usar.
containers-loss-filesystemChanges = { $count ->
        [one] { NUM($count) } archivo cambió
       *[other] { NUM($count) } archivos cambiaron
    } dentro del contenedor desde que se creó. Haz antes commit en una imagen para conservarlos.
containers-loss-downtime = Se detiene mientras se intercambian los dos, normalmente durante unos segundos.
containers-loss-stackRedeploy = Pertenece al stack { $stack }: el próximo despliegue del stack lo recrea a partir de su archivo compose, sin esta edición.
containers-commit-title = Hacer commit de { $name } en una imagen
containers-commit-description = La nueva imagen contiene los archivos del contenedor tal como están ahora, incluidos los cambios hechos dentro, y sus ajustes. Los volúmenes no se incluyen.
containers-commit-imageName = Nombre de la imagen
containers-commit-imageNameHint = Como repo:etiqueta.
containers-commit-nameTaken = Ya hay una imagen llamada { $name }. El nombre pasa a la nueva imagen; la anterior conserva sus otros nombres o se queda sin etiqueta.
containers-commit-message = Mensaje
containers-commit-messagePlaceholder = Antes de la actualización
containers-commit-author = Autor
containers-commit-changes = Cambios
containers-commit-changesHint = Instrucciones de Dockerfile, una por línea: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME o WORKDIR.
containers-commit-pause = Pausar el contenedor durante el commit
containers-commit-pauseOn = Sus procesos se congelan durante los pocos segundos que tarda el commit, así que los archivos que se están escribiendo quedan coherentes.
containers-commit-pauseOff = Sigue en ejecución, así que los archivos que se están escribiendo pueden quedar a medias.
containers-commit-submit = Hacer commit
containers-commit-committing = Haciendo commit…
containers-commit-done = Commit hecho en { $reference }
containers-commit-openImages = Abrir imágenes
containers-export-tarArchive = Archivo tar
containers-export-pick = Exportar el sistema de archivos de { $name }
containers-export-title = Exportando { $name }
containers-export-to = En { $path }
containers-export-written = Escrito
containers-export-finished = Listo. Impórtalo en la página Imágenes o con docker import.
containers-export-done = Exportado a { $destination }
containers-palette-run = Ejecutar un contenedor
