import type { Container } from '@/shared/api';

export interface StackGroup {
  stack: string;
  containers: Container[];
  running: number;
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

/** Nests containers under their Stack, like Docker Desktop's list. */
export function groupByStack(containers: Container[], query = '', onlyRunning = false): Grouped {
  const visible = containers.filter(
    (c) => matches(c, query) && (!onlyRunning || c.state === 'running'),
  );
  const byStack = new Map<string, Container[]>();
  const standalone: Container[] = [];
  for (const c of visible) {
    if (c.stack) byStack.set(c.stack, [...(byStack.get(c.stack) ?? []), c]);
    else standalone.push(c);
  }
  const groups = [...byStack.entries()]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([stack, members]) => ({
      stack,
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
