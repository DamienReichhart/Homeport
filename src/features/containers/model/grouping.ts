import type { Container } from '@/shared/api';

export interface StackGroup {
  stack: string;
  /** A swarm stack's tasks on this Host, rather than a Compose project. */
  swarm: boolean;
  containers: Container[];
  running: number;
}

/** The label `docker stack deploy` puts on a swarm stack's containers. */
export const STACK_NAMESPACE = 'com.docker.stack.namespace';

/** The Compose project of a container, or else the swarm stack its task belongs to. */
function stackOf(c: Container): { stack: string; swarm: boolean } | null {
  if (c.stack) return { stack: c.stack, swarm: false };
  const namespace = c.labels[STACK_NAMESPACE];
  return namespace ? { stack: namespace, swarm: true } : null;
}

export interface Grouped {
  groups: StackGroup[];
  standalone: Container[];
}

/** Whether a container matches the search box: name, image, Stack, service or id prefix. */
export function matches(c: Container, query: string): boolean {
  const q = query.trim().toLowerCase();
  if (!q) return true;
  return (
    c.name.toLowerCase().includes(q) ||
    c.image.toLowerCase().includes(q) ||
    (c.stack?.toLowerCase().includes(q) ?? false) ||
    (c.service?.toLowerCase().includes(q) ?? false) ||
    c.id.startsWith(q)
  );
}

/**
 * Nests containers under their Stack, like Docker Desktop's list: a Compose
 * project's, or a swarm stack's tasks running on this Host.
 */
export function groupByStack(containers: Container[], query = '', onlyRunning = false): Grouped {
  const visible = containers.filter(
    (c) => matches(c, query) && (!onlyRunning || c.state === 'running'),
  );
  const byStack = new Map<string, { stack: string; swarm: boolean; members: Container[] }>();
  const standalone: Container[] = [];
  for (const c of visible) {
    const of = stackOf(c);
    if (!of) {
      standalone.push(c);
      continue;
    }
    const key = `${of.swarm ? 'swarm' : 'compose'}:${of.stack}`;
    const group = byStack.get(key) ?? { ...of, members: [] };
    group.members.push(c);
    byStack.set(key, group);
  }
  const groups = [...byStack.values()]
    .sort((a, b) => a.stack.localeCompare(b.stack) || Number(a.swarm) - Number(b.swarm))
    .map(({ stack, swarm, members }) => ({
      stack,
      swarm,
      containers: members.sort((a, b) => a.name.localeCompare(b.name)),
      running: members.filter((c) => c.state === 'running').length,
    }));
  return { groups, standalone: standalone.sort((a, b) => a.name.localeCompare(b.name)) };
}

/** `0.0.0.0:8080->80/tcp`-style labels, published ports first. */
export function portLabels(c: Container): { label: string; publicPort: number | null }[] {
  return [...c.ports]
    .sort((a, b) => Number(b.publicPort !== null) - Number(a.publicPort !== null))
    .map((p) => ({
      label: p.publicPort
        ? `${p.publicPort}:${p.privatePort}/${p.protocol}`
        : `${p.privatePort}/${p.protocol}`,
      publicPort: p.publicPort,
    }));
}
