import { resourceQueries, useEveryHost } from '@/core/host';
import { MIN_QUERY, search, type SearchHit } from '../model/search';

/**
 * Searches the containers, Stacks, images, volumes and networks of every
 * connected Host. Lists are loaded only while `active` and once the query
 * has two characters; Engine events keep them fresh after that.
 */
export function useResourceSearch(
  query: string,
  active: boolean,
): { hits: SearchHit[]; skipped: number; loading: boolean } {
  const enabled = active && query.trim().length >= MIN_QUERY;
  const containers = useEveryHost(resourceQueries.containers, enabled);
  const stacks = useEveryHost(resourceQueries.stacks, enabled);
  const images = useEveryHost(resourceQueries.images, enabled);
  const volumes = useEveryHost(resourceQueries.volumes, enabled);
  const networks = useEveryHost(resourceQueries.networks, enabled);
  if (!enabled) return { hits: [], skipped: 0, loading: false };
  const hosts = containers.results.map((r, i) => ({
    hostId: r.hostId,
    hostName: r.hostName,
    containers: r.data,
    stacks: stacks.results[i]?.data,
    images: images.results[i]?.data,
    volumes: volumes.results[i]?.data,
    networks: networks.results[i]?.data,
  }));
  return {
    hits: search(hosts, query),
    skipped: containers.skipped.length,
    loading: [containers, stacks, images, volumes, networks].some((all) =>
      all.results.some((r) => r.isPending),
    ),
  };
}
