import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { nomadKeys } from '@/core/nomad';
import { commands, type NomadDrainSpec } from '@/shared/api';
import { t } from '@/shared/i18n';

function useRefresh(clusterId: string) {
  const client = useQueryClient();
  return () => client.invalidateQueries({ queryKey: nomadKeys.cluster(clusterId) });
}

/** Starts draining a node; the dialog shows a refusal in place. */
export function useDrainNode(clusterId: string, nodeId: string, onDone: () => void) {
  const refresh = useRefresh(clusterId);
  return useMutation({
    meta: { silent: true },
    mutationFn: (spec: NomadDrainSpec) =>
      commands.clustersNomadNodeDrain(clusterId, nodeId, spec, false),
    onSuccess: async () => {
      await refresh();
      toast.success(t('nomad.nodes.drainStarted'));
      onDone();
    },
  });
}

/** Cancels a node's drain, making it eligible again when asked. */
export function useCancelDrain(clusterId: string, nodeId: string) {
  const refresh = useRefresh(clusterId);
  return useMutation({
    mutationFn: (markEligible: boolean) =>
      commands.clustersNomadNodeDrain(clusterId, nodeId, null, markEligible),
    onSuccess: async () => {
      await refresh();
      toast.success(t('nomad.nodes.drainCancelled'));
    },
  });
}

/** Marks a node eligible or ineligible for new allocations. */
export function useNodeEligibility(clusterId: string, nodeId: string) {
  const refresh = useRefresh(clusterId);
  return useMutation({
    mutationFn: (eligible: boolean) =>
      commands.clustersNomadNodeEligibility(clusterId, nodeId, eligible),
    onSuccess: refresh,
  });
}
