import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { commands, type InstallRequest } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { helmKeys } from './queries';
import { t } from '@/shared/i18n';

/** Adds a public chart repository to helm's list; the form shows a failure. */
export function useAddRepository(onAdded: () => void) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: ({ name, url }: { name: string; url: string }) => commands.helmReposAdd(name, url),
    onSuccess: async (_, { name }) => {
      toast.success(t('helm.repos.added', { name }));
      await client.invalidateQueries({ queryKey: helmKeys.repositories });
      onAdded();
    },
  });
}

export function useRemoveRepository() {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (name: string) => commands.helmReposRemove(name),
    onSuccess: () => client.invalidateQueries({ queryKey: helmKeys.repositories }),
    onError: (e) => toastError(e, t('helm.repos.removeFailed')),
  });
}

/** Renders what an install would apply, with the Cluster's help; the dialog shows a failure. */
export function usePreviewInstall(clusterId: string) {
  return useMutation({
    mutationFn: (request: InstallRequest) =>
      commands.clustersHelmPreviewInstall(clusterId, request),
  });
}
