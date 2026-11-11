### The `networks` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/networks.ts:
### change the text there, then run it again.

networks-nav = Reti
networks-title = Reti
networks-summary = { $count ->
        [one] { NUM($count) } rete
       *[other] { NUM($count) } reti
    }
networks-cleanUp = Ripulisci
networks-create = Crea rete
networks-search = Cerca reti
networks-empty = Nessuna rete
networks-column-name = Nome
networks-column-stack = Stack
networks-column-driver = Driver
networks-column-subnet = Sottorete
networks-column-containers = Container
networks-builtIn = predefinita
networks-internal = interna
networks-inspectOne = Ispeziona { $name }
networks-removeOne = Rimuovi { $name }
networks-remove-title = Rimuovere la rete { $name }?
networks-remove-attached = Ha ancora { $count ->
        [one] { NUM($count) } container collegato
       *[other] { NUM($count) } container collegati
    }; prima scollegali.
networks-remove-unused = Nessun container la usa. I container che vi fanno riferimento per nome non riusciranno ad avviarsi.
networks-prune-title = Rimuovere le reti inutilizzate?
networks-prune-description = Viene rimossa ogni rete personalizzata a cui non è collegato alcun container.
networks-connected = Collegato
networks-disconnected = Scollegato
networks-removed = { $name } rimossa
networks-pruned = { $count ->
        [one] { NUM($count) } rete rimossa
       *[other] { NUM($count) } reti rimosse
    }
networks-createDialog-title = Crea una rete
networks-createDialog-name = Nome
networks-createDialog-driver = Driver
networks-createDialog-subnet = Sottorete
networks-createDialog-gateway = Gateway
networks-createDialog-automatic = automatico
networks-createDialog-internal = Interna — nessun accesso alle reti esterne
networks-createDialog-attachable = Collegabile
networks-dialog-containers = Container
networks-dialog-raw = JSON originale
networks-dialog-noContainer = Nessun container collegato.
networks-dialog-disconnect = Scollega
networks-dialog-containerToConnect = Container da collegare
networks-dialog-connectPlaceholder = Collega un container…
networks-dialog-connect = Collega
