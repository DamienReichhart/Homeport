import { useQueryClient, type QueryKey } from '@tanstack/react-query';
import { useEffect } from 'react';
import { useHostStatusUpdates } from '@/features/hosts';
import { events, keys, type EngineEvent } from '@/shared/api';

/** Which cached lists an Engine event makes stale. */
export function staleKeys(hostId: string, kind: string): QueryKey[] {
  switch (kind) {
    case 'container':
      return [
        keys.containers(hostId),
        keys.stacks(hostId),
        keys.system(hostId),
        keys.volumes(hostId),
        keys.networks(hostId),
      ];
    case 'image':
      return [keys.images(hostId), keys.system(hostId)];
    case 'volume':
      return [keys.volumes(hostId), keys.system(hostId)];
    case 'network':
      return [keys.networks(hostId)];
    default:
      return [];
  }
}

const EVENT_HISTORY = 500;

/**
 * Keeps cached Engine data fresh from pushed events instead of polling;
 * bursts (a stack starting) collapse into one refetch per list.
 */
export function LiveUpdates() {
  const client = useQueryClient();
  useHostStatusUpdates();

  useEffect(() => {
    const pending = new Map<string, QueryKey>();
    let timer: ReturnType<typeof setTimeout> | undefined;
    const flush = () => {
      pending.forEach((key) => void client.invalidateQueries({ queryKey: key }));
      pending.clear();
    };
    const unlisten = events.engineEventReceived.listen(({ payload }) => {
      client.setQueryData<EngineEvent[]>(keys.events(payload.hostId), (list) =>
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

  return null;
}
