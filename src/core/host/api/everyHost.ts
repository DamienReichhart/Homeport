import { useQueries, type UseQueryOptions } from '@tanstack/react-query';
import type { HostView } from '@/shared/api';
import { supports } from '../model/engine';
import { useHosts } from './queries';

/** One connected Host's answer to a query run on every Host. */
export interface HostResult<T> {
  hostId: string;
  hostName: string;
  data: T | undefined;
  error: Error | null;
  isPending: boolean;
}

/**
 * Runs `query` (one of `resourceQueries`) for every connected Host whose
 * Engine speaks the Docker Engine API, while `enabled`. Hosts that aren't
 * connected are `skipped`, not failures, so one down Host never blocks the
 * others; Hosts of another Engine (containerd) aren't asked at all.
 */
export function useEveryHost<T, K extends readonly unknown[]>(
  query: (hostId: string) => UseQueryOptions<T, Error, T, K>,
  enabled = true,
): { results: HostResult<T>[]; skipped: HostView[]; isPending: boolean } {
  const hosts = useHosts();
  const all = (hosts.data ?? []).filter((h) => supports(h, 'docker'));
  const connected = all.filter((h) => h.status.state === 'connected');
  const results = useQueries({
    queries: connected.map((h) => ({ ...query(h.host.id), enabled })),
  });
  return {
    results: connected.map((h, i) => {
      const result = results[i];
      return {
        hostId: h.host.id,
        hostName: h.host.name,
        data: result?.data,
        error: result?.error ?? null,
        isPending: result?.isPending ?? true,
      };
    }),
    skipped: all.filter((h) => h.status.state !== 'connected'),
    isPending: hosts.isPending,
  };
}
