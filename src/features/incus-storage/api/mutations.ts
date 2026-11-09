import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type VolumeRequest } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Makes and deletes a pool's custom volumes. Making shows its failure in the dialog. */
export function useVolumes(hostId: string, project: string, pool: string) {
  const client = useQueryClient();
  const refresh = () => client.invalidateQueries({ queryKey: hostKeys.incus(hostId, 'storage') });
  const create = useMutation({
    meta: { silent: true },
    mutationFn: (request: VolumeRequest) =>
      commands.incusVolumeCreate(hostId, project, pool, request),
    onSuccess: (_, request) => toast.success(t('incus.storage.created', { name: request.name })),
    onSettled: () => refresh(),
  });
  const remove = useMutation({
    mutationFn: (name: string) => commands.incusVolumeDelete(hostId, project, pool, name),
    onSuccess: (_, name) => toast.success(t('incus.storage.deleted', { name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
  return { create, remove };
}
