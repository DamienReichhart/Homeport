/** The home page, in English. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport — One app for every container Engine',
  'home.hero.title': 'One app for every container Engine.',
  'home.hero.lead':
    'Homeport runs on your machine and installs nothing on your servers. Free and open source.',
  'home.hero.docs': 'Read the docs',
  'home.map.label':
    'An example fleet drawn as a line map. This machine is at the end of five lines: the local socket reaches This Mac, SSH reaches prod-eu-1, nas and edge-1, TLS reaches staging and vps-fra-1, TCP reaches lan-box, and the cluster API reaches k8s-prod, jobs and lab. One Stack on vps-fra-1 needs you.',
  'home.map.thisMachine': 'This machine',
  'home.map.needsYou': '1 Stack needs you',
  'home.map.keyNeedsYou': 'Needs you',
  'home.map.example': 'An example fleet',

  // Home: the questions people open it for.
  'home.questions.title': 'One question, one place.',
  'home.questions.lead':
    'You open Homeport to find something out, then get back to work. Each answer is a step from the Board.',
  'home.questions.up.title': 'Is it up?',
  'home.questions.up.body':
    'The Board shows every Host, Swarm and Cluster at once, and lists what needs you before anything that is fine.',
  'home.questions.up.link': 'The Board and Overview',
  'home.questions.up.screenshot':
    'The Board: a tile for each of three Hosts, one container that needs you, then the containers of every Host in one table.',
  'home.questions.died.title': 'Why did it die?',
  'home.questions.died.body':
    'Open a container and its logs are already streaming. A terminal, its files, live charts and the inspect document are a tab away.',
  'home.questions.died.link': 'Working with containers',
  'home.questions.died.screenshot':
    "A container's Logs tab, streaming output with search, timestamps and stream filters.",
  'home.questions.changed.title': 'What changed?',
  'home.questions.changed.body':
    'Each Host keeps a running record of what its Engine did: the container that restarted, the image that was pulled, the volume that was mounted.',
  'home.questions.changed.link': 'Events',
  'home.questions.changed.screenshot':
    "A Host's Events page: health checks, restarts, pulls and mounts, newest first, with the Stack each belongs to.",
  'home.questions.act.title': 'Then change it, and see the change first.',
  'home.questions.act.body':
    'A redeploy shows what will change before it runs. Every deploy is kept, so you can roll back, and Update pulls newer images in one step.',
  'home.questions.act.link': 'Managing Stacks',
  'home.questions.act.screenshot':
    "A Managed Stack's definition: the compose.yaml editor, its environment and the Deploy button.",
  'home.questions.background.title': 'Watching in the background',
  'home.questions.background.body':
    'Every Host’s status in the system tray, notifications when a container crashes or a Host drops, and Homeport starting at login, hidden until you need it.',
  'home.questions.keyboard.title': 'Keyboard first',
  'home.questions.keyboard.body':
    'A command palette to go anywhere, switch Host or run a command, and to find any container, image or volume on whichever Host has it. Shortcuts for every page, listed with ?.',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': 'Nothing moves in.',
  'home.lines.lead':
    'Homeport stays on your machine and visits your servers. No agent to install, no web panel to expose, no account to create and no telemetry.',
  'home.lines.line': 'Line',
  'home.lines.reaches': 'Reaches',
  'home.lines.needs': 'What it needs on the server',
  'home.lines.socket.name': 'Local socket',
  'home.lines.socket.reaches':
    'Docker Desktop, OrbStack, Colima, Rancher Desktop and Podman machines on this computer, found for you.',
  'home.lines.socket.needs': 'Nothing. The Engine is already on this machine.',
  'home.lines.ssh.reaches':
    'Docker Engine, Podman, containerd and Incus on a server, through bastions if need be.',
  'home.lines.ssh.needs':
    'The port you already have open. With the key Homeport makes, one line in {file}.',
  'home.lines.tls.reaches':
    "A Docker-compatible Engine's API, with your CA and client certificate.",
  'home.lines.tls.needs': 'The Engine’s own API port (usually {port}), which you expose yourself.',
  'home.lines.tcp.reaches': "A Docker-compatible Engine's API, unencrypted.",
  'home.lines.tcp.needs': 'The Engine’s API port (usually {port}). Only on a network you trust.',
  'home.lines.cluster.name': 'Cluster API',
  'home.lines.cluster.reaches':
    'Kubernetes from your kubeconfig, Nomad by its agent’s address, Incus over HTTPS with a trust token.',
  'home.lines.cluster.needs': 'The API the cluster already serves. Nothing is installed on a node.',
  'home.lines.advice':
    'When you can, use SSH: it adds nothing to the server you’re not already running.',
  'home.lines.import':
    'Already described your servers? Import them from {file}, jump servers included, or from your Docker contexts.',
  'home.lines.link': 'How connections work',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': 'Every Engine, in the same words.',
  'home.engines.lead':
    'A Host is a Host and a Stack is a Stack, whichever Engine runs it. Where an Engine has something of its own, Homeport shows that too.',
  'home.engines.docker.body':
    'Containers, images, volumes, networks and Compose Stacks: kept in Homeport, linked to a folder, or synced from a git repository.',
  'home.engines.swarm.body':
    'Nodes, services, tasks, secrets and configs, found through its manager Hosts. Deploy Stacks to the Swarm.',
  'home.engines.podman.body':
    'Pods, and Quadlet units installed and started under systemd, through Podman’s own API.',
  'home.engines.containerd.body':
    'Namespaces, tasks, logs, a terminal and the snapshots eating your disk, with no Docker.',
  'home.engines.incus.body':
    'System containers and virtual machines: their console, snapshots, images, profiles, storage and networks.',
  'home.engines.kubernetes.body':
    'Workloads, Pods, Services and any API resource, live. Logs, terminals, port forwards, YAML with a preview, Helm releases.',
  'home.engines.nomad.body':
    'Jobs run from HCL with their plan, their allocations and logs, a shell in a task, and node drains.',

  // Home: security.
  'home.security.title': 'Your keys stay sealed.',
  'home.security.lead':
    'Homeport holds the credentials that open your servers, so it treats them that way.',
  'home.security.rest.title': 'Encrypted at rest',
  'home.security.rest.body':
    'SSH keys, TLS certificates, Stack environments and repository tokens are sealed with XChaCha20-Poly1305. The key lives in your OS keychain, not next to the data.',
  'home.security.disk.title': 'Never on disk in clear',
  'home.security.disk.body':
    'Homeport speaks SSH itself, so decrypted keys stay in memory. No key files, no changes to your {dir}.',
  'home.security.home.title': 'Nothing phones home',
  'home.security.home.body':
    'No account, no telemetry, no tracking. Every release is built in the open, with checksums and build provenance.',
  'home.security.link': 'Security and your data',

  // Home: the close, where the visitor starts.
  'home.close.title': 'Start from this machine.',
  'home.close.body':
    'Download Homeport on the computer you work from, then add your Hosts. It’s free, open source under the GNU AGPL, and built in the open on GitHub.',
  'home.close.github': 'View on GitHub',
  'home.desktops.windows.versions': '10 and 11, x64 and ARM64',
  'home.desktops.windows.formats': 'Installer (.exe) or MSI',
  'home.desktops.macos.versions': 'Apple silicon and Intel',
  'home.desktops.macos.formats': 'Disk image (.dmg)',
  'home.desktops.linux.versions': 'x64 and ARM64',
  'home.desktops.linux.formats': 'apt, dnf, zypper or AppImage',
};
