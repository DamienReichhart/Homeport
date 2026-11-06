import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type ContainerAction } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t, type MessageKey } from '@/shared/i18n';

const DONE: Partial<Record<ContainerAction, MessageKey>> = {
  start: 'containers.done.start',
  stop: 'containers.done.stop',
  restart: 'containers.done.restart',
};

const count = (byHost: Map<string, string[]>) =>
  [...byHost.values()].reduce((n, ids) => n + ids.length, 0);

/**
 * Start, stop, restart and remove for a selection spanning Hosts: each Host
 * is asked for its own containers, and every Host's list refreshes.
 */
export function useBoardActions() {
  const client = useQueryClient();
  const refresh = (byHost: Map<string, string[]>) =>
    Promise.all(
      [...byHost.keys()].map((hostId) =>
        client.invalidateQueries({ queryKey: hostKeys.containers(hostId) }),
      ),
    );
  const perHost = (
    byHost: Map<string, string[]>,
    each: (hostId: string, id: string) => Promise<unknown>,
  ) =>
    Promise.all([...byHost].map(([hostId, ids]) => Promise.all(ids.map((id) => each(hostId, id)))));

  const act = useMutation({
    mutationFn: async ({
      byHost,
      action,
    }: {
      byHost: Map<string, string[]>;
      action: 'start' | 'stop' | 'restart';
    }) => {
      try {
        await perHost(byHost, (hostId, id) => commands.containersAct(hostId, id, action));
      } finally {
        void refresh(byHost);
      }
      return { byHost, action };
    },
    onSuccess: ({ byHost, action }) => {
      const done = DONE[action];
      if (done) toast.success(t(done, { count: count(byHost) }));
    },
    onError: (e) => toastError(e),
  });
  const remove = useMutation({
    mutationFn: async (byHost: Map<string, string[]>) => {
      try {
        await perHost(byHost, (hostId, id) => commands.containersRemove(hostId, id, true, true));
      } finally {
        void refresh(byHost);
      }
      return byHost;
    },
    onSuccess: (byHost) => toast.success(t('containers.done.remove', { count: count(byHost) })),
    onError: (e) => toastError(e),
  });
  return { act, remove };
}
