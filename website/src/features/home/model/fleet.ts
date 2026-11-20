/**
 * The example fleet the home page draws as a line map: this machine at the
 * terminus, one line per way of reaching an Engine, one station per Engine.
 * The names and versions are an illustration, labelled as one on the page;
 * the lines and what each reaches are what Homeport really does.
 */

/** A way Homeport reaches an Engine, drawn as a line of its own colour. */
export type LineId = 'socket' | 'ssh' | 'tls' | 'tcp' | 'cluster';

export interface Station {
  /** The Host or Cluster's name, as it would appear in the app. */
  host: string;
  /** Its Engine or orchestrator and version. */
  engine: string;
  /** Whether something on it needs you: the one red station on the map. */
  signal?: boolean;
  /** Where the station sits on the map (viewBox units), and which side its label goes. */
  x: number;
  y: number;
  label: 'above' | 'below';
  /** The label's alignment to the station; `end` for a station at the map's edge. */
  anchor?: 'start' | 'middle' | 'end';
}

export interface Line {
  id: LineId;
  /** The CSS custom property that holds the line's colour. */
  colour: string;
  /** The line's path, from the terminus to its last station (viewBox units). */
  path: string;
  stations: Station[];
}

/** The map's drawing area. */
export const MAP = { width: 1200, height: 560 } as const;

/** The terminus: this machine, where every line starts. */
export const TERMINUS = { x: 20, y: 392, width: 30, height: 96 } as const;

/*
 * The lines leave the terminus as one bundle (SSH on top, the socket at the
 * foot), clear the title panel, then turn at 45 degrees towards their
 * stations, so no two lines cross.
 */
export const LINES: Line[] = [
  {
    id: 'ssh',
    colour: '--hp-line-ssh',
    path: 'M50 404 H600 L840 164 H1176',
    stations: [
      { host: 'prod-eu-1', engine: 'Docker Engine 28.5', x: 890, y: 164, label: 'above' },
      { host: 'nas', engine: 'Podman 5.4', x: 1030, y: 164, label: 'below' },
      {
        host: 'edge-1',
        engine: 'containerd 2.1',
        x: 1176,
        y: 164,
        label: 'above',
        anchor: 'end',
      },
    ],
  },
  {
    id: 'tls',
    colour: '--hp-line-tls',
    path: 'M50 422 H640 L756 306 H1176',
    stations: [
      { host: 'staging', engine: 'Docker Engine 28.1', x: 890, y: 306, label: 'below' },
      {
        host: 'vps-fra-1',
        engine: 'Docker Engine 27.3',
        signal: true,
        x: 1176,
        y: 306,
        label: 'below',
        anchor: 'end',
      },
    ],
  },
  {
    id: 'tcp',
    colour: '--hp-line-tcp',
    path: 'M50 440 H1010',
    stations: [{ host: 'lan-box', engine: 'Docker Engine 28.1', x: 1010, y: 440, label: 'above' }],
  },
  {
    id: 'cluster',
    colour: '--hp-line-cluster',
    path: 'M50 458 H600 L682 540 H1176',
    stations: [
      { host: 'k8s-prod', engine: 'Kubernetes 1.33', x: 800, y: 540, label: 'above' },
      { host: 'jobs', engine: 'Nomad 1.10', x: 980, y: 540, label: 'above' },
      { host: 'lab', engine: 'Incus 6.12', x: 1176, y: 540, label: 'above', anchor: 'end' },
    ],
  },
  {
    id: 'socket',
    colour: '--hp-line-socket',
    path: 'M50 476 H330',
    stations: [{ host: 'laptop', engine: 'OrbStack', x: 330, y: 476, label: 'below' }],
  },
];

/** The lines in the order the key lists them: nearest first. */
export const KEY_ORDER: LineId[] = ['socket', 'ssh', 'tls', 'tcp', 'cluster'];

export const lineById = (id: LineId): Line => LINES.find((line) => line.id === id)!;

/** A line's name in the key: the transport's own name, or the page's words for the socket and the cluster API. */
export const lineName = (
  id: LineId,
  s: { 'home.lines.socket.name': string; 'home.lines.cluster.name': string },
): string =>
  id === 'socket'
    ? s['home.lines.socket.name']
    : id === 'cluster'
      ? s['home.lines.cluster.name']
      : id.toUpperCase();
