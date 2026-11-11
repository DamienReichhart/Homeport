---
title: I tuoi dati e i backup
description: Dove Homeport conserva i suoi dati su ogni sistema, come farne il backup e ripristinarli, e cosa inserire di nuovo quando i segreti vanno persi.
---

Homeport conserva ciò che sa sul tuo computer: come raggiungere i tuoi host e cluster, le definizioni e gli ambienti dei tuoi stack, i tuoi accessi ai registri, i repository e le impostazioni. Questa pagina dice dove si trova tutto questo, cosa copiare e come recuperarlo. Per come è protetto, vedi [La sicurezza e i tuoi dati](/docs/reference/security/).

Ciò che è in esecuzione sui tuoi host e cluster non fa parte dei dati di Homeport: container, immagini, volumi e oggetti Kubernetes o Nomad restano dove sono, e nessun backup di Homeport li contiene. Per copiare il contenuto di un volume, [esegui il backup in un file](/docs/guides/volumes-networks/#backup-e-ripristino) dalla pagina Volumi.

## Cosa si trova dove

### La cartella dei dati

| Sistema | Cartella                                          |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

Su Linux la cartella si trova sotto `$XDG_DATA_HOME` quando è impostata. Se Homeport non riesce ad aprire il suo database, la sua finestra mostra il percorso completo del file, con **Apri la cartella**.

| Nella cartella                       | Cos’è                                                                                                                                                                                       |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `homeport.db`                        | Il database: ogni host, cluster, stack, accesso al registro, repository e impostazione, la cronologia delle distribuzioni e gli eventi degli Engine. I segreti al suo interno sono cifrati. |
| `homeport.db-wal`, `homeport.db-shm` | Il write-ahead log di SQLite e il suo indice, presenti mentre Homeport è in esecuzione. Le modifiche più recenti possono trovarsi solo nel file `-wal` finché Homeport non viene chiuso.    |
| `homeport.db.before-restore-<date>`  | Una copia del database fatta subito prima di ogni [ripristino](#ripristinare-un-backup). Elimina quelle che non ti servono più.                                                             |
| `repositories/` e `synced-stacks/`   | I cloni e le copie di lavoro dei repository degli [stack sincronizzati](/docs/guides/synced-stacks/). Homeport li ricrea dai repository stessi.                                             |

[Cosa memorizza Homeport](/docs/reference/security/#cosa-memorizza-homeport) elenca cosa contiene il database, e quali parti sono cifrate.

### La voce del portachiavi

La chiave che cifra i segreti non si trova nella cartella. È una voce nel portachiavi del tuo sistema operativo, con il servizio `com.homeport.app` e l’account `secrets-key`: in **Accesso Portachiavi** su macOS, **Gestione credenziali → Credenziali di Windows** su Windows, e **Password e chiavi** (Seahorse) o KWallet su Linux. Appartiene al tuo account utente su questo computer.

Il database e quella voce vanno insieme. Un database senza la sua chiave si apre comunque, con ogni nome, indirizzo e file compose, ma nessuno dei suoi segreti può essere decifrato: host e cluster mostrano **Credenziali perse**. Per questo una copia della cartella non basta per passare a un altro computer, e per questo il [backup nell’app](#eseguire-il-backup-in-homeport) è quello consigliato.

### I log e la posizione della finestra

| Sistema | Log                                     | Dimensioni e posizione della finestra                               |
| ------- | --------------------------------------- | ------------------------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Logs/com.homeport.app/`      | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` | `%APPDATA%\com.homeport.app\.window-state.json`                     |

Non vale la pena fare il backup di nessuno dei due. **Impostazioni → Informazioni → Apri la cartella dei log** apre i log.

### Cosa resta tuo

- **Le cartelle degli stack collegati.** Homeport conserva solo dove si trovano la cartella e i file compose di uno [stack collegato](/docs/guides/stacks/#stack-collegati). La cartella, i suoi file compose e il suo `.env` sono tuoi: fanne il backup come fai con gli altri tuoi progetti, di solito con git.
- **Il tuo kubeconfig, `~/.ssh` e `~/.docker`.** Homeport li legge quando importi da essi e non li modifica mai.
- **I repository di chart e gli accessi di helm.** Sono di helm, condivisi con il tuo terminale.

## Eseguire il backup in Homeport

**Impostazioni → Dati → Esegui backup…** scrive tutto in un unico file `.homeport`. È il backup da preferire, perché contiene i segreti stessi, protetti da una passphrase invece che dal portachiavi di questo computer: si ripristina su qualsiasi computer.

1. Apri **Impostazioni**, e sotto **Dati** scegli **Esegui backup…**.
2. Digita due volte una passphrase di almeno 12 caratteri. Homeport non la memorizza mai, e nulla può aprire il file senza di essa.
3. Scegli dove salvare il file.

Homeport dice poi cosa ha salvato, e nomina ciò che ha dovuto salvare senza il suo segreto perché quel segreto non poteva essere decifrato su questo computer.

| Nel backup                                                                                                    | Non nel backup                                                        |
| ------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| Gli host, con le loro chiavi, passphrase, certificati, bastion e le chiavi dei server considerate attendibili | Gli eventi degli Engine                                               |
| I cluster, con i loro certificati, token e impostazioni dei plugin exec                                       | La cronologia delle distribuzioni                                     |
| I file compose e gli ambienti degli stack gestiti                                                             | Le cartelle e i file degli stack collegati                            |
| I percorsi delle cartelle e dei file degli stack collegati                                                    | I cloni degli stack sincronizzati (ricreati alla sincronizzazione)    |
| Il repository, il branch o il tag, le impostazioni di sincronizzazione e l’ambiente degli stack sincronizzati | I log, e le dimensioni e la posizione della finestra                  |
| Gli stack distribuiti su uno Swarm                                                                            | Ciò che si trova sui tuoi host e cluster: container, immagini, volumi |
| Gli accessi ai registri, con le loro password e i loro token                                                  | I repository di chart di helm                                         |
| I repository, con le loro chiavi di deploy e i loro token                                                     | Gli inoltri di porta, che terminano quando Homeport viene chiuso      |
| Le tue impostazioni                                                                                           |                                                                       |

L’intero file è cifrato con la passphrase, quindi senza di essa il file non rivela nulla, nemmeno il nome di un host. Contiene ogni segreto di Homeport: conservalo, insieme alla sua passphrase, al sicuro quanto le chiavi stesse. [Backup](/docs/reference/security/#backup) descrive la cifratura.

Esegui un backup prima di provare una prerelease, prima di reinstallare il sistema, e ogni volta che hai aggiunto host che non vorresti configurare di nuovo.

### Ripristinare un backup

1. Su questo o su un altro computer, apri **Impostazioni**, e sotto **Dati** scegli **Ripristina…**.
2. Scegli il file `.homeport` e digita la sua passphrase. Homeport elenca ciò che contiene il file, e quale Homeport l’ha creato e quando, prima di modificare qualsiasi cosa.
3. Scegli **Sostituisci tutto**.

Il ripristino sostituisce tutti i dati di Homeport con quelli del backup; non li unisce. Homeport copia prima il database attuale in `homeport.db.before-restore-<date>` nella cartella dei dati, poi sostituisce tutto in un solo passaggio, così un errore lascia i tuoi dati com’erano. Ogni segreto viene cifrato di nuovo con la chiave del portachiavi di questo computer.

Dopo:

- Host e cluster si riconnettono subito, senza chiedere una chiave, una password o una chiave del server da considerare attendibile.
- Uno stack collegato la cui cartella non è su questo computer, allo stesso percorso, appare come esterno finché non c’è.
- Gli stack sincronizzati clonano di nuovo il loro repository alla sincronizzazione successiva.
- La pagina Eventi e la cronologia delle distribuzioni ripartono vuote.

Un backup creato da un Homeport più vecchio si ripristina in uno più recente. Uno creato da un Homeport più recente viene rifiutato: aggiorna prima Homeport.

Per tornare a ciò che avevi prima di un ripristino, esci da Homeport, rinomina `homeport.db` in qualcos’altro, rinomina in `homeport.db` il `homeport.db.before-restore-<date>` che vuoi, elimina eventuali `homeport.db-wal` e `homeport.db-shm` rimasti accanto, e apri di nuovo Homeport.

### Spostare solo alcuni host

**Esporta** e **Importa** nella pagina Host spostano gli host che scegli, con i loro segreti, in un file dello stesso tipo, e li aggiungono agli host che l’altro computer ha già. Vedi [passare a un altro computer](/docs/reference/security/#passare-a-un-altro-computer).

## Copiare tu stesso i file

Una copia del database è adatta a un backup di sistema di questo computer, come Time Machine, Cronologia file o un tuo script. Conserva ciò che il backup nell’app esclude, gli eventi degli Engine e la cronologia delle distribuzioni, ma **non la chiave del portachiavi**: si ripristina con i suoi segreti solo per lo stesso utente sullo stesso computer, o su uno il cui portachiavi è stato ripristinato insieme a essa. Altrove, ogni nome, indirizzo e file compose torna, e i segreti vanno [inseriti di nuovo](#se-i-segreti-vanno-persi).

### Mentre Homeport è chiuso

Esci da Homeport, da **Esci** nell’area di notifica se è [chiuso nell’area di notifica](/docs/guides/settings/#desktop), poi copia l’intera cartella dei dati. Una volta chiuso Homeport, `homeport.db` contiene tutto e i file `-wal` e `-shm` non ci sono più.

### Mentre Homeport è in esecuzione

Non copiare solo `homeport.db` mentre Homeport è in esecuzione: le modifiche più recenti possono trovarsi ancora in `homeport.db-wal`, e una copia fatta durante una scrittura può essere incoerente. Chiedi invece a SQLite una copia coerente, con lo strumento a riga di comando `sqlite3` (incluso in macOS; su Linux e Windows, installalo dal pacchetto `sqlite3` della tua distribuzione o da [sqlite.org](https://sqlite.org/download.html)):

```sh
# Linux
sqlite3 ~/.local/share/com.homeport.app/homeport.db ".backup '$HOME/homeport-backup.db'"

# macOS
sqlite3 "$HOME/Library/Application Support/com.homeport.app/homeport.db" ".backup '$HOME/homeport-backup.db'"
```

```powershell
# Windows
sqlite3 "$env:APPDATA\com.homeport.app\homeport.db" ".backup '$HOME\homeport-backup.db'"
```

Il risultato è un unico file completo, senza alcun `-wal` accanto. Per verificarlo:

```sh
sqlite3 ~/homeport-backup.db "PRAGMA integrity_check;"
```

Stampa `ok`.

### Ripristinare una copia

1. Esci da Homeport.
2. Nella cartella dei dati, sposta altrove `homeport.db`, ed elimina `homeport.db-wal` e `homeport.db-shm` se ci sono: appartengono al database che stai sostituendo.
3. Metti lì la tua copia, con il nome `homeport.db`. Se hai copiato l’intera cartella, rimetti invece la cartella.
4. Apri Homeport.

Homeport aggiorna una copia creata da una versione più vecchia quando la apre. Una copia usata per ultima da un Homeport più recente viene rifiutata, intatta, con [un messaggio che indica la versione da installare](/docs/reference/troubleshooting/#questo-database-è-stato-usato-lultima-volta-da-homeport--o-da-una-versione-successiva).

## Se i segreti vanno persi

Dopo un ripristino del portachiavi, un nuovo account utente, o un database copiato aperto su un altro computer, Homeport conserva tutto ciò che riesce ancora a leggere e segnala ciò che non riesce. Nulla viene eliminato, e ciò che non è un segreto (nomi, indirizzi, file compose, percorsi delle cartelle) è intatto. Inserisci di nuovo:

| Cosa                                    | Come                                                                                                                                                                                    |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Host TLS**                            | Nella pagina dell’host, carica di nuovo il certificato della CA, il certificato client e la chiave client, e scegli **Salva**.                                                          |
| **Host SSH con una chiave importata**   | Incolla di nuovo la chiave privata e la sua passphrase, e scegli **Salva**.                                                                                                             |
| **Host SSH con una chiave di Homeport** | Scegli **Rigenera la chiave**, poi [autorizza la nuova chiave](/docs/hosts/ssh/#autorizza-la-chiave-di-homeport) sul server. Rimuovi lì la vecchia riga `homeport@`.                    |
| **Host Incus tramite HTTPS**            | Rimuovi l’host e [aggiungilo di nuovo con un nuovo token di fiducia](/docs/hosts/incus/#tramite-https-con-un-token-di-fiducia).                                                         |
| **Cluster Kubernetes**                  | **Aggiorna dal kubeconfig** nella pagina del cluster.                                                                                                                                   |
| **Cluster Nomad**                       | Incolla il token ACL, e i certificati se li usa, nelle **Impostazioni** del cluster.                                                                                                    |
| **Ambienti degli stack**                | Digita di nuovo l’ambiente di uno stack gestito o sincronizzato nella sua pagina. Gli stack collegati conservano il loro nel `.env` della loro cartella, che Homeport non ha mai avuto. |
| **Accessi ai registri**                 | In **Impostazioni → Registri**, modifica ogni accesso e digita la sua password o il suo token.                                                                                          |
| **Repository**                          | In **Impostazioni → Repository**, crea una **Nuova chiave** e aggiungila al repository sulla tua forge, oppure **Sostituisci il token**.                                                |

Gli host con socket locale, TCP e ssh-agent non contengono alcun segreto e continuano a funzionare. Gli ambienti registrati nella cronologia delle distribuzioni non possono essere inseriti di nuovo; i file compose della cronologia restano leggibili.

Se invece Homeport segnala che manca la voce del portachiavi con cui sono stati sigillati i suoi segreti, non vi ha ancora rinunciato: torna al portachiavi che contiene la voce, oppure scegli **Ricomincia con una nuova chiave** in **Impostazioni → Questo computer** prima di inserirli di nuovo.

Vedi [Credenziali perse](/docs/reference/troubleshooting/#credenziali-perse) nella risoluzione dei problemi.

## Rimuovere i tuoi dati

[Rimuovere i tuoi dati](/docs/reference/updating/#rimuovere-i-tuoi-dati) elenca le cartelle e la voce del portachiavi da eliminare, e le chiavi da rimuovere dai tuoi server.
