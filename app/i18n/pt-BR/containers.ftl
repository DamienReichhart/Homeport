### The `containers` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/containers.ts:
### change the text there, then run it again.

containers-nav = Contêineres
containers-title = Contêineres
containers-summary = { $running } em execução de { $count }
containers-runContainer = Executar contêiner
containers-search = Pesquisar contêineres
containers-onlyRunning = Somente em execução
containers-empty-title = Nenhum contêiner
containers-empty-description = Execute um a partir de uma imagem ou implante uma stack.
containers-noMatch = Nenhum contêiner corresponde.
containers-column-name = Nome
containers-column-image = Imagem
containers-column-status = Status
containers-column-ports = Portas
containers-column-cpu = CPU
containers-column-memory = Memória
containers-column-created = Criado
containers-selectOne = Selecionar { $name }
containers-selectStack = Selecionar a stack { $stack }
containers-stack-running = { $running }/{ $count } em execução
containers-stack-swarm = Swarm
containers-stack-open = Abrir stack
containers-inPod = No pod do Podman { $pod }
containers-fromUnit = Criado pela unidade Quadlet { $unit }
containers-selected = { $count } selecionados
containers-action-start = Iniciar
containers-action-stop = Parar
containers-action-restart = Reiniciar
containers-action-pause = Pausar
containers-action-resume = Retomar
containers-action-kill = Encerrar à força
containers-action-rename = Renomear
containers-action-edit = Editar
containers-action-duplicate = Duplicar
containers-action-logs = Logs
containers-action-terminal = Abrir terminal
containers-action-commit = Fazer commit em imagem…
containers-action-export = Exportar sistema de arquivos…
containers-kill-title = Encerrar { $name } à força?
containers-kill-description = O processo principal recebe SIGKILL imediatamente, sem chance de encerrar de forma limpa.
containers-done-start = { $count ->
        [one] Contêiner iniciado
       *[other] { NUM($count) } contêineres iniciados
    }
containers-done-stop = { $count ->
        [one] Contêiner parado
       *[other] { NUM($count) } contêineres parados
    }
containers-done-restart = { $count ->
        [one] Contêiner reiniciado
       *[other] { NUM($count) } contêineres reiniciados
    }
containers-done-pause = { $count ->
        [one] Contêiner pausado
       *[other] { NUM($count) } contêineres pausados
    }
containers-done-unpause = { $count ->
        [one] Contêiner retomado
       *[other] { NUM($count) } contêineres retomados
    }
containers-done-kill = { $count ->
        [one] Contêiner encerrado à força
       *[other] { NUM($count) } contêineres encerrados à força
    }
containers-done-remove = { $count ->
        [one] Contêiner removido
       *[other] { NUM($count) } contêineres removidos
    }
containers-remove-title = Remover { $name }?
containers-remove-titleMany = Remover { $count ->
        [one] { NUM($count) } contêiner
       *[other] { NUM($count) } contêineres
    }?
containers-remove-deleted = { $count ->
        [one] O contêiner, a camada gravável e os volumes anônimos dele são excluídos.
       *[other] Os contêineres, a camada gravável e os volumes anônimos deles são excluídos.
    }
containers-remove-running = { $count } em execução serão parados antes.
containers-remove-namedKept = Os volumes nomeados são mantidos.
containers-detail-gone = Este contêiner não existe mais.
containers-detail-ambiguous = O ID de mais de um contêiner começa com { $id }.
containers-detail-backToList = Voltar aos contêineres
containers-tab-logs = Logs
containers-tab-inspect = Inspecionar
containers-tab-health = Integridade
containers-tab-exec = Exec
containers-tab-files = Arquivos
containers-tab-stats = Estatísticas
containers-tab-processes = Processos
containers-notRunning = O contêiner não está em execução.
containers-logs-tail-100 = Últimas 100
containers-logs-tail-1000 = Últimas 1 000
containers-logs-tail-10000 = Últimas 10 000
containers-logs-tail-all = Tudo
containers-logs-since-all = Qualquer momento
containers-logs-since-300 = Últimos 5 min
containers-logs-since-3600 = Última hora
containers-logs-since-86400 = Últimas 24 h
containers-logs-history = Histórico
containers-logs-sinceLabel = Desde
containers-logs-timestamps = Carimbos de data/hora
containers-logs-follow = Acompanhar
containers-logs-find = Localizar nos logs
containers-logs-previous = Correspondência anterior
containers-logs-next = Próxima correspondência
containers-logs-clear = Limpar
containers-logs-download = Baixar logs
containers-logs-saveTitle = Salvar logs
containers-logs-saved = Logs salvos
containers-logs-saving = Salvando logs… { $percent }
containers-logs-stdout = stdout
containers-logs-stderr = stderr
containers-logs-end = — fim dos logs —
containers-inspect-raw = JSON bruto
containers-inspect-container = Contêiner
containers-inspect-health = Integridade
containers-inspect-environment = Ambiente
containers-inspect-mounts = Montagens
containers-inspect-ports = Portas
containers-inspect-networks = Redes
containers-inspect-labels = Rótulos
containers-inspect-id = ID
containers-inspect-image = Imagem
containers-inspect-created = Criado
containers-inspect-state = Estado
containers-inspect-started = Iniciado
containers-inspect-entrypoint = Entrypoint
containers-inspect-command = Comando
containers-inspect-workingDir = Diretório de trabalho
containers-inspect-user = Usuário
containers-inspect-restartPolicy = Política de reinício
containers-inspect-networkMode = Modo de rede
containers-inspect-exited = { $status } (saída { $code })
containers-inspect-outOfMemory = { $state }, sem memória
containers-inspect-restarted = { $policy } · reiniciado { $count }×
containers-inspect-restartedMax = { $policy } (máx. { $max }) · reiniciado { $count }×
containers-inspect-imageDefault = padrão da imagem
containers-inspect-notPublished = não publicada
containers-inspect-readWrite = leitura e gravação
containers-inspect-readOnly = somente leitura
containers-inspect-failingStreak = { $count } falhas seguidas
containers-inspect-noHealthcheck = A imagem não define nenhuma verificação de integridade.
containers-inspect-column-started = Iniciado
containers-inspect-column-exit = Saída
containers-inspect-column-output = Resultado
containers-inspect-column-variable = Variável
containers-inspect-column-value = Valor
containers-inspect-column-type = Tipo
containers-inspect-column-source = Origem
containers-inspect-column-destination = Destino
containers-inspect-column-mode = Modo
containers-inspect-column-container = Contêiner
containers-inspect-column-publishedOn = Publicada em
containers-inspect-column-network = Rede
containers-inspect-column-ip = IP
containers-inspect-column-gateway = Gateway
containers-inspect-column-aliases = Aliases
containers-inspect-column-label = Rótulo
containers-health-none = Este contêiner não tem verificação de integridade.
containers-health-title = Verificação de integridade
containers-health-command = Comando
containers-health-timingLabel = Tempos
containers-health-timing = a cada { $interval }, tempo limite { $timeout }, { $retries ->
        [one] { NUM($retries) } tentativa
       *[other] { NUM($retries) } tentativas
    }, período inicial { $start }
containers-health-failingStreak = Falhas seguidas
containers-health-lastResults = Últimos resultados
containers-health-noRuns = Nenhuma verificação foi executada ainda.
containers-health-noOutput = Sem saída.
containers-health-showLess = Mostrar menos
containers-health-showMore = Mostrar { $count ->
        [one] mais { NUM($count) } linha
       *[other] mais { NUM($count) } linhas
    }
containers-health-exit = saída { $code }
containers-health-took = · levou { $duration }
containers-health-milliseconds = { $duration } ms
containers-health-seconds = { $duration } s
containers-exec-startFirst = Inicie o contêiner para abrir um terminal.
containers-exec-shell = Shell
containers-exec-shellNumber = Shell { $number }
containers-exec-closeOne = Fechar { $title }
containers-exec-new = Novo terminal
containers-exec-command = Comando
containers-exec-commandPlaceholder = bash, ou sh quando não houver
containers-exec-user = Usuário
containers-exec-directory = Diretório
containers-exec-default = padrão
containers-exec-open = Abrir
containers-exec-none = Nenhum terminal aberto.
containers-exec-exited = [processo encerrado]
containers-exec-exitedWithCode = [processo encerrado com o código { $code }]
containers-files-startFirst = Inicie o contêiner para navegar pelos arquivos dele.
containers-files-saveOne = Salvar { $name }
containers-files-uploadTo = Enviar para { $path }
containers-files-uploadHere = Enviar para cá
containers-files-saved = { $name } salvo
containers-files-savedArchive = { $name } salvo como arquivo .tar
containers-files-downloading = Salvando { $name }
containers-files-uploading = Enviando para { $path }: { $done } de { $count }
containers-files-uploaded = { $count ->
        [one] { NUM($count) } arquivo enviado
       *[other] { NUM($count) } arquivos enviados
    } para { $path }
containers-stats-startFirst = Inicie o contêiner para ver o uso de recursos dele.
containers-stats-cpu = CPU
containers-stats-memory = Memória
containers-stats-memoryNow = · { $used } de { $limit }
containers-stats-used = Usada
containers-stats-network = Rede
containers-stats-networkNow = · { $received } de entrada, { $sent } de saída
containers-stats-received = Recebido
containers-stats-sent = Enviado
containers-stats-disk = E/S de disco
containers-stats-processes = · { $count ->
        [one] { NUM($count) } processo
       *[other] { NUM($count) } processos
    }
containers-stats-read = Lido
containers-stats-written = Gravado
containers-run-title = Executar um contêiner
containers-run-description = A imagem é baixada antes se este host não a tiver.
containers-run-submit = Executar
containers-run-starting = Iniciando…
containers-run-started = Contêiner iniciado
containers-run-image = Imagem
containers-run-name = Nome
containers-run-namePlaceholder = Gerado se estiver em branco
containers-run-nameKept = Mantido. Use Renomear para alterá-lo.
containers-run-command = Comando
containers-run-commandPlaceholder = O padrão da imagem
containers-run-environment = Ambiente
containers-run-network = Rede
containers-run-pod = Pod
containers-run-noPod = Nenhum
containers-run-podOwnsNetwork = Os contêineres em { $pod } compartilham a rede dele: as portas, a rede, o nome do host e o DNS são os do pod.
containers-run-defaultNetwork = Padrão (bridge)
containers-run-restart = Reinício
containers-run-restartPolicy = Política de reinício
containers-run-restart-no = Nunca
containers-run-restart-onFailure = Em caso de falha
containers-run-restart-unlessStopped = A menos que seja parado
containers-run-restart-always = Sempre
containers-run-autoRemove = Remover o contêiner quando ele for encerrado
containers-run-ports = Portas
containers-run-hostPort = Porta do host
containers-run-hostPortPlaceholder = Porta do host (qualquer)
containers-run-containerPort = Porta do contêiner
containers-run-protocol = Protocolo
containers-run-removePort = Remover porta
containers-run-addPort = Adicionar porta
containers-run-volumes = Volumes
containers-run-source = Origem
containers-run-sourcePlaceholder = Nome do volume ou /caminho/no/host
containers-run-containerPath = Caminho no contêiner
containers-run-readOnly = somente leitura
containers-run-removeVolume = Remover volume
containers-run-addVolume = Adicionar volume
containers-run-removeValue = Remover { $value }
containers-run-error-size = Escreva um tamanho como 512m ou 2g.
containers-run-error-cpus = Escreva um número de CPUs como 0.5 ou 2.
containers-run-error-cpuShares = Escreva um número inteiro como 512.
containers-run-error-memorySwap = Escreva um tamanho como 1g, ou -1 para swap ilimitado.
containers-run-error-gpuCount = Escreva quantas GPUs, como 1.
containers-run-error-duration = Escreva uma duração como 30s ou 1m30s.
containers-run-error-retries = Escreva um número inteiro como 3.
containers-advanced-title = Opções avançadas
containers-advanced-hasErrors = Contém erros
containers-advanced-resources = Recursos
containers-advanced-labels = Rótulos
containers-advanced-security = Segurança
containers-advanced-devices = Dispositivos
containers-advanced-healthcheck = Verificação de integridade
containers-advanced-network = Identidade de rede
containers-advanced-logging = Logs
containers-advanced-cpus = Limite de CPU
containers-advanced-cpusHint = CPUs, como 0.5 ou 2.
containers-advanced-cpusPlaceholder = Todas as CPUs
containers-advanced-cpuShares = CPU shares
containers-advanced-cpuSharesHint = Peso em relação aos outros contêineres; 1024 é o padrão.
containers-advanced-memory = Limite de memória
containers-advanced-memoryHint = Como 512m ou 2g.
containers-advanced-memoryPlaceholder = Sem limite
containers-advanced-memoryReservation = Reserva de memória
containers-advanced-memoryReservationHint = Um limite flexível, respeitado quando falta memória no host.
containers-advanced-memorySwap = Limite de memória e swap
containers-advanced-memorySwapHint = Memória mais swap; -1 para swap ilimitado.
containers-advanced-memorySwapPlaceholder = O dobro do limite de memória
containers-advanced-none = Nenhum
containers-advanced-labelsHint = Um chave=valor por linha, como proxies reversos e ferramentas de backup os leem.
containers-advanced-capAdd = Adicionar capabilities
containers-advanced-capAddHint = Como NET_ADMIN para uma VPN.
containers-advanced-capDrop = Remover capabilities
containers-advanced-capDropHint = ALL remove todas.
containers-advanced-privileged = Privilegiado
containers-advanced-privilegedTitle = O contêiner recebe acesso total ao host
containers-advanced-privilegedWarning = Ele pode usar todos os dispositivos e capabilities, e qualquer coisa executada nele pode assumir o controle do host. Prefira adicionar as capabilities e os dispositivos de que ele precisa.
containers-advanced-hostDevice = Dispositivo do host
containers-advanced-containerDevice = Dispositivo do contêiner
containers-advanced-samePath = Mesmo caminho
containers-advanced-permissions = Permissões
containers-advanced-removeDevice = Remover dispositivo
containers-advanced-addDevice = Adicionar dispositivo
containers-advanced-gpus = GPUs
containers-advanced-gpusHint = Requer o container toolkit do fabricante da GPU instalado no host.
containers-advanced-gpus-all = Todas as GPUs
containers-advanced-gpus-count = Um número de GPUs
containers-advanced-gpus-ids = GPUs escolhidas
containers-advanced-gpuCount = Quantas
containers-advanced-gpuIds = Índices ou UUIDs das GPUs
containers-advanced-gpuIdsHint = Separados por vírgulas.
containers-advanced-health-image = A da própria imagem, se houver
containers-advanced-health-custom = Um comando
containers-advanced-health-disabled = Nenhuma (desativar a da imagem)
containers-advanced-healthCommand = Comando de verificação
containers-advanced-healthCommandHint = Executado com /bin/sh -c; código de saída 0 significa íntegro.
containers-advanced-interval = Intervalo
containers-advanced-timeout = Tempo limite
containers-advanced-retries = Tentativas
containers-advanced-startPeriod = Período inicial
containers-advanced-hostname = Nome do host
containers-advanced-hostnamePlaceholder = O ID do contêiner
containers-advanced-domainname = Nome de domínio
containers-advanced-dns = Servidores DNS
containers-advanced-dnsHint = Um endereço por linha.
containers-advanced-dnsSearch = Domínios de pesquisa DNS
containers-advanced-dnsSearchHint = Um por linha.
containers-advanced-extraHosts = Hosts extras
containers-advanced-extraHostsHint = Um nome:endereço por linha; host-gateway representa o endereço do host.
containers-advanced-logDriver = Driver de log
containers-advanced-logDriverHint = Com none, a aba Logs fica vazia.
containers-advanced-logDriverPlaceholder = O padrão do Engine
containers-advanced-logOptions = Opções do driver
containers-advanced-logOptionsHint = Um chave=valor por linha; max-size e max-file fazem a rotação dos logs json-file.
containers-rename-title = Renomear { $name }
containers-rename-newName = Novo nome
containers-duplicate-title = Duplicar { $name }
containers-duplicate-sharedPorts = { $name } já publica { $count ->
        [one] a porta do host
       *[other] as portas do host
    } { $ports }.
containers-duplicate-sharedPortsFix = { $count ->
        [one] A cópia não pode iniciar enquanto os dois a usam
       *[other] A cópia não pode iniciar enquanto os dois as usam
    }: altere a porta do host ou deixe-a em branco para o Engine escolher uma.
containers-edit-title = Editar { $name }
containers-edit-description = Aplicar recria o contêiner com estas configurações, mantendo o nome, as redes e os volumes nomeados dele.
containers-edit-apply = Aplicar
containers-edit-recreating = Recriando…
containers-edit-recreated = Contêiner recriado
containers-edit-confirmTitle = Recriar { $name }?
containers-edit-nothingLost = Nada mais é perdido: o nome, as redes e os volumes nomeados dele são mantidos.
containers-edit-keptBut = O nome, as redes e os volumes nomeados dele são mantidos, mas não:
containers-edit-commitFirst = Fazer commit em uma imagem antes…
containers-edit-recreate = Recriar
containers-loss-anonymousVolume = O volume anônimo em { $path }: o novo contêiner começa com um vazio. O antigo é mantido como volume não usado.
containers-loss-filesystemChanges = { $count ->
        [one] { NUM($count) } arquivo foi alterado
       *[other] { NUM($count) } arquivos foram alterados
    } dentro do contêiner desde que ele foi criado. Faça commit dele em uma imagem antes para mantê-los.
containers-loss-downtime = Ele para enquanto os dois são trocados, normalmente por alguns segundos.
containers-loss-stackRedeploy = Ele pertence à stack { $stack }: a próxima implantação da stack o recria a partir do arquivo compose dela, sem esta edição.
containers-commit-title = Fazer commit de { $name } em uma imagem
containers-commit-description = A nova imagem contém os arquivos do contêiner como estão agora, incluindo as alterações feitas dentro dele, e as configurações dele. Os volumes não são incluídos.
containers-commit-imageName = Nome da imagem
containers-commit-imageNameHint = Como repo:tag.
containers-commit-nameTaken = Já existe uma imagem chamada { $name }. O nome passa para a nova imagem; a antiga mantém os outros nomes dela ou fica sem tag.
containers-commit-message = Mensagem
containers-commit-messagePlaceholder = Antes da atualização
containers-commit-author = Autor
containers-commit-changes = Alterações
containers-commit-changesHint = Instruções de Dockerfile, uma por linha: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME ou WORKDIR.
containers-commit-pause = Pausar o contêiner durante o commit
containers-commit-pauseOn = Os processos dele ficam congelados pelos poucos segundos que o commit leva, para que os arquivos sendo gravados fiquem consistentes.
containers-commit-pauseOff = Ele continua em execução, então os arquivos sendo gravados podem ser pegos pela metade.
containers-commit-submit = Fazer commit
containers-commit-committing = Fazendo commit…
containers-commit-done = Commit feito em { $reference }
containers-commit-openImages = Abrir imagens
containers-export-tarArchive = Arquivo tar
containers-export-pick = Exportar o sistema de arquivos de { $name }
containers-export-title = Exportando { $name }
containers-export-to = Para { $path }
containers-export-written = Gravado
containers-export-finished = Concluído. Importe-o na página Imagens ou com docker import.
containers-export-done = Exportado para { $destination }
containers-palette-run = Executar um contêiner
