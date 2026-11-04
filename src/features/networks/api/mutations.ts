import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type NetworkInput, type NetworkView } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

function useRefreshNetworks(hostId: string) {
  const client = useQueryClient();
  return () => client.invalidateQueries({ queryKey: hostKeys.networks(hostId) });
}

/** Creates a network; the dialog shows a failure in place. */
export function useCreateNetwork(hostId: string, onCreated: () => void) {
  const refresh = useRefreshNetworks(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: (input: NetworkInput) => commands.networksCreate(hostId, input),
    onSuccess: async () => {
      await refresh();
      onCreated();
    },
  });
}

/** Connects containers to a network, or disconnects them, reporting the outcome as a toast. */
export function useNetworkMembership(hostId: string, networkId: string, onDone: () => void) {
  const refresh = useRefreshNetworks(hostId);
  const connect = useMutation({
    mutationFn: (container: string) => commands.networksConnect(hostId, networkId, container),
    onSuccess: () => {
      void refresh();
      onDone();
      toast.success(t('networks.connected'));
    },
    onError: (e) => toastError(e),
  });
  const disconnect = useMutation({
    mutationFn: (container: string) =>
      commands.networksDisconnect(hostId, networkId, container, false),
    onSuccess: () => {
      void refresh();
      onDone();
      toast.success(t('networks.disconnected'));
    },
    onError: (e) => toastError(e),
  });
  return { connect, disconnect };
}

export function useRemoveNetwork(hostId: string) {
  const refresh = useRefreshNetworks(hostId);
  return useMutation({
    mutationFn: (network: NetworkView) => commands.networksRemove(hostId, network.id),
    onSuccess: (_, network) => toast.success(t('networks.removed', { name: network.name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Removes every custom network no container is attached to. */
export function usePruneNetworks(hostId: string) {
  const refresh = useRefreshNetworks(hostId);
  return useMutation({
    mutationFn: () => commands.networksPrune(hostId),
    onSuccess: async (result) => {
      toast.success(t('networks.pruned', { count: result.removed.length }));
      await refresh();
    },
  });
}
