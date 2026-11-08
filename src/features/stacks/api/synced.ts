import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { commands, type StackOwner, type SyncedDraft, type SyncedStack } from '@/shared/api';
import { stacksKey } from './owner';
import { t } from '@/shared/i18n';

export const syncedKeys = {
  /** A Synced Stack's record; its environment is cached under it. */
  stack: (syncedId: string) => ['synced-stack', syncedId] as const,
};

export const syncedQueries = {
  stack: (syncedId: string) =>
    queryOptions({
      queryKey: syncedKeys.stack(syncedId),
      queryFn: () => commands.stacksSyncedGet(syncedId),
    }),
  /** Its files at the fetched commit, read-only. */
  files: (syncedId: string, fetched: string | null) =>
    queryOptions({
      queryKey: [...syncedKeys.stack(syncedId), 'files', fetched] as const,
      queryFn: () => commands.stacksSyncedFiles(syncedId),
      retry: false,
    }),
  env: (syncedId: string) =>
    queryOptions({
      queryKey: [...syncedKeys.stack(syncedId), 'env'],
      queryFn: () => commands.stacksSyncedGetEnv(syncedId),
    }),
};

interface NewSynced {
  draft: SyncedDraft;
  env: string;
  deploy: boolean;
}

/**
 * Creates a Synced Stack, fetching its repository first; `onSaved` runs once
 * the Stack list has it. A failure is shown in place.
 */
export function useCreateSynced(
  owner: StackOwner,
  onSaved: (stack: SyncedStack, deploy: boolean) => void,
) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ draft, env }: NewSynced) => commands.stacksSyncedCreate(owner, draft, env),
    onSuccess: async (stack, { deploy }) => {
      await client.invalidateQueries({ queryKey: stacksKey(owner) });
      onSaved(stack, deploy);
    },
  });
}

/** Fetches a Synced Stack's ref and says what deploying it would change; shown in place. */
export function useSyncReview(syncedId: string) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: () => commands.stacksSyncedSync(syncedId),
    onSuccess: (review) => client.setQueryData(syncedKeys.stack(syncedId), review.stack),
  });
}

/** Saves a Synced Stack's environment; shown in place on failure. */
export function useSaveSyncedEnv(syncedId: string) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: (env: string) => commands.stacksSyncedSetEnv(syncedId, env),
    onSuccess: async () => {
      toast.success(t('stacks.synced.envSaved'));
      await client.invalidateQueries({ queryKey: syncedKeys.stack(syncedId) });
    },
  });
}

/** Turns a Synced Stack's polling and auto-deploy on or off. */
export function useSetSync(owner: StackOwner, syncedId: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: ({ poll, autoDeploy }: { poll: boolean; autoDeploy: boolean }) =>
      commands.stacksSyncedSetSync(syncedId, poll, autoDeploy),
    onSuccess: async (stack) => {
      client.setQueryData(syncedKeys.stack(syncedId), stack);
      await client.invalidateQueries({ queryKey: stacksKey(owner) });
    },
  });
}

/** Forgets a Synced Stack: its containers are left alone and it becomes External. */
export function useForgetSynced(owner: StackOwner, syncedId: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: () => commands.stacksSyncedForget(syncedId),
    onSuccess: () => client.invalidateQueries({ queryKey: stacksKey(owner) }),
  });
}
