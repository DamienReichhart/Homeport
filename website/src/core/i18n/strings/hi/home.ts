import type { Translation } from '../../types';
import type en from '../en/home';

/** The home page, in Hindi. Still in English: translate each value of ../en/home.ts. */
export default {
  // Home: the hero and its board.
  'home.title': 'Homeport — Every Docker Engine, on one board.',
  'home.hero.new': 'New',
  'home.hero.release': 'Homeport {version} is out',
  'home.hero.title': 'Every Docker Engine, on one board.',
  'home.hero.lead':
    'Homeport runs on your machine and reaches your Hosts over SSH, TLS or TCP. No agent on the server. Free, open source, Windows, macOS, Linux.',
  'home.hero.source': 'Read the source',
  'home.board.caption':
    'An example board: five Hosts, their Engines and how Homeport reaches them. One needs attention.',
  'home.board.host': 'Host',
  'home.board.engine': 'Engine',
  'home.board.via': 'Via',
  'home.board.stacks': 'Stacks',
  'home.board.status': 'Status',
  'home.board.allRunning': 'All running',
  'home.board.needsYou': '1 Stack needs you',
  'home.board.note': 'Healthy is plain. Only what needs you turns red.',
  'home.runtimes.label': 'Works with',
  'home.runtimes.title': 'Works with what you already run',

  // Home: where it lives.
  'home.visits.eyebrow': 'Where it lives',
  'home.visits.title': 'Homeport stays on your machine and visits your servers. It never moves in.',
  'home.visits.agent.title': 'No agent',
  'home.visits.agent.body':
    'Nothing to install, update or secure on the server. Homeport talks to the Engine that is already there.',
  'home.visits.panel.title': 'No web panel',
  'home.visits.panel.body':
    'No dashboard deployed on your servers, and no login page waiting for someone to find it.',
  'home.visits.account.title': 'No account',
  'home.visits.account.body':
    'No sign-in, no seat, no subscription and no telemetry. It shows you your containers, and that is all.',
  'home.visits.ports.title': 'What it needs on the server, honestly',
  'home.visits.ports.ssh':
    'The port you already have open. With the key Homeport makes, one line in {file}.',
  'home.visits.ports.tls': 'The Engine’s own API port (usually {port}), which you expose yourself.',
  'home.visits.ports.tcp':
    'The Engine’s API port (usually {port}), unencrypted. Only on a network you trust.',
  'home.visits.ports.advice':
    'When you can, use SSH: it adds nothing to the server you’re not already running.',
  'home.visits.link': 'How connections work',

  // Home: the features.
  'home.features.eyebrow': 'Features',
  'home.features.title': 'Everything your Engines run, in one window',
  'home.features.lead':
    'The day-to-day of Docker — what is running, what it says, what it costs — without remembering which server it lives on.',
  'home.features.hosts.title': 'Hosts, near and far',
  'home.features.hosts.body':
    'Local Engines and Podman machines are found for you. Remote ones join over SSH, through bastions if need be, mutual TLS or TCP, or straight from your ~/.ssh/config and Docker contexts. Each Host reconnects on its own when the network blinks.',
  'home.features.containers.title': 'Containers at a glance',
  'home.features.containers.body':
    'Grouped by Stack, with live CPU and memory and the reason behind an unhealthy one. Run with limits, devices and healthchecks, then edit, duplicate, commit or export a container.',
  'home.features.stacks.title': 'Compose Stacks',
  'home.features.stacks.body':
    'Start from a template, link a project folder on this computer, or follow a git repository, then deploy to any Host — builds included. See what changes before each redeploy or sync, roll back, and update to newer images in one step.',
  'home.features.swarm.title': 'Docker Swarm',
  'home.features.swarm.body':
    'Swarms show up through their manager Hosts. Manage nodes, services, tasks, secrets and configs, deploy Stacks to a Swarm, and start, join or leave one from a Host, with nothing installed on any node.',
  'home.features.podman.title': 'Podman, as Podman',
  'home.features.podman.body':
    'Homeport recognises Podman when it connects and adds what only Podman has: pods, created with their ports and run containers in, and Quadlet units, installed, edited and started under systemd, through Podman’s own API.',
  'home.features.containerd.title': 'containerd, directly',
  'home.features.containerd.body':
    'Connect to containerd on its socket, locally or over SSH, with no Docker: pick a namespace, start, stop and signal its containers’ tasks, read their logs, open a terminal, pull images with your registry logins, and find the snapshots eating your disk.',
  'home.features.incus.title': 'Incus instances',
  'home.features.incus.body':
    'Add Incus over its socket, SSH or HTTPS with a trust token, and run its system containers and virtual machines: create them from a remote image, start, freeze and stop them, open their console or a shell, take and restore snapshots, and manage images, profiles, storage and networks.',
  'home.features.clusters.title': 'Kubernetes and Nomad clusters',
  'home.features.clusters.body':
    'Add Kubernetes clusters from your kubeconfig — EKS, GKE and AKS included — or find local ones in one click. Browse workloads, Pods, Services, configuration and any API resource live; follow logs, open a terminal, forward ports, scale and apply YAML with a preview, and install, upgrade and roll back Helm charts. Add Nomad clusters by address to run jobs from HCL with their plan, scale and stop them, follow logs, open a shell and drain nodes. Nothing is installed on a cluster.',
  'home.features.images.title': 'Images',
  'home.features.images.body':
    'Search Docker Hub, pull and push with your registry logins, build from a folder on your computer, save and load archives, and see when a newer image is out.',
  'home.features.volumes.title': 'Volumes and networks',
  'home.features.volumes.body':
    'See which containers use what. Create, connect and disconnect, with deletion that tells you exactly what you are about to lose.',
  'home.features.board.title': 'One board for every Host',
  'home.features.board.body':
    'Every container of every Host in one table, filtered and acted on together. Each Host on one page too: engine details, disk usage with a one-dialog clean-up, and a live stream of what the Engine is doing.',
  'home.features.keyboard.title': 'Keyboard first',
  'home.features.keyboard.body':
    'A command palette to go anywhere, switch Host or run a command, and to find any container, image or volume on whichever Host has it. Shortcuts for every page, listed with ?.',
  'home.features.background.title': 'Watching in the background',
  'home.features.background.body':
    'Every Host’s status in the system tray, notifications when a container crashes or a Host drops, and Homeport starting at login, hidden until you need it.',

  // Home: Hosts.
  'home.hosts.eyebrow': 'Hosts',
  'home.hosts.title': 'Remote Engines without the ceremony',
  'home.hosts.body':
    "Add a server once and it stays on the board. For SSH, Homeport makes a key just for that Host and hands you the one command that authorises it. It checks the server's fingerprint on first contact, and warns you if it ever changes.",
  'home.hosts.import':
    'Already described them? Import your servers from {file}, jump servers included, or your Docker contexts, certificates included. Moving to a new computer, take them along in one file locked with a passphrase.',
  'home.hosts.socket.name': 'Local socket',
  'home.hosts.socket.detail': 'Unix sockets and Windows named pipes, detected automatically.',
  'home.hosts.ssh.detail':
    'A dedicated Ed25519 key per Host, your own key, ssh-agent or Pageant; through bastions too.',
  'home.hosts.tls.detail': 'Mutual TLS with your CA, client certificate and key.',
  'home.hosts.tcp.detail': 'Plain, unencrypted TCP, only for an Engine on a network you trust.',
  'home.hosts.link': 'How connections work',
  'home.hosts.screenshot':
    'The Add host page with SSH selected: server, port, user, remote socket path and a new key for this Host.',

  // Home: containers.
  'home.containers.eyebrow': 'Containers',
  'home.containers.title': 'Everything about a container, one click away',
  'home.containers.body':
    'Open a container and its logs are already streaming. Switch tabs for an interactive terminal, its files, live charts, processes and the full inspect document — on a Host across the world as easily as on your laptop.',
  'home.containers.logs.title': 'Logs',
  'home.containers.logs.body': 'Follow, search, filter by stream and time, download.',
  'home.containers.terminal.title': 'Terminal',
  'home.containers.terminal.body': 'Interactive shells, as many as you need.',
  'home.containers.files.title': 'Files',
  'home.containers.files.body': 'Browse, upload and download, folders as .tar.',
  'home.containers.stats.title': 'Stats',
  'home.containers.stats.body': 'CPU, memory, network and disk I/O charts.',
  'home.containers.link': 'Working with containers',
  'home.containers.screenshot':
    "A container's Logs tab, streaming output with search, timestamps and stream filters.",

  // Home: Stacks.
  'home.stacks.eyebrow': 'Stacks',
  'home.stacks.title': 'Compose Stacks, managed, linked, synced or found',
  'home.stacks.body':
    'Keep a compose file and its environment in Homeport and deploy it to any Host — start from a template, combine several files, choose profiles; variables are stored encrypted, never written to disk. Or link a project folder on this computer: Homeport edits its files in place and deploys from it, builds included. Or follow a branch of a git repository, with a deploy key or a token: each sync shows the new commits and what they change before deploying, and while Homeport is open it checks for them, deploying on its own if you ask. Every redeploy shows what changes first, each deploy is kept so you can roll back, and Update pulls newer images and redeploys in one step. Stacks someone deployed by hand show up too: start, stop or remove them, or adopt them.',
  'home.stacks.link': 'Managing Stacks',
  'home.stacks.screenshot':
    "A Managed Stack's definition: the compose.yaml editor, its environment and the Save and redeploy button.",

  // Home: security.
  'home.security.eyebrow': 'Security',
  'home.security.title': 'Your keys stay sealed',
  'home.security.lead':
    'Homeport handles the credentials that open your servers, so it treats them that way.',
  'home.security.rest.title': 'Encrypted at rest',
  'home.security.rest.body':
    'SSH keys, TLS certificates, Stack environments and repository tokens are sealed with XChaCha20-Poly1305. The key lives in your OS keychain, not next to the data.',
  'home.security.disk.title': 'Never on disk in clear',
  'home.security.disk.body':
    'Homeport speaks SSH itself, so decrypted keys stay in memory. No key files, no changes to your {dir}.',
  'home.security.home.title': 'Nothing phones home',
  'home.security.home.body':
    'No account, no telemetry, no tracking. Homeport talks to your Engines and nothing else. Every release is built in the open, with checksums and build provenance.',
  'home.security.link': 'Security and your data',

  // Home: desktops and the closing call.
  'home.desktops.eyebrow': 'Download',
  'home.desktops.title': 'Native on every desktop',
  'home.desktops.lead':
    'A small, fast app built with Tauri and Rust — not a browser tab in disguise. Installers for x64 and ARM that keep Homeport up to date.',
  'home.desktops.windows.versions': '10 and 11 · x64 and ARM64',
  'home.desktops.windows.formats': 'Installer (.exe) · MSI',
  'home.desktops.macos.versions': 'Apple silicon and Intel',
  'home.desktops.macos.formats': 'Disk image (.dmg)',
  'home.desktops.linux.versions': 'x64 and ARM64',
  'home.desktops.linux.formats': 'apt · dnf · zypper · AppImage',
  'home.cta.title': 'Nothing to install on your servers.',
  'home.cta.body':
    'Download Homeport on the machine you work from and add your Hosts. It’s free, open source under the GNU AGPL, and built in the open on GitHub.',
  'home.cta.github': 'View on GitHub',
} satisfies Translation<typeof en>;
