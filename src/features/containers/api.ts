import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import { commands, keys, subscribe, type ContainerAction, type ContainerUsage } from '@/shared/api';
import { toastError } from '@/shared/ui';

export function useContainers(hostId: string) {
  return useQuery({
    queryKey: keys.containers(hostId),
    queryFn: () => commands.containersList(hostId),
  });
}

/** Live CPU and memory of every running container, keyed by id. */
export function useUsage(hostId: string): Map<string, ContainerUsage> {
  const [usage, setUsage] = useState(new Map<string, ContainerUsage>());
  useEffect(
    () =>
      subscribe<ContainerUsage[]>(
        (ch) => commands.usageSubscribe(hostId, ch),
        (list) => setUsage(new Map(list.map((u) => [u.id, u]))),
      ),
    [hostId],
  );
  return usage;
}

const PAST: Record<ContainerAction, string> = {
  start: 'Started',
  stop: 'Stopped',
  restart: 'Restarted',
  pause: 'Paused',
  unpause: 'Resumed',
  kill: 'Killed',
};

/** Lifecycle mutations for containers, reporting the outcome as a toast. */
export function useContainerMutations(hostId: string) {
  const client = useQueryClient();
  const refresh = () => client.invalidateQueries({ queryKey: keys.containers(hostId) });
  const act = useMutation({
    mutationFn: async ({ ids, action }: { ids: string[]; action: ContainerAction }) => {
      await Promise.all(ids.map((id) => commands.containersAct(hostId, id, action)));
      return { ids, action };
    },
    onSuccess: ({ ids, action }) => {
      toast.success(
        `${PAST[action]} ${ids.length === 1 ? 'container' : `${ids.length} containers`}`,
      );
      void refresh();
    },
    onError: (e) => toastError(e),
  });
  const remove = useMutation({
    mutationFn: async ({ ids, volumes }: { ids: string[]; volumes: boolean }) => {
      await Promise.all(ids.map((id) => commands.containersRemove(hostId, id, true, volumes)));
      return ids;
    },
    onSuccess: (ids) => {
      toast.success(`Removed ${ids.length === 1 ? 'container' : `${ids.length} containers`}`);
      void refresh();
    },
    onError: (e) => toastError(e),
  });
  return { act, remove };
}
