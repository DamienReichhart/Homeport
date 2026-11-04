import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { commands, type Registry, type RegistryDraft } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { registryKeys } from './queries';

function useRefreshRegistries() {
  const client = useQueryClient();
  return () => client.invalidateQueries({ queryKey: registryKeys.all });
}

/** Adds a login or changes one; the dialog shows a failure in place. */
export function useSaveRegistry(id: string | null, onSaved: () => void) {
  const refresh = useRefreshRegistries();
  return useMutation({
    meta: { silent: true },
    mutationFn: (draft: RegistryDraft) => commands.registriesSave(id, draft),
    onSuccess: async (saved) => {
      toast.success(`Saved the login for ${saved.address}`);
      await refresh();
      onSaved();
    },
  });
}

/** Checks a login from this machine; the dialog shows the result in place. */
export function useCheckRegistry(id: string | null) {
  return useMutation({
    meta: { silent: true },
    mutationFn: (draft: RegistryDraft) => commands.registriesCheck(id, draft),
  });
}

export function useRemoveRegistry() {
  const refresh = useRefreshRegistries();
  return useMutation({
    mutationFn: (registry: Registry) => commands.registriesRemove(registry.id),
    onSuccess: (_, registry) => toast.success(`Removed the login for ${registry.address}`),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}
