/** The home page, in German. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport – Eine App für jede Container-Engine',
  'home.hero.title': 'Eine App für jede Container-Engine.',
  'home.hero.lead':
    'Homeport läuft auf Ihrem Computer und installiert nichts auf Ihren Servern. Kostenlos und Open Source.',
  'home.hero.docs': 'Dokumentation lesen',
  'home.map.label':
    'Eine Beispielflotte als Linienplan. Dieser Computer liegt am Ende von fünf Linien: Der lokale Socket erreicht This Mac, SSH erreicht prod-eu-1, nas und edge-1, TLS erreicht staging und vps-fra-1, TCP erreicht lan-box, und die Cluster-API erreicht k8s-prod, jobs und lab. Ein Stack auf vps-fra-1 braucht Sie.',
  'home.map.thisMachine': 'Dieser Computer',
  'home.map.needsYou': '1 Stack braucht Sie',
  'home.map.keyNeedsYou': 'Braucht dich',
  'home.map.example': 'Eine Beispielflotte',

  // Home: the questions people open it for.
  'home.questions.title': 'Eine Frage, ein Ort.',
  'home.questions.lead':
    'Sie öffnen Homeport, um etwas herauszufinden, und kehren dann an die Arbeit zurück. Jede Antwort ist einen Schritt vom Board entfernt.',
  'home.questions.up.title': 'Läuft es?',
  'home.questions.up.body':
    'Das Board zeigt alle Hosts, Swarms und Cluster auf einmal und listet zuerst auf, was Sie braucht, und erst danach, was in Ordnung ist.',
  'home.questions.up.link': 'Das Board und die Übersicht',
  'home.questions.up.screenshot':
    'Das Board: eine Kachel für jeden von drei Hosts, ein Container, der Sie braucht, dann die Container aller Hosts in einer Tabelle.',
  'home.questions.died.title': 'Warum ist es abgestürzt?',
  'home.questions.died.body':
    'Öffnen Sie einen Container, und seine Logs laufen schon. Ein Terminal, seine Dateien, Live-Diagramme und das Inspect-Dokument sind nur einen Tab entfernt.',
  'home.questions.died.link': 'Mit Containern arbeiten',
  'home.questions.died.screenshot':
    'Der Tab „Logs“ eines Containers, der die Ausgabe mit Suche, Zeitstempeln und Stream-Filtern streamt.',
  'home.questions.changed.title': 'Was hat sich geändert?',
  'home.questions.changed.body':
    'Jeder Host führt laufend Buch darüber, was seine Engine getan hat: der Container, der neu gestartet ist, das Image, das gepullt wurde, das Volume, das eingehängt wurde.',
  'home.questions.changed.link': 'Ereignisse',
  'home.questions.changed.screenshot':
    'Die Seite „Ereignisse“ eines Hosts: Healthchecks, Neustarts, Pulls und Mounts, das Neueste zuerst, jeweils mit dem Stack, zu dem sie gehören.',
  'home.questions.act.title': 'Dann ändern Sie es und sehen die Änderung vorher.',
  'home.questions.act.body':
    'Eine Neubereitstellung zeigt, was sich ändert, bevor sie läuft. Jede Bereitstellung wird aufbewahrt, damit Sie zurücksetzen können, und „Aktualisieren“ pullt neuere Images in einem Schritt.',
  'home.questions.act.link': 'Stacks verwalten',
  'home.questions.act.screenshot':
    'Die Definition eines verwalteten Stacks: der Editor für compose.yaml, seine Umgebung und die Schaltfläche „Bereitstellen“.',
  'home.questions.background.title': 'Im Hintergrund wachsam',
  'home.questions.background.body':
    'Der Status jedes Hosts im Infobereich, Benachrichtigungen, wenn ein Container abstürzt oder ein Host wegfällt, und Homeport startet bei der Anmeldung, verborgen, bis Sie es brauchen.',
  'home.questions.keyboard.title': 'Tastatur zuerst',
  'home.questions.keyboard.body':
    'Eine Befehlspalette, um überallhin zu gelangen, den Host zu wechseln oder einen Befehl auszuführen, und um jeden Container, jedes Image oder Volume auf dem Host zu finden, der es hat. Tastenkürzel für jede Seite, aufgelistet mit ?.',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': 'Nichts zieht ein.',
  'home.lines.lead':
    'Homeport bleibt auf Ihrem Computer und besucht Ihre Server. Kein Agent, den Sie installieren, kein Web-Panel, das Sie freigeben, kein Konto, das Sie anlegen müssen, und keine Telemetrie.',
  'home.lines.line': 'Linie',
  'home.lines.reaches': 'Erreicht',
  'home.lines.needs': 'Was es auf dem Server braucht',
  'home.lines.socket.name': 'Lokaler Socket',
  'home.lines.socket.reaches':
    'Docker Desktop, OrbStack, Colima, Rancher Desktop und Podman-Maschinen auf diesem Computer, für Sie gefunden.',
  'home.lines.socket.needs': 'Nichts. Die Engine ist schon auf diesem Computer.',
  'home.lines.ssh.reaches':
    'Docker Engine, Podman, containerd und Incus auf einem Server, bei Bedarf über Bastionen.',
  'home.lines.ssh.needs':
    'Den Port, den Sie schon offen haben. Mit dem Schlüssel, den Homeport erzeugt, eine Zeile in {file}.',
  'home.lines.tls.reaches':
    'Die API einer Docker-kompatiblen Engine, mit Ihrer CA und Ihrem Client-Zertifikat.',
  'home.lines.tls.needs':
    'Den eigenen API-Port der Engine (meist {port}), den Sie selbst freigeben.',
  'home.lines.tcp.reaches': 'Die API einer Docker-kompatiblen Engine, unverschlüsselt.',
  'home.lines.tcp.needs':
    'Den API-Port der Engine (meist {port}). Nur in einem Netzwerk, dem Sie vertrauen.',
  'home.lines.cluster.name': 'Cluster-API',
  'home.lines.cluster.reaches':
    'Kubernetes aus Ihrer kubeconfig, Nomad über die Adresse seines Agents, Incus über HTTPS mit einem Vertrauens-Token.',
  'home.lines.cluster.needs':
    'Die API, die der Cluster schon bereitstellt. Auf keinem Node wird etwas installiert.',
  'home.lines.advice':
    'Nutzen Sie SSH, wenn Sie können: Es fügt dem Server nichts hinzu, was dort nicht schon läuft.',
  'home.lines.import':
    'Ihre Server sind schon beschrieben? Importieren Sie sie aus {file}, Bastionen inklusive, oder aus Ihren Docker-Kontexten.',
  'home.lines.link': 'Wie Verbindungen funktionieren',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': 'Jede Engine, in denselben Worten.',
  'home.engines.lead':
    'Ein Host ist ein Host und ein Stack ist ein Stack, egal welche Engine ihn ausführt. Wo eine Engine etwas Eigenes hat, zeigt Homeport auch das.',
  'home.engines.docker.body':
    'Container, Images, Volumes, Netzwerke und Compose-Stacks: in Homeport aufbewahrt, mit einem Ordner verknüpft oder aus einem Git-Repository synchronisiert.',
  'home.engines.swarm.body':
    'Nodes, Services, Tasks, Secrets und Configs, gefunden über seine Manager-Hosts. Stellen Sie Stacks im Swarm bereit.',
  'home.engines.podman.body':
    'Pods und Quadlet-Units, unter systemd installiert und gestartet, über die eigene API von Podman.',
  'home.engines.containerd.body':
    'Namespaces, containerd-Tasks, Logs, ein Terminal und die Snapshots, die Ihren Speicherplatz belegen, ganz ohne Docker.',
  'home.engines.incus.body':
    'Systemcontainer und virtuelle Maschinen: ihre Konsole, Snapshots, Images, Profile, Speicher und Netzwerke.',
  'home.engines.kubernetes.body':
    'Workloads, Pods, Services und jede API-Ressource, live. Logs, Terminals, Portweiterleitungen, YAML mit Vorschau, Helm-Releases.',
  'home.engines.nomad.body':
    'Jobs aus HCL mit ihrem Plan ausführen, ihre Allocations und Logs, eine Shell in einem Task und Nodes leeren.',

  // Home: security.
  'home.security.title': 'Ihre Schlüssel bleiben versiegelt.',
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
    'Kein Konto, keine Telemetrie, kein Tracking. Jedes Release wird öffentlich gebaut, mit Prüfsummen und Build-Provenienz.',
  'home.security.link': 'Sicherheit und Ihre Daten',

  // Home: the close, where the visitor starts.
  'home.close.title': 'Beginnen Sie auf diesem Computer.',
  'home.close.body':
    'Laden Sie Homeport auf den Computer herunter, an dem Sie arbeiten, und fügen Sie dann Ihre Hosts hinzu. Es ist kostenlos, Open Source unter der GNU AGPL und wird öffentlich auf GitHub entwickelt.',
  'home.close.github': 'Auf GitHub ansehen',
  'home.desktops.windows.versions': '10 und 11, x64 und ARM64',
  'home.desktops.windows.formats': 'Installationsprogramm (.exe) oder MSI',
  'home.desktops.macos.versions': 'Apple Silicon und Intel',
  'home.desktops.macos.formats': 'Disk-Image (.dmg)',
  'home.desktops.linux.versions': 'x64 und ARM64',
  'home.desktops.linux.formats': 'apt, dnf, zypper oder AppImage',
};
