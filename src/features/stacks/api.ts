import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { commands, keys, type StackAction, type StackState } from '@/shared/api';
import type { Tone } from '@/shared/ui';
import { toastError } from '@/shared/ui';

export function useStacks(hostId: string) {
  return useQuery({ queryKey: keys.stacks(hostId), queryFn: () => commands.stacksList(hostId) });
}

export const STATE_LABEL: Record<StackState, string> = {
  running: 'Running',
  partial: 'Partly running',
  stopped: 'Stopped',
  notDeployed: 'Not deployed',
};

export const STATE_TONE: Record<StackState, Tone> = {
  running: 'success',
  partial: 'warning',
  stopped: 'muted',
  notDeployed: 'info',
};

/** Start, stop, restart and remove for any Stack, through the Engine API. */
export function useStackMutations(hostId: string) {
  const client = useQueryClient();
  const refresh = () =>
    Promise.all([
      client.invalidateQueries({ queryKey: keys.stacks(hostId) }),
      client.invalidateQueries({ queryKey: keys.containers(hostId) }),
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
      toast.success(`Removed the containers of ${name}`);
      await refresh();
    },
    onError: (e) => toastError(e),
  });
  return { act, remove };
}
