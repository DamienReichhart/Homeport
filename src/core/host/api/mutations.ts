import { useMutation, useQueryClient } from '@tanstack/react-query';
import { commands, type HostInput, type HostView } from '@/shared/api';
import { hostKeys } from './keys';

/** Adds a Host; `onAdded` runs once the Host list has it. */
export function useCreateHost(onAdded: (view: HostView) => void, silent = false) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent },
    mutationFn: (input: HostInput) => commands.hostsCreate(input),
    onSuccess: async (view) => {
      await client.invalidateQueries({ queryKey: hostKeys.list });
      onAdded(view);
    },
  });
}

export function useReconnectHost() {
  return useMutation({ mutationFn: (hostId: string) => commands.hostsReconnect(hostId) });
}
