import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
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

/** Asks every registry again, bypassing the cache, and shows what it found. */
export function useCheckForNewerImages(hostId: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: () => commands.imagesFreshness(hostId, true),
    onSuccess: (results) => {
      client.setQueryData(hostKeys.freshness(hostId), results);
      const newer = new Set(results.filter((r) => r.freshness === 'newer').map((r) => r.reference))
        .size;
      toast.success(
        newer === 0
          ? 'Every image is up to date'
          : `${newer} ${newer === 1 ? 'image has' : 'images have'} a newer version`,
      );
    },
  });
}
