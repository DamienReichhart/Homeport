---
title: Release Helm
description: Elenca, installa, aggiorna, ripristina e disinstalla le release Helm di un cluster, e gestisci i repository di chart, tramite la CLI helm di questo computer.
---

**Helm**, nella barra laterale di un cluster, mostra le release [Helm](https://helm.sh) presenti su di esso: le stesse release che `helm list` mostra in un terminale, che siano state installate da Homeport o no.

## Di cosa ha bisogno Homeport

Homeport esegue il comando `helm`, versione 3 o successiva, su **questo** computer. Installalo con `brew install helm` su macOS, `winget install Helm.Helm` (o Chocolatey, o Scoop) su Windows, e `snap install helm --classic`, il pacchetto della tua distribuzione o lo script di helm.sh su Linux. **Impostazioni → Questo computer** mostra l’helm trovato da Homeport; senza, la pagina Helm dice cosa installare per il tuo sistema, e ogni altra pagina del cluster funziona.

Homeport non passa a helm il tuo kubeconfig. Per ogni comando helm, apre una connessione privata al cluster sull’indirizzo di loopback di questo computer e dà a helm un kubeconfig temporaneo che punta a essa, con un token monouso e nessuna credenziale del cluster. Homeport aggiunge le credenziali del cluster stesso — certificati, token, token dei plugin exec — a ogni richiesta lungo il percorso, come per ogni altra pagina, ed elimina il kubeconfig quando il comando termina. Così helm raggiunge ogni cluster che raggiunge Homeport, compresi i cluster cloud con plugin exec, e nessuna credenziale viene scritta su disco.

Helm conserva le sue release come Secret nel Namespace di ogni release: Homeport non installa nulla sul cluster.

## Release

L’elenco mostra per ogni release il nome, il Namespace, il chart e la versione del chart, la versione dell’app, la revisione, lo stato e quando è stata modificata l’ultima volta. Scegli un Namespace in alto a destra, o tutti. helm non ha aggiornamenti in tempo reale, quindi l’elenco lo interroga di nuovo ogni 30 secondi mentre è aperto, e **Aggiorna** lo interroga subito.

Una release **failed** è segnata in rosso, e una **pending** di un’installazione, un aggiornamento o un rollback (`pending-install`, `pending-upgrade`, `pending-rollback`) in ambra: una release pending è bloccata finché quell’operazione non termina.

Se le tue credenziali non possono elencare i Secret in ogni Namespace, Homeport elenca invece le release del Namespace predefinito del cluster e lo dice; scegli un altro Namespace per vederne le release.

## Installare un chart

**Installa chart** (o <kbd>n</kbd>) nella scheda Release installa un chart sul cluster selezionato:

1. **Scegli il chart.** Cerca tra i chart dei [repository](#chart-repositories) di helm e fai clic su **Usa**, oppure, in **Da un registro OCI**, indica un riferimento come `oci://registry-1.docker.io/bitnamicharts/nginx`, che si installa senza aggiungere un repository.
2. **Scegli la sua versione, il nome e il Namespace.** Le versioni di un chart di repository sono elencate, dalla più recente; la versione di un chart OCI si digita, o si lascia vuota per la più recente. La release prende inizialmente il nome del chart: al massimo 53 lettere minuscole, cifre, trattini e punti. Il Namespace è quello mostrato dalla pagina (o quello predefinito del cluster), e viene creato se non esiste, a meno che tu non tolga la spunta.
3. **Modifica i suoi valori.** L’editor parte dai valori predefiniti del chart (`helm show values`); cambia ciò che ti serve, e **Ripristina i valori predefiniti del chart** li rimette. I valori vengono verificati come YAML prima che helm venga eseguito, e passano a helm tramite il suo standard input — mai in un file.
4. **Anteprima**, facoltativa: helm esegue il rendering del chart con i tuoi valori e con l’aiuto del cluster (`--dry-run=server`), e Homeport mostra il manifest che applicherebbe, hook compresi, e le note del chart. Nulla cambia sul cluster.
5. **Installa.** L’output di helm arriva in streaming, come lo stampa `helm install`. Con **Attendi che le sue risorse siano pronte**, helm attende che i Deployment, i Pod e i Service della release siano pronti, per al massimo i minuti indicati (5 per impostazione predefinita), e segna la release come failed se non lo sono. **Apri la release** apre poi la sua pagina.

I chart da una cartella locale o da un file `.tgz` non si installano da Homeport; nemmeno i chart da registri OCI che richiedono un accesso, a meno che tu non abbia effettuato l’accesso con `helm registry login` in un terminale.

## La pagina di una release

Fai clic su una release per aprirla:

- **Cronologia** — ogni revisione, dalla più recente, con la sua data, lo stato, il chart, la versione dell’app e la descrizione di helm di ciò che è successo (`Install complete`, `Upgrade "web" failed: …`).
- **Valori** — i valori forniti all’installazione o all’aggiornamento (**Forniti dall’utente**), o ogni valore con cui è stato eseguito il rendering del chart, compresi i predefiniti (**Calcolati**).
- **Note** — ciò che il chart stampa dopo un’installazione (il suo `NOTES.txt`).
- **Manifest** — gli oggetti Kubernetes prodotti dal rendering della revisione, in YAML.
- **Risorse** — quegli oggetti per kind, nome e Namespace, ciascuno dei quali apre la sua pagina in Homeport: la sua pagina propria per Deployment, Pod, Service e simili, la sua pagina in [Risorse API](/docs/clusters/api-resources/) per il resto.

Valori, Note, Manifest e Risorse mostrano la revisione attuale; scegline una precedente sopra di essi per vedere con cosa era stata installata.

## Aggiornare una release

**Aggiorna**, nella pagina di una release, cambia la versione del suo chart, i suoi valori, o entrambi:

- **Chart** — il chart da cui aggiornare. helm non registra da dove proviene un chart, quindi Homeport cerca un chart con quel nome nei tuoi repository, prima quelli che hanno la versione attuale della release. Scegline un altro con **Un altro chart…**, come `repository/chart` o un riferimento `oci://` — che è ciò di cui ha bisogno una release installata da un registro OCI.
- **Versione del chart** — le versioni di quel chart, dalla più recente, o digitata per un chart OCI.
- **Valori** — l’editor parte dai valori forniti dall’utente della release. Sostituiscono quelli della release: i valori predefiniti del chart completano il resto. **Modifiche ai valori** li mostra affiancati a quelli attuali, e **Ripristina i valori attuali** li rimette.
- **Attendi che le sue risorse siano pronte**, per al massimo i minuti indicati.

**Anteprima** esegue il rendering dell’aggiornamento con l’aiuto del cluster senza cambiare nulla, e mostra il nuovo manifest e cosa cambia rispetto a quello attuale. **Aggiorna** esegue `helm upgrade`, con l’output di helm in tempo reale. Ogni aggiornamento è una nuova revisione in **Cronologia**.

## Ripristinare

In **Cronologia**, **Torna alla revisione N** su qualsiasi revisione precedente chiede prima conferma, indicando le versioni del chart e dell’app attuali e successive, poi esegue `helm rollback`: helm rimette il chart e i valori di quella revisione come nuova revisione, quindi anche un rollback può essere a sua volta annullato con un rollback. Spunta **Attendi che le sue risorse siano pronte** perché helm le attenda.

Una release bloccata in **pending** dopo un’installazione, un aggiornamento o un rollback interrotto è bloccata: helm rifiuta di modificarla finché non termina. Riportarla alla sua ultima revisione distribuita la sblocca.

## Disinstallare

**Disinstalla**, nella pagina di una release, indica la release, il suo Namespace e il suo cluster ed elenca gli oggetti che verranno eliminati con essa, prima che venga eseguito `helm uninstall`. Gli oggetti che un chart segna da conservare (`helm.sh/resource-policy: keep`) e le PersistentVolumeClaim degli StatefulSet restano sul cluster. **Conserva la cronologia** lascia la release elencata come `uninstalled`, così può essere ripristinata; senza, la release scompare dall’elenco. **Attendi che i suoi oggetti vengano eliminati** fa sì che helm li attenda.

## Repository di chart

**Repository di chart**, la seconda scheda della pagina Helm, elenca i repository che helm conosce su questo computer: lo stesso elenco di `helm repo list` in un terminale, condiviso da ogni cluster. Homeport non tiene un elenco proprio, quindi un repository aggiunto in un terminale compare qui, e uno aggiunto qui compare in `helm repo list`.

- **Aggiungi repository** chiede un nome (lettere, cifre, punti, trattini e trattini bassi) e l’URL `https://` del repository, come `https://charts.bitnami.com/bitnami`, e scarica il suo indice.
- **Aggiorna tutti**, o **Aggiorna** su un singolo repository, scarica gli indici più recenti, come fa `helm repo update`, con l’output di helm mostrato in tempo reale. I chart pubblicati dall’ultimo aggiornamento compaiono solo dopo un aggiornamento.
- **Rimuovi** dimentica un repository, dopo la tua conferma. Le release installate da esso continuano a funzionare.

**Trova chart** cerca tra i chart di questi repository per nome o descrizione, come fa `helm search repo`: digita `postgres` dopo aver aggiunto il repository di Bitnami per vedere i suoi chart PostgreSQL, con le loro versioni più recenti del chart e dell’app.

Homeport aggiunge solo repository pubblici. Uno che richiede un accesso si aggiunge in un terminale, con `helm repo add --username …`, dove helm conserva la sua password come fa sempre; compare poi qui e funziona come gli altri. Un chart in un registro OCI (`oci://…`) non ha bisogno di un repository.

## Quando non funziona

- **La CLI helm non è installata**: installa helm 3 come sopra, poi riapri la pagina.
- **Helm 2**: Homeport gestisce solo Helm 3 e successivi. Installa un helm attuale.
- **Non consentito**: helm legge le release dai Secret; elencarli richiede `list` su `secrets` nel Namespace. Dettagli indica cosa è stato rifiutato.
- **Un’altra installazione, aggiornamento o rollback è in corso**: la release è bloccata da un’operazione pending. Attendi che finisca, o riportala alla sua ultima revisione distribuita.
- **Esiste già una release con questo nome**: i nomi delle release sono unici in un Namespace. Scegline un altro, o aggiorna la release esistente.
- **Chart o versione non trovati**: controlla il nome e la versione; un chart di repository richiede che il suo repository sia aggiunto e [aggiornato](#chart-repositories). Per aggiornare una release il cui chart non è in nessuno dei tuoi repository, aggiungi prima quel repository.
- **I valori non sono YAML valido**: il messaggio indica la riga; correggila e riprova.
- **Attesa interrotta**: con **Attendi**, una risorsa non è stata pronta in tempo. Guarda i Pod e gli eventi della release per capire cosa è bloccato.
- **Non è un repository di chart, o non è raggiungibile**: l’URL non serve alcun `index.yaml`, o questo computer non riesce a raggiungerlo. Dettagli mostra cosa ha detto helm.
- **Non connesso**: helm passa dalla connessione del cluster in Homeport; la pagina del cluster dice perché non è connesso.
