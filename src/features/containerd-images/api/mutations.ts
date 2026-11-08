import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Removes an image no container of the namespace was made from. */
export function useRemoveImage(hostId: string, namespace: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (name: string) => commands.containerdImageRemove(hostId, namespace, name),
    onSuccess: (_, name) => toast.success(t('containerd.images.removed', { name })),
    onError: (e) => toastError(e),
    onSettled: () =>
      Promise.all([
        client.invalidateQueries({ queryKey: hostKeys.containerdImages(hostId) }),
        client.invalidateQueries({ queryKey: hostKeys.containerdNamespaces(hostId) }),
      ]),
  });
}
