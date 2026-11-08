import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type BridgeRequest } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Makes and deletes managed bridges. Making shows its failure in the dialog. */
export function useNetworks(hostId: string, project: string) {
  const client = useQueryClient();
  const refresh = () => client.invalidateQueries({ queryKey: hostKeys.incus(hostId, 'networks') });
  const create = useMutation({
    meta: { silent: true },
    mutationFn: (request: BridgeRequest) => commands.incusBridgeCreate(hostId, project, request),
    onSuccess: (_, request) => toast.success(t('incus.networks.created', { name: request.name })),
    onSettled: () => refresh(),
  });
  const remove = useMutation({
    mutationFn: (name: string) => commands.incusNetworkDelete(hostId, project, name),
    onSuccess: (_, name) => toast.success(t('incus.networks.deleted', { name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
  return { create, remove };
}
