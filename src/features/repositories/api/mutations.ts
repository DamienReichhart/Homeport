import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { commands, type Repository, type RepositoryDraft } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { repositoryLabel } from '../model/repository';
import { repositoryKeys } from './queries';
import { t } from '@/shared/i18n';

function useRefreshRepositories() {
  const client = useQueryClient();
  return () => client.invalidateQueries({ queryKey: repositoryKeys.all });
}

/** Adds a repository; the dialog shows a failure in place. */
export function useCreateRepository(onCreated: (repository: Repository) => void) {
  const refresh = useRefreshRepositories();
  return useMutation({
    meta: { silent: true },
    mutationFn: (draft: RepositoryDraft) => commands.repositoriesCreate(draft),
    onSuccess: async (created) => {
      toast.success(t('repositories.added', { repository: repositoryLabel(created) }));
      await refresh();
      onCreated(created);
    },
  });
}

/** Lists a repository's branches and tags from its server; shown in place. */
export function useTestRepository() {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: (id: string) => commands.repositoriesTest(id),
    onSuccess: (refs, id) => {
      client.setQueryData(repositoryKeys.refs(id), refs);
      // A server key trusted on first use shows up in the list.
      void client.invalidateQueries({ queryKey: repositoryKeys.list });
    },
  });
}

export function useRotateKey() {
  const refresh = useRefreshRepositories();
  return useMutation({
    mutationFn: (id: string) => commands.repositoriesRotateKey(id),
    onSuccess: () => toast.success(t('repositories.keyRotated')),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Replaces a token; the dialog shows a failure in place. */
export function useSetToken(id: string, onSaved: () => void) {
  const refresh = useRefreshRepositories();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ username, token }: { username: string; token: string }) =>
      commands.repositoriesSetToken(id, username, token),
    onSuccess: async () => {
      toast.success(t('repositories.tokenReplaced'));
      await refresh();
      onSaved();
    },
  });
}

export function useResetHostKey() {
  const refresh = useRefreshRepositories();
  return useMutation({
    mutationFn: (id: string) => commands.repositoriesResetHostKey(id),
    onSuccess: () => toast.success(t('repositories.hostKeyReset')),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

export function useRemoveRepository() {
  const refresh = useRefreshRepositories();
  return useMutation({
    mutationFn: (repository: Repository) => commands.repositoriesRemove(repository.id),
    onSuccess: (_, repository) =>
      toast.success(t('repositories.removed', { repository: repositoryLabel(repository) })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}
