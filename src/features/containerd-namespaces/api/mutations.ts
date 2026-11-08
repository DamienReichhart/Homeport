import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Creates an empty namespace; the dialog shows a failure in place. */
export function useCreateNamespace(hostId: string, onCreated: (name: string) => void) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: (name: string) => commands.containerdNamespaceCreate(hostId, name),
    onSuccess: async (_, name) => {
      await client.invalidateQueries({ queryKey: hostKeys.containerdNamespaces(hostId) });
      toast.success(t('containerd.namespaces.created', { name }));
      onCreated(name);
    },
  });
}

/** Removes an empty namespace; containerd refuses one that holds anything. */
export function useRemoveNamespace(hostId: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (name: string) => commands.containerdNamespaceRemove(hostId, name),
    onSuccess: (_, name) => toast.success(t('containerd.namespaces.removed', { name })),
    onError: (e) => toastError(e),
    onSettled: () => client.invalidateQueries({ queryKey: hostKeys.containerdNamespaces(hostId) }),
  });
}
