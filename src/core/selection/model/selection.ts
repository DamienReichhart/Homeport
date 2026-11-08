import type { Selection } from '@/shared/api';

/**
 * What the resource pages show: one Host or one Cluster (the settings'
 * type). `null` before anything is added.
 */
export type { Selection };

export type SelectionKind = Selection['kind'];

/** A Host or a Cluster the selection may fall back to. */
export interface Candidate {
  id: string;
  /** Connected, so its pages show something at once. */
  usable: boolean;
}

/** The remembered selection, from the settings of this or an older version. */
export function rememberedSelection(settings: {
  lastSelection?: Selection | null;
  lastHostId?: string | null;
}): Selection | null {
  if (settings.lastSelection) return settings.lastSelection;
  return settings.lastHostId ? { kind: 'host', id: settings.lastHostId } : null;
}

/**
 * The selection the pages use: `chosen` while it still exists, else the
 * first usable Host, the first Host, the first usable Cluster or the first
 * Cluster. A list still loading (`undefined`) keeps `chosen` as it is.
 */
export function resolveSelection(
  chosen: Selection | null,
  hosts: Candidate[] | undefined,
  clusters: Candidate[] | undefined,
): Selection | null {
  if (chosen) {
    const list = chosen.kind === 'host' ? hosts : clusters;
    if (!list || list.some((c) => c.id === chosen.id)) return chosen;
  } else if (!hosts) {
    return null;
  }
  const host = hosts?.find((h) => h.usable) ?? hosts?.[0];
  if (host) return { kind: 'host', id: host.id };
  const cluster = clusters?.find((c) => c.usable) ?? clusters?.[0];
  if (cluster) return { kind: 'cluster', id: cluster.id };
  return null;
}

/** Whether two selections point at the same Host or Cluster. */
export const sameSelection = (a: Selection | null, b: Selection | null) =>
  a?.kind === b?.kind && a?.id === b?.id;
