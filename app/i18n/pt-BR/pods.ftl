### The `pods` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/pods.ts:
### change the text there, then run it again.

pods-nav = Pods
pods-title = Pods
pods-summary = { $count ->
        [one] { NUM($count) } Pod
       *[other] { NUM($count) } Pods
    }
pods-search = Pesquisar Pods
pods-empty = Nenhum Pod aqui.
pods-column-name = Nome
pods-column-namespace = Namespace
pods-column-status = Status
pods-column-ready = Prontos
pods-column-restarts = Reinícios
pods-column-node = Node
pods-column-age = Idade
pods-tab-overview = Visão geral
pods-tab-conditions = Condições
pods-tab-events = Eventos
pods-tab-source = YAML
pods-detail-readyOf = { $ready } de { $total } contêineres prontos
pods-detail-namespace = Namespace
pods-detail-node = Node
pods-detail-ip = IP do Pod
pods-detail-hostIp = IP do Node
pods-detail-qos = Classe de QoS
pods-detail-serviceAccount = Service account
pods-detail-owner = Controlado por
pods-detail-age = Idade
pods-detail-labels = Rótulos
pods-detail-annotations = Anotações
pods-container-name = Contêiner
pods-container-init = init
pods-container-image = Imagem
pods-container-state = Estado
pods-container-ready = Pronto
pods-container-isReady = Sim
pods-container-notReady = Não
pods-container-restarts = Reinícios
pods-container-restartsWhy = { $count } (último: { $reason })
pods-container-started = Iniciado
pods-container-ports = Portas
pods-state-running = Em execução
pods-state-waiting = Aguardando
pods-state-terminated = Encerrado
pods-tab-logs = Logs
pods-logs-container = Contêiner
pods-logs-initContainer = { $name } (init)
pods-logs-previous = Instância anterior
pods-logs-noRestart = Ele não reiniciou, então não há instância anterior.
pods-logs-pod = Pod
pods-logs-allPods = { $count ->
        [one] O Pod
       *[other] Todos os { NUM($count) } Pods
    }
pods-logs-noPods = Ele não executa nenhum Pod no momento.
pods-tab-terminal = Terminal
pods-terminal-shell = Shell
pods-terminal-command = Comando
pods-terminal-connect = Conectar
pods-terminal-reconnect = Reconectar
pods-terminal-notRunning = Este contêiner não está em execução.
pods-terminal-exited = O processo foi encerrado.
pods-terminal-exitedWithCode = O processo foi encerrado com o código { $code }.
pods-column-cpu = CPU
pods-column-memory = Memória
pods-tab-metrics = Métricas
pods-metrics-noSample = O metrics-server ainda não tem amostra deste Pod: ele pode ter acabado de iniciar ou não estar em execução.
