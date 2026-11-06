import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type FileEntry, type VolumeView } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

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
    onSuccess: (_, volume) => toast.success(t('volumes.deleted', { name: volume.name })),
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
        t('volumes.pruned', { count: r.removed.length, space: formatBytes(r.spaceReclaimed) }),
      );
      await refresh();
    },
  });
}

/** Saves a file, or a folder as a `.tar`, from a browsed volume to `target` on this machine. */
export function useDownloadFromVolume(hostId: string, session: string) {
  return useMutation({
    mutationFn: ({ entry, target }: { entry: FileEntry; target: string }) =>
      commands.volumesBrowseDownload(hostId, session, entry.path, target),
    onSuccess: (archived, { entry }) =>
      toast.success(
        archived
          ? t('volumes.browse.savedArchive', { name: entry.name })
          : t('volumes.browse.saved', { name: entry.name }),
      ),
    onError: (e) => toastError(e),
  });
}
