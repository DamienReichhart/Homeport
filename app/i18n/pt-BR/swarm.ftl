### The `swarm` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/swarm.ts:
### change the text there, then run it again.

swarm-nav = Swarms
swarm-badge = Swarm
swarm-reachable = Um manager está conectado
swarm-unreachable = Nenhum manager conectado
swarm-hostCounts = { $managers ->
        [one] { NUM($managers) } host manager
       *[other] { NUM($managers) } hosts managers
    }, { $workers ->
        [one] { NUM($workers) } host worker
       *[other] { NUM($workers) } hosts workers
    }
swarm-host-managerOf = Manager de { $swarm }
swarm-host-workerOf = Worker de { $swarm }
swarm-role-manager = Manager
swarm-role-worker = Worker
swarm-tab-summary = Resumo
swarm-tab-nodes = Nós
swarm-tab-services = Serviços
swarm-tab-tasks = Tarefas
swarm-tab-stacks = Stacks
swarm-tab-secrets = Secrets
swarm-tab-configs = Configs
swarm-summary-title = Este Swarm
swarm-summary-managerHosts = Hosts managers
swarm-summary-workerHosts = Hosts workers
swarm-summary-nodes = Nós
swarm-summary-nodeCounts = { $nodes ->
        [one] { NUM($nodes) } nó
       *[other] { NUM($nodes) } nós
    }, { $managers ->
        [one] { NUM($managers) } manager
       *[other] { NUM($managers) } managers
    }
swarm-summary-services = Serviços
swarm-summary-created = Criado
swarm-summary-id = ID do Swarm
swarm-nodes-search = Pesquisar nós
swarm-nodes-column-hostname = Nó
swarm-nodes-column-role = Função
swarm-nodes-column-availability = Disponibilidade
swarm-nodes-column-state = Estado
swarm-nodes-column-engine = Engine
swarm-nodes-column-address = Endereço
swarm-nodes-column-labels = Rótulos
swarm-nodes-isHost = host { $name }
swarm-nodes-availability-active = Ativo
swarm-nodes-availability-pause = Pausado
swarm-nodes-availability-drain = Drenado
swarm-nodes-manager-leader = líder
swarm-nodes-manager-reachable = acessível
swarm-nodes-manager-unreachable = inacessível
swarm-nodes-actionsFor = Ações de { $name }
swarm-nodes-action-activate = Ativar
swarm-nodes-action-pause = Pausar
swarm-nodes-action-drain = Drenar
swarm-nodes-action-promote = Promover a manager
swarm-nodes-action-demote = Rebaixar a worker
swarm-nodes-action-labels = Editar rótulos
swarm-nodes-action-tasks = Mostrar as tarefas dele
swarm-nodes-updated = { $name } atualizado
swarm-nodes-removed = { $name } removido do Swarm
swarm-nodes-labels-title = Rótulos de { $name }
swarm-nodes-labels-description = Restrições de posicionamento como node.labels.zone==a escolhem nós por estes rótulos.
swarm-nodes-labels-key = Chave
swarm-nodes-labels-value = Valor
swarm-nodes-labels-add = Adicionar um rótulo
swarm-nodes-labels-removeOne = Remover o rótulo { $key }
swarm-nodes-remove-title = Remover { $name } do Swarm?
swarm-nodes-remove-ready = Este nó ainda está ativo. Removê-lo à força deixa o Engine dele acreditando que está no Swarm até que saia por conta própria; as tarefas dele são reagendadas em outro lugar.
swarm-nodes-remove-down = Este nó está fora do ar. As tarefas dele são reagendadas nos outros nós.
swarm-nodes-remove-force = Forçar: removê-lo mesmo estando ativo
swarm-services-search = Pesquisar serviços
swarm-services-empty = Nenhum serviço é executado neste Swarm
swarm-services-column-name = Nome
swarm-services-column-image = Imagem
swarm-services-column-mode = Modo
swarm-services-column-replicas = Em execução
swarm-services-column-ports = Portas
swarm-services-column-stack = Stack
swarm-services-column-update = Atualização
swarm-services-mode-replicated = Replicado
swarm-services-mode-replicatedCount = Replicado ({ $count })
swarm-services-mode-global = Global
swarm-services-mode-replicatedJob = Job
swarm-services-mode-globalJob = Job global
swarm-services-actionsFor = Ações de { $name }
swarm-services-action-logs = Logs
swarm-services-action-scale = Escalar
swarm-services-action-rollback = Reverter
swarm-services-scaled = { $name } escalado para { $count ->
        [one] { NUM($count) } réplica
       *[other] { NUM($count) } réplicas
    }
swarm-services-rolledBack = { $name } está sendo revertido
swarm-services-removed = { $name } removido
swarm-services-scale-title = Escalar { $name }
swarm-services-scale-description = O Swarm inicia ou para tarefas até que esta quantidade esteja em execução. 0 para o serviço sem removê-lo.
swarm-services-scale-replicas = Réplicas
swarm-services-scale-invalid = Digite um número inteiro de 0 a 10000.
swarm-services-rollback-title = Reverter { $name }?
swarm-services-rollback-description = As configurações anteriores dele (imagem, ambiente, réplicas…) são restauradas, e as tarefas dele passam para elas da mesma forma que em uma atualização.
swarm-services-remove-title = Remover o serviço { $name }?
swarm-services-remove-description = { $count ->
        [0] Ele não executa nenhuma tarefa agora.
        [one] A tarefa em execução dele é parada e removida.
       *[other] As { NUM($count) } tarefas em execução dele são paradas e removidas.
    } Os volumes dele são mantidos.
swarm-service-tab-tasks = Tarefas
swarm-service-tab-logs = Logs
swarm-service-tab-inspect = Inspecionar
swarm-service-gone = Este serviço não existe mais.
swarm-service-backToList = Voltar aos serviços
swarm-tasks-column-task = Tarefa
swarm-tasks-column-node = Nó
swarm-tasks-column-state = Estado
swarm-tasks-column-error = Mensagem
swarm-tasks-column-image = Imagem
swarm-tasks-desired = deveria estar { $state }
swarm-tasks-openContainer = Abrir contêiner
swarm-tasks-history = Mostrar histórico
swarm-tasks-onNode = Tarefas em { $name }
swarm-tasks-allNodes = Mostrar todos os nós
swarm-tasks-empty = Nenhuma tarefa
swarm-objects-searchSecrets = Pesquisar secrets
swarm-objects-searchConfigs = Pesquisar configs
swarm-objects-createSecret = Criar secret
swarm-objects-createConfig = Criar config
swarm-objects-secretNote = O valor de um secret vai para o Swarm uma vez e não pode ser lido de volta, nem pelo Homeport nem por ninguém: só os serviços que o recebem o veem.
swarm-objects-noSecrets = Nenhum secret
swarm-objects-noConfigs = Nenhum config
swarm-objects-column-name = Nome
swarm-objects-column-services = Usado por
swarm-objects-column-labels = Rótulos
swarm-objects-column-created = Criado
swarm-objects-column-updated = Atualizado
swarm-objects-unused = Não usado
swarm-objects-viewOne = Ver { $name }
swarm-objects-removeOne = Remover { $name }
swarm-objects-removed = { $name } removido
swarm-objects-remove-title = Remover { $name }?
swarm-objects-remove-used = O Swarm recusa enquanto serviços o usam: { $services }. Remova-o deles antes.
swarm-objects-remove-unused = Nenhum serviço o usa. Isso não pode ser desfeito.
swarm-objects-binary = Conteúdo binário, { $size ->
        [one] { NUM($size) } byte
       *[other] { NUM($size) } bytes
    }.
swarm-objects-create-secretTitle = Criar um secret
swarm-objects-create-configTitle = Criar um config
swarm-objects-create-secretDescription = O valor é enviado ao Swarm e nunca é guardado pelo Homeport. Os serviços o leem como um arquivo em /run/secrets/.
swarm-objects-create-configDescription = Os serviços o leem como um arquivo onde quer que o montem. Configs podem ser vistos de novo depois.
swarm-objects-create-name = Nome
swarm-objects-create-secretPlaceholder = db_password
swarm-objects-create-configPlaceholder = nginx_conf
swarm-objects-create-value = Valor
swarm-objects-create-content = Conteúdo
swarm-objects-create-show = Mostrar
swarm-objects-create-hide = Ocultar
swarm-objects-create-loadFile = Carregar de arquivo…
swarm-objects-create-pickFile = Escolha o arquivo a enviar
swarm-objects-create-clearFile = Digitar o valor em vez disso
swarm-objects-create-fileNote = O arquivo é lido quando você o cria, enviado ao Swarm e não é guardado.
swarm-services-created = { $name } criado
swarm-services-updating = { $name } está passando para as novas configurações
swarm-service-update = atualização { $state }
swarm-create-open = Criar serviço
swarm-create-title = Criar um serviço
swarm-create-description = O Swarm baixa a imagem nos nós que escolher e mantém esta quantidade de tarefas em execução. Posicionamento, limites de recursos e verificações de integridade podem ser definidos com a CLI do docker e são mostrados em Inspecionar.
swarm-create-image = Imagem
swarm-create-imagePlaceholder = nginx:1.27
swarm-create-name = Nome
swarm-create-mode = Modo
swarm-create-restart = Reiniciar
swarm-create-restart-default = Sempre (padrão)
swarm-create-restart-onFailure = Em caso de falha
swarm-create-restart-none = Nunca
swarm-create-command = Comando
swarm-create-commandPlaceholder = O comando da própria imagem
swarm-create-env = Ambiente
swarm-create-hostMode = Publicar somente no nó que executa cada tarefa, em vez de em todos os nós
swarm-create-networks = Redes overlay
swarm-create-secrets = Secrets
swarm-create-configs = Configs
swarm-create-mountedAt = Onde { $name } é montado
swarm-create-secretTarget = /run/secrets/{ $name }
swarm-create-configTarget = /{ $name }
swarm-create-submit = Criar
swarm-edit-open = Editar
swarm-edit-title = Editar { $name }
swarm-edit-description = Só o que você alterar aqui é enviado; o resto do serviço é mantido. O Swarm substitui as tarefas dele aos poucos com as novas configurações.
swarm-edit-redeploy = Reimplantar todas as tarefas, baixando a imagem de novo mesmo que a tag seja a mesma
swarm-edit-submit = Atualizar
swarm-membership-card-title = Swarm
swarm-membership-card-none = Este host não está em um Swarm.
swarm-membership-card-state = O nó dele está { $state }.
swarm-membership-card-ports = Inicie um Swarm aqui ou entre em um. Os nós de um Swarm precisam se alcançar em 2377/tcp, 7946/tcp e udp, e 4789/udp; o Homeport não pode abrir essas portas para você.
swarm-membership-created = Swarm criado
swarm-membership-joined = Entrou no Swarm
swarm-membership-left = Saiu do Swarm
swarm-membership-advertise = Endereço anunciado
swarm-membership-advertisePlaceholder = Escolhido pelo Engine
swarm-membership-advertiseHelp = O endereço pelo qual os outros nós alcançam este host. Deixe em branco para o Engine escolher, o que funciona quando a máquina tem um só endereço.
swarm-membership-listen = Endereço de escuta
swarm-membership-init-open = Iniciar um Swarm
swarm-membership-init-title = Iniciar um Swarm em { $name }
swarm-membership-init-description = Este host se torna o primeiro manager de um novo Swarm. Outros hosts podem então entrar nele como workers ou managers.
swarm-membership-init-submit = Iniciar o Swarm
swarm-membership-join-open = Entrar em um Swarm
swarm-membership-join-title = Colocar { $name } em um Swarm
swarm-membership-join-description = Escolha um Swarm que o Homeport conhece, e o Homeport busca o token de entrada dele em um manager conectado. Para outro Swarm, cole o token e o endereço de um manager obtidos com docker swarm join-token.
swarm-membership-join-swarm = Swarm
swarm-membership-join-paste = Outro Swarm, pelo token
swarm-membership-join-token = Token de entrada
swarm-membership-join-address = Endereço do manager
swarm-membership-join-addressPlaceholder = 192.168.1.10:2377
swarm-membership-join-as = Entrar como
swarm-membership-join-submit = Entrar
swarm-membership-leave-open = Sair do Swarm
swarm-membership-leave-title = Fazer { $name } sair do Swarm dele?
swarm-membership-leave-worker = As tarefas dele param aqui e são reagendadas nos outros nós. O nó dele continua listado como fora do ar até que um manager o remova.
swarm-membership-leave-manager = Ele deixa de ser manager e nó. Os outros managers mantêm o Swarm funcionando; rebaixá-lo antes é mais suave.
swarm-membership-leave-last = Este é o último manager do Swarm: o Swarm vai junto, com todos os serviços, secrets e configs. Os outros nós ficam em um Swarm sem manager.
swarm-membership-leave-force = Forçar: sair mesmo sendo manager
swarm-membership-leave-typeName = Digite { $name } para confirmar
swarm-membership-leave-submit = Sair
swarm-join-title = Comandos de entrada
swarm-join-description = Para máquinas que o Homeport não gerencia: execute um destes lá. Os hosts que o Homeport gerencia entram pela página deles, sem copiar nada.
swarm-join-reveal = Mostrar os comandos de entrada
swarm-join-hide = Ocultá-los
swarm-join-worker = Como worker
swarm-join-manager = Como manager
swarm-stacks-search = Pesquisar stacks
swarm-stacks-new = Nova stack
swarm-stacks-deploy = Implantar
swarm-stacks-redeploy = Reimplantar
swarm-stacks-adopt = Adotar
swarm-stacks-link = Vincular a uma pasta…
swarm-stacks-linkTitle = Escolha a pasta com os arquivos compose de { $name }
swarm-stacks-linked = { $name } agora é uma stack vinculada
swarm-stacks-empty = Nenhuma stack neste Swarm
swarm-stacks-emptyHint = As stacks implantadas com docker stack deploy aparecem aqui, assim como as que você implanta neste Swarm pelo Homeport.
swarm-stacks-backToList = Voltar às stacks
swarm-stacks-services = Serviços
swarm-stacks-noServices = Nada foi implantado ainda.
swarm-stacks-parts = Redes, secrets e configs
swarm-stacks-networks = Redes
swarm-stacks-noStartStop = Uma stack do swarm não pode ser parada ou iniciada como um todo: escale os serviços dela para 0 em vez disso, nas páginas deles.
swarm-stacks-removed = { $name } removida
swarm-stacks-removedPartly = { $name } removida, exceto:
swarm-stacks-remove-title = Remover a stack { $name }?
swarm-stacks-remove-description = Os serviços dela são removidos, depois as redes, os secrets e os configs, como faz o docker stack rm. Os volumes dela são mantidos em cada nó.
