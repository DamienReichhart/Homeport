### The `pods` namespace, in Español (es).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/es/pods.ts:
### change the text there, then run it again.

pods-nav = Pods
pods-title = Pods
pods-summary = { $count ->
        [one] { NUM($count) } Pod
       *[other] { NUM($count) } Pods
    }
pods-search = Buscar Pods
pods-empty = No hay Pods aquí.
pods-column-name = Nombre
pods-column-namespace = Namespace
pods-column-status = Estado
pods-column-ready = Listos
pods-column-restarts = Reinicios
pods-column-node = Node
pods-column-age = Antigüedad
pods-tab-overview = Resumen
pods-tab-conditions = Condiciones
pods-tab-events = Eventos
pods-tab-source = YAML
pods-detail-readyOf = { $ready } de { $total } contenedores listos
pods-detail-namespace = Namespace
pods-detail-node = Node
pods-detail-ip = IP del Pod
pods-detail-hostIp = IP del Node
pods-detail-qos = Clase QoS
pods-detail-serviceAccount = Cuenta de servicio
pods-detail-owner = Controlado por
pods-detail-age = Antigüedad
pods-detail-labels = Etiquetas
pods-detail-annotations = Anotaciones
pods-container-name = Contenedor
pods-container-init = init
pods-container-image = Imagen
pods-container-state = Estado
pods-container-ready = Listo
pods-container-isReady = Sí
pods-container-notReady = No
pods-container-restarts = Reinicios
pods-container-restartsWhy = { $count } (último: { $reason })
pods-container-started = Iniciado
pods-container-ports = Puertos
pods-state-running = En ejecución
pods-state-waiting = En espera
pods-state-terminated = Terminado
pods-tab-logs = Logs
pods-logs-container = Contenedor
pods-logs-initContainer = { $name } (init)
pods-logs-previous = Instancia anterior
pods-logs-noRestart = No se ha reiniciado, así que no hay ninguna instancia anterior.
pods-logs-pod = Pod
pods-logs-allPods = { $count ->
        [one] El Pod
       *[other] Todos los { NUM($count) } Pods
    }
pods-logs-noPods = Ahora mismo no ejecuta ningún Pod.
pods-tab-terminal = Terminal
pods-terminal-shell = Shell
pods-terminal-command = Comando
pods-terminal-connect = Conectar
pods-terminal-reconnect = Volver a conectar
pods-terminal-notRunning = Este contenedor no se está ejecutando.
pods-terminal-exited = El proceso terminó.
pods-terminal-exitedWithCode = El proceso terminó con el código { $code }.
pods-column-cpu = CPU
pods-column-memory = Memoria
pods-tab-metrics = Métricas
pods-metrics-noSample = metrics-server aún no tiene ninguna muestra de este Pod: puede que acabe de iniciarse o que no esté en ejecución.
