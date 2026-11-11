### The `networks` namespace, in Español (es).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/es/networks.ts:
### change the text there, then run it again.

networks-nav = Redes
networks-title = Redes
networks-summary = { $count ->
        [one] { NUM($count) } red
       *[other] { NUM($count) } redes
    }
networks-cleanUp = Limpiar
networks-create = Crear red
networks-search = Buscar redes
networks-empty = No hay redes
networks-column-name = Nombre
networks-column-stack = Stack
networks-column-driver = Driver
networks-column-subnet = Subred
networks-column-containers = Contenedores
networks-builtIn = integrada
networks-internal = interna
networks-inspectOne = Inspeccionar { $name }
networks-removeOne = Eliminar { $name }
networks-remove-title = ¿Eliminar la red { $name }?
networks-remove-attached = Todavía tiene { $count ->
        [one] { NUM($count) } contenedor conectado
       *[other] { NUM($count) } contenedores conectados
    }; desconéctalos primero.
networks-remove-unused = Ningún contenedor la usa. Los contenedores que la mencionan por su nombre no podrán iniciarse.
networks-prune-title = ¿Eliminar las redes sin usar?
networks-prune-description = Se elimina toda red personalizada a la que no esté conectado ningún contenedor.
networks-connected = Conectado
networks-disconnected = Desconectado
networks-removed = Se eliminó { $name }
networks-pruned = { $count ->
        [one] Se eliminó { NUM($count) } red
       *[other] Se eliminaron { NUM($count) } redes
    }
networks-createDialog-title = Crear una red
networks-createDialog-name = Nombre
networks-createDialog-driver = Driver
networks-createDialog-subnet = Subred
networks-createDialog-gateway = Puerta de enlace
networks-createDialog-automatic = automática
networks-createDialog-internal = Interna: sin acceso a redes externas
networks-createDialog-attachable = Conectable
networks-dialog-containers = Contenedores
networks-dialog-raw = JSON sin procesar
networks-dialog-noContainer = No hay ningún contenedor conectado.
networks-dialog-disconnect = Desconectar
networks-dialog-containerToConnect = Contenedor que conectar
networks-dialog-connectPlaceholder = Conectar un contenedor…
networks-dialog-connect = Conectar
