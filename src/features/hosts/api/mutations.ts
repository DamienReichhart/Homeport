import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type ImportChoice, type ImportOutcome } from '@/shared/api';
import { t } from '@/shared/i18n';
import { toHostInput, type HostFormState } from '../model/form';
import type { ImportSource } from '../model/importRows';

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
      toast.success(t('hosts.saved'));
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

/** Trusts the key an SSH server presented: the Host's, or the bastion's at `bastion`. */
export function useTrustHostKey(hostId: string) {
  const refresh = useRefreshHosts();
  return useMutation({
    mutationFn: ({ key, bastion }: { key: string; bastion: string | null }) =>
      commands.hostsTrustKey(hostId, key, bastion),
    onSuccess: refresh,
  });
}

/** Replaces a Host's generated SSH key. */
export function useRegenerateKey(hostId: string) {
  const refresh = useRefreshHosts();
  return useMutation({
    mutationFn: () => commands.hostsRegenerateKey(hostId),
    onSuccess: refresh,
  });
}

/** Forgets a Host. */
export function useDeleteHost(hostId: string) {
  const refresh = useRefreshHosts();
  return useMutation({
    mutationFn: () => commands.hostsDelete(hostId),
    onSuccess: refresh,
  });
}

/** Creates the picked drafts as Hosts; the dialog shows what failed. */
export function useImportHosts(source: ImportSource, onDone: (outcome: ImportOutcome) => void) {
  const refresh = useRefreshHosts();
  return useMutation({
    meta: { silent: true },
    mutationFn: (choices: ImportChoice[]) =>
      source === 'sshConfig'
        ? commands.hostsSshConfigImport(choices)
        : commands.hostsDockerContextsImport(choices),
    onSuccess: async (outcome) => {
      await refresh();
      if (outcome.created.length > 0) {
        toast.success(t('hosts.import.done', { count: outcome.created.length }));
      }
      onDone(outcome);
    },
  });
}
