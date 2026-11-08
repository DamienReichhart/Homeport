import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Removes a snapshot nothing uses and nothing is built on. */
export function useRemoveSnapshot(hostId: string, namespace: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: ({ snapshotter, key }: { snapshotter: string; key: string }) =>
      commands.containerdSnapshotRemove(hostId, namespace, snapshotter, key),
    onSuccess: (_, { key }) => toast.success(t('containerd.snapshots.removed', { key })),
    onError: (e) => toastError(e),
    onSettled: () => client.invalidateQueries({ queryKey: hostKeys.containerdSnapshots(hostId) }),
  });
}
