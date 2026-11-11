### The `swarm` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/swarm.ts:
### change the text there, then run it again.

swarm-nav = Swarm
swarm-badge = Swarm
swarm-reachable = Un manager è connesso
swarm-unreachable = Nessun manager connesso
swarm-hostCounts = { $managers ->
        [one] { NUM($managers) } host manager
       *[other] { NUM($managers) } host manager
    }, { $workers ->
        [one] { NUM($workers) } host worker
       *[other] { NUM($workers) } host worker
    }
swarm-host-managerOf = Manager di { $swarm }
swarm-host-workerOf = Worker di { $swarm }
swarm-role-manager = Manager
swarm-role-worker = Worker
swarm-tab-summary = Riepilogo
swarm-tab-nodes = Nodi
swarm-tab-services = Servizi
swarm-tab-tasks = Task
swarm-tab-stacks = Stack
swarm-tab-secrets = Segreti
swarm-tab-configs = Config
swarm-summary-title = Questo Swarm
swarm-summary-managerHosts = Host manager
swarm-summary-workerHosts = Host worker
swarm-summary-nodes = Nodi
swarm-summary-nodeCounts = { $nodes ->
        [one] { NUM($nodes) } nodo
       *[other] { NUM($nodes) } nodi
    }, { $managers ->
        [one] { NUM($managers) } manager
       *[other] { NUM($managers) } manager
    }
swarm-summary-services = Servizi
swarm-summary-created = Creato
swarm-summary-id = ID dello Swarm
swarm-nodes-search = Cerca nodi
swarm-nodes-column-hostname = Nodo
swarm-nodes-column-role = Ruolo
swarm-nodes-column-availability = Disponibilità
swarm-nodes-column-state = Stato
swarm-nodes-column-engine = Engine
swarm-nodes-column-address = Indirizzo
swarm-nodes-column-labels = Etichette
swarm-nodes-isHost = host { $name }
swarm-nodes-availability-active = Attivo
swarm-nodes-availability-pause = In pausa
swarm-nodes-availability-drain = Svuotato
swarm-nodes-manager-leader = leader
swarm-nodes-manager-reachable = raggiungibile
swarm-nodes-manager-unreachable = non raggiungibile
swarm-nodes-actionsFor = Azioni per { $name }
swarm-nodes-action-activate = Attiva
swarm-nodes-action-pause = Metti in pausa
swarm-nodes-action-drain = Svuota
swarm-nodes-action-promote = Promuovi a manager
swarm-nodes-action-demote = Retrocedi a worker
swarm-nodes-action-labels = Modifica le etichette
swarm-nodes-action-tasks = Mostra i suoi task
swarm-nodes-updated = { $name } aggiornato
swarm-nodes-removed = { $name } rimosso dallo Swarm
swarm-nodes-labels-title = Etichette di { $name }
swarm-nodes-labels-description = I vincoli di posizionamento come node.labels.zone==a scelgono i nodi in base a queste etichette.
swarm-nodes-labels-key = Chiave
swarm-nodes-labels-value = Valore
swarm-nodes-labels-add = Aggiungi un’etichetta
swarm-nodes-labels-removeOne = Rimuovi l’etichetta { $key }
swarm-nodes-remove-title = Rimuovere { $name } dallo Swarm?
swarm-nodes-remove-ready = Questo nodo è ancora attivo. Rimuoverlo forzatamente lascia il suo Engine convinto di far parte dello Swarm finché non ne esce da solo; i suoi task vengono ripianificati altrove.
swarm-nodes-remove-down = Questo nodo è inattivo. I suoi task vengono ripianificati sugli altri nodi.
swarm-nodes-remove-force = Forza: rimuovilo anche se è attivo
swarm-services-search = Cerca servizi
swarm-services-empty = Nessun servizio in esecuzione su questo Swarm
swarm-services-column-name = Nome
swarm-services-column-image = Immagine
swarm-services-column-mode = Modalità
swarm-services-column-replicas = In esecuzione
swarm-services-column-ports = Porte
swarm-services-column-stack = Stack
swarm-services-column-update = Aggiornamento
swarm-services-mode-replicated = Replicato
swarm-services-mode-replicatedCount = Replicato ({ $count })
swarm-services-mode-global = Globale
swarm-services-mode-replicatedJob = Job
swarm-services-mode-globalJob = Job globale
swarm-services-actionsFor = Azioni per { $name }
swarm-services-action-logs = Log
swarm-services-action-scale = Ridimensiona
swarm-services-action-rollback = Annulla aggiornamento
swarm-services-scaled = { $name } ridimensionato a { $count ->
        [one] { NUM($count) } replica
       *[other] { NUM($count) } repliche
    }
swarm-services-rolledBack = Rollback di { $name } in corso
swarm-services-removed = { $name } rimosso
swarm-services-scale-title = Ridimensiona { $name }
swarm-services-scale-description = Lo Swarm avvia o arresta task finché non ne sono in esecuzione questo numero. 0 arresta il servizio senza rimuoverlo.
swarm-services-scale-replicas = Repliche
swarm-services-scale-invalid = Inserisci un numero intero da 0 a 10000.
swarm-services-rollback-title = Annullare l’aggiornamento di { $name }?
swarm-services-rollback-description = Vengono ripristinate le sue impostazioni precedenti (immagine, ambiente, repliche…), e i suoi task passano a esse come avviene con un aggiornamento.
swarm-services-remove-title = Rimuovere il servizio { $name }?
swarm-services-remove-description = { $count ->
        [0] Al momento non esegue alcun task.
        [one] Il suo task in esecuzione viene arrestato e rimosso.
       *[other] I suoi { NUM($count) } task in esecuzione vengono arrestati e rimossi.
    } I suoi volumi vengono conservati.
swarm-service-tab-tasks = Task
swarm-service-tab-logs = Log
swarm-service-tab-inspect = Ispeziona
swarm-service-gone = Questo servizio non esiste più.
swarm-service-backToList = Torna ai servizi
swarm-tasks-column-task = Task
swarm-tasks-column-node = Nodo
swarm-tasks-column-state = Stato
swarm-tasks-column-error = Messaggio
swarm-tasks-column-image = Immagine
swarm-tasks-desired = dovrebbe essere { $state }
swarm-tasks-openContainer = Apri il container
swarm-tasks-history = Mostra la cronologia
swarm-tasks-onNode = Task su { $name }
swarm-tasks-allNodes = Mostra tutti i nodi
swarm-tasks-empty = Nessun task
swarm-objects-searchSecrets = Cerca segreti
swarm-objects-searchConfigs = Cerca config
swarm-objects-createSecret = Crea segreto
swarm-objects-createConfig = Crea config
swarm-objects-secretNote = Il valore di un segreto viene inviato allo Swarm una volta e non può essere riletto, né da Homeport né da nessun altro: lo vedono solo i servizi a cui viene assegnato.
swarm-objects-noSecrets = Nessun segreto
swarm-objects-noConfigs = Nessun config
swarm-objects-column-name = Nome
swarm-objects-column-services = Usato da
swarm-objects-column-labels = Etichette
swarm-objects-column-created = Creato
swarm-objects-column-updated = Aggiornato
swarm-objects-unused = Inutilizzato
swarm-objects-viewOne = Visualizza { $name }
swarm-objects-removeOne = Rimuovi { $name }
swarm-objects-removed = { $name } rimosso
swarm-objects-remove-title = Rimuovere { $name }?
swarm-objects-remove-used = Lo Swarm rifiuta finché lo usano dei servizi: { $services }. Prima rimuovilo da essi.
swarm-objects-remove-unused = Nessun servizio lo usa. L’operazione non può essere annullata.
swarm-objects-binary = Contenuto binario, { $size ->
        [one] { NUM($size) } byte
       *[other] { NUM($size) } byte
    }.
swarm-objects-create-secretTitle = Crea un segreto
swarm-objects-create-configTitle = Crea un config
swarm-objects-create-secretDescription = Il valore viene inviato allo Swarm e mai conservato da Homeport. I servizi lo leggono come file in /run/secrets/.
swarm-objects-create-configDescription = I servizi lo leggono come file ovunque lo montino. I config possono essere visualizzati di nuovo in seguito.
swarm-objects-create-name = Nome
swarm-objects-create-secretPlaceholder = db_password
swarm-objects-create-configPlaceholder = nginx_conf
swarm-objects-create-value = Valore
swarm-objects-create-content = Contenuto
swarm-objects-create-show = Mostra
swarm-objects-create-hide = Nascondi
swarm-objects-create-loadFile = Carica da file…
swarm-objects-create-pickFile = Scegli il file da inviare
swarm-objects-create-clearFile = Digita invece il valore
swarm-objects-create-fileNote = Il file viene letto quando lo crei, inviato allo Swarm e non conservato.
swarm-services-created = { $name } creato
swarm-services-updating = { $name } sta passando alle nuove impostazioni
swarm-service-update = aggiornamento { $state }
swarm-create-open = Crea servizio
swarm-create-title = Crea un servizio
swarm-create-description = Lo Swarm scarica l’immagine sui nodi che sceglie e mantiene in esecuzione questo numero di task. Posizionamento, limiti di risorse e controlli di integrità si possono impostare con la CLI docker e sono mostrati in Ispeziona.
swarm-create-image = Immagine
swarm-create-imagePlaceholder = nginx:1.27
swarm-create-name = Nome
swarm-create-mode = Modalità
swarm-create-restart = Riavvio
swarm-create-restart-default = Sempre (predefinito)
swarm-create-restart-onFailure = In caso di errore
swarm-create-restart-none = Mai
swarm-create-command = Comando
swarm-create-commandPlaceholder = Il comando dell’immagine
swarm-create-env = Ambiente
swarm-create-hostMode = Pubblica solo sul nodo che esegue ciascun task, invece che su tutti i nodi
swarm-create-networks = Reti overlay
swarm-create-secrets = Segreti
swarm-create-configs = Config
swarm-create-mountedAt = Dove viene montato { $name }
swarm-create-secretTarget = /run/secrets/{ $name }
swarm-create-configTarget = /{ $name }
swarm-create-submit = Crea
swarm-edit-open = Modifica
swarm-edit-title = Modifica { $name }
swarm-edit-description = Viene inviato solo ciò che modifichi qui; il resto del servizio viene mantenuto. Lo Swarm sostituisce i suoi task pochi alla volta con le nuove impostazioni.
swarm-edit-redeploy = Ridistribuisci ogni task, scaricando di nuovo l’immagine anche se il suo tag è lo stesso
swarm-edit-submit = Aggiorna
swarm-membership-card-title = Swarm
swarm-membership-card-none = Questo host non fa parte di uno Swarm.
swarm-membership-card-state = Il suo nodo è { $state }.
swarm-membership-card-ports = Avvia qui uno Swarm, o unisciti a uno esistente. I nodi di uno Swarm devono raggiungersi a vicenda su 2377/tcp, 7946/tcp e udp, e 4789/udp; Homeport non può aprire queste porte per te.
swarm-membership-created = Swarm creato
swarm-membership-joined = Unito allo Swarm
swarm-membership-left = Uscito dallo Swarm
swarm-membership-advertise = Indirizzo annunciato
swarm-membership-advertisePlaceholder = Scelto dall’Engine
swarm-membership-advertiseHelp = L’indirizzo a cui gli altri nodi raggiungono questo host. Lascialo vuoto per farlo scegliere all’Engine, il che funziona quando il computer ha un solo indirizzo.
swarm-membership-listen = Indirizzo di ascolto
swarm-membership-init-open = Inizializza uno Swarm
swarm-membership-init-title = Avvia uno Swarm su { $name }
swarm-membership-init-description = Questo host diventa il primo manager di un nuovo Swarm. Altri host potranno poi unirsi come worker o manager.
swarm-membership-init-submit = Avvia lo Swarm
swarm-membership-join-open = Unisciti a uno Swarm
swarm-membership-join-title = Unisci { $name } a uno Swarm
swarm-membership-join-description = Scegli uno Swarm che Homeport conosce, e Homeport recupera il suo token di adesione da un manager connesso. Per un altro Swarm, incolla il token e l’indirizzo di un manager ottenuti da docker swarm join-token.
swarm-membership-join-swarm = Swarm
swarm-membership-join-paste = Un altro Swarm, tramite il suo token
swarm-membership-join-token = Token di adesione
swarm-membership-join-address = Indirizzo del manager
swarm-membership-join-addressPlaceholder = 192.168.1.10:2377
swarm-membership-join-as = Unisci come
swarm-membership-join-submit = Unisci
swarm-membership-leave-open = Esci dallo Swarm
swarm-membership-leave-title = Far uscire { $name } dal suo Swarm?
swarm-membership-leave-worker = I suoi task si arrestano qui e vengono ripianificati sugli altri nodi. Il suo nodo resta elencato come inattivo finché un manager non lo rimuove.
swarm-membership-leave-manager = Smette di essere un manager e un nodo. Gli altri manager mantengono attivo lo Swarm; retrocederlo prima è più delicato.
swarm-membership-leave-last = Questo è l’ultimo manager dello Swarm: lo Swarm se ne va con esso, con ogni servizio, segreto e config. Gli altri nodi restano in uno Swarm senza manager.
swarm-membership-leave-force = Forza: esci anche se è un manager
swarm-membership-leave-typeName = Digita { $name } per confermare
swarm-membership-leave-submit = Esci
swarm-join-title = Comandi di adesione
swarm-join-description = Per i computer che Homeport non gestisce: esegui lì uno di questi comandi. Gli host gestiti da Homeport si uniscono invece dalla loro pagina, senza copiare nulla.
swarm-join-reveal = Mostra i comandi di adesione
swarm-join-hide = Nascondili
swarm-join-worker = Come worker
swarm-join-manager = Come manager
swarm-stacks-search = Cerca stack
swarm-stacks-new = Nuovo stack
swarm-stacks-deploy = Distribuisci
swarm-stacks-redeploy = Ridistribuisci
swarm-stacks-adopt = Adotta
swarm-stacks-link = Collega a una cartella…
swarm-stacks-linkTitle = Scegli la cartella con i file compose di { $name }
swarm-stacks-linked = { $name } ora è uno stack collegato
swarm-stacks-empty = Nessuno stack su questo Swarm
swarm-stacks-emptyHint = Qui compaiono gli stack distribuiti con docker stack deploy, così come quelli che distribuisci su questo Swarm da Homeport.
swarm-stacks-backToList = Torna agli stack
swarm-stacks-services = Servizi
swarm-stacks-noServices = Non è ancora stato distribuito nulla.
swarm-stacks-parts = Reti, segreti e config
swarm-stacks-networks = Reti
swarm-stacks-noStartStop = Uno stack swarm non può essere arrestato o avviato nel suo insieme: ridimensiona invece i suoi servizi a 0, dalle loro pagine.
swarm-stacks-removed = { $name } rimosso
swarm-stacks-removedPartly = { $name } rimosso, tranne:
swarm-stacks-remove-title = Rimuovere lo stack { $name }?
swarm-stacks-remove-description = Vengono rimossi i suoi servizi, poi le sue reti, segreti e config, come fa docker stack rm. I suoi volumi vengono conservati su ogni nodo.
