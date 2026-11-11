### The `hosts` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/hosts.ts:
### change the text there, then run it again.

hosts-title = Host
hosts-description = I Docker Engine a cui Homeport si connette.
hosts-add = Aggiungi host
hosts-empty = Ancora nessun host.
hosts-column-name = Nome
hosts-column-type = Tipo
hosts-column-address = Indirizzo
hosts-column-status = Stato
hosts-statusTitle = { $label }: { $detail }
hosts-saved = Host salvato
hosts-notFound = Questo host non esiste più.
hosts-addPage-description = Connetti Homeport a un Docker Engine.
hosts-switcher-label = Host
hosts-switcher-placeholder = Nessun host
hosts-switcher-add = Aggiungi host…
hosts-switcher-manage = Gestisci gli host
hosts-switcher-hosts = Host
hosts-switcher-clusters = Cluster
hosts-switcher-addCluster = Aggiungi cluster…
hosts-switcher-manageClusters = Gestisci i cluster
hosts-switcher-whyUnusable = Perché { $name } non può essere usato
hosts-detail-enabled = Attivato
hosts-detail-reconnect = Riconnetti
hosts-connection-title = Connessione
hosts-remove-title = Rimuovi l’host
hosts-remove-description = Homeport dimentica questo host, le sue credenziali e le definizioni dei suoi stack gestiti. Nulla di ciò che è in esecuzione sull’host viene toccato.
hosts-remove-confirmTitle = Rimuovere { $name }?
hosts-remove-confirmDescription = Elimina la connessione memorizzata, la chiave SSH o i certificati, e le definizioni compose dei suoi stack gestiti. Container, immagini e volumi sull’host restano come sono.
hosts-sshSetup-title = Autorizza Homeport sul server
hosts-sshSetup-description = Esegui questi comandi su <mono>{ $server }</mono> come utente SSH, poi riconnettiti.
hosts-sshSetup-descriptionBastions = Homeport accede a ciascuno di questi con la stessa chiave. Aggiungila su ognuno, come suo utente SSH, poi riconnettiti.
hosts-sshSetup-servers = Aggiungila su:
hosts-sshSetup-addKey = 1. Aggiungi la chiave pubblica
hosts-sshSetup-dockerGroup = 2. Consenti all’utente di raggiungere il socket dell’Engine (se necessario)
hosts-sshSetup-fingerprint = Impronta della chiave <mono>{ $fingerprint }</mono>
hosts-regenerate-button = Rigenera la chiave
hosts-regenerate-title = Rigenerare la chiave SSH?
hosts-regenerate-description = La chiave attuale smette di funzionare. Dovrai aggiungere la nuova chiave pubblica al server.
hosts-regenerate-confirm = Rigenera
hosts-trust-untrusted-title = È il server giusto?
hosts-trust-untrusted-description = Prima connessione. Il server si identifica con l’impronta della chiave <mono>{ $fingerprint }</mono>. Confrontala con <mono>{ $command }</mono> sul server.
hosts-trust-untrusted-button = Considera attendibile questa chiave
hosts-trust-untrusted-bastionTitle = È il bastion giusto?
hosts-trust-untrusted-bastionDescription = Prima connessione al bastion <mono>{ $bastion }</mono>. Si identifica con l’impronta della chiave <mono>{ $fingerprint }</mono>. Confrontala con <mono>{ $command }</mono> sul bastion.
hosts-trust-mismatch-bastionTitle = La chiave del bastion { $bastion } è cambiata
hosts-trust-mismatch-title = La chiave del server è cambiata
hosts-trust-mismatch-description = Era attesa <mono>{ $expected }</mono> ma il server ha presentato <mono>{ $fingerprint }</mono>. Succede dopo aver reinstallato il server, ma può anche significare che qualcuno sta intercettando la connessione.
hosts-trust-mismatch-button = Considera attendibile la nuova chiave
hosts-trust-mismatch-confirmTitle = Considerare attendibile la nuova chiave del server?
hosts-trust-mismatch-confirmDescription = Fallo solo se sai perché la chiave è cambiata. Nuova impronta: { $fingerprint }.
hosts-trust-mismatch-confirm = Considera attendibile
hosts-kind-ssh = SSH
hosts-kind-tls = TLS
hosts-kind-tcp = TCP
hosts-kind-local = Socket locale
hosts-form-keepHint = Lascia vuoto per mantenere il valore memorizzato.
hosts-form-name = Nome
hosts-form-engine = Engine
hosts-form-engineHint = Compatibile con Docker comprende Docker e Podman; Homeport li distingue quando si connette. containerd si raggiunge tramite il suo socket, qui o tramite SSH; Incus anche tramite HTTPS (TLS).
hosts-form-socketOnly = { $engine } risponde solo sul suo socket: connettiti in locale o tramite SSH.
hosts-form-noPlainTcp = { $engine } non risponde tramite TCP non cifrato: usa TLS, SSH o il suo socket.
hosts-form-socketPath = Percorso del socket
hosts-form-socketPathHint = Un socket Unix, o una named pipe come { $pipe } su Windows.
hosts-form-address = Indirizzo
hosts-form-addressHint = host:porta, per esempio { $example }.
hosts-form-tcpWarning-title = Non cifrato e non autenticato
hosts-form-tcpWarning-description = Chiunque possa raggiungere questa porta controlla l’Engine. Al di fuori di una rete attendibile preferisci SSH o TLS.
hosts-form-loadFromFile = Carica da file…
hosts-incus-trust = Attendibilità
hosts-incus-trust-token = Token di fiducia
hosts-incus-trust-certificate = Certificato esistente
hosts-incus-token = Token di fiducia
hosts-incus-tokenHint = Esegui questo comando sul server Incus e incolla ciò che stampa. Homeport crea un proprio certificato, verifica quello del server con il token e si registra.
hosts-incus-tokenReads = Per { $name }, su { $addresses }
hosts-incus-tokenFingerprint = Certificato del server { $fingerprint }
hosts-incus-tokenExpires = Scade { $time }
hosts-incus-serverCert = Certificato del server da considerare attendibile
hosts-incus-serverCert-choose = Scegli il certificato del server
hosts-incus-serverCertHint = Il certificato di Incus (/var/lib/incus/server.crt). Homeport accetta esattamente questo, qualunque sia l’indirizzo.
hosts-incus-clientCert = Certificato client
hosts-incus-clientCertHint = Già considerato attendibile da Incus: incus config trust add-certificate.
hosts-incus-clientKey = Chiave client
hosts-keychain-title = Nessun portachiavi del sistema operativo
hosts-keychain-description = Le chiavi e i certificati sono cifrati con una chiave conservata nel portachiavi del sistema operativo, e non ce n’è uno disponibile. Usa invece ssh-agent, un socket locale o TCP.
hosts-ssh-server = Server
hosts-ssh-port = Porta
hosts-ssh-user = Utente
hosts-ssh-userHint = Deve avere accesso al socket di Docker, di solito facendo parte del gruppo docker.
hosts-ssh-socketPath = Percorso del socket remoto
hosts-ssh-socketPathHint = Docker rootless usa { $path }.
hosts-ssh-auth = Autenticazione
hosts-ssh-auth-generated = Nuova chiave per questo host
hosts-ssh-auth-imported = Chiave esistente
hosts-ssh-generated-editing = Questo host mantiene la sua chiave. Rigenerala dalla pagina dell’host.
hosts-ssh-generated-new = Homeport crea una chiave Ed25519 per questo host e ti mostra il comando che la autorizza sul server.
hosts-ssh-privateKey = Chiave privata
hosts-ssh-privateKey-choose = Scegli la chiave privata
hosts-ssh-privateKeyHint = Memorizzata cifrata; non lascia mai questo computer.
hosts-ssh-passphrase = Passphrase
hosts-bastions-title = Bastion
hosts-bastions-description = Server a cui Homeport accede prima, in ordine, quando questo accetta SSH solo da loro (ProxyJump).
hosts-bastions-add = Aggiungi bastion
hosts-bastions-one = Bastion { $n }
hosts-bastions-server = Server del bastion { $n }
hosts-bastions-port = Porta del bastion { $n }
hosts-bastions-user = Utente del bastion { $n }
hosts-bastions-moveUp = Sposta su il bastion { $n }
hosts-bastions-moveDown = Sposta giù il bastion { $n }
hosts-bastions-remove = Rimuovi il bastion { $n }
hosts-bastions-auth-sameAsHost = Come l’host
hosts-bastions-auth-imported = Una propria chiave
hosts-bastions-auth-agent = ssh-agent
hosts-import-title = Importa
hosts-import-description = Aggiungi i server descritti dalla tua configurazione SSH, o gli Engine dei tuoi contesti Docker.
hosts-import-sshConfig-button = Importa da ~/.ssh/config
hosts-import-sshConfig-title = Importa da ~/.ssh/config
hosts-import-sshConfig-description = Ogni server indicato dalla tua configurazione SSH, con utente, porta, chiave e bastion ProxyJump. Le chiavi vengono lette una volta e memorizzate cifrate; il file non viene modificato.
hosts-import-sshConfig-empty = La tua configurazione SSH non indica alcun server senza caratteri jolly.
hosts-import-dockerContexts-button = Importa i contesti Docker
hosts-import-dockerContexts-title = Importa i contesti Docker
hosts-import-dockerContexts-description = I contesti della CLI di Docker, con i loro certificati TLS. I certificati vengono letti una volta e memorizzati cifrati; ~/.docker non viene modificato.
hosts-import-dockerContexts-empty = La CLI di Docker non ha contesti oltre a default.
hosts-import-column-name = Nome dell’host
hosts-import-column-address = Indirizzo e socket
hosts-import-column-auth = Accede con
hosts-import-column-notes = Note
hosts-import-pick = Importa { $name }
hosts-import-name = Nome dell’host per { $name }
hosts-import-passphrase = Passphrase della chiave di { $name }
hosts-import-socket = Percorso del socket remoto di { $name }
hosts-import-via = tramite { $bastions }
hosts-import-nameTaken = Un host ha già questo nome; rinominalo per importarlo.
hosts-import-auth-key = la sua chiave
hosts-import-auth-agent = ssh-agent
hosts-import-auth-certificates = certificati TLS
hosts-import-auth-none = nulla
hosts-import-problem-nameEmpty = Dagli un nome.
hosts-import-problem-nameTwice = Due host avrebbero questo nome.
hosts-import-problem-nameTaken = Scegli un altro nome.
hosts-import-problem-passphrase = Inserisci la passphrase della chiave.
hosts-import-problem-socket = Il percorso del socket deve essere assoluto.
hosts-import-submit = { $count ->
        [0] Importa
        [one] Importa { NUM($count) } host
       *[other] Importa { NUM($count) } host
    }
hosts-import-failed = { $name } non è stato importato:
hosts-import-done = { $count ->
        [one] { NUM($count) } host importato
       *[other] { NUM($count) } host importati
    }
hosts-bundle-fileType = Esportazione di Homeport
hosts-bundle-passphrase = Passphrase
hosts-bundle-passphraseHint = Almeno { $min } caratteri. Homeport non la conserva: senza di essa, il file non può essere aperto.
hosts-bundle-confirmation = Ripeti la passphrase
hosts-bundle-problem-short = Usa almeno { $min } caratteri.
hosts-bundle-problem-mismatch = Le due passphrase sono diverse.
hosts-bundle-export-button = Esporta
hosts-bundle-export-title = Esporta host
hosts-bundle-export-description = Scrive gli host che scegli, con le loro chiavi, certificati e chiavi dei server attendibili, in un unico file cifrato con una passphrase. Importalo su un altro computer per usarli lì.
hosts-bundle-export-hosts = Host da esportare
hosts-bundle-export-saveTitle = Salva l’esportazione
hosts-bundle-export-submit = { $count ->
        [0] Esporta
        [one] Esporta { NUM($count) } host…
       *[other] Esporta { NUM($count) } host…
    }
hosts-bundle-export-done = { $count ->
        [one] { NUM($count) } host esportato
       *[other] { NUM($count) } host esportati
    }
hosts-bundle-export-withoutSecrets = Questi sono stati esportati senza le loro credenziali, che non possono essere aperte su questo computer:
hosts-bundle-import-button = Importa
hosts-bundle-import-title = Importa host da un’esportazione
hosts-bundle-import-description = Apri un file esportato da Homeport con la sua passphrase, poi scegli gli host da aggiungere. Le loro credenziali vengono memorizzate cifrate su questo computer.
hosts-bundle-import-choose = Scegli file…
hosts-bundle-import-chooseTitle = Scegli un’esportazione di Homeport
hosts-bundle-import-open = Apri
hosts-bundle-import-renamed = Un host si chiama già { $name }, quindi questo viene rinominato.
hosts-bundle-import-secretsMissing = Esportato senza le sue credenziali; inseriscile nella sua pagina dopo l’importazione.
hosts-tls-ca = Certificato CA
hosts-tls-ca-choose = Scegli il certificato CA
hosts-tls-cert = Certificato client
hosts-tls-cert-choose = Scegli il certificato client
hosts-tls-key = Chiave client
hosts-tls-key-choose = Scegli la chiave client
hosts-palette-add = Aggiungi host
hosts-palette-manage = Gestisci gli host
