### The `registries` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/registries.ts:
### change the text there, then run it again.

registries-title = Registri
registries-description = Accessi ai registri di container, usati per scaricare immagini private, inviare e compilare su ogni host.
registries-add = Aggiungi accesso
registries-noKeychain-title = Nessun portachiavi del sistema operativo
registries-noKeychain-description = Le password e i token dei registri sono cifrati con una chiave conservata nel portachiavi del sistema operativo, e non ce n’è uno disponibile. Le immagini possono comunque essere scaricate in forma anonima.
registries-empty = Ancora nessun accesso: le immagini vengono scaricate in forma anonima.
registries-editOne = Modifica l’accesso per { $registry }
registries-removeOne = Rimuovi l’accesso per { $registry }
registries-remove-title = Rimuovere l’accesso per { $registry }?
registries-remove-description = Le immagini da { $registry } verranno scaricate in forma anonima finché non aggiungi di nuovo un accesso.
registries-saved = Accesso per { $registry } salvato
registries-removed = Accesso per { $registry } rimosso
registries-dialog-editTitle = Modifica l’accesso per { $registry }
registries-dialog-addTitle = Aggiungi un accesso al registro
registries-dialog-description = Usato per scaricare, inviare e compilare su ogni host. La password o il token viene cifrato su questo computer e non viene più mostrato.
registries-dialog-address = Registro
registries-dialog-username = Nome utente
registries-dialog-secret = Password o token di accesso
registries-dialog-keepSecret = Lascia vuoto per mantenere quello memorizzato
registries-dialog-accepted = { $registry } ha accettato questo accesso.
registries-dialog-checking = Verifica in corso…
registries-dialog-check = Verifica l’accesso
registries-missing-address = Inserisci l’indirizzo del registro.
registries-missing-username = Inserisci il nome utente.
registries-missing-secret = Inserisci la password o il token di accesso.
registries-login-as = come { $username } su { $registry }
registries-login-anonymously = in forma anonima
registries-login-pulledAs = Scaricata come { $username } su { $registry }
registries-login-pulledAnonymously = Scaricata in forma anonima
registries-login-pushedAs = Inviata come { $username } su { $registry }
registries-login-pushedAnonymously = Inviata in forma anonima
registries-noLogin-title = Nessun accesso per { $registry }
registries-noLogin-description = La maggior parte dei registri rifiuta gli invii anonimi. Prima <link>aggiungi un accesso in Impostazioni → Registri</link>.
