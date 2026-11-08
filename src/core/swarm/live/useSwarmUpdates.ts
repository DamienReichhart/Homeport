import { useQueryClient, type QueryClient, type QueryKey } from '@tanstack/react-query';
import { useEffect } from 'react';
import { events, type HostView } from '@/shared/api';
import { swarmKeys } from '../api/keys';
import { managedSwarmId } from '../model/naming';

const HOSTS_KEY = ['hosts'];

/** Which of a Swarm's cached lists an Engine event on one of its managers makes stale. */
export function staleSwarmKeys(swarmId: string, kind: string): QueryKey[] {
  switch (kind) {
    case 'node':
      return [
        swarmKeys.list,
        swarmKeys.summary(swarmId),
        swarmKeys.nodes(swarmId),
        swarmKeys.tasks(swarmId),
      ];
    case 'service':
      return [
        swarmKeys.summary(swarmId),
        swarmKeys.services(swarmId),
        swarmKeys.tasks(swarmId),
        swarmKeys.stacks(swarmId),
      ];
    case 'container':
      return [swarmKeys.tasks(swarmId), swarmKeys.services(swarmId)];
    case 'secret':
      return [swarmKeys.secrets(swarmId), swarmKeys.stacks(swarmId)];
    case 'config':
      return [swarmKeys.configs(swarmId), swarmKeys.stacks(swarmId)];
    default:
      return [];
  }
}

const hostsOf = (client: QueryClient) => client.getQueryData<HostView[]>(HOSTS_KEY);

/**
 * Keeps the Swarms' cached data fresh: Engine events on a manager Host
 * refresh its Swarm's lists, and any Host status change the list of Swarms.
 * Bursts collapse into one refetch per list.
 */
export function useSwarmUpdates() {
  const client = useQueryClient();
  useEffect(() => {
    const pending = new Map<string, QueryKey>();
    let timer: ReturnType<typeof setTimeout> | undefined;
    const flush = () => {
      pending.forEach((key) => void client.invalidateQueries({ queryKey: key }));
      pending.clear();
    };
    const later = (keys: QueryKey[]) => {
      if (keys.length === 0) return;
      for (const key of keys) pending.set(JSON.stringify(key), key);
      clearTimeout(timer);
      timer = setTimeout(flush, 250);
    };
    const stopEvents = events.engineEventReceived.listen(({ payload }) => {
      const swarmId = managedSwarmId(hostsOf(client), payload.hostId);
      if (swarmId) later(staleSwarmKeys(swarmId, payload.event.kind));
    });
    const stopStatus = events.hostStatusChanged.listen(() => later([swarmKeys.list]));
    return () => {
      clearTimeout(timer);
      void stopEvents.then((stop) => stop());
      void stopStatus.then((stop) => stop());
    };
  }, [client]);
}
