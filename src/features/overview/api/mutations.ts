import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type PruneSelection } from '@/shared/api';
import { describePruneReport, toPruneRequest } from '../model/prune';

/** Cleans up a Host; the dialog shows a failure in place. */
export function usePruneHost(hostId: string, onDone: () => void) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: (selection: PruneSelection) =>
      commands.systemPrune(hostId, toPruneRequest(selection)),
    onSuccess: async (report) => {
      toast.success(describePruneReport(report));
      await client.invalidateQueries({ queryKey: hostKeys.scope(hostId) });
      onDone();
    },
  });
}
