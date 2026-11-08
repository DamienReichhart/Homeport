import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type IncusImage } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

function useRefresh(hostId: string) {
  const client = useQueryClient();
  return () =>
    Promise.all([
      client.invalidateQueries({ queryKey: hostKeys.incus(hostId, 'images') }),
      client.invalidateQueries({ queryKey: hostKeys.incusProjects(hostId) }),
    ]);
}

/** A short name for an image in messages: its first alias, or its fingerprint's start. */
export const imageName = (image: IncusImage) => image.aliases[0] ?? image.fingerprint.slice(0, 12);

/** Deletes an image; Instances made from it keep their own copy. */
export function useDeleteImage(hostId: string, project: string) {
  const refresh = useRefresh(hostId);
  return useMutation({
    mutationFn: (image: IncusImage) =>
      commands.incusImageDelete(hostId, project, image.fingerprint),
    onSuccess: (_, image) => toast.success(t('incus.images.deleted', { name: imageName(image) })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Fetches a newer build of an image from where it came from. */
export function useRefreshImage(hostId: string, project: string) {
  const refresh = useRefresh(hostId);
  return useMutation({
    mutationFn: (image: IncusImage) =>
      commands.incusImageRefresh(hostId, project, image.fingerprint),
    onSuccess: (_, image) => toast.success(t('incus.images.refreshed', { name: imageName(image) })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Adds or removes an image's alias. Adding shows its failure in the dialog. */
export function useAliases(hostId: string, project: string) {
  const refresh = useRefresh(hostId);
  const add = useMutation({
    meta: { silent: true },
    mutationFn: ({ image, name }: { image: IncusImage; name: string }) =>
      commands.incusImageAliasAdd(hostId, project, image.fingerprint, name),
    onSettled: () => refresh(),
  });
  const remove = useMutation({
    mutationFn: (name: string) => commands.incusImageAliasRemove(hostId, project, name),
    onSuccess: (_, name) => toast.success(t('incus.images.aliasRemoved', { name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
  return { add, remove };
}
