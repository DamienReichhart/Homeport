### The `podmanpods` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/podmanpods.ts:
### change the text there, then run it again.

podmanpods-nav = Pods
podmanpods-title = Pods
podmanpods-summary = { $count ->
        [one] { NUM($count) } pod
       *[other] { NUM($count) } pods
    }
podmanpods-search = Pesquisar pods
podmanpods-empty = Nenhum pod
podmanpods-emptyDescription = Um pod agrupa contêineres que compartilham uma rede e iniciam e param juntos. Crie um e depois execute contêineres nele.
podmanpods-new = Novo pod
podmanpods-column-name = Nome
podmanpods-column-status = Status
podmanpods-column-containers = Contêineres
podmanpods-column-networks = Redes
podmanpods-column-created = Criado
podmanpods-running = { $running } de { $count } em execução
podmanpods-status-running = Em execução
podmanpods-status-degraded = Parcialmente em execução
podmanpods-status-paused = Pausado
podmanpods-status-created = Criado
podmanpods-status-stopped = Parado
podmanpods-status-failed = Falhou
podmanpods-action-start = Iniciar
podmanpods-action-stop = Parar
podmanpods-action-restart = Reiniciar
podmanpods-action-pause = Pausar
podmanpods-action-unpause = Retomar
podmanpods-actionOne-start = Iniciar { $name }
podmanpods-actionOne-stop = Parar { $name }
podmanpods-actionOne-restart = Reiniciar { $name }
podmanpods-actionOne-pause = Pausar { $name }
podmanpods-actionOne-unpause = Retomar { $name }
podmanpods-removeOne = Remover { $name }
podmanpods-done-start = { $name } iniciado
podmanpods-done-stop = { $name } parado
podmanpods-done-restart = { $name } reiniciado
podmanpods-done-pause = { $name } pausado
podmanpods-done-unpause = { $name } retomado
podmanpods-removed = { $name } removido, { $count ->
        [0] sem contêineres
        [one] com { NUM($count) } contêiner
       *[other] com { NUM($count) } contêineres
    }
podmanpods-created = { $name } criado
podmanpods-remove-title = Remover { $name }?
podmanpods-remove-stopped = { $count ->
        [0] O pod e seu contêiner infra são removidos.
        [one] O pod, seu contêiner infra e seu { NUM($count) } contêiner são removidos.
       *[other] O pod, seu contêiner infra e seus { NUM($count) } contêineres são removidos.
    } Os volumes deles são mantidos.
podmanpods-remove-running = { $count ->
        [0] O pod está em execução: ele é parado e depois removido com seu contêiner infra.
        [one] O pod está em execução: seu { NUM($count) } contêiner é parado e depois removido com o pod e seu contêiner infra.
       *[other] O pod está em execução: seus { NUM($count) } contêineres são parados e depois removidos com o pod e seu contêiner infra.
    } Os volumes deles são mantidos.
podmanpods-remove-stopAndRemove = Parar e remover
podmanpods-detail-containers = { $count ->
        [0] Contêineres
        [one] { NUM($count) } contêiner
       *[other] { NUM($count) } contêineres
    }
podmanpods-detail-noContainers = Nada é executado neste pod ainda além do seu contêiner infra.
podmanpods-detail-infra = Infra
podmanpods-detail-settings = Configurações
podmanpods-detail-ports = Portas publicadas
podmanpods-detail-noPorts = Nenhuma
podmanpods-detail-shared = Namespaces compartilhados
podmanpods-detail-networks = Redes
podmanpods-detail-hostname = Nome do host
podmanpods-detail-created = Criado
podmanpods-detail-labels = Rótulos
podmanpods-detail-command = Criado com
podmanpods-detail-runContainer = Executar contêiner
podmanpods-detail-runContainerIn = Executar um contêiner em { $name }
podmanpods-newDialog-title = Novo pod
podmanpods-newDialog-description = Seus contêineres compartilham uma rede, então as portas são publicadas no pod, para todos eles.
podmanpods-newDialog-name = Nome
podmanpods-newDialog-portsHint = Deixe a porta do host em branco para o Podman escolher uma livre. Contêineres executados no pod não podem publicar portas próprias.
podmanpods-newDialog-labels = Rótulos
podmanpods-newDialog-create = Criar pod
podmanpods-form-nameInvalid = Comece com uma letra ou um dígito e depois use letras, dígitos, _, . ou -.
podmanpods-form-portInvalid = Portas são números de 1 a 65535.
podmanpods-form-labelInvalid = Escreva um rótulo chave=valor por linha.
