### The `volumes` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/volumes.ts:
### change the text there, then run it again.

volumes-nav = Volumes
volumes-title = Volumes
volumes-summary = { $count ->
        [one] { NUM($count) } volume
       *[other] { NUM($count) } volumes
    } · { $unused } não usados
volumes-cleanUp = Limpar
volumes-cleanUp-anonymous = Volumes anônimos não usados
volumes-cleanUp-all = Todos os volumes não usados
volumes-create = Criar volume
volumes-search = Pesquisar volumes
volumes-empty-title = Nenhum volume
volumes-empty-description = Volumes mantêm os dados dos contêineres entre reinicializações e recriações.
volumes-column-name = Nome
volumes-column-stack = Stack
volumes-column-driver = Driver
volumes-column-size = Tamanho
volumes-size-label = Tamanho:
volumes-size-measuring = Medindo
volumes-size-unknown = O driver deste volume não informa o tamanho dele.
volumes-size-refresh = Atualizar tamanhos
volumes-column-usedBy = Usado por
volumes-column-created = Criado
volumes-unused = Não usado
volumes-inspectOne = Inspecionar { $name }
volumes-deleteOne = Excluir { $name }
volumes-browseOne = Navegar pelos arquivos de { $name }
volumes-browse-title = Arquivos de { $name }
volumes-browse-description = Somente leitura: nada aqui pode alterar o volume, mesmo enquanto contêineres o usam.
volumes-browse-saveOne = Salvar { $name }
volumes-browse-saved = { $name } salvo
volumes-browse-savedArchive = { $name } salvo como arquivo .tar
volumes-helper-needed = O Homeport abre volumes com um contêiner auxiliar de curta duração, que precisa da imagem { $image } (cerca de { $size }). Este host não a tem.
volumes-helper-removed = O Homeport só a baixa se você concordar, e a remove de novo quando terminar, a menos que outra coisa a use.
volumes-helper-pull = Baixar { $image }
volumes-helper-pulling = Baixando { $image }…
volumes-helper-progress = Progresso do download
volumes-moreActions = Mais ações para { $name }
volumes-backupMenu = Fazer backup em um arquivo…
volumes-restoreMenu = Restaurar de um arquivo…
volumes-cloneMenu = Clonar…
volumes-clone-title = Clonar { $volume }
volumes-clone-description = Copia os arquivos dele, com seus proprietários e permissões, para um novo volume neste host. A cópia não pertence a nenhuma stack.
volumes-clone-name = Nome da cópia
volumes-clone-start = Clonar
volumes-clone-into = Para { $target }
volumes-clone-copying = Copiando…
volumes-clone-done = { $source } clonado para { $target }
volumes-restoreFromFile = Restaurar de arquivo
volumes-backup-title = Fazer backup de { $volume }
volumes-backup-saveAs = Fazer backup de { $volume } em
volumes-backup-to = Em { $path }
volumes-backup-done = Backup de { $volume } feito em { $path }
volumes-restore-pick = Restaurar um volume de
volumes-restore-title = Restaurar um volume
volumes-restore-titleInto = Restaurar { $volume }
volumes-restore-from = De { $path }
volumes-restore-into = Restaurar em
volumes-restore-newVolume = Um novo volume
volumes-restore-existingVolume = Um volume existente
volumes-restore-volume = Volume
volumes-restore-chooseVolume = Escolha um volume
volumes-restore-nameTaken = Já existe um volume chamado { $volume }.
volumes-restore-start = Restaurar
volumes-restore-done = { $volume } restaurado
volumes-restore-cancelled = A restauração de { $volume } foi cancelada; o volume pode estar parcialmente gravado.
volumes-transfer-written = Gravado
volumes-transfer-sent = Enviado
volumes-transfer-archives = Arquivos tar
volumes-flow-running-backup = { $containers } { $count ->
        [one] está
       *[other] estão
    } em execução e { $count ->
        [one] usa
       *[other] usam
    } { $volume }. Um backup feito enquanto gravam nele pode ficar inconsistente, como um banco de dados pego no meio de uma gravação.
volumes-flow-running-restore = { $containers } { $count ->
        [one] está
       *[other] estão
    } em execução e { $count ->
        [one] usa
       *[other] usam
    } { $volume }. O Homeport { $count ->
        [one] o para
       *[other] os para
    } durante a restauração e { $count ->
        [one] o inicia
       *[other] os inicia
    } novamente depois.
volumes-flow-running-clone = { $containers } { $count ->
        [one] está
       *[other] estão
    } em execução e { $count ->
        [one] usa
       *[other] usam
    } { $volume }. Uma cópia feita enquanto gravam nele pode ficar inconsistente, como um banco de dados pego no meio de uma gravação.
volumes-flow-anyway-backup = Fazer backup mesmo assim
volumes-flow-anyway-clone = Copiar mesmo assim
volumes-flow-stopDuring-backup = { $count ->
        [one] Pará-lo
       *[other] Pará-los
    } durante o backup
volumes-flow-stopDuring-restore = Parar e restaurar
volumes-flow-stopDuring-clone = { $count ->
        [one] Pará-lo
       *[other] Pará-los
    } durante a cópia
volumes-flow-replace = { $volume } já contém arquivos. Restaurar os exclui e coloca os do backup no lugar deles.
volumes-flow-replaceConfirm = Substituir o conteúdo
volumes-usedBy = Usado por { $containers }.
volumes-notUsed = Nenhum contêiner usa este volume.
volumes-createDialog-title = Criar um volume
volumes-createDialog-name = Nome
volumes-createDialog-driver = Driver
volumes-deleted = { $name } excluído
volumes-pruned = { $count ->
        [one] { NUM($count) } volume excluído
       *[other] { NUM($count) } volumes excluídos
    }, { $space } recuperados
volumes-remove-title = Excluir o volume { $name }?
volumes-remove-warning = <strong>Todos os dados deste volume são excluídos permanentemente.</strong>
volumes-remove-mounted = {" "}Ele está montado por { $containers }; remova esses contêineres antes.
volumes-remove-confirm = Excluir volume e dados
volumes-prune-allTitle = Excluir todos os volumes não usados?
volumes-prune-anonymousTitle = Excluir volumes anônimos não usados?
volumes-prune-allWarning = Os dados de todos os volumes que nenhum contêiner usa são excluídos permanentemente, inclusive os volumes nomeados.
volumes-prune-anonymousWarning = Os dados dos volumes sem nome que nenhum contêiner usa são excluídos permanentemente.
volumes-prune-confirm = Excluir volumes e dados
