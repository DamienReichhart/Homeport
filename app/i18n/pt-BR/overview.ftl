### The `overview` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/overview.ts:
### change the text there, then run it again.

overview-nav = Visão geral
overview-title = Visão geral
overview-addHost = Adicionar host
overview-addCluster = Adicionar cluster
overview-cluster-nodes = { $ready } de { $total } Nodes prontos
overview-cluster-nodesNotAllowed = Nodes: não permitido
overview-cluster-pods = { $running } de { $total } Pods em execução
overview-cluster-podsNotAllowed = Pods: não permitido
overview-cluster-notAllowed = Não permitido.
overview-cluster-versionTitle = Cluster
overview-cluster-version = Versão
overview-cluster-platform = Plataforma
overview-cluster-server = Servidor da API
overview-nomad-region = Região
overview-nomad-address = Endereço
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = Todos os namespaces
overview-cluster-nodesTitle = Nodes
overview-cluster-nodesReady = prontos
overview-cluster-allNodes = Todos os Nodes
overview-cluster-podsTitle = Pods
overview-cluster-running = { $count } em execução
overview-cluster-pending = { $count } pendentes
overview-cluster-crashing = { $count } falhando
overview-cluster-done = { $count } concluídos ou outros
overview-cluster-allPods = Todos os Pods
overview-cluster-unreadyTitle = Cargas de trabalho não prontas
overview-cluster-allReady = Todos os Deployments, StatefulSets e DaemonSets estão com os Pods prontos.
overview-cluster-warningsTitle = Avisos recentes
overview-cluster-noWarnings = Nenhum evento de aviso.
overview-cluster-allEvents = Todos os eventos
overview-host-running = <strong>{ $running }</strong> / { $total } em execução
overview-host-images = { $count ->
        [one] { NUM($count) } imagem
       *[other] { NUM($count) } imagens
    }
overview-host-containerdContainers = { $count ->
        [one] { NUM($count) } contêiner
       *[other] { NUM($count) } contêineres
    } em { $namespaces ->
        [one] { NUM($namespaces) } namespace
       *[other] { NUM($namespaces) } namespaces
    }
overview-host-incusInstances = { $count ->
        [one] { NUM($count) } instância
       *[other] { NUM($count) } instâncias
    } em { $projects ->
        [one] { NUM($projects) } projeto
       *[other] { NUM($projects) } projetos
    }
overview-incus-title = Incus
overview-incus-version = Versão
overview-incus-os = Sistema
overview-incus-arch = Arquitetura
overview-incus-projects = Projetos
overview-incus-shown = Mostrado
overview-incus-holds = { $instances ->
        [one] { NUM($instances) } instância
       *[other] { NUM($instances) } instâncias
    }, { $images ->
        [one] { NUM($images) } imagem
       *[other] { NUM($images) } imagens
    }
overview-incus-instances = Instâncias
overview-incus-images = Imagens
overview-incus-storage = Armazenamento
overview-containerd-title = containerd
overview-containerd-version = Versão
overview-containerd-revision = Revisão
overview-containerd-platform = Plataforma
overview-containerd-namespaces = Namespaces
overview-containerd-shown = Mostrado
overview-containerd-holds = { $containers ->
        [one] { NUM($containers) } contêiner
       *[other] { NUM($containers) } contêineres
    }, { $images ->
        [one] { NUM($images) } imagem
       *[other] { NUM($images) } imagens
    }
overview-containerd-containers = Contêineres
overview-containerd-images = Imagens
overview-containerd-snapshots = Snapshots
overview-host-cpu = { $percent } de { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPUs
    }
overview-host-cpuTitle = Uso de CPU dos contêineres, 100% por CPU
overview-host-memoryTitle = Uso de memória dos contêineres
overview-engine-title = Engine
overview-engine-name = Nome
overview-engine-version = Versão
overview-engine-os = SO
overview-engine-kernel = Kernel
overview-engine-architecture = Arquitetura
overview-engine-resources = Recursos
overview-engine-resourcesValue = { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPUs
    } · { $memory }
overview-engine-storageDriver = Driver de armazenamento
overview-engine-rootDir = Diretório raiz
overview-disk-title = Uso de disco
overview-disk-cleanUp = Limpar…
overview-disk-column-type = Tipo
overview-disk-column-total = Total
overview-disk-column-active = Em uso
overview-disk-column-size = Tamanho
overview-disk-column-reclaimable = Recuperável
overview-disk-images = Imagens
overview-disk-containers = Contêineres
overview-disk-volumes = Volumes
overview-disk-buildCache = Cache de build
overview-disk-partial = Este Docker Engine é anterior à versão 29 e informa menos: os tamanhos das imagens são estimados, e os tamanhos dos volumes e do cache de build não estão disponíveis.
overview-events-title = Eventos recentes
overview-events-all = Todos os eventos
overview-events-empty = Nada aconteceu desde que o Homeport se conectou.
overview-prune-title = Limpar
overview-prune-description = Escolha o que remover deste host. Isso não pode ser desfeito.
overview-prune-confirm = Limpar
overview-prune-confirmWithVolumes = Excluir, incluindo os dados dos volumes
overview-prune-containers = Contêineres parados
overview-prune-containers-detail = Remove todos os contêineres que não estão em execução.
overview-prune-images = Imagens órfãs
overview-prune-images-detail = Camadas sem tag deixadas por builds e downloads.
overview-prune-allImages = Todas as imagens não usadas
overview-prune-allImages-detail = Também remove imagens com tag que nenhum contêiner usa.
overview-prune-networks = Redes não usadas
overview-prune-networks-detail = Redes personalizadas sem nenhum contêiner conectado.
overview-prune-buildCache = Cache de build
overview-prune-buildCache-detail = Camadas de build em cache.
overview-prune-volumes = Volumes não usados
overview-prune-volumes-detail = Exclui os dados de todos os volumes que nenhum contêiner usa, inclusive os volumes nomeados.
overview-pruned = { $containers ->
        [one] { NUM($containers) } contêiner
       *[other] { NUM($containers) } contêineres
    }, { $images ->
        [one] { NUM($images) } imagem
       *[other] { NUM($images) } imagens
    }, { $networks ->
        [one] { NUM($networks) } rede
       *[other] { NUM($networks) } redes
    } e { $volumes ->
        [one] { NUM($volumes) } volume
       *[other] { NUM($volumes) } volumes
    } removidos — { $space } recuperados.
overview-palette-prune = Limpar o host selecionado
overview-host-onBoard = Os contêineres dele no Quadro
