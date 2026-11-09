import type { Translation } from '../../types';
import type en from '../en/home';

/** The home page, in Italian. */
export default {
  // Home: the hero and its board.
  'home.title': 'Homeport — Ogni Docker Engine, in un’unica bacheca.',
  'home.hero.new': 'Novità',
  'home.hero.release': 'È uscito Homeport {version}',
  'home.hero.title': 'Ogni Docker Engine, in un’unica bacheca.',
  'home.hero.lead':
    'Homeport funziona sul tuo computer e raggiunge i tuoi host tramite SSH, TLS o TCP. Nessun agente sul server. Gratuito, open source, Windows, macOS, Linux.',
  'home.hero.source': 'Leggi il codice sorgente',
  'home.board.caption':
    'Una bacheca di esempio: cinque host, i loro Engine e come Homeport li raggiunge. Uno richiede attenzione.',
  'home.board.host': 'Host',
  'home.board.engine': 'Engine',
  'home.board.via': 'Tramite',
  'home.board.stacks': 'Stack',
  'home.board.status': 'Stato',
  'home.board.allRunning': 'Tutti in esecuzione',
  'home.board.needsYou': '1 stack richiede attenzione',
  'home.board.note':
    'Ciò che è integro resta neutro. Diventa rosso solo ciò che richiede attenzione.',
  'home.runtimes.label': 'Funziona con',
  'home.runtimes.title': 'Funziona con ciò che usi già',

  // Home: where it lives.
  'home.visits.eyebrow': 'Dove si trova',
  'home.visits.title':
    'Homeport resta sul tuo computer e fa visita ai tuoi server. Non vi si trasferisce mai.',
  'home.visits.agent.title': 'Nessun agente',
  'home.visits.agent.body':
    'Niente da installare, aggiornare o proteggere sul server. Homeport parla con l’Engine che è già lì.',
  'home.visits.panel.title': 'Nessun pannello web',
  'home.visits.panel.body':
    'Nessuna dashboard distribuita sui tuoi server, e nessuna pagina di accesso in attesa che qualcuno la trovi.',
  'home.visits.account.title': 'Nessun account',
  'home.visits.account.body':
    'Nessun accesso, nessuna licenza per utente, nessun abbonamento e nessuna telemetria. Ti mostra i tuoi container, e niente altro.',
  'home.visits.ports.title': 'Cosa richiede sul server, onestamente',
  'home.visits.ports.ssh':
    'La porta che hai già aperta. Con la chiave che crea Homeport, una riga in {file}.',
  'home.visits.ports.tls':
    'La porta dell’API dell’Engine (di solito {port}), che esponi tu stesso.',
  'home.visits.ports.tcp':
    'La porta dell’API dell’Engine (di solito {port}), senza cifratura. Solo su una rete di cui ti fidi.',
  'home.visits.ports.advice':
    'Quando puoi, usa SSH: non aggiunge al server nulla che tu non stia già eseguendo.',
  'home.visits.link': 'Come funzionano le connessioni',

  // Home: the features.
  'home.features.eyebrow': 'Funzionalità',
  'home.features.title': 'Tutto ciò che eseguono i tuoi Engine, in un’unica finestra',
  'home.features.lead':
    'La quotidianità di Docker — cosa è in esecuzione, cosa dice, quanto consuma — senza dover ricordare su quale server si trova.',
  'home.features.hosts.title': 'Host, vicini e lontani',
  'home.features.hosts.body':
    'Gli Engine locali e le macchine Podman vengono trovati per te. Quelli remoti si aggiungono tramite SSH, attraverso dei bastion se serve, TLS reciproco o TCP, oppure direttamente dal tuo ~/.ssh/config e dai contesti Docker. Ogni host si riconnette da solo quando la rete ha un’interruzione.',
  'home.features.containers.title': 'Container a colpo d’occhio',
  'home.features.containers.body':
    'Raggruppati per stack, con CPU e memoria in tempo reale e il motivo per cui uno non è integro. Eseguili con limiti, dispositivi e controlli di integrità, poi modifica, duplica, esegui il commit o esporta un container.',
  'home.features.stacks.title': 'Stack Compose',
  'home.features.stacks.body':
    'Parti da un modello, collega una cartella di progetto di questo computer o segui un repository git, poi distribuisci su qualsiasi host, build comprese. Vedi cosa cambia prima di ogni ridistribuzione o sincronizzazione, ripristina, e aggiorna a immagini più recenti in un solo passaggio.',
  'home.features.swarm.title': 'Docker Swarm',
  'home.features.swarm.body':
    'Gli Swarm compaiono tramite i loro host manager. Gestisci nodi, servizi, task, segreti e config, distribuisci stack su uno Swarm, e da un host crea uno Swarm, entra in uno esistente o lascialo, senza installare nulla su alcun nodo.',
  'home.features.podman.title': 'Podman, da Podman',
  'home.features.podman.body':
    'Homeport riconosce Podman quando si connette e aggiunge ciò che solo Podman ha: i pod, creati con le loro porte e con i container da eseguirvi, e le unità Quadlet, installate, modificate e avviate sotto systemd, tramite l’API di Podman stesso.',
  'home.features.containerd.title': 'containerd, direttamente',
  'home.features.containerd.body':
    'Connettiti a containerd sul suo socket, in locale o tramite SSH, senza Docker: scegli un namespace, avvia, arresta e invia segnali ai task dei suoi container, leggine i log, apri un terminale, scarica immagini con i tuoi accessi ai registri, e trova gli snapshot che occupano il tuo disco.',
  'home.features.incus.title': 'Istanze Incus',
  'home.features.incus.body':
    'Aggiungi Incus tramite il suo socket, SSH o HTTPS con un token di fiducia, ed esegui i suoi container di sistema e le sue macchine virtuali: creali da un’immagine remota, avviali, congelali e arrestali, apri la loro console o una shell, crea e ripristina snapshot, e gestisci immagini, profili, storage e reti.',
  'home.features.clusters.title': 'Cluster Kubernetes e Nomad',
  'home.features.clusters.body':
    'Aggiungi cluster Kubernetes dal tuo kubeconfig — compresi EKS, GKE e AKS — o trova quelli locali con un clic. Esplora in tempo reale carichi di lavoro, Pod, Service, configurazione e qualsiasi risorsa dell’API; segui i log, apri un terminale, inoltra porte, ridimensiona e applica YAML con un’anteprima, e installa, aggiorna e ripristina chart Helm. Aggiungi cluster Nomad tramite indirizzo per eseguire job da HCL con il loro piano, ridimensionarli e arrestarli, seguire i log, aprire una shell e svuotare i nodi. Non viene installato nulla su alcun cluster.',
  'home.features.images.title': 'Immagini',
  'home.features.images.body':
    'Cerca su Docker Hub, scarica e invia con i tuoi accessi ai registri, esegui build da una cartella del tuo computer, salva e carica archivi, e scopri quando esce un’immagine più recente.',
  'home.features.volumes.title': 'Volumi e reti',
  'home.features.volumes.body':
    'Vedi quali container usano cosa. Crea, collega e scollega, con un’eliminazione che ti dice esattamente cosa stai per perdere.',
  'home.features.board.title': 'Un’unica bacheca per tutti gli host',
  'home.features.board.body':
    'Tutti i container di tutti gli host in un’unica tabella, da filtrare e su cui agire insieme. E ogni host anche in una sua pagina: dettagli dell’Engine, uso del disco con una pulizia in un’unica finestra di dialogo, e un flusso in tempo reale di ciò che sta facendo l’Engine.',
  'home.features.keyboard.title': 'Prima la tastiera',
  'home.features.keyboard.body':
    'Un riquadro comandi per andare ovunque, cambiare host o eseguire un comando, e per trovare qualsiasi container, immagine o volume su qualunque host si trovi. Scorciatoie per ogni pagina, elencate con ?.',
  'home.features.background.title': 'Sorveglianza in background',
  'home.features.background.body':
    'Lo stato di ogni host nell’area di notifica, notifiche quando un container si arresta in modo anomalo o un host si disconnette, e Homeport che si avvia all’accesso, nascosto finché non ti serve.',

  // Home: Hosts.
  'home.hosts.eyebrow': 'Host',
  'home.hosts.title': 'Engine remoti senza complicazioni',
  'home.hosts.body':
    'Aggiungi un server una volta e resta nella bacheca. Per SSH, Homeport crea una chiave solo per quell’host e ti fornisce l’unico comando che la autorizza. Controlla l’impronta digitale del server al primo contatto, e ti avvisa se dovesse cambiare.',
  'home.hosts.import':
    'Li hai già descritti? Importa i tuoi server da {file}, compresi i bastion, o i tuoi contesti Docker, compresi i certificati. Se passi a un nuovo computer, portali con te in un unico file protetto da una passphrase.',
  'home.hosts.socket.name': 'Socket locale',
  'home.hosts.socket.detail': 'Socket Unix e named pipe di Windows, rilevati automaticamente.',
  'home.hosts.ssh.detail':
    'Una chiave Ed25519 dedicata per ogni host, la tua chiave, ssh-agent o Pageant; anche attraverso dei bastion.',
  'home.hosts.tls.detail': 'TLS reciproco con la tua CA, il certificato client e la chiave.',
  'home.hosts.tcp.detail':
    'TCP semplice, senza cifratura, solo per un Engine su una rete di cui ti fidi.',
  'home.hosts.link': 'Come funzionano le connessioni',
  'home.hosts.screenshot':
    'La pagina Aggiungi host con SSH selezionato: server, porta, utente, percorso del socket remoto e una nuova chiave per questo host.',

  // Home: containers.
  'home.containers.eyebrow': 'Container',
  'home.containers.title': 'Tutto di un container, a un clic di distanza',
  'home.containers.body':
    'Apri un container e i suoi log sono già in streaming. Cambia scheda per un terminale interattivo, i suoi file, grafici in tempo reale, i processi e il documento inspect completo — su un host dall’altra parte del mondo con la stessa facilità che sul tuo portatile.',
  'home.containers.logs.title': 'Log',
  'home.containers.logs.body': 'Segui, cerca, filtra per flusso e per ora, scarica.',
  'home.containers.terminal.title': 'Terminale',
  'home.containers.terminal.body': 'Shell interattive, quante ne servono.',
  'home.containers.files.title': 'File',
  'home.containers.files.body': 'Sfoglia, carica e scarica, le cartelle come .tar.',
  'home.containers.stats.title': 'Statistiche',
  'home.containers.stats.body': 'Grafici di CPU, memoria, rete e I/O del disco.',
  'home.containers.link': 'Lavorare con i container',
  'home.containers.screenshot':
    'La scheda Log di un container, con l’output in streaming, la ricerca, gli orari e i filtri per flusso.',

  // Home: Stacks.
  'home.stacks.eyebrow': 'Stack',
  'home.stacks.title': 'Stack Compose, gestiti, collegati, sincronizzati o trovati',
  'home.stacks.body':
    'Conserva un file compose e il suo ambiente in Homeport e distribuiscilo su qualsiasi host — parti da un modello, combina più file, scegli i profili; le variabili sono archiviate cifrate, mai scritte su disco. Oppure collega una cartella di progetto di questo computer: Homeport ne modifica i file sul posto e distribuisce da lì, build comprese. Oppure segui un branch di un repository git, con una chiave di deploy o un token: ogni sincronizzazione mostra i nuovi commit e cosa cambiano prima di distribuire, e mentre Homeport è aperto li cerca, distribuendo da solo se lo chiedi. Ogni ridistribuzione mostra prima cosa cambia, ogni distribuzione viene conservata per poterla ripristinare, e Aggiorna scarica le immagini più recenti e ridistribuisce in un solo passaggio. Compaiono anche gli stack che qualcuno ha distribuito a mano: avviali, arrestali o rimuovili, oppure adottali.',
  'home.stacks.link': 'Gestire gli stack',
  'home.stacks.screenshot':
    'La definizione di uno stack gestito: l’editor di compose.yaml, il suo ambiente e il pulsante Salva e ridistribuisci.',

  // Home: security.
  'home.security.eyebrow': 'Sicurezza',
  'home.security.title': 'Le tue chiavi restano sigillate',
  'home.security.lead':
    'Homeport gestisce le credenziali che aprono i tuoi server, quindi le tratta come tali.',
  'home.security.rest.title': 'Cifrate a riposo',
  'home.security.rest.body':
    'Chiavi SSH, certificati TLS, ambienti degli stack e token dei repository sono sigillati con XChaCha20-Poly1305. La chiave si trova nel portachiavi del tuo sistema operativo, non accanto ai dati.',
  'home.security.disk.title': 'Mai in chiaro su disco',
  'home.security.disk.body':
    'Homeport parla SSH da solo, quindi le chiavi decifrate restano in memoria. Nessun file di chiave, nessuna modifica alla tua cartella {dir}.',
  'home.security.home.title': 'Nessun dato inviato a casa',
  'home.security.home.body':
    'Nessun account, nessuna telemetria, nessun tracciamento. Homeport parla con i tuoi Engine e con nient’altro. Ogni release viene compilata pubblicamente, con checksum e provenienza della build.',
  'home.security.link': 'La sicurezza e i tuoi dati',

  // Home: desktops and the closing call.
  'home.desktops.eyebrow': 'Download',
  'home.desktops.title': 'Nativo su ogni desktop',
  'home.desktops.lead':
    'Un’app piccola e veloce, costruita con Tauri e Rust — non una scheda del browser travestita. Programmi di installazione per x64 e ARM che mantengono Homeport aggiornato.',
  'home.desktops.windows.versions': '10 e 11 · x64 e ARM64',
  'home.desktops.windows.formats': 'Programma di installazione (.exe) · MSI',
  'home.desktops.macos.versions': 'Apple silicon e Intel',
  'home.desktops.macos.formats': 'Immagine disco (.dmg)',
  'home.desktops.linux.versions': 'x64 e ARM64',
  'home.desktops.linux.formats': 'apt · dnf · zypper · AppImage',
  'home.cta.title': 'Niente da installare sui tuoi server.',
  'home.cta.body':
    'Scarica Homeport sul computer da cui lavori e aggiungi i tuoi host. È gratuito, open source con licenza GNU AGPL, e sviluppato pubblicamente su GitHub.',
  'home.cta.github': 'Vedi su GitHub',
} satisfies Translation<typeof en>;
