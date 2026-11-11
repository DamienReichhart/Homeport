### The `networks` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/networks.ts:
### change the text there, then run it again.

networks-nav = Redes
networks-title = Redes
networks-summary = { $count ->
        [one] { NUM($count) } rede
       *[other] { NUM($count) } redes
    }
networks-cleanUp = Limpar
networks-create = Criar rede
networks-search = Pesquisar redes
networks-empty = Nenhuma rede
networks-column-name = Nome
networks-column-stack = Stack
networks-column-driver = Driver
networks-column-subnet = Sub-rede
networks-column-containers = Contêineres
networks-builtIn = integrada
networks-internal = interna
networks-inspectOne = Inspecionar { $name }
networks-removeOne = Remover { $name }
networks-remove-title = Remover a rede { $name }?
networks-remove-attached = Ela ainda tem { $count ->
        [one] { NUM($count) } contêiner conectado
       *[other] { NUM($count) } contêineres conectados
    }; desconecte-os antes.
networks-remove-unused = Nenhum contêiner a usa. Contêineres que a referenciam pelo nome não vão conseguir iniciar.
networks-prune-title = Remover redes não usadas?
networks-prune-description = Todas as redes personalizadas sem nenhum contêiner conectado são removidas.
networks-connected = Conectado
networks-disconnected = Desconectado
networks-removed = { $name } removida
networks-pruned = { $count ->
        [one] { NUM($count) } rede removida
       *[other] { NUM($count) } redes removidas
    }
networks-createDialog-title = Criar uma rede
networks-createDialog-name = Nome
networks-createDialog-driver = Driver
networks-createDialog-subnet = Sub-rede
networks-createDialog-gateway = Gateway
networks-createDialog-automatic = automático
networks-createDialog-internal = Interna — sem acesso a redes externas
networks-createDialog-attachable = Conectável
networks-dialog-containers = Contêineres
networks-dialog-raw = JSON bruto
networks-dialog-noContainer = Nenhum contêiner está conectado.
networks-dialog-disconnect = Desconectar
networks-dialog-containerToConnect = Contêiner a conectar
networks-dialog-connectPlaceholder = Conectar um contêiner…
networks-dialog-connect = Conectar
