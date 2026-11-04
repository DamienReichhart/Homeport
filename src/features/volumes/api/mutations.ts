import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type VolumeView } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
import { toastError } from '@/shared/ui';

function useRefreshVolumes(hostId: string) {
  const client = useQueryClient();
  return () => client.invalidateQueries({ queryKey: hostKeys.volumes(hostId) });
}

/** Creates a volume; the dialog shows a failure in place. */
export function useCreateVolume(hostId: string, onCreated: () => void) {
  const refresh = useRefreshVolumes(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ name, driver }: { name: string; driver: string }) =>
      commands.volumesCreate(hostId, { name, driver: driver || null, labels: {} }),
    onSuccess: async () => {
      await refresh();
      onCreated();
    },
  });
}

export function useRemoveVolume(hostId: string) {
  const refresh = useRefreshVolumes(hostId);
  return useMutation({
    mutationFn: (volume: VolumeView) => commands.volumesRemove(hostId, volume.name, false),
    onSuccess: (_, volume) => toast.success(`Deleted ${volume.name}`),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Deletes unused volumes: anonymous ones only, or `all` of them. */
export function usePruneVolumes(hostId: string) {
  const refresh = useRefreshVolumes(hostId);
  return useMutation({
    mutationFn: (all: boolean) => commands.volumesPrune(hostId, all),
    onSuccess: async (r) => {
      toast.success(
        `Deleted ${r.removed.length} volumes, ${formatBytes(r.spaceReclaimed)} reclaimed`,
      );
      await refresh();
    },
  });
}
