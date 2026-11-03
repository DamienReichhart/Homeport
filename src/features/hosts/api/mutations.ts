import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';
import { toHostInput, type HostFormState } from '../model/form';

function useRefreshHosts() {
  const client = useQueryClient();
  return () => client.invalidateQueries({ queryKey: hostKeys.list });
}

/** Saves a Host's connection; the page shows a failure in place. */
export function useUpdateHost(hostId: string) {
  const refresh = useRefreshHosts();
  return useMutation({
    meta: { silent: true },
    mutationFn: (form: HostFormState) => commands.hostsUpdate(hostId, toHostInput(form, true)),
    onSuccess: async () => {
      await refresh();
      toast.success('Host saved');
    },
  });
}

/** Reconnecting and enabling or disabling a Host. */
export function useHostConnection(hostId: string) {
  const refresh = useRefreshHosts();
  const reconnect = useMutation({
    mutationFn: () => commands.hostsReconnect(hostId),
    onSuccess: refresh,
  });
  const enable = useMutation({
    mutationFn: (enabled: boolean) => commands.hostsSetEnabled(hostId, enabled),
    onSuccess: refresh,
  });
  return { reconnect, enable };
}

/** Trusts the key an SSH server presented. */
export function useTrustHostKey(hostId: string) {
  const refresh = useRefreshHosts();
  return useMutation({
    mutationFn: (key: string) => commands.hostsTrustKey(hostId, key),
    onSuccess: refresh,
  });
}

/** Replaces a Host's generated SSH key; the confirmation handles a failure. */
export function useRegenerateKey(hostId: string) {
  const refresh = useRefreshHosts();
  return useMutation({
    meta: { silent: true },
    mutationFn: () => commands.hostsRegenerateKey(hostId),
    onSuccess: refresh,
  });
}

/** Forgets a Host; the confirmation handles a failure. */
export function useDeleteHost(hostId: string) {
  const refresh = useRefreshHosts();
  return useMutation({
    meta: { silent: true },
    mutationFn: () => commands.hostsDelete(hostId),
    onSuccess: refresh,
  });
}
