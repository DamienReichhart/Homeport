import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

function useRefresh(hostId: string) {
  const client = useQueryClient();
  return () => client.invalidateQueries({ queryKey: hostKeys.incus(hostId, 'profiles') });
}

/** Saves a profile's YAML; the page shows a refusal in place. */
export function useSaveProfile(hostId: string, project: string, name: string) {
  const refresh = useRefresh(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ yaml, etag }: { yaml: string; etag: string | null }) =>
      commands.incusProfileUpdate(hostId, project, name, yaml, etag),
    onSuccess: async () => {
      toast.success(t('incus.profiles.saved', { name }));
      await refresh();
    },
  });
}

/** Makes an empty profile; the dialog shows a refusal in place. */
export function useCreateProfile(hostId: string, project: string) {
  const refresh = useRefresh(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ name, description }: { name: string; description: string }) =>
      commands.incusProfileCreate(hostId, project, name, description),
    onSuccess: (_, { name }) => toast.success(t('incus.profiles.created', { name })),
    onSettled: () => refresh(),
  });
}

/** Deletes an unused profile. */
export function useDeleteProfile(hostId: string, project: string) {
  const refresh = useRefresh(hostId);
  return useMutation({
    mutationFn: (name: string) => commands.incusProfileDelete(hostId, project, name),
    onSuccess: (_, name) => toast.success(t('incus.profiles.deleted', { name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}
