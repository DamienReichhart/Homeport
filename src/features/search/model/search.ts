import type { Container, Image, NetworkView, Stack, VolumeView } from '@/shared/api';

export type ResourceKind = 'container' | 'image' | 'volume' | 'network' | 'stack';

/** The kinds, in the order the palette lists them. */
export const KINDS: ResourceKind[] = ['container', 'stack', 'image', 'volume', 'network'];

/** One Host's resources, as far as they're loaded. */
export interface HostResources {
  hostId: string;
  hostName: string;
  containers?: Container[];
  images?: Image[];
  volumes?: VolumeView[];
  networks?: NetworkView[];
  stacks?: Stack[];
}

/** A resource that matches, on the Host it's on. */
export interface SearchHit {
  kind: ResourceKind;
  hostId: string;
  hostName: string;
  /** What opens it: a container's id, or the name of anything else. */
  id: string;
  label: string;
  /** A container's image. */
  detail?: string;
  /** `running` or `stopped`, for containers and Stacks. */
  state?: 'running' | 'stopped';
  score: number;
}

/** The fewest characters a search needs, so one letter doesn't load every list. */
export const MIN_QUERY = 2;

/**
 * How well `query` matches `texts` (lower case): 3 when one starts with it,
 * 2 when a word in one does, 1 when one contains it, 0 otherwise.
 */
function rank(texts: string[], query: string): number {
  let best = 0;
  for (const text of texts) {
    const lower = text.toLowerCase();
    if (lower.startsWith(query)) return 3;
    if (lower.split(/[\s\-_/.:@]+/).some((word) => word.startsWith(query)))
      best = Math.max(best, 2);
    else if (lower.includes(query)) best = Math.max(best, 1);
  }
  return best;
}

const short = (id: string) => id.replace(/^sha256:/, '').slice(0, 12);

/** Every resource of `hosts` matching `query`, best first, then by name and Host. */
export function search(hosts: HostResources[], query: string): SearchHit[] {
  const q = query.trim().toLowerCase();
  if (q.length < MIN_QUERY) return [];
  const hits: SearchHit[] = [];
  const add = (hit: Omit<SearchHit, 'score'>, texts: string[]) => {
    const score = rank(texts, q);
    if (score > 0) hits.push({ ...hit, score });
  };
  for (const { hostId, hostName, containers, images, volumes, networks, stacks } of hosts) {
    const on = { hostId, hostName };
    for (const c of containers ?? []) {
      add(
        {
          ...on,
          kind: 'container',
          id: c.id,
          label: c.name,
          detail: c.image,
          state: c.state === 'running' ? 'running' : 'stopped',
        },
        [c.name, short(c.id), c.image, c.stack ?? ''],
      );
    }
    for (const s of stacks ?? []) {
      add(
        {
          ...on,
          kind: 'stack',
          id: s.name,
          label: s.name,
          state: s.running > 0 ? 'running' : 'stopped',
        },
        [s.name],
      );
    }
    for (const image of images ?? []) {
      const name = image.tags[0] ?? short(image.id);
      add({ ...on, kind: 'image', id: name, label: name }, [...image.tags, short(image.id)]);
    }
    for (const v of volumes ?? [])
      add({ ...on, kind: 'volume', id: v.name, label: v.name }, [v.name]);
    for (const n of networks ?? []) {
      add({ ...on, kind: 'network', id: n.name, label: n.name }, [n.name, short(n.id)]);
    }
  }
  return hits.sort(
    (a, b) =>
      b.score - a.score || a.label.localeCompare(b.label) || a.hostName.localeCompare(b.hostName),
  );
}

/** Where a hit opens: a container or Stack's page, or the list it's in, searched for it. */
export function hitPath(hit: SearchHit): string {
  const q = `?q=${encodeURIComponent(hit.label)}`;
  switch (hit.kind) {
    case 'container':
      return `/containers/${hit.id}/logs`;
    case 'stack':
      return `/stacks/${encodeURIComponent(hit.id)}`;
    case 'image':
      return `/images${q}`;
    case 'volume':
      return `/volumes${q}`;
    case 'network':
      return `/networks${q}`;
  }
}
