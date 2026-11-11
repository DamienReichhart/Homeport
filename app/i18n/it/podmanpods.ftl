### The `podmanpods` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/podmanpods.ts:
### change the text there, then run it again.

podmanpods-nav = Pod
podmanpods-title = Pod
podmanpods-summary = { $count ->
        [one] { NUM($count) } pod
       *[other] { NUM($count) } pod
    }
podmanpods-search = Cerca pod
podmanpods-empty = Nessun pod
podmanpods-emptyDescription = Un pod raggruppa container che condividono una rete e si avviano e arrestano insieme. Creane uno, poi esegui dei container al suo interno.
podmanpods-new = Nuovo pod
podmanpods-column-name = Nome
podmanpods-column-status = Stato
podmanpods-column-containers = Container
podmanpods-column-networks = Reti
podmanpods-column-created = Creato
podmanpods-running = { $running } di { $count } in esecuzione
podmanpods-status-running = In esecuzione
podmanpods-status-degraded = In parte in esecuzione
podmanpods-status-paused = In pausa
podmanpods-status-created = Creato
podmanpods-status-stopped = Arrestato
podmanpods-status-failed = Non riuscito
podmanpods-action-start = Avvia
podmanpods-action-stop = Arresta
podmanpods-action-restart = Riavvia
podmanpods-action-pause = Sospendi
podmanpods-action-unpause = Riprendi
podmanpods-actionOne-start = Avvia { $name }
podmanpods-actionOne-stop = Arresta { $name }
podmanpods-actionOne-restart = Riavvia { $name }
podmanpods-actionOne-pause = Sospendi { $name }
podmanpods-actionOne-unpause = Riprendi { $name }
podmanpods-removeOne = Rimuovi { $name }
podmanpods-done-start = { $name } avviato
podmanpods-done-stop = { $name } arrestato
podmanpods-done-restart = { $name } riavviato
podmanpods-done-pause = { $name } sospeso
podmanpods-done-unpause = { $name } ripreso
podmanpods-removed = { $name } rimosso, { $count ->
        [0] senza container
        [one] con { NUM($count) } container
       *[other] con { NUM($count) } container
    }
podmanpods-created = { $name } creato
podmanpods-remove-title = Rimuovere { $name }?
podmanpods-remove-stopped = { $count ->
        [0] Vengono rimossi il pod e il suo container infra.
        [one] Vengono rimossi il pod, il suo container infra e il suo { NUM($count) } container.
       *[other] Vengono rimossi il pod, il suo container infra e i suoi { NUM($count) } container.
    } I loro volumi vengono conservati.
podmanpods-remove-running = { $count ->
        [0] Il pod è in esecuzione: viene arrestato, poi rimosso con il suo container infra.
        [one] Il pod è in esecuzione: il suo { NUM($count) } container viene arrestato, poi rimosso con il pod e il suo container infra.
       *[other] Il pod è in esecuzione: i suoi { NUM($count) } container vengono arrestati, poi rimossi con il pod e il suo container infra.
    } I loro volumi vengono conservati.
podmanpods-remove-stopAndRemove = Arresta e rimuovi
podmanpods-detail-containers = { $count ->
        [0] Container
        [one] { NUM($count) } container
       *[other] { NUM($count) } container
    }
podmanpods-detail-noContainers = In questo pod non è ancora in esecuzione nulla oltre al suo container infra.
podmanpods-detail-infra = Infra
podmanpods-detail-settings = Impostazioni
podmanpods-detail-ports = Porte pubblicate
podmanpods-detail-noPorts = Nessuna
podmanpods-detail-shared = Namespace condivisi
podmanpods-detail-networks = Reti
podmanpods-detail-hostname = Nome host
podmanpods-detail-created = Creato
podmanpods-detail-labels = Etichette
podmanpods-detail-command = Creato con
podmanpods-detail-runContainer = Esegui container
podmanpods-detail-runContainerIn = Esegui un container in { $name }
podmanpods-newDialog-title = Nuovo pod
podmanpods-newDialog-description = I suoi container condividono un’unica rete, quindi le porte vengono pubblicate sul pod, per tutti loro.
podmanpods-newDialog-name = Nome
podmanpods-newDialog-portsHint = Lascia vuota la porta dell’host per far scegliere a Podman una porta libera. I container eseguiti nel pod non possono pubblicare porte proprie.
podmanpods-newDialog-labels = Etichette
podmanpods-newDialog-create = Crea pod
podmanpods-form-nameInvalid = Inizia con una lettera o una cifra, poi usa lettere, cifre, _, . o -.
podmanpods-form-portInvalid = Le porte sono numeri da 1 a 65535.
podmanpods-form-labelInvalid = Scrivi un’etichetta chiave=valore per riga.
