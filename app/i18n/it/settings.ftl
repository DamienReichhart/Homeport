### The `settings` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/settings.ts:
### change the text there, then run it again.

settings-nav = Impostazioni
settings-title = Impostazioni
settings-appearance-title = Aspetto
settings-appearance-theme = Tema
settings-theme-system = Come il sistema
settings-theme-light = Chiaro
settings-theme-dark = Scuro
settings-appearance-language = Lingua
settings-language-system = Come il sistema ({ $language })
settings-desktop-title = Desktop
settings-desktop-description = Come funziona Homeport su questo computer.
settings-desktop-closeToTray = Chiudi nell’area di notifica
settings-desktop-closeToTrayHint = Chiudendo la finestra, Homeport e le sue connessioni agli host restano attivi nell’area di notifica. Esci dall’icona nell’area di notifica. Sui desktop che nascondono le icone dell’area di notifica, come GNOME senza un’estensione, l’icona potrebbe non comparire.
settings-desktop-noTray = Questo desktop non ha un’area di notifica che Homeport possa usare, quindi chiudere la finestra lo fa uscire.
settings-desktop-startAtLogin = Avvia all’accesso
settings-desktop-startAtLoginHint = Homeport si avvia quando accedi a questo computer, così controlla i tuoi host fin dall’inizio.
settings-desktop-startHidden = Avvia nascosto nell’area di notifica
settings-desktop-startHiddenHint = Quando si avvia all’accesso, Homeport resta nell’area di notifica senza aprire la finestra.
settings-desktop-startHiddenNoTray = Richiede un’area di notifica da cui aprire la finestra, che questo desktop non ha.
settings-notifications-title = Notifiche
settings-notifications-description = Homeport ti avvisa, anche con la finestra chiusa nell’area di notifica, quando:
settings-notifications-containerDied = Un container si arresta con un errore o esaurisce la memoria
settings-notifications-containerUnhealthy = Un container diventa non integro
settings-notifications-hostUnreachable = Un host connesso diventa non raggiungibile
settings-notifications-hostBack = Un host non raggiungibile torna disponibile
settings-machine-title = Questo computer
settings-machine-description = Cosa può usare Homeport qui.
settings-machine-keychain = Portachiavi del sistema operativo
settings-machine-keychainAvailable = Disponibile — chiavi SSH, certificati, ambienti degli stack e accessi ai registri vengono memorizzati cifrati.
settings-machine-keychainUnavailable = Non disponibile — chiavi SSH, certificati TLS, ambienti degli stack e accessi ai registri non possono essere memorizzati.
settings-machine-keychainEntryMissing = Voce mancante: il portachiavi non ha una voce per la chiave con cui sono stati sigillati i segreti di Homeport, quindi non possono essere aperti né memorizzati.
settings-machine-startOverDescription = Se Homeport sta usando un portachiavi diverso da prima, torna a quello e riavvia Homeport. Se la voce è sparita definitivamente, ricomincia con una nuova chiave.
settings-machine-startOver = Ricomincia con una nuova chiave
settings-machine-startOverTitle = Ricominciare con una nuova chiave?
settings-machine-startOverConfirmDescription = Una nuova chiave non può aprire ciò che è stato sigillato con quella mancante: dovrai inserire di nuovo le credenziali di ogni host, cluster, registro e repository, e l’ambiente di ogni stack, oppure ripristinare un backup.
settings-machine-compose = CLI di Compose
settings-machine-composeMissing = Gli stack possono essere avviati e arrestati, ma non distribuiti.
settings-machine-stackDeploy = CLI di Docker, per gli Swarm
settings-machine-stackDeployMissing = Gli stack non possono essere distribuiti su uno Swarm; tutto il resto degli Swarm funziona.
settings-machine-git = git
settings-machine-helm = CLI di Helm
settings-sync-title = Stack sincronizzati
settings-sync-interval = Controlla i repository ogni
settings-sync-minutes = minuti
settings-data-title = Dati
settings-data-description = Cosa conserva Homeport su questo computer.
settings-data-backup = Esegui backup…
settings-data-restore = Ripristina…
settings-data-backupHint = Salva ogni host, stack, accesso al registro, repository e impostazione, con i loro segreti, in un unico file protetto da passphrase, e ripristinalo su questo o su un altro computer.
settings-backup-title = Backup dei dati di Homeport
settings-backup-description = Il file contiene ogni host, cluster, stack, accesso al registro, repository e impostazione, e le loro chiavi, certificati, token, password e ambienti. Chiunque abbia il file e la sua passphrase può usarli, quindi custodisci entrambi. Gli eventi degli Engine e la cronologia delle distribuzioni non sono inclusi.
settings-backup-passphrase = Passphrase
settings-backup-passphraseHint = Almeno { $min } caratteri. Non può essere recuperata.
settings-backup-confirmation = Ripeti la passphrase
settings-backup-problem-short = Usa almeno { $min } caratteri.
settings-backup-problem-mismatch = Le due passphrase sono diverse.
settings-backup-start = Esegui backup…
settings-backup-saveTitle = Salva il backup come
settings-backup-fileType = Backup di Homeport
settings-backup-done = Backup di { $hosts ->
        [one] { NUM($hosts) } host
       *[other] { NUM($hosts) } host
    } e { $stacks ->
        [one] { NUM($stacks) } stack
       *[other] { NUM($stacks) } stack
    } eseguito
settings-backup-withoutSecrets = Questi elementi sono stati salvati senza i loro segreti, che non è stato possibile aprire:
settings-restore-pick = Ripristina da
settings-restore-title = Ripristina i dati di Homeport
settings-restore-from = Da { $path }
settings-restore-open = Apri
settings-restore-madeBy = Backup eseguito da Homeport { $version } il { $date }. Contiene:
settings-restore-holds = Contenuto del backup
settings-restore-hosts = { $count ->
        [one] { NUM($count) } host
       *[other] { NUM($count) } host
    }
settings-restore-clusters = { $count ->
        [one] { NUM($count) } cluster
       *[other] { NUM($count) } cluster
    }
settings-restore-stacks = { $count ->
        [one] { NUM($count) } stack
       *[other] { NUM($count) } stack
    }
settings-restore-registries = { $count ->
        [one] { NUM($count) } accesso al registro
       *[other] { NUM($count) } accessi ai registri
    }
settings-restore-repositories = { $count ->
        [one] { NUM($count) } repository
       *[other] { NUM($count) } repository
    }
settings-restore-settings = Le tue impostazioni
settings-restore-withoutSecrets = Questi sono stati salvati senza i loro segreti, che allora non è stato possibile aprire. Tornano senza di essi, al posto dei segreti che hanno qui, e gli accessi ai registri tra questi non tornano affatto:
settings-restore-withoutSecretsList = Salvati senza i loro segreti
settings-restore-replaces = Il ripristino sostituisce tutti gli host, gli stack, gli accessi ai registri, i repository e le impostazioni che hai ora, e dimentica gli eventi degli Engine e la cronologia delle distribuzioni. Prima viene conservata una copia dei dati attuali nella cartella dei dati di Homeport.
settings-restore-confirm = Sostituisci tutto
settings-restore-done = { $hosts ->
        [one] { NUM($hosts) } host
       *[other] { NUM($hosts) } host
    } e { $stacks ->
        [one] { NUM($stacks) } stack
       *[other] { NUM($stacks) } stack
    } ripristinati
settings-data-keepEvents = Conserva gli eventi degli Engine per
settings-data-days = giorni
settings-data-keepEventsHint = Ogni host conserva al massimo 10.000 eventi per la pagina Eventi. Quelli più vecchi vengono rimossi ogni ora.
settings-machine-gitMissing = Serve agli stack sincronizzati; tutti gli altri tipi di stack funzionano senza.
settings-machine-helmMissing = Le release Helm non possono essere elencate né installate; tutto il resto dei cluster funziona.
settings-about-tagline = Ogni Docker Engine, in un’unica bacheca.
settings-about-openLogFolder = Apri la cartella dei log
settings-about-licenses = Licenze
settings-about-exportDiagnostics = Esporta diagnostica…
settings-diagnostics-title = Esporta diagnostica
settings-diagnostics-description = Un file zip da allegare a una segnalazione di bug: un riepilogo delle versioni e dei tipi di host e cluster che usi, e i log di Homeport senza i segreti. Non viene inviato nulla: decidi tu se condividere il file.
settings-diagnostics-includeNames = Includi nomi e indirizzi
settings-diagnostics-includeNamesHelp = Se disattivato, host e cluster compaiono come host-1, cluster-1… nel riepilogo e nei log.
settings-diagnostics-files = File nello zip
settings-diagnostics-save = Salva…
settings-diagnostics-saveTitle = Salva diagnostica
settings-diagnostics-fileType = Archivio zip
settings-diagnostics-done = Diagnostica salvata in { $path }.
settings-licenses-title = Licenze
settings-licenses-description = Homeport è software libero distribuito con la GNU Affero General Public License, versione 3, e si basa sui pacchetti open source elencati qui sotto.
settings-licenses-source = Il codice sorgente di questa versione è pubblicato con la stessa licenza.
settings-licenses-sourceLink = Sorgente della v{ $version }
settings-licenses-thirdParty = Software di terze parti
settings-licenses-missing-title = Non inclusi in questa build
settings-licenses-missing-description = Le build di rilascio li includono; esegui <code>{ $command }</code> per generarli in una build di sviluppo.
settings-palette-group = Impostazioni
settings-palette-themeLight = Tema: chiaro
settings-palette-themeDark = Tema: scuro
settings-palette-themeSystem = Tema: come il sistema
