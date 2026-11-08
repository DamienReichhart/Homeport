import { useMutation, useQueryClient } from '@tanstack/react-query';
import { commands } from '@/shared/api';
import { clusterKeys } from './keys';

/** What applying YAML would change: a dry run; nothing changes. */
export function useApplyPreview(clusterId: string) {
  return useMutation({
    mutationFn: ({ yaml, namespace }: { yaml: string; namespace: string | null }) =>
      commands.clustersApplyPreview(clusterId, yaml, namespace),
  });
}

/** Applies YAML with server-side apply; refreshes everything shown of the Cluster. */
export function useApply(clusterId: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: ({
      yaml,
      namespace,
      force,
    }: {
      yaml: string;
      namespace: string | null;
      force: boolean;
    }) => commands.clustersApply(clusterId, yaml, namespace, force),
    onSettled: () => client.invalidateQueries({ queryKey: clusterKeys.scope(clusterId) }),
  });
}
