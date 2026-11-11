### The `volumes` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/volumes.ts:
### change the text there, then run it again.

volumes-nav = Volumi
volumes-title = Volumi
volumes-summary = { $count ->
        [one] { NUM($count) } volume
       *[other] { NUM($count) } volumi
    } · { $unused } inutilizzati
volumes-cleanUp = Ripulisci
volumes-cleanUp-anonymous = Volumi anonimi inutilizzati
volumes-cleanUp-all = Tutti i volumi inutilizzati
volumes-create = Crea volume
volumes-search = Cerca volumi
volumes-empty-title = Nessun volume
volumes-empty-description = I volumi conservano i dati dei container tra riavvii e ricreazioni.
volumes-column-name = Nome
volumes-column-stack = Stack
volumes-column-driver = Driver
volumes-column-size = Dimensione
volumes-size-label = Dimensione:
volumes-size-measuring = Misurazione
volumes-size-unknown = Il driver di questo volume non ne indica la dimensione.
volumes-size-refresh = Aggiorna le dimensioni
volumes-column-usedBy = Usato da
volumes-column-created = Creato
volumes-unused = Inutilizzato
volumes-inspectOne = Ispeziona { $name }
volumes-deleteOne = Elimina { $name }
volumes-browseOne = Sfoglia i file di { $name }
volumes-browse-title = File di { $name }
volumes-browse-description = Sola lettura: niente qui può modificare il volume, anche mentre i container lo usano.
volumes-browse-saveOne = Salva { $name }
volumes-browse-saved = { $name } salvato
volumes-browse-savedArchive = { $name } salvato come archivio .tar
volumes-helper-needed = Homeport apre i volumi con un container ausiliario di breve durata, che richiede l’immagine { $image } (circa { $size }). Questo host non ce l’ha.
volumes-helper-removed = Homeport la scarica solo se sei d’accordo, e la rimuove di nuovo quando ha finito, a meno che non la usi qualcos’altro.
volumes-helper-pull = Scarica { $image }
volumes-helper-pulling = Download di { $image }…
volumes-helper-progress = Avanzamento del download
volumes-moreActions = Altre azioni per { $name }
volumes-backupMenu = Esegui il backup su file…
volumes-restoreMenu = Ripristina da file…
volumes-cloneMenu = Clona…
volumes-clone-title = Clona { $volume }
volumes-clone-description = Copia i suoi file, con proprietari e permessi, in un nuovo volume su questo host. La copia non appartiene ad alcuno stack.
volumes-clone-name = Nome della copia
volumes-clone-start = Clona
volumes-clone-into = In { $target }
volumes-clone-copying = Copia in corso…
volumes-clone-done = { $source } clonato in { $target }
volumes-restoreFromFile = Ripristina da file
volumes-backup-title = Backup di { $volume }
volumes-backup-saveAs = Esegui il backup di { $volume } in
volumes-backup-to = In { $path }
volumes-backup-done = Backup di { $volume } eseguito in { $path }
volumes-restore-pick = Ripristina un volume da
volumes-restore-title = Ripristina un volume
volumes-restore-titleInto = Ripristina { $volume }
volumes-restore-from = Da { $path }
volumes-restore-into = Ripristina in
volumes-restore-newVolume = Un nuovo volume
volumes-restore-existingVolume = Un volume esistente
volumes-restore-volume = Volume
volumes-restore-chooseVolume = Scegli un volume
volumes-restore-nameTaken = Esiste già un volume chiamato { $volume }.
volumes-restore-start = Ripristina
volumes-restore-done = { $volume } ripristinato
volumes-restore-cancelled = Il ripristino di { $volume } è stato annullato; il volume potrebbe essere scritto solo in parte.
volumes-transfer-written = Scritti
volumes-transfer-sent = Inviati
volumes-transfer-archives = Archivi tar
volumes-flow-running-backup = { $containers } { $count ->
        [one] è
       *[other] sono
    } in esecuzione e { $count ->
        [one] usa
       *[other] usano
    } { $volume }. Un backup eseguito mentre vi scrivono potrebbe essere incoerente, per esempio un database colto a metà di una scrittura.
volumes-flow-running-restore = { $containers } { $count ->
        [one] è
       *[other] sono
    } in esecuzione e { $count ->
        [one] usa
       *[other] usano
    } { $volume }. Homeport { $count ->
        [one] lo arresta
       *[other] li arresta
    } durante il ripristino e { $count ->
        [one] lo riavvia
       *[other] li riavvia
    } al termine.
volumes-flow-running-clone = { $containers } { $count ->
        [one] è
       *[other] sono
    } in esecuzione e { $count ->
        [one] usa
       *[other] usano
    } { $volume }. Una copia eseguita mentre vi scrivono potrebbe essere incoerente, per esempio un database colto a metà di una scrittura.
volumes-flow-anyway-backup = Esegui comunque il backup
volumes-flow-anyway-clone = Copia comunque
volumes-flow-stopDuring-backup = { $count ->
        [one] Arrestalo
       *[other] Arrestali
    } durante il backup
volumes-flow-stopDuring-restore = Arresta e ripristina
volumes-flow-stopDuring-clone = { $count ->
        [one] Arrestalo
       *[other] Arrestali
    } durante la copia
volumes-flow-replace = { $volume } contiene già dei file. Il ripristino li elimina e mette al loro posto quelli del backup.
volumes-flow-replaceConfirm = Sostituisci il contenuto
volumes-usedBy = Usato da { $containers }.
volumes-notUsed = Nessun container usa questo volume.
volumes-createDialog-title = Crea un volume
volumes-createDialog-name = Nome
volumes-createDialog-driver = Driver
volumes-deleted = { $name } eliminato
volumes-pruned = { $count ->
        [one] { NUM($count) } volume eliminato
       *[other] { NUM($count) } volumi eliminati
    }, { $space } recuperati
volumes-remove-title = Eliminare il volume { $name }?
volumes-remove-warning = <strong>Tutti i dati di questo volume vengono eliminati definitivamente.</strong>
volumes-remove-mounted = {" "}È montato da { $containers }; prima rimuovi quei container.
volumes-remove-confirm = Elimina volume e dati
volumes-prune-allTitle = Eliminare tutti i volumi inutilizzati?
volumes-prune-anonymousTitle = Eliminare i volumi anonimi inutilizzati?
volumes-prune-allWarning = I dati di ogni volume non usato da alcun container vengono eliminati definitivamente, compresi i volumi con nome.
volumes-prune-anonymousWarning = I dati dei volumi senza nome non usati da alcun container vengono eliminati definitivamente.
volumes-prune-confirm = Elimina volumi e dati
