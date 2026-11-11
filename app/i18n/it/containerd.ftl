### The `containerd` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/containerd.ts:
### change the text there, then run it again.

containerd-state-none = Nessun task
containerd-state-created = Creato
containerd-state-running = In esecuzione
containerd-state-paused = In pausa
containerd-state-pausing = Sospensione in corso
containerd-state-stopped = Arrestato
containerd-state-unknown = Sconosciuto
containerd-state-exited = Terminato ({ $code })
containerd-createdBy-kubernetes = Kubernetes
containerd-createdBy-nerdctl = nerdctl
containerd-createdBy-docker = Docker
containerd-createdBy-other = ctr o altro
containerd-containers-nav = Container
containerd-containers-title = Container
containerd-containers-summary = { $count ->
        [one] { NUM($count) } container
       *[other] { NUM($count) } container
    }, { $running } in esecuzione, nel namespace { $namespace }.
containerd-containers-search = Filtra per nome, id o immagine
containerd-containers-showSandboxes = Mostra i container pause dei Pod
containerd-containers-sandbox = pause
containerd-containers-empty = Nessun container in { $namespace }
containerd-containers-emptyDescription = Qui compaiono i container creati con nerdctl, ctr o Kubernetes. Scegli un altro namespace sotto il selettore degli host per vederne altri.
containerd-containers-column-name = Nome
containerd-containers-column-image = Immagine
containerd-containers-column-state = Task
containerd-containers-column-createdBy = Creato da
containerd-containers-column-created = Creato
containerd-containers-action-start = Avvia
containerd-containers-action-stop = Arresta
containerd-containers-action-pause = Sospendi
containerd-containers-action-resume = Riprendi
containerd-containers-action-kill = Invia segnale
containerd-containers-actionOne-start = Avvia { $name }
containerd-containers-actionOne-stop = Arresta { $name }
containerd-containers-actionOne-pause = Sospendi { $name }
containerd-containers-actionOne-resume = Riprendi { $name }
containerd-containers-removeOne = Rimuovi { $name }
containerd-containers-done-start = { $name } avviato
containerd-containers-done-stop = { $name } arrestato
containerd-containers-done-pause = { $name } sospeso
containerd-containers-done-resume = { $name } ripreso
containerd-containers-done-kill = { $signal } inviato a { $name }
containerd-containers-removed = { $name } rimosso
containerd-containers-kubelet = Questo container è gestito da Kubernetes: il kubelet lo riavvia dopo un arresto e lo ricrea dopo una rimozione. Modifica invece il suo Pod sul cluster.
containerd-containers-remove-title = Rimuovere { $name }?
containerd-containers-remove-running = Il suo task è in esecuzione: Homeport lo termina, poi rimuove il task e il container.
containerd-containers-remove-stopped = Homeport rimuove il suo task e il container.
containerd-containers-remove-snapshot = Rimuovi anche il suo snapshot ({ $key }), con i file che ha modificato
containerd-containers-remove-killAndRemove = Termina e rimuovi
containerd-containers-kill-title = Invia un segnale a { $name }
containerd-containers-kill-description = Al processo principale del suo task, come fa ctr task kill -s. Un nome come SIGHUP, o un numero.
containerd-containers-kill-signal = Segnale
containerd-containers-kill-send = Invia
containerd-containers-backToList = Torna ai container
containerd-containers-tab-overview = Panoramica
containerd-containers-tab-logs = Log
containerd-containers-tab-exec = Exec
containerd-containers-tab-spec = Spec
containerd-logs-from = Da { $path }
containerd-logs-none-title = L’output di questo container non viene registrato
containerd-logs-none-description = containerd non conserva log: è lo strumento che avvia un task a decidere dove va il suo output. I container creati con ctr e con la maggior parte degli altri client non lo inviano da nessuna parte; nerdctl e Kubernetes lo scrivono in un file, che Homeport legge.
containerd-logs-driver-title = nerdctl invia l’output di questo container a { $driver }
containerd-logs-driver-description = Homeport legge solo i log json-file predefiniti di nerdctl. Leggili dove li conserva { $driver }, o esegui di nuovo il container con il driver json-file.
containerd-exec-startFirst = Avvia il container per aprirvi un terminale.
containerd-exec-command = Comando
containerd-exec-commandPlaceholder = Una shell, a meno che tu non indichi un comando
containerd-exec-user = ID utente
containerd-exec-workingDir = Cartella
containerd-exec-open = Apri di nuovo
containerd-exec-exited = Il processo è terminato.
containerd-exec-exitedWithCode = Il processo è terminato con codice { $code }.
containerd-containers-field-id = ID
containerd-containers-field-namespace = Namespace
containerd-containers-field-image = Immagine
containerd-containers-field-runtime = Runtime
containerd-containers-field-snapshot = Snapshot
containerd-containers-field-pid = PID
containerd-containers-field-exit = Ultima uscita
containerd-containers-field-pod = Pod Kubernetes
containerd-containers-field-created = Creato
containerd-containers-field-updated = Aggiornato
containerd-containers-field-extensions = Estensioni
containerd-containers-labels = Etichette
containerd-containers-noSpec = containerd non contiene alcuna spec di runtime per questo container.
containerd-images-nav = Immagini
containerd-images-title = Immagini
containerd-images-summary = { $count ->
        [one] { NUM($count) } immagine
       *[other] { NUM($count) } immagini
    } nel namespace { $namespace }.
containerd-images-pull = Scarica immagine
containerd-images-pulled = { $reference } scaricata in { $namespace }
containerd-images-removed = { $name } rimossa
containerd-images-search = Filtra per nome
containerd-images-empty = Nessuna immagine in { $namespace }
containerd-images-emptyDescription = Scaricane una: containerd la recupera, poi la decomprime per la piattaforma di questo host così che i suoi container possano essere eseguiti.
containerd-images-column-name = Nome
containerd-images-column-platforms = Piattaforme
containerd-images-column-size = Dimensione
containerd-images-column-containers = Container
containerd-images-column-created = Creata
containerd-images-sizeUnknown = Il suo contenuto per la piattaforma di questo host non è tutto qui.
containerd-images-removeOne = Rimuovi { $name }
containerd-images-inUse = Da questa immagine sono stati creati container di questo namespace. Prima rimuovili.
containerd-images-remove-title = Rimuovere { $name }?
containerd-images-remove-description = containerd rimuove il nome, e libera il suo contenuto e i suoi snapshot quando nient’altro li trattiene.
containerd-snapshots-nav = Snapshot
containerd-snapshots-title = Snapshot
containerd-snapshots-summary = { $count ->
        [one] { NUM($count) } snapshot
       *[other] { NUM($count) } snapshot
    } in { $namespace } occupano { $total }; { $unused ->
        [0] nessuno è inutilizzato
        [one] { NUM($unused) } è inutilizzato
       *[other] { NUM($unused) } sono inutilizzati
    } ({ $unusedSize }).
containerd-snapshots-snapshotter = Snapshotter
containerd-snapshots-unusedOnly = Solo inutilizzati
containerd-snapshots-unused = Inutilizzato
containerd-snapshots-usedByContainer = container { $id }
containerd-snapshots-usedByImage = immagine { $name }
containerd-snapshots-empty = Nessuno snapshot in { $snapshotter }
containerd-snapshots-emptyDescription = Gli snapshot sono i layer delle immagini decompresse e i layer scrivibili dei container.
containerd-snapshots-column-key = Chiave
containerd-snapshots-column-kind = Tipo
containerd-snapshots-column-size = Dimensione
containerd-snapshots-column-usedBy = Usato da
containerd-snapshots-kind-committed = Layer
containerd-snapshots-kind-active = Scrivibile
containerd-snapshots-kind-view = Vista
containerd-snapshots-kind-unknown = Sconosciuto
containerd-snapshots-removeOne = Rimuovi { $key }
containerd-snapshots-removed = Snapshot { $key } rimosso
containerd-snapshots-inUse = Qualcosa lo usa o è costruito su di esso. Prima rimuovi il container o l’immagine.
containerd-snapshots-remove-title = Rimuovere lo snapshot { $key }?
containerd-snapshots-remove-description = Nulla lo usa. I suoi file, { $size }, vengono eliminati dall’host.
containerd-namespaces-nav = Namespace
containerd-namespaces-title = Namespace
containerd-namespaces-summary = { $count ->
        [one] { NUM($count) } namespace
       *[other] { NUM($count) } namespace
    }. Ogni oggetto containerd vive in uno di essi: nerdctl usa default, Kubernetes k8s.io, Docker moby.
containerd-namespaces-new = Nuovo namespace
containerd-namespaces-created = Namespace { $name } creato
containerd-namespaces-removed = Namespace { $name } rimosso
containerd-namespaces-shown = Mostrato
containerd-namespaces-show = Mostra
containerd-namespaces-removeOne = Rimuovi { $name }
containerd-namespaces-notEmpty = Solo i namespace vuoti possono essere rimossi. Prima rimuovi i suoi container e le sue immagini.
containerd-namespaces-remove-title = Rimuovere il namespace { $name }?
containerd-namespaces-remove-description = containerd rifiuta se contiene ancora snapshot o contenuti che i conteggi non mostrano.
containerd-namespaces-column-name = Nome
containerd-namespaces-column-containers = Container
containerd-namespaces-column-images = Immagini
containerd-namespaces-column-labels = Etichette
containerd-namespaces-empty = Nessun namespace
containerd-namespaces-emptyDescription = containerd crea default la prima volta che vi viene creato qualcosa.
containerd-namespaces-newDialog-title = Nuovo namespace
containerd-namespaces-newDialog-description = Un namespace vuoto, per tenere container e immagini separati dagli altri.
containerd-namespaces-newDialog-name = Nome
containerd-namespaces-newDialog-invalid = Usa lettere e cifre, separate da singoli punti, trattini o trattini bassi.
containerd-namespaces-newDialog-create = Crea
