import { useQueryClient, type QueryKey } from '@tanstack/react-query';
import { useEffect } from 'react';
import { events, type EngineEvent } from '@/shared/api';
import { hostKeys } from '../api/keys';
import { appendEvent } from '../model/events';

/**
 * Which cached lists an Engine event makes stale. containerd's events
 * (`task`, `snapshot`, `namespace`, `content`, and its own `container` and
 * `image`) refresh its pages in every namespace.
 */
export function staleKeys(hostId: string, kind: string): QueryKey[] {
  switch (kind) {
    case 'task':
      return [hostKeys.containerdContainers(hostId)];
    case 'snapshot':
      return [hostKeys.containerdSnapshots(hostId)];
    case 'namespace':
      return [hostKeys.containerdNamespaces(hostId)];
    case 'content':
      return [hostKeys.containerdImages(hostId)];
    case 'container':
      return [
        hostKeys.containerdContainers(hostId),
        hostKeys.containerdSnapshots(hostId),
        hostKeys.containerdNamespaces(hostId),
        hostKeys.containers(hostId),
        hostKeys.stacks(hostId),
        hostKeys.system(hostId),
        hostKeys.volumes(hostId),
        hostKeys.networks(hostId),
        // A pod's and a Quadlet unit's status are their containers'.
        hostKeys.pods(hostId),
        hostKeys.quadlets(hostId),
      ];
    case 'pod':
      return [hostKeys.pods(hostId)];
    case 'image':
      return [
        hostKeys.images(hostId),
        hostKeys.system(hostId),
        hostKeys.containerdImages(hostId),
        hostKeys.containerdNamespaces(hostId),
      ];
    case 'volume':
      return [hostKeys.volumes(hostId), hostKeys.volumeSizes(hostId), hostKeys.system(hostId)];
    case 'network':
      return [hostKeys.networks(hostId)];
    default:
      return [];
  }
}

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
        list ? appendEvent(list, payload.event) : list,
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
