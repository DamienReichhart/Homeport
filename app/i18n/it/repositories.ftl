### The `repositories` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/repositories.ts:
### change the text there, then run it again.

repositories-title = Repository
repositories-description = Repository git da cui vengono distribuiti gli stack sincronizzati, su ogni host. Homeport si limita a leggerli.
repositories-add = Aggiungi repository
repositories-empty = Ancora nessun repository. Aggiungine uno per creare uno stack sincronizzato.
repositories-manageOne = Gestisci l’accesso a { $repository }
repositories-removeOne = Rimuovi { $repository }
repositories-remove-title = Rimuovere { $repository }?
repositories-remove-description = Homeport dimentica la sua chiave di deploy o il suo token ed elimina il suo clone su questo computer. Il repository stesso non viene toccato. Rimuovi la chiave di deploy anche dal repository sulla tua forge.
repositories-noKeychain = Le chiavi di deploy e i token sono cifrati con una chiave conservata nel portachiavi del sistema operativo, e non ce n’è uno disponibile, quindi puoi aggiungere solo repository pubblici.
repositories-public = Un repository pubblico: Homeport lo legge senza credenziali.
repositories-added = { $repository } aggiunto
repositories-removed = { $repository } rimosso
repositories-keyRotated = Nuova chiave di deploy creata. Aggiungila al repository al posto di quella vecchia.
repositories-tokenReplaced = Token sostituito
repositories-hostKeyReset = La prossima volta Homeport considererà attendibile la chiave presentata dal server.
repositories-auth-withKey = Con una chiave di deploy, su { $host }
repositories-auth-withToken = Come { $username } con un token, su { $host }
repositories-auth-public = Pubblico, su { $host }
repositories-dialog-addTitle = Aggiungi un repository
repositories-dialog-description = Homeport lo clona su questo computer per distribuire da lì gli stack sincronizzati. Non esegue mai commit né push.
repositories-dialog-url = URL del repository
repositories-dialog-auth = Accesso
repositories-dialog-authKey = Chiave di deploy
repositories-dialog-authToken = Token
repositories-dialog-authNone = Pubblico
repositories-dialog-authKeyHelp = Homeport crea una chiave solo per questo repository. La aggiungi tu al repository con accesso in lettura; le tue chiavi SSH non vengono mai usate.
repositories-dialog-authTokenHelp = Un token di accesso che può leggere il contenuto del repository, inviato tramite HTTPS. Cifrato su questo computer e mai più mostrato.
repositories-dialog-authNoneHelp = Per un repository pubblico tramite HTTPS. Non viene memorizzato nulla.
repositories-dialog-username = Nome utente
repositories-dialog-token = Token di accesso
repositories-dialog-add = Aggiungi
repositories-dialog-addWithKey = Aggiungi e crea una chiave di deploy
repositories-missing-url = Inserisci l’URL del repository.
repositories-missing-validUrl = Inserisci un URL SSH o HTTPS, come git@github.com:owner/repo.git o https://gitlab.com/owner/repo.git.
repositories-missing-auth = Scegli come Homeport raggiunge questo repository.
repositories-missing-username = Inserisci il nome utente a cui appartiene il token.
repositories-missing-token = Inserisci il token di accesso.
repositories-deployKey-title = Chiave di deploy
repositories-deployKey-github = GitHub: nel repository, <strong>Settings → Deploy keys → Add deploy key</strong>.
repositories-deployKey-gitlab = GitLab: nel progetto, <strong>Settings → Repository → Deploy keys</strong>.
repositories-deployKey-gitea = Gitea e Forgejo: nel repository, <strong>Settings → Deploy keys → Add deploy key</strong>.
repositories-deployKey-readOnly = Lascia disattivato l’accesso in scrittura: Homeport si limita a leggere. Poi verifica l’accesso qui sotto.
repositories-deployKey-fingerprint = Impronta: <mono>{ $fingerprint }</mono>
repositories-rotate-button = Nuova chiave
repositories-rotate-title = Creare una nuova chiave di deploy?
repositories-rotate-description = La chiave attuale smette di funzionare per Homeport. Aggiungi quella nuova al repository e rimuovi lì quella vecchia.
repositories-rotate-confirm = Crea una nuova chiave
repositories-token-replace = Sostituisci il token
repositories-token-save = Salva il token
repositories-hostKey-trusted = Chiave attendibile di { $host }: <mono>{ $fingerprint }</mono>
repositories-hostKey-reset = Reimposta
repositories-hostKey-resetTitle = Reimpostare la chiave attendibile di { $host }?
repositories-hostKey-resetDescription = Fallo solo se { $host } ha cambiato la sua chiave SSH e hai verificato la nuova impronta sul suo sito web. Homeport considererà attendibile qualsiasi chiave presenti la prossima volta.
repositories-test-button = Verifica l’accesso
repositories-test-testing = Verifica in corso…
repositories-test-success = Homeport può leggerlo: { $branches ->
        [one] { NUM($branches) } branch
       *[other] { NUM($branches) } branch
    } e { $tags ->
        [one] { NUM($tags) } tag
       *[other] { NUM($tags) } tag
    }.
