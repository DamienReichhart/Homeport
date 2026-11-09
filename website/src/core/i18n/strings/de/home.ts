import type { Translation } from '../../types';
import type en from '../en/home';

/** The home page, in German. */
export default {
  // Home: the hero and its board.
  'home.title': 'Homeport – Jede Docker Engine, auf einem Board.',
  'home.hero.new': 'Neu',
  'home.hero.release': 'Homeport {version} ist erschienen',
  'home.hero.title': 'Jede Docker Engine, auf einem Board.',
  'home.hero.lead':
    'Homeport läuft auf Ihrem Computer und erreicht Ihre Hosts über SSH, TLS oder TCP. Kein Agent auf dem Server. Kostenlos, Open Source, Windows, macOS, Linux.',
  'home.hero.source': 'Quellcode lesen',
  'home.board.caption':
    'Ein Beispiel-Board: fünf Hosts, ihre Engines und wie Homeport sie erreicht. Einer braucht Aufmerksamkeit.',
  'home.board.host': 'Host',
  'home.board.engine': 'Engine',
  'home.board.via': 'Über',
  'home.board.stacks': 'Stacks',
  'home.board.status': 'Status',
  'home.board.allRunning': 'Alle laufen',
  'home.board.needsYou': '1 Stack braucht Sie',
  'home.board.note': 'Was gesund ist, bleibt schlicht. Nur was Sie braucht, wird rot.',
  'home.runtimes.label': 'Funktioniert mit',
  'home.runtimes.title': 'Funktioniert mit dem, was Sie schon betreiben',

  // Home: where it lives.
  'home.visits.eyebrow': 'Wo es zu Hause ist',
  'home.visits.title':
    'Homeport bleibt auf Ihrem Computer und besucht Ihre Server. Es zieht nie dort ein.',
  'home.visits.agent.title': 'Kein Agent',
  'home.visits.agent.body':
    'Auf dem Server gibt es nichts zu installieren, zu aktualisieren oder abzusichern. Homeport spricht mit der Engine, die schon dort ist.',
  'home.visits.panel.title': 'Kein Web-Panel',
  'home.visits.panel.body':
    'Kein Dashboard auf Ihren Servern und keine Anmeldeseite, die darauf wartet, dass jemand sie findet.',
  'home.visits.account.title': 'Kein Konto',
  'home.visits.account.body':
    'Keine Anmeldung, keine Lizenzplätze, kein Abo und keine Telemetrie. Es zeigt Ihnen Ihre Container, und das ist alles.',
  'home.visits.ports.title': 'Was es auf dem Server braucht, ehrlich gesagt',
  'home.visits.ports.ssh':
    'Den Port, den Sie schon offen haben. Mit dem Schlüssel, den Homeport erzeugt, eine Zeile in {file}.',
  'home.visits.ports.tls':
    'Den eigenen API-Port der Engine (meist {port}), den Sie selbst freigeben.',
  'home.visits.ports.tcp':
    'Den API-Port der Engine (meist {port}), unverschlüsselt. Nur in einem Netzwerk, dem Sie vertrauen.',
  'home.visits.ports.advice':
    'Nutzen Sie SSH, wenn Sie können: Es fügt dem Server nichts hinzu, was dort nicht schon läuft.',
  'home.visits.link': 'Wie Verbindungen funktionieren',

  // Home: the features.
  'home.features.eyebrow': 'Funktionen',
  'home.features.title': 'Alles, was Ihre Engines ausführen, in einem Fenster',
  'home.features.lead':
    'Der Docker-Alltag – was läuft, was es meldet, was es kostet – ohne sich merken zu müssen, auf welchem Server es liegt.',
  'home.features.hosts.title': 'Hosts, nah und fern',
  'home.features.hosts.body':
    'Lokale Engines und Podman-Maschinen werden für Sie gefunden. Entfernte kommen über SSH hinzu, bei Bedarf über Bastionen, über Mutual TLS oder TCP, oder direkt aus Ihrer ~/.ssh/config und Ihren Docker-Kontexten. Jeder Host verbindet sich von selbst neu, wenn das Netzwerk kurz aussetzt.',
  'home.features.containers.title': 'Container auf einen Blick',
  'home.features.containers.body':
    'Nach Stack gruppiert, mit CPU und Arbeitsspeicher live und dem Grund, warum einer nicht gesund ist. Mit Limits, Geräten und Healthchecks ausführen, dann einen Container bearbeiten, duplizieren, committen oder exportieren.',
  'home.features.stacks.title': 'Compose-Stacks',
  'home.features.stacks.body':
    'Beginnen Sie mit einer Vorlage, verknüpfen Sie einen Projektordner auf diesem Computer oder folgen Sie einem Git-Repository, und stellen Sie dann auf einem beliebigen Host bereit – Builds inklusive. Sehen Sie vor jeder Neubereitstellung oder Synchronisierung, was sich ändert, setzen Sie zurück und aktualisieren Sie in einem Schritt auf neuere Images.',
  'home.features.swarm.title': 'Docker Swarm',
  'home.features.swarm.body':
    'Swarms erscheinen über ihre Manager-Hosts. Verwalten Sie Nodes, Services, Tasks, Secrets und Configs, stellen Sie Stacks in einem Swarm bereit und starten Sie einen Swarm von einem Host aus, treten Sie ihm bei oder verlassen Sie ihn, ohne etwas auf einem Node zu installieren.',
  'home.features.podman.title': 'Podman, als Podman',
  'home.features.podman.body':
    'Homeport erkennt Podman beim Verbinden und ergänzt, was nur Podman hat: Pods, mit ihren Ports erstellt und mit darin ausgeführten Containern, und Quadlet-Units, unter systemd installiert, bearbeitet und gestartet, über die eigene API von Podman.',
  'home.features.containerd.title': 'containerd, direkt',
  'home.features.containerd.body':
    'Verbinden Sie sich mit containerd über seinen Socket, lokal oder über SSH, ganz ohne Docker: Wählen Sie einen containerd-Namespace, starten, stoppen und signalisieren Sie die containerd-Tasks seiner Container, lesen Sie ihre Logs, öffnen Sie ein Terminal, pullen Sie Images mit Ihren Registry-Anmeldungen und finden Sie die Snapshots, die Ihren Speicherplatz belegen.',
  'home.features.incus.title': 'Incus-Instanzen',
  'home.features.incus.body':
    'Fügen Sie Incus über seinen Socket, SSH oder HTTPS mit einem Vertrauens-Token hinzu und betreiben Sie seine Systemcontainer und virtuellen Maschinen: Erstellen Sie sie aus einem entfernten Image, starten, einfrieren und stoppen Sie sie, öffnen Sie ihre Konsole oder eine Shell, erstellen und stellen Sie Snapshots wieder her und verwalten Sie Images, Profile, Speicher und Netzwerke.',
  'home.features.clusters.title': 'Kubernetes- und Nomad-Cluster',
  'home.features.clusters.body':
    'Fügen Sie Kubernetes-Cluster aus Ihrer kubeconfig hinzu – EKS, GKE und AKS inklusive – oder finden Sie lokale mit einem Klick. Durchsuchen Sie Workloads, Pods, Services, Konfiguration und jede API-Ressource live; folgen Sie Logs, öffnen Sie ein Terminal, richten Sie Portweiterleitungen ein, skalieren Sie und wenden Sie YAML mit Vorschau an, und installieren, aktualisieren (Upgrade) und setzen Sie Helm-Charts zurück. Fügen Sie Nomad-Cluster per Adresse hinzu, um Jobs aus HCL mit ihrem Plan auszuführen, sie zu skalieren und zu stoppen, Logs zu folgen, eine Shell zu öffnen und Nodes zu leeren. Auf einem Cluster wird nichts installiert.',
  'home.features.images.title': 'Images',
  'home.features.images.body':
    'Durchsuchen Sie Docker Hub, pullen und pushen Sie mit Ihren Registry-Anmeldungen, bauen Sie aus einem Ordner auf Ihrem Computer, speichern und laden Sie Archive und sehen Sie, wenn ein neueres Image erschienen ist.',
  'home.features.volumes.title': 'Volumes und Netzwerke',
  'home.features.volumes.body':
    'Sehen Sie, welche Container was verwenden. Erstellen, verbinden und trennen, mit einem Löschen, das Ihnen genau sagt, was Sie gleich verlieren.',
  'home.features.board.title': 'Ein Board für jeden Host',
  'home.features.board.body':
    'Jeder Container jedes Hosts in einer Tabelle, gemeinsam gefiltert und bearbeitet. Auch jeder Host auf einer eigenen Seite: Details der Engine, Speicherbelegung mit Aufräumen in einem Dialog und ein Live-Stream dessen, was die Engine gerade tut.',
  'home.features.keyboard.title': 'Tastatur zuerst',
  'home.features.keyboard.body':
    'Eine Befehlspalette, um überallhin zu gelangen, den Host zu wechseln oder einen Befehl auszuführen, und um jeden Container, jedes Image oder Volume auf dem Host zu finden, der es hat. Tastenkürzel für jede Seite, aufgelistet mit ?.',
  'home.features.background.title': 'Im Hintergrund wachsam',
  'home.features.background.body':
    'Der Status jedes Hosts im Infobereich, Benachrichtigungen, wenn ein Container abstürzt oder ein Host wegfällt, und Homeport startet bei der Anmeldung, verborgen, bis Sie es brauchen.',

  // Home: Hosts.
  'home.hosts.eyebrow': 'Hosts',
  'home.hosts.title': 'Entfernte Engines ohne Umstände',
  'home.hosts.body':
    'Fügen Sie einen Server einmal hinzu, und er bleibt auf dem Board. Für SSH erzeugt Homeport einen Schlüssel nur für diesen Host und gibt Ihnen den einen Befehl, der ihn autorisiert. Es prüft beim ersten Kontakt den Fingerabdruck des Servers und warnt Sie, falls er sich je ändert.',
  'home.hosts.import':
    'Schon beschrieben? Importieren Sie Ihre Server aus {file}, Sprungserver inklusive, oder Ihre Docker-Kontexte, Zertifikate inklusive. Beim Umzug auf einen neuen Computer nehmen Sie sie in einer einzigen, mit einer Passphrase geschützten Datei mit.',
  'home.hosts.socket.name': 'Lokaler Socket',
  'home.hosts.socket.detail': 'Unix-Sockets und Windows-Named-Pipes, automatisch erkannt.',
  'home.hosts.ssh.detail':
    'Ein eigener Ed25519-Schlüssel pro Host, Ihr eigener Schlüssel, ssh-agent oder Pageant; auch über Bastionen.',
  'home.hosts.tls.detail': 'Mutual TLS mit Ihrer CA, Ihrem Client-Zertifikat und Schlüssel.',
  'home.hosts.tcp.detail':
    'Einfaches, unverschlüsseltes TCP, nur für eine Engine in einem Netzwerk, dem Sie vertrauen.',
  'home.hosts.link': 'Wie Verbindungen funktionieren',
  'home.hosts.screenshot':
    'Die Seite „Host hinzufügen“ mit ausgewähltem SSH: Server, Port, Benutzer, Pfad des entfernten Sockets und ein neuer Schlüssel für diesen Host.',

  // Home: containers.
  'home.containers.eyebrow': 'Container',
  'home.containers.title': 'Alles über einen Container, einen Klick entfernt',
  'home.containers.body':
    'Öffnen Sie einen Container, und seine Logs laufen schon. Wechseln Sie die Tabs für ein interaktives Terminal, seine Dateien, Live-Diagramme, Prozesse und das vollständige Inspect-Dokument – auf einem Host am anderen Ende der Welt so einfach wie auf Ihrem Laptop.',
  'home.containers.logs.title': 'Logs',
  'home.containers.logs.body': 'Folgen, suchen, nach Stream und Zeit filtern, herunterladen.',
  'home.containers.terminal.title': 'Terminal',
  'home.containers.terminal.body': 'Interaktive Shells, so viele Sie brauchen.',
  'home.containers.files.title': 'Dateien',
  'home.containers.files.body': 'Durchsuchen, hochladen und herunterladen, Ordner als .tar.',
  'home.containers.stats.title': 'Statistik',
  'home.containers.stats.body': 'Diagramme für CPU, Arbeitsspeicher, Netzwerk und Datenträger-I/O.',
  'home.containers.link': 'Mit Containern arbeiten',
  'home.containers.screenshot':
    'Der Tab „Logs“ eines Containers, der die Ausgabe mit Suche, Zeitstempeln und Stream-Filtern streamt.',

  // Home: Stacks.
  'home.stacks.eyebrow': 'Stacks',
  'home.stacks.title': 'Compose-Stacks: verwaltet, verknüpft, synchronisiert oder gefunden',
  'home.stacks.body':
    'Bewahren Sie eine Compose-Datei und ihre Umgebung in Homeport auf und stellen Sie sie auf einem beliebigen Host bereit – beginnen Sie mit einer Vorlage, kombinieren Sie mehrere Dateien, wählen Sie Profile; Variablen werden verschlüsselt gespeichert und nie auf die Festplatte geschrieben. Oder verknüpfen Sie einen Projektordner auf diesem Computer: Homeport bearbeitet seine Dateien direkt und stellt daraus bereit, Builds inklusive. Oder folgen Sie einem Branch eines Git-Repositorys, mit einem Deploy-Key oder einem Token: Jede Synchronisierung zeigt die neuen Commits und was sie ändern, bevor bereitgestellt wird, und solange Homeport geöffnet ist, prüft es auf neue Commits und stellt auf Wunsch selbst bereit. Jede Neubereitstellung zeigt zuerst, was sich ändert, jede Bereitstellung wird aufbewahrt, damit Sie zurücksetzen können, und „Aktualisieren“ pullt neuere Images und stellt in einem Schritt neu bereit. Stacks, die jemand von Hand bereitgestellt hat, erscheinen ebenfalls: Starten, stoppen oder entfernen Sie sie, oder übernehmen Sie sie.',
  'home.stacks.link': 'Stacks verwalten',
  'home.stacks.screenshot':
    'Die Definition eines verwalteten Stacks: der Editor für compose.yaml, seine Umgebung und die Schaltfläche „Speichern und neu bereitstellen“.',

  // Home: security.
  'home.security.eyebrow': 'Sicherheit',
  'home.security.title': 'Ihre Schlüssel bleiben versiegelt',
  'home.security.lead':
    'Homeport verwaltet die Zugangsdaten, die Ihre Server öffnen, und behandelt sie entsprechend.',
  'home.security.rest.title': 'Im Ruhezustand verschlüsselt',
  'home.security.rest.body':
    'SSH-Schlüssel, TLS-Zertifikate, Stack-Umgebungen und Repository-Tokens werden mit XChaCha20-Poly1305 versiegelt. Der Schlüssel liegt im Schlüsselbund Ihres Betriebssystems, nicht neben den Daten.',
  'home.security.disk.title': 'Nie im Klartext auf der Festplatte',
  'home.security.disk.body':
    'Homeport spricht SSH selbst, daher bleiben entschlüsselte Schlüssel im Arbeitsspeicher. Keine Schlüsseldateien, keine Änderungen an Ihrem {dir}.',
  'home.security.home.title': 'Nichts telefoniert nach Hause',
  'home.security.home.body':
    'Kein Konto, keine Telemetrie, kein Tracking. Homeport spricht mit Ihren Engines und mit nichts anderem. Jedes Release wird öffentlich gebaut, mit Prüfsummen und Build-Provenienz.',
  'home.security.link': 'Sicherheit und Ihre Daten',

  // Home: desktops and the closing call.
  'home.desktops.eyebrow': 'Download',
  'home.desktops.title': 'Nativ auf jedem Desktop',
  'home.desktops.lead':
    'Eine kleine, schnelle App, gebaut mit Tauri und Rust – kein verkleideter Browser-Tab. Installationsprogramme für x64 und ARM, die Homeport aktuell halten.',
  'home.desktops.windows.versions': '10 und 11 · x64 und ARM64',
  'home.desktops.windows.formats': 'Installationsprogramm (.exe) · MSI',
  'home.desktops.macos.versions': 'Apple Silicon und Intel',
  'home.desktops.macos.formats': 'Disk-Image (.dmg)',
  'home.desktops.linux.versions': 'x64 und ARM64',
  'home.desktops.linux.formats': 'apt · dnf · zypper · AppImage',
  'home.cta.title': 'Nichts auf Ihren Servern zu installieren.',
  'home.cta.body':
    'Laden Sie Homeport auf den Computer herunter, an dem Sie arbeiten, und fügen Sie Ihre Hosts hinzu. Es ist kostenlos, Open Source unter der GNU AGPL und wird öffentlich auf GitHub entwickelt.',
  'home.cta.github': 'Auf GitHub ansehen',
} satisfies Translation<typeof en>;
