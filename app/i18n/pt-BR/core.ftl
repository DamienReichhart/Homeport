### The `core` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/core.ts:
### change the text there, then run it again.

core-host-engineVersion = Engine { $version }
core-host-podmanVersion = Podman { $version }
core-host-containerdVersion = containerd { $version }
core-host-incusVersion = Incus { $version }
core-engine-docker = Compatível com Docker
core-engine-containerd = containerd
core-engine-incus = Incus
core-unsupported-title = { $name } não pode mostrar esta página
core-unsupported-docker = Esta página precisa de um Engine compatível com Docker, e { $name } executa { $engine }.
core-unsupported-podman = Esta página é para o Podman, e { $name } executa outro Engine compatível com Docker. Escolha um host Podman no seletor para usá-la.
core-unsupported-containerd = Esta página é para o containerd, e { $name } executa { $engine }. Escolha um host containerd no seletor para usá-la.
core-unsupported-incus = Esta página é para o Incus, e { $name } executa { $engine }. Escolha um host Incus no seletor para usá-la.
core-unsupported-overview = Ir para a Visão geral
core-cluster-kind-kubernetes = Kubernetes
core-cluster-kind-nomad = Nomad
core-cluster-unsupported-title = { $name } não pode mostrar esta página
core-cluster-unsupported-description = Esta página é para clusters { $kind }, e { $name } é um cluster { $current }. Escolha um cluster { $kind } no seletor para usá-la.
core-containerd-namespace = Namespace
core-containerd-holds = { $containers ->
        [one] { NUM($containers) } contêiner
       *[other] { NUM($containers) } contêineres
    }, { $images ->
        [one] { NUM($images) } imagem
       *[other] { NUM($images) } imagens
    }
core-containerd-manage = Gerenciar namespaces
core-containerd-rememberFailed = O Homeport não conseguiu lembrar o namespace que você escolheu
core-incus-project = Projeto
core-incus-holds = { $instances ->
        [one] { NUM($instances) } instância
       *[other] { NUM($instances) } instâncias
    }, { $images ->
        [one] { NUM($images) } imagem
       *[other] { NUM($images) } imagens
    }
core-incus-rememberFailed = O Homeport não conseguiu lembrar o projeto que você escolheu
core-selection-rememberFailed = O Homeport não conseguiu lembrar o que você selecionou
core-connection-local = Local
core-status-connected = Conectado
core-status-connecting = Conectando
core-status-disabled = Desativado
core-status-unreachable = Inacessível
core-status-unreachableDetail = { $message } — tentando novamente em { $secs }s
core-status-unauthorized = Acesso recusado
core-status-hostKeyUntrusted = Chave do servidor não confiável
core-status-hostKeyUntrustedDetail = Confira a impressão digital { $fingerprint } e confie nela para conectar.
core-status-hostKeyMismatch = A chave do servidor mudou
core-status-hostKeyMismatchDetail = Esperada { $expected }, recebida { $actual }.
core-status-credentialsLost = Credenciais perdidas
core-status-credentialsLostDetail = As credenciais armazenadas não podem ser descriptografadas. Digite-as novamente ou gere a chave de novo.
core-status-keychainUnavailable = Chaveiro indisponível
core-status-invalid = Configuração incorreta
core-swarm-label = Swarm de { $host }
core-swarm-labelUnknown = Swarm { $id }
core-swarm-gone-title = Este Swarm não é mais conhecido
core-swarm-gone-description = Nenhum dos hosts aos quais o Homeport se conecta pertence a ele agora. Adicione um dos seus managers como host para gerenciá-lo.
core-cluster-serverVersion = Kubernetes { $version }
core-cluster-nomadVersion = Nomad { $version }, região { $region }
core-cluster-nomadRegion = Nomad, região { $region }
core-cluster-credentialsLostDetail = As credenciais armazenadas não podem ser descriptografadas. Atualize o cluster a partir do seu kubeconfig.
core-cluster-reconnectFailed = O Homeport não conseguiu se reconectar ao cluster
core-cluster-gate-none = Nenhum cluster selecionado
core-cluster-gate-noneDescription = Escolha um cluster na barra lateral ou adicione um a partir do seu kubeconfig.
core-cluster-gate-add = Adicionar um cluster
core-cluster-gate-openSettings = Abrir configurações do cluster
core-localClusters-title = Clusters nesta máquina
core-localClusters-description = Kubernetes em execução localmente, como o seu kubeconfig o alcança.
core-localClusters-add = Adicionar
core-localClusters-added = Adicionado
core-localClusters-notRunning = Não está em execução
core-localClusters-addFailed = O Homeport não conseguiu adicionar o cluster
core-localClusters-missing = O cluster { $tool } { $name } está em execução, mas o seu kubeconfig não o alcança. Adicione-o com:
core-localClusters-tool-dockerDesktop = Docker Desktop
core-localClusters-tool-orbstack = OrbStack
core-localClusters-tool-rancherDesktop = Rancher Desktop
core-localClusters-tool-kind = kind
core-localClusters-tool-k3d = k3d
core-localClusters-tool-minikube = minikube
core-namespace-label = Namespace
core-namespace-all = Todos os namespaces
core-namespace-type = Namespace
core-namespace-cantList = Suas credenciais não podem listar namespaces. Digite aquele que você tem permissão para usar.
core-namespace-rememberFailed = O Homeport não conseguiu lembrar o namespace
core-health-ready = Pronto
core-health-progressing = Em andamento
core-health-degraded = Degradado
core-health-failed = Falhou
core-health-paused = Pausado
core-events-type = Tipo
core-events-reason = Motivo
core-events-message = Mensagem
core-events-count = Contagem
core-events-lastSeen = Visto por último
core-events-none = Nenhum evento. O Kubernetes os mantém por cerca de uma hora.
core-conditions-type = Condição
core-conditions-status = Status
core-conditions-reason = Motivo
core-conditions-message = Mensagem
core-conditions-since = Desde
core-conditions-none = Nenhuma condição informada.
core-labels-none = Nenhum
core-yaml-label = YAML
core-list-empty = Nenhum aqui.
core-yaml-copy = Copiar YAML
core-gate-welcome-title = Bem-vindo ao Homeport
core-gate-welcome-description = Traga um Docker Engine para começar — o desta máquina ou um remoto via SSH ou TLS — ou um cluster Kubernetes do seu kubeconfig.
core-gate-addAnother = Adicionar outro host
core-gate-addCluster = Adicionar um cluster
core-gate-connecting = Conectando a { $name }…
core-gate-statusTitle = { $name }: { $status }
core-gate-retryingOnItsOwn = O Homeport tenta novamente por conta própria em { $secs }s.
core-gate-retryNow = Tentar agora
core-gate-openSettings = Abrir configurações do host
core-localEngines-title = Encontrados nesta máquina
core-localEngines-description = Engines respondendo em sockets locais.
core-localEngines-add = Adicionar
core-freshness-check = Verificar imagens mais recentes
core-freshness-checking = Verificando…
core-freshness-allUpToDate = Todas as imagens estão atualizadas
core-freshness-newerCount = { $count ->
        [one] { NUM($count) } imagem tem
       *[other] { NUM($count) } imagens têm
    } uma versão mais recente
core-freshness-newerOne = Uma versão mais recente de { $reference } foi publicada
core-freshness-newerMany = Imagens mais recentes foram publicadas: { $references }
core-delete-title = Excluir { $kind } { $name }?
core-delete-titleMany = Excluir { $count } objetos { $kind }?
core-delete-titleObjects = Excluir { $count } objetos?
core-delete-one = { $kind } { $name } será excluído de { $cluster }.
core-delete-oneIn = { $kind } { $name } será excluído do namespace { $namespace } em { $cluster }.
core-delete-many = Estes serão excluídos de { $cluster }: { $names }.
core-delete-manyIn = Estes serão excluídos do namespace { $namespace } em { $cluster }: { $names }.
core-delete-namespace = Tudo o que está no namespace vai junto: seus workloads, Pods, Services, ConfigMaps e Secrets.
core-delete-recreated = { $kind } { $name } o controla, então iniciará um novo Pod no lugar dele.
core-delete-pods = Seus Pods são excluídos junto.
core-delete-cronJob = Ele não inicia mais Jobs; os que ele iniciou são excluídos junto.
core-delete-node = O Homeport não o drena antes: seus Pods permanecem até o Node se registrar novamente ou serem reagendados.
core-delete-confirm = Excluir
core-delete-failed = O Homeport não conseguiu excluir { $name }
core-delete-done = { $count ->
        [one] { $name } excluído
       *[other] { NUM($count) } objetos excluídos
    }
core-delete-action = Excluir
core-delete-selected = Excluir { $count }
core-apply-preview = Pré-visualizar alterações
core-apply-apply = Aplicar
core-apply-force = Aplicar mesmo assim
core-apply-stale = O YAML mudou desde esta pré-visualização.
core-apply-noChanges = Nenhuma alteração: o cluster já contém isto.
core-apply-confirmTitle = Aplicar { $count ->
        [one] { NUM($count) } alteração
       *[other] { NUM($count) } alterações
    } em { $cluster }?
core-apply-confirmDescription = O Homeport aplica com server-side apply como o field manager homeport, como a pré-visualização mostra.
core-apply-forceTitle = Assumir os campos em conflito?
core-apply-forceDescription = As ferramentas que os gerenciam em { $cluster } podem revertê-los ou deixar de gerenciá-los. O Homeport passa a ser o gerenciador deles.
core-apply-done = Aplicado em { $cluster }
core-apply-diffOf = Alterações em { $kind } { $name }
core-apply-action-created = Criado
core-apply-action-changed = Alterado
core-apply-action-unchanged = Sem alterações
core-apply-action-failed = Não é possível aplicar
core-yaml-edit = Editar
core-yaml-editTitle = Editar { $kind } { $name }
core-yaml-editDescription = Em { $cluster }. Altere o YAML, pré-visualize o que ele altera e depois aplique.
core-metrics-cpu = CPU
core-metrics-memory = Memória
core-metrics-used = Uso
core-metrics-request = Solicitação
core-metrics-limit = Limite
core-metrics-allocatable = Alocável
core-metrics-ofLimit = { $percent } do limite
core-metrics-ofRequest = { $percent } da solicitação
core-metrics-ofAllocatable = { $percent } do alocável
core-metrics-notInstalled-title = O metrics-server não está instalado
core-metrics-notInstalled-body = Este cluster não oferece a Metrics API, de onde vêm a CPU e a memória. O Homeport não instala nada em um cluster: quando o metrics-server estiver instalado, elas aparecem aqui.
core-metrics-notInstalled-link = Como instalar o metrics-server
core-metrics-notResponding = A Metrics API não responde: o metrics-server pode estar iniciando ou não estar em execução. O Homeport tenta de novo a cada 15 segundos.
core-metrics-waiting = Aguardando a primeira amostra do metrics-server…
core-metrics-kept = Uma amostra a cada 15 segundos, guardada enquanto esta página estiver aberta, por no máximo 15 minutos.
