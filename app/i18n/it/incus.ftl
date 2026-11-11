### The `incus` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/incus.ts:
### change the text there, then run it again.

incus-delete = Elimina
incus-status-running = In esecuzione
incus-status-stopped = Arrestata
incus-status-frozen = Congelata
incus-status-starting = Avvio in corso
incus-status-stopping = Arresto in corso
incus-status-freezing = Congelamento in corso
incus-status-error = Errore
incus-status-unknown = Sconosciuto
incus-type-container = Container
incus-type-virtualMachine = Macchina virtuale
incus-action-start = Avvia
incus-action-stop = Arresta
incus-action-restart = Riavvia
incus-action-freeze = Congela
incus-action-unfreeze = Scongela
incus-action-forceStop = Forza l’arresto
incus-actionOne-start = Avvia { $name }
incus-actionOne-stop = Arresta { $name }
incus-actionOne-restart = Riavvia { $name }
incus-actionOne-freeze = Congela { $name }
incus-actionOne-unfreeze = Scongela { $name }
incus-instances-nav = Istanze
incus-instances-title = Istanze
incus-instances-summary = { $count ->
        [one] { NUM($count) } istanza
       *[other] { NUM($count) } istanze
    }, { $running } in esecuzione, nel progetto { $project }.
incus-instances-search = Filtra per nome o descrizione
incus-instances-filter-all = Tutte
incus-instances-empty = Nessuna istanza in { $project }
incus-instances-emptyDescription = Qui compaiono i container di sistema e le macchine virtuali di questo progetto. Scegli un altro progetto sotto il selettore degli host per vederne altri.
incus-instances-more = Altre azioni per { $name }
incus-instances-column-name = Nome
incus-instances-column-type = Tipo
incus-instances-column-status = Stato
incus-instances-column-image = Immagine
incus-instances-column-addresses = Indirizzi
incus-instances-column-memory = Memoria
incus-instances-column-created = Creata
incus-instances-done-start = { $name } avviata
incus-instances-done-stop = { $name } arrestata
incus-instances-done-restart = { $name } riavviata
incus-instances-done-freeze = { $name } congelata
incus-instances-done-unfreeze = { $name } scongelata
incus-instances-deleted = { $name } eliminata
incus-instances-delete-title = Eliminare { $name }?
incus-instances-delete-description = L’istanza e il suo disco vengono eliminati dal progetto { $project }. L’operazione non può essere annullata.
incus-instances-delete-snapshots = { $count ->
        [one] Il suo snapshot viene eliminato
       *[other] I suoi { NUM($count) } snapshot vengono eliminati
    } insieme a essa.
incus-instances-delete-snapshotsNamed = { $count ->
        [one] Il suo snapshot { $names } viene eliminato
       *[other] I suoi snapshot { $names } vengono eliminati
    } insieme a essa.
incus-instances-delete-stopFirst = Prima arrestala: è in esecuzione
incus-instances-delete-stopAndDelete = Arresta ed elimina
incus-instances-backToList = Torna alle istanze
incus-instances-tab-overview = Panoramica
incus-instances-tab-console = Console
incus-instances-tab-exec = Exec
incus-instances-tab-snapshots = Snapshot
incus-instances-tab-config = Configurazione
incus-instances-tab-devices = Dispositivi
incus-instances-tab-raw = Originale
incus-instances-field-project = Progetto
incus-instances-field-architecture = Architettura
incus-instances-field-addresses = Indirizzi
incus-instances-field-cpu = Tempo CPU
incus-instances-field-memory = Memoria
incus-instances-field-processes = Processi
incus-instances-field-created = Creata
incus-instances-field-lastUsed = Ultimo utilizzo
incus-instances-field-snapshots = Snapshot
incus-instances-field-location = Membro del cluster
incus-instances-field-description = Descrizione
incus-instances-cpuSeconds = { $seconds } s
incus-instances-profiles = Profili
incus-instances-noConfig = Nessuna configurazione.
incus-instances-noDevices = Nessun dispositivo.
incus-instances-config-key = Chiave
incus-instances-config-value = Valore
incus-instances-config-fromProfile = Da un profilo
incus-progress-waiting = In attesa di Incus…
incus-remote-server = Server di immagini
incus-remote-search = Filtra per sistema, release o alias
incus-remote-list = Immagini sul server
incus-remote-none = Nessuna immagine corrisponde.
incus-remote-alias = Alias
incus-images-nav = Immagini
incus-images-title = Immagini
incus-images-summary = { $count ->
        [one] { NUM($count) } immagine
       *[other] { NUM($count) } immagini
    } nel progetto { $project }.
incus-images-search = Filtra per alias, impronta o descrizione
incus-images-empty = Nessuna immagine in { $project }
incus-images-emptyDescription = Le immagini vengono scaricate quando si crea un’istanza da una di esse, oppure in anticipo con Ottieni immagine.
incus-images-get = Ottieni immagine
incus-images-get-title = Ottieni un’immagine
incus-images-get-description = Incus la scarica nel progetto { $project } dal server di immagini, così le istanze si avviano rapidamente da essa.
incus-images-get-autoUpdate = Mantienila aggiornata dal server
incus-images-get-keepAlias = Assegnale lo stesso alias qui
incus-images-get-confirm = Ottieni immagine
incus-images-get-done = L’immagine è pronta.
incus-images-copied = { $alias } copiata in { $project }
incus-images-column-aliases = Alias
incus-images-column-description = Descrizione
incus-images-column-type = Tipo
incus-images-column-arch = Architettura
incus-images-column-size = Dimensione
incus-images-column-uploaded = Aggiunta
incus-images-cached = in cache
incus-images-cachedHint = Scaricata per creare un’istanza; Incus la elimina quando nulla la usa da un po’ di tempo.
incus-images-aliasAdd = Aggiungi un alias a { $name }
incus-images-aliasRemove = Rimuovi l’alias { $name }
incus-images-aliasRemoved = Alias { $name } rimosso
incus-images-alias-title = Aggiungi un alias a { $name }
incus-images-alias-name = Alias
incus-images-alias-add = Aggiungi alias
incus-images-refreshOne = Aggiorna { $name } dal suo server
incus-images-refreshed = { $name } aggiornata
incus-images-deleteOne = Elimina { $name }
incus-images-deleted = { $name } eliminata
incus-images-delete-title = Eliminare l’immagine { $name }?
incus-images-delete-description = Le istanze create da essa continuano a funzionare: ognuna ha la propria copia. Per quelle nuove dovrà essere scaricata di nuovo.
incus-profiles-nav = Profili
incus-profiles-title = Profili
incus-profiles-summary = { $count ->
        [one] { NUM($count) } profilo
       *[other] { NUM($count) } profili
    } nel progetto { $project }.
incus-profiles-search = Filtra per nome o descrizione
incus-profiles-empty = Nessun profilo in { $project }
incus-profiles-new = Nuovo profilo
incus-profiles-new-title = Nuovo profilo
incus-profiles-new-description = Un profilo vuoto nel progetto { $project }. Aggiungi la sua configurazione e i suoi dispositivi una volta creato.
incus-profiles-new-name = Nome
incus-profiles-new-descriptionField = Descrizione
incus-profiles-created = Profilo { $name } creato
incus-profiles-column-name = Nome
incus-profiles-column-description = Descrizione
incus-profiles-column-settings = Impostazioni
incus-profiles-column-usedBy = Usato da
incus-profiles-settings = { $config ->
        [one] { NUM($config) } chiave
       *[other] { NUM($config) } chiavi
    }, { $devices ->
        [one] { NUM($devices) } dispositivo
       *[other] { NUM($devices) } dispositivi
    }
incus-profiles-defaultKept = Incus mantiene il profilo default.
incus-profiles-inUse = Usato da { $names }: prima rimuovilo da loro.
incus-profiles-deleteOne = Elimina { $name }
incus-profiles-deleted = Profilo { $name } eliminato
incus-profiles-delete-title = Eliminare il profilo { $name }?
incus-profiles-delete-description = Viene eliminato dal progetto { $project }. Nessuna istanza lo usa.
incus-profiles-backToList = Torna ai profili
incus-profiles-noDescription = Nessuna descrizione
incus-profiles-reload = Ricarica
incus-profiles-saved = Profilo { $name } salvato
incus-profiles-usedBy = Usato da
incus-profiles-editHint = La sua descrizione, configurazione e dispositivi, come li mostra incus profile edit. Il salvataggio li applica alle istanze che lo usano, come fa Incus; se qualcuno lo ha modificato dopo il caricamento, Homeport rifiuta invece di sovrascriverlo.
incus-profiles-yamlOf = YAML di { $name }
incus-new-action = Nuova istanza
incus-new-title = Nuova istanza
incus-new-description = Un container di sistema o una macchina virtuale nel progetto { $project }, da un’immagine.
incus-new-name = Nome
incus-new-image = Immagine
incus-new-origin-remote = Da un server di immagini
incus-new-origin-local = Dalle immagini di questo host
incus-new-localImage = Immagine su questo host
incus-new-localImagePlaceholder = Scegli un’immagine
incus-new-profiles = Profili
incus-new-cpu = CPU
incus-new-memory = Memoria
incus-new-disk = Disco radice
incus-new-limitsHint = Facoltativo: se lasciato vuoto, decidono i profili. Le dimensioni si scrivono come 512MiB o 20GiB.
incus-new-start = Avviala una volta creata
incus-new-confirm = Crea
incus-new-open = Aprila
incus-new-made = { $name } creata
incus-new-done = { $name } è pronta.
incus-new-problem-name = Chiamala con lettere, cifre e trattini, iniziando con una lettera, fino a 63 caratteri.
incus-new-problem-image = Scegli un’immagine, o digita il suo alias.
incus-new-problem-cpu = Le CPU sono un numero intero a partire da 1.
incus-new-problem-memory = La memoria si scrive come 512MiB o 2GiB.
incus-new-problem-disk = Il disco radice si scrive come 20GiB.
incus-exec-startFirst = Avvia l’istanza per eseguirvi comandi.
incus-exec-command = Comando
incus-exec-commandPlaceholder = Una shell (bash, altrimenti sh)
incus-exec-user = ID utente
incus-exec-cwd = Cartella
incus-exec-open = Apri
incus-exec-exited = Il comando è terminato.
incus-exec-exitedWithCode = Il comando è terminato con codice { $code }.
incus-console-startFirst = Avvia l’istanza per collegarti alla sua console.
incus-console-hint = La sua console testuale, come la mostra incus console: il prompt di accesso, o un avvio mentre avviene. Lasciando la scheda ti scolleghi senza arrestare nulla.
incus-console-reattach = Ricollega
incus-console-detached = Scollegato dalla console.
incus-snapshots-take = Crea snapshot
incus-snapshots-take-title = Crea uno snapshot di { $name }
incus-snapshots-take-description = Una copia del suo disco così com’è ora, a cui ripristinarlo in seguito: prima di un aggiornamento o di una modifica rischiosa.
incus-snapshots-take-name = Nome
incus-snapshots-take-namePlaceholder = snap0, snap1… se lasciato vuoto
incus-snapshots-take-expiry = Conservalo
incus-snapshots-take-stateful = Includi lo stato di esecuzione (memoria)
incus-snapshots-expiry-never = Finché non viene eliminato
incus-snapshots-expiry-1d = Un giorno
incus-snapshots-expiry-7d = Una settimana
incus-snapshots-expiry-30d = 30 giorni
incus-snapshots-taken = Snapshot di { $name } creato
incus-snapshots-empty = Nessuno snapshot
incus-snapshots-emptyDescription = Creane uno prima di modificare qualcosa che potresti voler annullare.
incus-snapshots-column-name = Nome
incus-snapshots-column-created = Creato
incus-snapshots-column-expires = Scade
incus-snapshots-column-size = Dimensione
incus-snapshots-stateful = con stato
incus-snapshots-restore = Ripristina
incus-snapshots-restore-title = Ripristinare { $name } a { $snapshot }?
incus-snapshots-restore-description = Tutto ciò che è cambiato sul suo disco dal { $time }, quando è stato creato { $snapshot }, va perso. Un’istanza in esecuzione si riavvia.
incus-snapshots-restored = { $name } ripristinata a { $snapshot }
incus-snapshots-deleteOne = Elimina { $snapshot }
incus-snapshots-delete-title = Eliminare lo snapshot { $snapshot }?
incus-snapshots-delete-description = L’istanza resta com’è; questo snapshot non potrà più essere ripristinato.
incus-snapshots-deleted = Snapshot { $snapshot } eliminato
incus-storage-nav = Archiviazione
incus-storage-title = Archiviazione
incus-storage-summary = { $count ->
        [one] { NUM($count) } pool di archiviazione
       *[other] { NUM($count) } pool di archiviazione
    }; volumi del progetto { $project }.
incus-storage-empty = Nessun pool di archiviazione
incus-storage-emptyDescription = Creane uno con incus storage create, o incus admin init.
incus-storage-usage = Spazio usato in { $pool }
incus-storage-space = { $used } di { $total } usati ({ $percent })
incus-storage-spaceUnknown = Il suo driver non indica quanto è pieno.
incus-storage-volumesOf = Volumi in { $pool }
incus-storage-noVolumes = Nessun volume di questo progetto nel pool.
incus-storage-new = Nuovo volume
incus-storage-new-title = Nuovo volume in { $pool }
incus-storage-new-description = Un volume personalizzato del progetto { $project }, da collegare alle istanze per dati che sopravvivono a esse.
incus-storage-new-name = Nome
incus-storage-new-size = Dimensione
incus-storage-new-content = Contenuto
incus-storage-content-filesystem = Filesystem
incus-storage-content-block = Dispositivo a blocchi
incus-storage-created = Volume { $name } creato
incus-storage-column-name = Nome
incus-storage-column-type = Tipo
incus-storage-column-content = Contenuto
incus-storage-column-size = Dimensione
incus-storage-column-usedBy = Usato da
incus-storage-inUse = Usato da { $names }: prima scollegalo.
incus-storage-deleteOne = Elimina { $name }
incus-storage-delete-title = Eliminare il volume { $name }?
incus-storage-delete-description = I suoi dati vengono eliminati da { $pool }. Nulla lo usa.
incus-storage-deleted = Volume { $name } eliminato
incus-networks-nav = Reti
incus-networks-title = Reti
incus-networks-summary = { $count ->
        [one] { NUM($count) } rete visibile
       *[other] { NUM($count) } reti visibili
    } dal progetto { $project }.
incus-networks-search = Filtra per nome o tipo
incus-networks-empty = Nessuna rete
incus-networks-new = Nuovo bridge
incus-networks-new-title = Nuovo bridge
incus-networks-new-description = Un bridge gestito: Incus gli assegna indirizzi, DHCP e DNS, perché le istanze vi si colleghino tramite un profilo o un dispositivo.
incus-networks-new-name = Nome
incus-networks-new-ipv4 = IPv4
incus-networks-new-ipv6 = IPv6
incus-networks-new-addressHint = auto sceglie una sottorete libera, none lo disattiva; oppure indica l’indirizzo del bridge con il suo prefisso, come 10.0.5.1/24.
incus-networks-new-nat = Maschera il traffico IPv4 in uscita (NAT)
incus-networks-created = Bridge { $name } creato
incus-networks-column-name = Nome
incus-networks-column-type = Tipo
incus-networks-column-ipv4 = IPv4
incus-networks-column-ipv6 = IPv6
incus-networks-column-usedBy = Usata da
incus-networks-managed = Gestita da Incus
incus-networks-unmanaged = Di questo computer
incus-networks-inUse = Usata da { $names }: prima rimuovila da loro.
incus-networks-deleteOne = Elimina { $name }
incus-networks-delete-title = Eliminare la rete { $name }?
incus-networks-delete-description = Incus rimuove il bridge con il suo DHCP e DNS. Nulla lo usa.
incus-networks-deleted = Rete { $name } eliminata
incus-networks-gone = La rete { $name } non esiste più.
incus-networks-backToList = Torna alle reti
incus-networks-field-managed = Creata da
incus-networks-field-state = Stato
incus-networks-field-addresses = Indirizzi sull’host
incus-networks-field-ipv4 = Impostazione IPv4
incus-networks-field-ipv6 = Impostazione IPv6
incus-networks-field-nat = NAT
incus-networks-field-mtu = MTU
incus-networks-field-hwaddr = Indirizzo MAC
incus-networks-yes = Sì
incus-networks-no = No
incus-networks-usedBy = Usata da
incus-networks-leases = Lease DHCP
incus-networks-noLeases = Nessun lease.
incus-networks-lease-hostname = Nome host
incus-networks-lease-address = Indirizzo
incus-networks-lease-hwaddr = Indirizzo MAC
incus-networks-lease-type = Tipo
