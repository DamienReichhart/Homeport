### The `incus` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/incus.ts:
### change the text there, then run it again.

incus-delete = Excluir
incus-status-running = Em execução
incus-status-stopped = Parada
incus-status-frozen = Congelada
incus-status-starting = Iniciando
incus-status-stopping = Parando
incus-status-freezing = Congelando
incus-status-error = Erro
incus-status-unknown = Desconhecido
incus-type-container = Contêiner
incus-type-virtualMachine = Máquina virtual
incus-action-start = Iniciar
incus-action-stop = Parar
incus-action-restart = Reiniciar
incus-action-freeze = Congelar
incus-action-unfreeze = Descongelar
incus-action-forceStop = Forçar parada
incus-actionOne-start = Iniciar { $name }
incus-actionOne-stop = Parar { $name }
incus-actionOne-restart = Reiniciar { $name }
incus-actionOne-freeze = Congelar { $name }
incus-actionOne-unfreeze = Descongelar { $name }
incus-instances-nav = Instâncias
incus-instances-title = Instâncias
incus-instances-summary = { $count ->
        [one] { NUM($count) } instância
       *[other] { NUM($count) } instâncias
    }, { $running } em execução, no projeto { $project }.
incus-instances-search = Filtrar por nome ou descrição
incus-instances-filter-all = Todas
incus-instances-empty = Nenhuma instância em { $project }
incus-instances-emptyDescription = Os contêineres de sistema e as máquinas virtuais deste projeto aparecem aqui. Escolha outro projeto abaixo do seletor de hosts para ver outros.
incus-instances-more = Mais ações para { $name }
incus-instances-column-name = Nome
incus-instances-column-type = Tipo
incus-instances-column-status = Status
incus-instances-column-image = Imagem
incus-instances-column-addresses = Endereços
incus-instances-column-memory = Memória
incus-instances-column-created = Criada
incus-instances-done-start = { $name } iniciada
incus-instances-done-stop = { $name } parada
incus-instances-done-restart = { $name } reiniciada
incus-instances-done-freeze = { $name } congelada
incus-instances-done-unfreeze = { $name } descongelada
incus-instances-deleted = { $name } excluída
incus-instances-delete-title = Excluir { $name }?
incus-instances-delete-description = A instância e o disco dela são excluídos do projeto { $project }. Isso não pode ser desfeito.
incus-instances-delete-snapshots = { $count ->
        [one] O snapshot dela é excluído
       *[other] Os { NUM($count) } snapshots dela são excluídos
    } junto com ela.
incus-instances-delete-snapshotsNamed = { $count ->
        [one] O snapshot { $names } dela é excluído
       *[other] Os snapshots { $names } dela são excluídos
    } junto com ela.
incus-instances-delete-stopFirst = Pare-a antes: ela está em execução
incus-instances-delete-stopAndDelete = Parar e excluir
incus-instances-backToList = Voltar às instâncias
incus-instances-tab-overview = Visão geral
incus-instances-tab-console = Console
incus-instances-tab-exec = Exec
incus-instances-tab-snapshots = Snapshots
incus-instances-tab-config = Configuração
incus-instances-tab-devices = Dispositivos
incus-instances-tab-raw = Bruto
incus-instances-field-project = Projeto
incus-instances-field-architecture = Arquitetura
incus-instances-field-addresses = Endereços
incus-instances-field-cpu = Tempo de CPU
incus-instances-field-memory = Memória
incus-instances-field-processes = Processos
incus-instances-field-created = Criada
incus-instances-field-lastUsed = Último uso
incus-instances-field-snapshots = Snapshots
incus-instances-field-location = Membro do cluster
incus-instances-field-description = Descrição
incus-instances-cpuSeconds = { $seconds } s
incus-instances-profiles = Perfis
incus-instances-noConfig = Nenhuma configuração.
incus-instances-noDevices = Nenhum dispositivo.
incus-instances-config-key = Chave
incus-instances-config-value = Valor
incus-instances-config-fromProfile = De um perfil
incus-progress-waiting = Aguardando o Incus…
incus-remote-server = Servidor de imagens
incus-remote-search = Filtrar por sistema, versão ou alias
incus-remote-list = Imagens no servidor
incus-remote-none = Nenhuma imagem corresponde.
incus-remote-alias = Alias
incus-images-nav = Imagens
incus-images-title = Imagens
incus-images-summary = { $count ->
        [one] { NUM($count) } imagem
       *[other] { NUM($count) } imagens
    } no projeto { $project }.
incus-images-search = Filtrar por alias, impressão digital ou descrição
incus-images-empty = Nenhuma imagem em { $project }
incus-images-emptyDescription = As imagens são baixadas quando uma instância é criada a partir de uma, ou antecipadamente com Obter imagem.
incus-images-get = Obter imagem
incus-images-get-title = Obter uma imagem
incus-images-get-description = O Incus a baixa no projeto { $project } a partir do servidor de imagens, para que as instâncias iniciem a partir dela rapidamente.
incus-images-get-autoUpdate = Mantê-la atualizada a partir do servidor
incus-images-get-keepAlias = Dar a ela o mesmo alias aqui
incus-images-get-confirm = Obter imagem
incus-images-get-done = A imagem está pronta.
incus-images-copied = { $alias } copiada para { $project }
incus-images-column-aliases = Aliases
incus-images-column-description = Descrição
incus-images-column-type = Tipo
incus-images-column-arch = Arquitetura
incus-images-column-size = Tamanho
incus-images-column-uploaded = Adicionada
incus-images-cached = em cache
incus-images-cachedHint = Baixada para criar uma instância; o Incus a exclui quando nada a usa por um tempo.
incus-images-aliasAdd = Adicionar um alias a { $name }
incus-images-aliasRemove = Remover o alias { $name }
incus-images-aliasRemoved = Alias { $name } removido
incus-images-alias-title = Adicionar um alias a { $name }
incus-images-alias-name = Alias
incus-images-alias-add = Adicionar alias
incus-images-refreshOne = Atualizar { $name } a partir do servidor dela
incus-images-refreshed = { $name } atualizada
incus-images-deleteOne = Excluir { $name }
incus-images-deleted = { $name } excluída
incus-images-delete-title = Excluir a imagem { $name }?
incus-images-delete-description = As instâncias criadas a partir dela continuam funcionando: cada uma tem a própria cópia. As novas precisam que ela seja baixada de novo.
incus-profiles-nav = Perfis
incus-profiles-title = Perfis
incus-profiles-summary = { $count ->
        [one] { NUM($count) } perfil
       *[other] { NUM($count) } perfis
    } no projeto { $project }.
incus-profiles-search = Filtrar por nome ou descrição
incus-profiles-empty = Nenhum perfil em { $project }
incus-profiles-new = Novo perfil
incus-profiles-new-title = Novo perfil
incus-profiles-new-description = Um perfil vazio no projeto { $project }. Adicione a configuração e os dispositivos dele depois de criado.
incus-profiles-new-name = Nome
incus-profiles-new-descriptionField = Descrição
incus-profiles-created = Perfil { $name } criado
incus-profiles-column-name = Nome
incus-profiles-column-description = Descrição
incus-profiles-column-settings = Configurações
incus-profiles-column-usedBy = Usado por
incus-profiles-settings = { $config ->
        [one] { NUM($config) } chave
       *[other] { NUM($config) } chaves
    }, { $devices ->
        [one] { NUM($devices) } dispositivo
       *[other] { NUM($devices) } dispositivos
    }
incus-profiles-defaultKept = O Incus mantém o perfil default.
incus-profiles-inUse = Usado por { $names }: remova-o deles antes.
incus-profiles-deleteOne = Excluir { $name }
incus-profiles-deleted = Perfil { $name } excluído
incus-profiles-delete-title = Excluir o perfil { $name }?
incus-profiles-delete-description = Ele é excluído do projeto { $project }. Nenhuma instância o usa.
incus-profiles-backToList = Voltar aos perfis
incus-profiles-noDescription = Sem descrição
incus-profiles-reload = Recarregar
incus-profiles-saved = Perfil { $name } salvo
incus-profiles-usedBy = Usado por
incus-profiles-editHint = A descrição, a configuração e os dispositivos dele, como o incus profile edit os mostra. Salvar os aplica às instâncias que o usam, como o Incus faz; se alguém o alterou desde que foi carregado, o Homeport recusa em vez de sobrescrever.
incus-profiles-yamlOf = YAML de { $name }
incus-new-action = Nova instância
incus-new-title = Nova instância
incus-new-description = Um contêiner de sistema ou uma máquina virtual no projeto { $project }, a partir de uma imagem.
incus-new-name = Nome
incus-new-image = Imagem
incus-new-origin-remote = De um servidor de imagens
incus-new-origin-local = Das imagens deste host
incus-new-localImage = Imagem neste host
incus-new-localImagePlaceholder = Escolha uma imagem
incus-new-profiles = Perfis
incus-new-cpu = CPUs
incus-new-memory = Memória
incus-new-disk = Disco raiz
incus-new-limitsHint = Opcional: em branco, os perfis decidem. Os tamanhos são escritos como 512MiB ou 20GiB.
incus-new-start = Iniciá-la depois de criada
incus-new-confirm = Criar
incus-new-open = Abri-la
incus-new-made = { $name } criada
incus-new-done = { $name } está pronta.
incus-new-problem-name = Dê um nome com letras, dígitos e hifens, começando com uma letra, com até 63 caracteres.
incus-new-problem-image = Escolha uma imagem ou digite o alias dela.
incus-new-problem-cpu = CPUs é um número inteiro a partir de 1.
incus-new-problem-memory = A memória é escrita como 512MiB ou 2GiB.
incus-new-problem-disk = O disco raiz é escrito como 20GiB.
incus-exec-startFirst = Inicie a instância para executar comandos nela.
incus-exec-command = Comando
incus-exec-commandPlaceholder = Um shell (bash, senão sh)
incus-exec-user = ID do usuário
incus-exec-cwd = Pasta
incus-exec-open = Abrir
incus-exec-exited = O comando terminou.
incus-exec-exitedWithCode = O comando foi encerrado com o código { $code }.
incus-console-startFirst = Inicie a instância para se conectar ao console dela.
incus-console-hint = O console de texto dela, como o incus console o mostra: o prompt de login ou uma inicialização enquanto acontece. Sair da aba desconecta sem parar nada.
incus-console-reattach = Reconectar
incus-console-detached = Desconectado do console.
incus-snapshots-take = Criar snapshot
incus-snapshots-take-title = Criar um snapshot de { $name }
incus-snapshots-take-description = Uma cópia do disco dela como está agora, para restaurá-la depois: antes de uma atualização ou de uma alteração arriscada.
incus-snapshots-take-name = Nome
incus-snapshots-take-namePlaceholder = snap0, snap1… quando em branco
incus-snapshots-take-expiry = Manter
incus-snapshots-take-stateful = Incluir o estado de execução (memória)
incus-snapshots-expiry-never = Até ser excluído
incus-snapshots-expiry-1d = Um dia
incus-snapshots-expiry-7d = Uma semana
incus-snapshots-expiry-30d = 30 dias
incus-snapshots-taken = Snapshot de { $name } criado
incus-snapshots-empty = Nenhum snapshot
incus-snapshots-emptyDescription = Crie um antes de alterar algo que você talvez queira desfazer.
incus-snapshots-column-name = Nome
incus-snapshots-column-created = Criado
incus-snapshots-column-expires = Expira
incus-snapshots-column-size = Tamanho
incus-snapshots-stateful = com estado
incus-snapshots-restore = Restaurar
incus-snapshots-restore-title = Restaurar { $name } para { $snapshot }?
incus-snapshots-restore-description = Tudo o que mudou no disco dela desde { $time }, quando { $snapshot } foi criado, é perdido. Uma instância em execução é reiniciada.
incus-snapshots-restored = { $name } restaurada para { $snapshot }
incus-snapshots-deleteOne = Excluir { $snapshot }
incus-snapshots-delete-title = Excluir o snapshot { $snapshot }?
incus-snapshots-delete-description = A instância permanece como está; este snapshot não poderá mais ser restaurado.
incus-snapshots-deleted = Snapshot { $snapshot } excluído
incus-storage-nav = Armazenamento
incus-storage-title = Armazenamento
incus-storage-summary = { $count ->
        [one] { NUM($count) } pool de armazenamento
       *[other] { NUM($count) } pools de armazenamento
    }; volumes do projeto { $project }.
incus-storage-empty = Nenhum pool de armazenamento
incus-storage-emptyDescription = Crie um com incus storage create ou incus admin init.
incus-storage-usage = Espaço usado em { $pool }
incus-storage-space = { $used } de { $total } usados ({ $percent })
incus-storage-spaceUnknown = O driver dele não informa quão cheio está.
incus-storage-volumesOf = Volumes em { $pool }
incus-storage-noVolumes = Nenhum volume deste projeto no pool.
incus-storage-new = Novo volume
incus-storage-new-title = Novo volume em { $pool }
incus-storage-new-description = Um volume personalizado do projeto { $project }, para anexar a instâncias, para dados que sobrevivem a elas.
incus-storage-new-name = Nome
incus-storage-new-size = Tamanho
incus-storage-new-content = Conteúdo
incus-storage-content-filesystem = Sistema de arquivos
incus-storage-content-block = Dispositivo de bloco
incus-storage-created = Volume { $name } criado
incus-storage-column-name = Nome
incus-storage-column-type = Tipo
incus-storage-column-content = Conteúdo
incus-storage-column-size = Tamanho
incus-storage-column-usedBy = Usado por
incus-storage-inUse = Usado por { $names }: desanexe-o antes.
incus-storage-deleteOne = Excluir { $name }
incus-storage-delete-title = Excluir o volume { $name }?
incus-storage-delete-description = Os dados dele são excluídos de { $pool }. Nada o usa.
incus-storage-deleted = Volume { $name } excluído
incus-networks-nav = Redes
incus-networks-title = Redes
incus-networks-summary = { $count ->
        [one] { NUM($count) } rede
       *[other] { NUM($count) } redes
    } que o projeto { $project } vê.
incus-networks-search = Filtrar por nome ou tipo
incus-networks-empty = Nenhuma rede
incus-networks-new = Nova bridge
incus-networks-new-title = Nova bridge
incus-networks-new-description = Uma bridge gerenciada: o Incus atribui a ela endereços, DHCP e DNS, para que as instâncias entrem nela por um perfil ou um dispositivo.
incus-networks-new-name = Nome
incus-networks-new-ipv4 = IPv4
incus-networks-new-ipv6 = IPv6
incus-networks-new-addressHint = auto escolhe uma sub-rede livre, none a desativa; ou informe o endereço da bridge com o prefixo, como 10.0.5.1/24.
incus-networks-new-nat = Mascarar o tráfego IPv4 de saída (NAT)
incus-networks-created = Bridge { $name } criada
incus-networks-column-name = Nome
incus-networks-column-type = Tipo
incus-networks-column-ipv4 = IPv4
incus-networks-column-ipv6 = IPv6
incus-networks-column-usedBy = Usada por
incus-networks-managed = Gerenciada pelo Incus
incus-networks-unmanaged = Desta máquina
incus-networks-inUse = Usada por { $names }: remova-a deles antes.
incus-networks-deleteOne = Excluir { $name }
incus-networks-delete-title = Excluir a rede { $name }?
incus-networks-delete-description = O Incus remove a bridge e o DHCP e o DNS dela. Nada a usa.
incus-networks-deleted = Rede { $name } excluída
incus-networks-gone = Não há mais nenhuma rede { $name }.
incus-networks-backToList = Voltar às redes
incus-networks-field-managed = Criada por
incus-networks-field-state = Estado
incus-networks-field-addresses = Endereços no host
incus-networks-field-ipv4 = Configuração IPv4
incus-networks-field-ipv6 = Configuração IPv6
incus-networks-field-nat = NAT
incus-networks-field-mtu = MTU
incus-networks-field-hwaddr = Endereço MAC
incus-networks-yes = Sim
incus-networks-no = Não
incus-networks-usedBy = Usada por
incus-networks-leases = Concessões DHCP
incus-networks-noLeases = Nenhuma concessão.
incus-networks-lease-hostname = Nome do host
incus-networks-lease-address = Endereço
incus-networks-lease-hwaddr = Endereço MAC
incus-networks-lease-type = Tipo
