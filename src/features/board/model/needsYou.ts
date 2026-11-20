import type { WorkloadView } from '@/shared/api';
import type { BoardRow } from './board';

/** Why something is on the Board's "Needs you" list. */
export type NeedReason =
  | { kind: 'host-unreachable'; message: string }
  | { kind: 'cluster-unreachable'; message: string }
  | { kind: 'unhealthy' }
  | { kind: 'dead' }
  | { kind: 'restarting' }
  | { kind: 'exited'; code: number }
  | { kind: 'workload-unready'; workload: WorkloadView }
  | { kind: 'stack-updates'; stack: string; containers: number };

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

/** A Stack on a Host whose containers run images with a newer version published. */
export interface StackUpdate {
  hostId: string;
  hostName: string;
  stack: string;
  containers: number;
}

/** The Stacks among `newer`, rows whose image has a newer version, with how many each. */
export function stackUpdates(newer: BoardRow[]): StackUpdate[] {
  const byStack = new Map<string, StackUpdate>();
  for (const row of newer) {
    if (!row.container.stack) continue;
    const key = `${row.hostId}/${row.container.stack}`;
    const known = byStack.get(key);
    if (known) known.containers += 1;
    else
      byStack.set(key, {
        hostId: row.hostId,
        hostName: row.hostName,
        stack: row.container.stack,
        containers: 1,
      });
  }
  return [...byStack.values()];
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
  'workload-unready': 'warning',
  'stack-updates': 'warning',
};

/**
 * Everything on the Board that asks for attention, red first: unreachable
 * Hosts and Clusters, unhealthy and dead containers and workloads with no
 * Pod ready, then restarting and failed containers, workloads short of Pods,
 * and Stacks with newer images. Healthy things never appear.
 */
export function needsYou({
  hosts,
  clusters,
  rows,
  unready = [],
  updates = [],
}: {
  hosts: { id: string; name: string; unreachable: string | null }[];
  clusters: { id: string; name: string; unreachable: string | null }[];
  rows: BoardRow[];
  unready?: { clusterId: string; clusterName: string; workloads: WorkloadView[] }[];
  updates?: StackUpdate[];
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
  for (const cluster of unready) {
    for (const workload of cluster.workloads) {
      needs.push({
        key: `workload/${cluster.clusterId}/${workload.kind}/${workload.namespace}/${workload.name}`,
        // None of its Pods ready means it's down, not just short.
        tone: workload.ready === 0 && workload.desired > 0 ? 'danger' : 'warning',
        reason: { kind: 'workload-unready', workload },
        targetId: cluster.clusterId,
        targetName: cluster.clusterName,
      });
    }
  }
  for (const update of updates) {
    needs.push({
      key: `stack/${update.hostId}/${update.stack}`,
      tone: 'warning',
      reason: { kind: 'stack-updates', stack: update.stack, containers: update.containers },
      targetId: update.hostId,
      targetName: update.hostName,
    });
  }
  // Stable: within a tone, the order above (targets first, then by Host).
  return needs.sort((a, b) => (a.tone === b.tone ? 0 : a.tone === 'danger' ? -1 : 1));
}
