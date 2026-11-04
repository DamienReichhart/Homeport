/**
 * The example Hosts on the home page's departure board. Sample data, not a
 * customer's: the names, versions and counts are made up to look like a real
 * board, and every connection type appears, and local Engines sit on one Mac.
 */
export type Tone = 'success' | 'warning' | 'danger';

export interface BoardRow {
  host: string;
  engine: string;
  via: 'socket' | 'ssh' | 'tls' | 'tcp';
  address: string;
  stacks: number;
  containers: number;
  status: string;
  tone: Tone;
  /** What the open row says, in a sentence. */
  detail: string;
  /** A status the row moves to and back from while the board is in view. */
  flips?: { status: string; tone: Tone };
}

export const boardRows: BoardRow[] = [
  {
    host: 'laptop',
    engine: 'OrbStack 1.9',
    via: 'socket',
    address: '~/.orbstack/run/docker.sock',
    stacks: 4,
    containers: 11,
    status: 'All running',
    tone: 'success',
    detail: 'Found on this machine, no setup. 4 Stacks and 11 containers, all running.',
  },
  {
    host: 'homelab',
    engine: 'Docker Engine 28.5',
    via: 'ssh',
    address: 'deploy@homelab.lan',
    stacks: 9,
    containers: 31,
    status: 'All running',
    tone: 'success',
    detail: '31 containers across 9 Stacks, reached over the SSH port it already had open.',
  },
  {
    host: 'nas',
    engine: 'Podman 5.2',
    via: 'ssh',
    address: 'admin@nas.lan',
    stacks: 2,
    containers: 5,
    status: 'All running',
    tone: 'success',
    detail: 'Podman’s Docker-compatible socket, over SSH. 2 Stacks, 5 containers.',
  },
  {
    host: 'vps-fra-1',
    engine: 'Docker Engine 27.3',
    via: 'tls',
    address: 'vps-fra-1.example.com:2376',
    stacks: 3,
    containers: 8,
    status: 'Needs you',
    tone: 'danger',
    detail:
      'In the Stack shop, api exited with code 137 four minutes ago. The other 7 containers are running.',
  },
  {
    host: 'client-prod',
    engine: 'Docker Engine 28.1',
    via: 'ssh',
    address: 'ops@client-prod.example.com',
    stacks: 6,
    containers: 19,
    status: 'All running',
    tone: 'success',
    detail: 'A client’s server, reached with a key Homeport made for it alone. 19 containers.',
  },
  {
    host: 'build-box',
    engine: 'Docker Engine 28.5',
    via: 'tcp',
    address: '192.168.1.40:2375',
    stacks: 1,
    containers: 3,
    status: 'All running',
    tone: 'success',
    detail: 'On the office network, over plain TCP. The runner restarts between jobs.',
    flips: { status: 'Restarting', tone: 'warning' },
  },
  {
    host: 'dev-vm',
    engine: 'Colima 0.9',
    via: 'socket',
    address: '~/.colima/default/docker.sock',
    stacks: 3,
    containers: 7,
    status: 'All running',
    tone: 'success',
    detail: 'A second Engine on the same laptop, side by side with OrbStack.',
  },
  {
    host: 'staging',
    engine: 'Docker Engine 28.4',
    via: 'tls',
    address: 'staging.example.com:2376',
    stacks: 5,
    containers: 14,
    status: 'All running',
    tone: 'success',
    detail: 'Mutual TLS with the team’s own certificate authority. 14 containers in 5 Stacks.',
  },
];

/** The row that is open when the page loads: the one that needs you. */
export const openRow = boardRows.findIndex((row) => row.tone === 'danger');
