/** The home page, in Italian. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport — Un’unica app per ogni Engine di container',
  'home.hero.title': 'Un’unica app per ogni Engine di container.',
  'home.hero.lead':
    'Homeport funziona sul tuo computer e non installa nulla sui tuoi server. Gratuito e open source.',
  'home.hero.docs': 'Leggi la documentazione',
  'home.map.label':
    'Un parco server di esempio disegnato come una mappa della metropolitana. Questo computer è il capolinea di cinque linee: il socket locale raggiunge This Mac, SSH raggiunge prod-eu-1, nas ed edge-1, TLS raggiunge staging e vps-fra-1, TCP raggiunge lan-box, e l’API del cluster raggiunge k8s-prod, jobs e lab. Uno stack su vps-fra-1 è da gestire.',
  'home.map.thisMachine': 'Questo computer',
  'home.map.needsYou': '1 stack da gestire',
  'home.map.keyNeedsYou': 'Da gestire',
  'home.map.example': 'Un parco server di esempio',

  // Home: the questions people open it for.
  'home.questions.title': 'Una domanda, un solo posto.',
  'home.questions.lead':
    'Apri Homeport per scoprire qualcosa, poi torni al lavoro. Ogni risposta è a un passo dalla Bacheca.',
  'home.questions.up.title': 'È attivo?',
  'home.questions.up.body':
    'La Bacheca mostra insieme ogni host, Swarm e cluster, ed elenca ciò che è da gestire prima di tutto ciò che va bene.',
  'home.questions.up.link': 'La Bacheca e la Panoramica',
  'home.questions.up.screenshot':
    'La Bacheca: un riquadro per ciascuno dei tre host, un container da gestire, poi i container di tutti gli host in un’unica tabella.',
  'home.questions.died.title': 'Perché si è fermato?',
  'home.questions.died.body':
    'Apri un container e i suoi log sono già in streaming. Un terminale, i suoi file, grafici in tempo reale e il documento inspect sono a una scheda di distanza.',
  'home.questions.died.link': 'Lavorare con i container',
  'home.questions.died.screenshot':
    'La scheda Log di un container, con l’output in streaming, la ricerca, gli orari e i filtri per flusso.',
  'home.questions.changed.title': 'Cos’è cambiato?',
  'home.questions.changed.body':
    'Ogni host tiene una cronologia di ciò che ha fatto il suo Engine: il container che si è riavviato, l’immagine che è stata scaricata, il volume che è stato montato.',
  'home.questions.changed.link': 'Eventi',
  'home.questions.changed.screenshot':
    'La pagina Eventi di un host: controlli di integrità, riavvii, download e montaggi, dal più recente, con lo stack a cui appartiene ciascuno.',
  'home.questions.act.title': 'Poi cambialo, e guarda prima cosa cambia.',
  'home.questions.act.body':
    'Una ridistribuzione mostra cosa cambierà prima di essere eseguita. Ogni distribuzione viene conservata, così puoi ripristinarla, e Aggiorna scarica le immagini più recenti in un solo passaggio.',
  'home.questions.act.link': 'Gestire gli stack',
  'home.questions.act.screenshot':
    'La definizione di uno stack gestito: l’editor di compose.yaml, il suo ambiente e il pulsante Distribuisci.',
  'home.questions.background.title': 'Sorveglianza in background',
  'home.questions.background.body':
    'Lo stato di ogni host nell’area di notifica, notifiche quando un container si arresta in modo anomalo o un host si disconnette, e Homeport che si avvia all’accesso, nascosto finché non ti serve.',
  'home.questions.keyboard.title': 'Prima la tastiera',
  'home.questions.keyboard.body':
    'Un riquadro comandi per andare ovunque, cambiare host o eseguire un comando, e per trovare qualsiasi container, immagine o volume su qualunque host si trovi. Scorciatoie per ogni pagina, elencate con ?.',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': 'Niente si trasferisce sui server.',
  'home.lines.lead':
    'Homeport resta sul tuo computer e fa visita ai tuoi server. Nessun agente da installare, nessun pannello web da esporre, nessun account da creare e nessuna telemetria.',
  'home.lines.line': 'Linea',
  'home.lines.reaches': 'Raggiunge',
  'home.lines.needs': 'Cosa richiede sul server',
  'home.lines.socket.name': 'Socket locale',
  'home.lines.socket.reaches':
    'Docker Desktop, OrbStack, Colima, Rancher Desktop e le macchine Podman di questo computer, trovati per te.',
  'home.lines.socket.needs': 'Niente. L’Engine è già su questo computer.',
  'home.lines.ssh.reaches':
    'Docker Engine, Podman, containerd e Incus su un server, attraverso dei bastion se serve.',
  'home.lines.ssh.needs':
    'La porta che hai già aperta. Con la chiave che crea Homeport, una riga in {file}.',
  'home.lines.tls.reaches':
    'L’API di un Engine compatibile con Docker, con la tua CA e il tuo certificato client.',
  'home.lines.tls.needs': 'La porta dell’API dell’Engine (di solito {port}), che esponi tu stesso.',
  'home.lines.tcp.reaches': 'L’API di un Engine compatibile con Docker, senza cifratura.',
  'home.lines.tcp.needs':
    'La porta dell’API dell’Engine (di solito {port}). Solo su una rete di cui ti fidi.',
  'home.lines.cluster.name': 'API del cluster',
  'home.lines.cluster.reaches':
    'Kubernetes dal tuo kubeconfig, Nomad tramite l’indirizzo del suo agente, Incus tramite HTTPS con un token di fiducia.',
  'home.lines.cluster.needs':
    'L’API che il cluster espone già. Non viene installato nulla su alcun nodo.',
  'home.lines.advice':
    'Quando puoi, usa SSH: non aggiunge al server nulla che tu non stia già eseguendo.',
  'home.lines.import':
    'Hai già descritto i tuoi server? Importali da {file}, compresi i bastion, o dai tuoi contesti Docker.',
  'home.lines.link': 'Come funzionano le connessioni',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': 'Ogni Engine, con le stesse parole.',
  'home.engines.lead':
    'Un host è un host e uno stack è uno stack, qualunque Engine lo esegua. Dove un Engine ha qualcosa di suo, Homeport mostra anche quello.',
  'home.engines.docker.body':
    'Container, immagini, volumi, reti e stack Compose: conservati in Homeport, collegati a una cartella o sincronizzati da un repository git.',
  'home.engines.swarm.body':
    'Nodi, servizi, task, segreti e config, trovati tramite i suoi host manager. Distribuisci stack sullo Swarm.',
  'home.engines.podman.body':
    'I pod, e le unità Quadlet installate e avviate sotto systemd, tramite l’API di Podman stesso.',
  'home.engines.containerd.body':
    'Namespace, task, log, un terminale e gli snapshot che occupano il tuo disco, senza Docker.',
  'home.engines.incus.body':
    'Container di sistema e macchine virtuali: la loro console, snapshot, immagini, profili, storage e reti.',
  'home.engines.kubernetes.body':
    'Carichi di lavoro, Pod, Service e qualsiasi risorsa dell’API, in tempo reale. Log, terminali, inoltri di porta, YAML con un’anteprima, release Helm.',
  'home.engines.nomad.body':
    'Job eseguiti da HCL con il loro piano, le loro allocation e i log, una shell in un task e lo svuotamento dei nodi.',

  // Home: security.
  'home.security.title': 'Le tue chiavi restano sigillate.',
  'home.security.lead':
    'Homeport custodisce le credenziali che aprono i tuoi server, quindi le tratta come tali.',
  'home.security.rest.title': 'Cifrate a riposo',
  'home.security.rest.body':
    'Chiavi SSH, certificati TLS, ambienti degli stack e token dei repository sono sigillati con XChaCha20-Poly1305. La chiave si trova nel portachiavi del tuo sistema operativo, non accanto ai dati.',
  'home.security.disk.title': 'Mai in chiaro su disco',
  'home.security.disk.body':
    'Homeport parla SSH da solo, quindi le chiavi decifrate restano in memoria. Nessun file di chiave, nessuna modifica alla tua cartella {dir}.',
  'home.security.home.title': 'Nessun dato inviato a casa',
  'home.security.home.body':
    'Nessun account, nessuna telemetria, nessun tracciamento. Ogni release viene compilata pubblicamente, con checksum e provenienza della build.',
  'home.security.link': 'La sicurezza e i tuoi dati',

  // Home: the close, where the visitor starts.
  'home.close.title': 'Parti da questo computer.',
  'home.close.body':
    'Scarica Homeport sul computer da cui lavori, poi aggiungi i tuoi host. È gratuito, open source con licenza GNU AGPL, e sviluppato pubblicamente su GitHub.',
  'home.close.github': 'Vedi su GitHub',
  'home.desktops.windows.versions': '10 e 11, x64 e ARM64',
  'home.desktops.windows.formats': 'Programma di installazione (.exe) o MSI',
  'home.desktops.macos.versions': 'Apple silicon e Intel',
  'home.desktops.macos.formats': 'Immagine disco (.dmg)',
  'home.desktops.linux.versions': 'x64 e ARM64',
  'home.desktops.linux.formats': 'apt, dnf, zypper o AppImage',
};
