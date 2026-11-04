import type { Container, Freshness, FreshnessResult, Image } from '@/shared/api';

/** The freshness of each reference and image pair, for quick lookups. */
export type FreshnessIndex = Map<string, Freshness>;

const key = (reference: string, imageId: string) => `${reference}\u0000${imageId}`;

export function indexFreshness(results: FreshnessResult[] | undefined): FreshnessIndex {
  return new Map((results ?? []).map((r) => [key(r.reference, r.imageId), r.freshness]));
}

/** Whether the registry has a newer image under the tag a container was created from. */
export const containerFreshness = (index: FreshnessIndex, c: Container): Freshness =>
  index.get(key(c.image, c.imageId)) ?? 'unknown';

/** The tags of an image that have a newer version in their registry. */
export const newerTags = (index: FreshnessIndex, image: Image): string[] =>
  image.tags.filter((t) => index.get(key(t, image.id)) === 'newer');

/**
 * One result for several (a Stack's services): newer if any is newer,
 * unknown only if all are, up to date otherwise.
 */
export function aggregateFreshness(all: Freshness[]): Freshness {
  if (all.includes('newer')) return 'newer';
  if (all.length === 0 || all.every((f) => f === 'unknown')) return 'unknown';
  return 'upToDate';
}

/** The sentence for one or more references that have a newer image. */
export function newerLabel(references: string[]): string {
  if (references.length === 1) return `A newer ${references[0]} was published`;
  return `Newer images were published: ${references.join(', ')}`;
}
