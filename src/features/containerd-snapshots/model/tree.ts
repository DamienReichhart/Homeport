import type { ContainerdSnapshot } from '@/shared/api';

export interface SnapshotRow {
  snapshot: ContainerdSnapshot;
  /** How many parents above it are listed too. */
  depth: number;
}

/** Whether nothing uses a snapshot and nothing is built on it, so it can go. */
export function removable(snapshot: ContainerdSnapshot): boolean {
  return snapshot.users.length === 0 && snapshot.children === 0;
}

const bySize = (a: ContainerdSnapshot, b: ContainerdSnapshot) =>
  (b.size ?? -1) - (a.size ?? -1) || a.key.localeCompare(b.key);

/**
 * The snapshots as a tree by parent, depth first: base layers (and those
 * whose parent isn't listed) first, each followed by what's built on it,
 * largest first at every level.
 */
export function snapshotTree(snapshots: ContainerdSnapshot[]): SnapshotRow[] {
  const keys = new Set(snapshots.map((s) => s.key));
  const children = new Map<string, ContainerdSnapshot[]>();
  const roots: ContainerdSnapshot[] = [];
  for (const s of snapshots) {
    if (s.parent && keys.has(s.parent)) {
      children.set(s.parent, [...(children.get(s.parent) ?? []), s]);
    } else roots.push(s);
  }
  const rows: SnapshotRow[] = [];
  const seen = new Set<string>();
  const visit = (s: ContainerdSnapshot, depth: number) => {
    if (seen.has(s.key)) return;
    seen.add(s.key);
    rows.push({ snapshot: s, depth });
    for (const child of [...(children.get(s.key) ?? [])].sort(bySize)) visit(child, depth + 1);
  };
  for (const root of roots.sort(bySize)) visit(root, 0);
  return rows;
}

/** A layer's chain ID, shortened for a table: `sha256:7761870a5aa9`. */
export function shortKey(key: string): string {
  const match = /^(sha256:)([0-9a-f]{64})$/.exec(key);
  return match ? `${match[1]}${match[2]!.slice(0, 12)}` : key;
}
