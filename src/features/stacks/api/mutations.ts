import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type ManagedStack, type StackAction } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';
import { stackKeys } from './queries';

/** Start, stop, restart and remove for any Stack, through the Engine API. */
export function useStackMutations(hostId: string) {
  const client = useQueryClient();
  const refresh = () =>
    Promise.all([
      client.invalidateQueries({ queryKey: hostKeys.stacks(hostId) }),
      client.invalidateQueries({ queryKey: hostKeys.containers(hostId) }),
    ]);
  const act = useMutation({
    mutationFn: ({ name, action }: { name: string; action: StackAction }) =>
      commands.stacksAct(hostId, name, action),
    onSuccess: refresh,
    onError: (e) => toastError(e),
  });
  const remove = useMutation({
    mutationFn: (name: string) => commands.stacksRemove(hostId, name),
    onSuccess: async (_, name) => {
      toast.success(t('stacks.removed', { name }));
      await refresh();
    },
    onError: (e) => toastError(e),
  });
  return { act, remove };
}

interface NewStack {
  name: string;
  compose: string;
  env: string;
  deploy: boolean;
}

/**
 * Stores a new Managed Stack. `onSaved` runs once the Stack list has it,
 * with whether the user asked to deploy it too. The page shows a failure in
 * place.
 */
export function useCreateStack(
  hostId: string,
  onSaved: (stack: ManagedStack, deploy: boolean) => void,
) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ name, compose, env }: NewStack) =>
      commands.stacksCreate(hostId, name, compose, env),
    onSuccess: async (stack, { deploy }) => {
      await client.invalidateQueries({ queryKey: hostKeys.stacks(hostId) });
      onSaved(stack, deploy);
    },
  });
}

interface Definition {
  compose: string;
  env: string;
  deploy: boolean;
}

/** Stores a Managed Stack's edited definition; `onSaved` gets whether to redeploy. */
export function useSaveDefinition(
  managedId: string,
  onSaved: (stack: ManagedStack, deploy: boolean) => void,
) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ compose, env }: Definition) => commands.stacksUpdate(managedId, compose, env),
    onSuccess: async (saved, { deploy }) => {
      await client.invalidateQueries({ queryKey: stackKeys.managed(managedId) });
      toast.success(t('stacks.definitionSaved'));
      onSaved(saved, deploy);
    },
  });
}

/** Deletes a Managed Stack's stored definition, leaving its containers: it becomes External. */
export function useForgetDefinition(hostId: string, managedId: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: () => commands.stacksForget(managedId),
    onSuccess: () => client.invalidateQueries({ queryKey: hostKeys.stacks(hostId) }),
  });
}
