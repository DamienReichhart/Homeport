import type { BoardRow } from './board';

/** Why something is on the Board's "Needs you" list. */
export type NeedReason =
  | { kind: 'host-unreachable'; message: string }
  | { kind: 'cluster-unreachable'; message: string }
  | { kind: 'unhealthy' }
  | { kind: 'dead' }
  | { kind: 'restarting' }
  | { kind: 'exited'; code: number };

export interface Need {
  key: string;
  /** Red when it needs you now, amber when it needs a look. */
  tone: 'danger' | 'warning';
  reason: NeedReason;
  /** The Host or Cluster it concerns. */
  targetId: string;
  targetName: string;
  /** The container concerned, if it is one. */
  row?: BoardRow;
}

/** The exit code in an Engine status such as "Exited (137) 3 hours ago". */
export function exitCode(status: string): number | null {
  const match = /^Exited \((-?\d+)\)/.exec(status);
  return match ? Number(match[1]) : null;
}

/** What a container on the Board needs, if anything. */
function containerNeed(row: BoardRow): NeedReason | null {
  const { container } = row;
  if (container.health === 'unhealthy') return { kind: 'unhealthy' };
  if (container.state === 'dead') return { kind: 'dead' };
  if (container.state === 'restarting') return { kind: 'restarting' };
  if (container.state === 'exited') {
    const code = exitCode(container.status);
    // 0 is a clean stop, 143 a stop by SIGTERM: neither is a failure.
    if (code !== null && code !== 0 && code !== 143) return { kind: 'exited', code };
  }
  return null;
}

const TONE: Record<NeedReason['kind'], Need['tone']> = {
  'host-unreachable': 'danger',
  'cluster-unreachable': 'danger',
  unhealthy: 'danger',
  dead: 'danger',
  restarting: 'warning',
  exited: 'warning',
};

/**
 * Everything on the Board that asks for attention, red first: unreachable
 * Hosts and Clusters, then unhealthy, dead, restarting and failed containers.
 * Healthy things never appear.
 */
export function needsYou({
  hosts,
  clusters,
  rows,
}: {
  hosts: { id: string; name: string; unreachable: string | null }[];
  clusters: { id: string; name: string; unreachable: string | null }[];
  rows: BoardRow[];
}): Need[] {
  const needs: Need[] = [];
  for (const host of hosts) {
    if (host.unreachable === null) continue;
    needs.push({
      key: `host/${host.id}`,
      tone: 'danger',
      reason: { kind: 'host-unreachable', message: host.unreachable },
      targetId: host.id,
      targetName: host.name,
    });
  }
  for (const cluster of clusters) {
    if (cluster.unreachable === null) continue;
    needs.push({
      key: `cluster/${cluster.id}`,
      tone: 'danger',
      reason: { kind: 'cluster-unreachable', message: cluster.unreachable },
      targetId: cluster.id,
      targetName: cluster.name,
    });
  }
  for (const row of rows) {
    const reason = containerNeed(row);
    if (!reason) continue;
    needs.push({
      key: row.key,
      tone: TONE[reason.kind],
      reason,
      targetId: row.hostId,
      targetName: row.hostName,
      row,
    });
  }
  // Stable: within a tone, the order above (targets first, then by Host).
  return needs.sort((a, b) => (a.tone === b.tone ? 0 : a.tone === 'danger' ? -1 : 1));
}
