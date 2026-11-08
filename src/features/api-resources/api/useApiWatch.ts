import { useQueryClient } from '@tanstack/react-query';
import { useEffect } from 'react';
import type { ApiType } from '@/shared/api';
import { watchApi } from './queries';

/** While mounted, refreshes `keys` whenever the objects of `api` change. */
export function useApiWatch(
  clusterId: string,
  api: ApiType | undefined,
  namespace: string | null,
  key: readonly unknown[],
) {
  const client = useQueryClient();
  const apiKey = api && JSON.stringify(api);
  const queryKey = JSON.stringify(key);
  useEffect(() => {
    if (!apiKey) return;
    return watchApi(
      clusterId,
      JSON.parse(apiKey) as ApiType,
      namespace,
      () => void client.invalidateQueries({ queryKey: JSON.parse(queryKey) as unknown[] }),
    );
  }, [client, clusterId, apiKey, namespace, queryKey]);
}
