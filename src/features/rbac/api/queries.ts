import { queryOptions, useMutation } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands, type AccessCheck, type Subject } from '@/shared/api';

const scope = (clusterId: string) => [...clusterKeys.scope(clusterId), 'rbac'] as const;

/** A Cluster's RBAC, read-only, under the Cluster's scope. */
export const rbacQueries = {
  index: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: [...scope(clusterId), 'index', namespace],
      queryFn: () => commands.clustersRbacIndex(clusterId, namespace),
      retry: false,
    }),
  subject: (clusterId: string, subject: Subject, namespace: string | null) =>
    queryOptions({
      queryKey: [...scope(clusterId), 'subject', subject, namespace],
      queryFn: () => commands.clustersRbacSubject(clusterId, subject, namespace),
      retry: false,
    }),
  own: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: [...scope(clusterId), 'own', namespace],
      queryFn: () => commands.clustersRbacOwn(clusterId, namespace),
      retry: false,
    }),
};

/** "Can I …?", asked of the API server. */
export function useCanI(clusterId: string) {
  return useMutation({
    mutationFn: (check: AccessCheck) => commands.clustersRbacCanI(clusterId, check),
  });
}
