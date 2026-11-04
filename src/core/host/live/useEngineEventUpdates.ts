import { useQueryClient, type QueryKey } from '@tanstack/react-query';
import { useEffect } from 'react';
import { events, type EngineEvent } from '@/shared/api';
import { hostKeys } from '../api/keys';

/** Which cached lists an Engine event makes stale. */
export function staleKeys(hostId: string, kind: string): QueryKey[] {
  switch (kind) {
    case 'container':
      return [
        hostKeys.containers(hostId),
        hostKeys.stacks(hostId),
        hostKeys.system(hostId),
        hostKeys.volumes(hostId),
        hostKeys.networks(hostId),
      ];
    case 'image':
      return [hostKeys.images(hostId), hostKeys.system(hostId)];
    case 'volume':
      return [hostKeys.volumes(hostId), hostKeys.system(hostId)];
    case 'network':
      return [hostKeys.networks(hostId)];
    default:
      return [];
  }
}

const EVENT_HISTORY = 500;

/**
 * Keeps cached Engine data fresh from pushed events instead of polling;
 * bursts (a stack starting) collapse into one refetch per list.
 */
export function useEngineEventUpdates() {
  const client = useQueryClient();
  useEffect(() => {
    const pending = new Map<string, QueryKey>();
    let timer: ReturnType<typeof setTimeout> | undefined;
    const flush = () => {
      pending.forEach((key) => void client.invalidateQueries({ queryKey: key }));
      pending.clear();
    };
    const unlisten = events.engineEventReceived.listen(({ payload }) => {
      client.setQueryData<EngineEvent[]>(hostKeys.events(payload.hostId), (list) =>
        list ? [...list.slice(-(EVENT_HISTORY - 1)), payload.event] : list,
      );
      for (const key of staleKeys(payload.hostId, payload.event.kind))
        pending.set(JSON.stringify(key), key);
      clearTimeout(timer);
      timer = setTimeout(flush, 250);
    });
    return () => {
      clearTimeout(timer);
      void unlisten.then((stop) => stop());
    };
  }, [client]);
}
