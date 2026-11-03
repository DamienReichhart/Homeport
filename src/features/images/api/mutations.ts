import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type Image } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
import { toastError } from '@/shared/ui';
import { imageName } from '../model/imageName';

function useRefreshImages(hostId: string) {
  const client = useQueryClient();
  return () => client.invalidateQueries({ queryKey: hostKeys.images(hostId) });
}

/** Adds a name to an image; the dialog shows a failure in place. */
export function useTagImage(hostId: string, imageId: string, onTagged: () => void) {
  const refresh = useRefreshImages(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: (target: string) => commands.imagesTag(hostId, imageId, target),
    onSuccess: async () => {
      await refresh();
      onTagged();
    },
  });
}

/** Removes an image, every one of its names included when it has several. */
export function useRemoveImage(hostId: string) {
  const refresh = useRefreshImages(hostId);
  return useMutation({
    mutationFn: (image: Image) => commands.imagesRemove(hostId, image.id, image.tags.length > 1),
    onSuccess: (_, image) => toast.success(`Removed ${imageName(image)}`),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Removes dangling images, or every unused one when `all` is set. */
export function usePruneImages(hostId: string) {
  const refresh = useRefreshImages(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: (all: boolean) => commands.imagesPrune(hostId, all),
    onSuccess: async (result) => {
      toast.success(
        `Removed ${result.removed.length} images, ${formatBytes(result.spaceReclaimed)} reclaimed`,
      );
      await refresh();
    },
  });
}
