import { queryOptions } from '@tanstack/react-query';
import { clusterKeys } from '@/core/cluster';
import { commands, type WorkloadKind } from '@/shared/api';
import { kindResource } from '../model/resource';

/** A workload's keys sit under its kind, so that kind's watch refreshes them. */
export const workloadQueries = {
  list: (clusterId: string, kind: WorkloadKind, namespace: string | null) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, kindResource(kind)), 'list', namespace],
      queryFn: () => commands.clustersWorkloadsList(clusterId, kind, namespace),
      retry: false,
    }),
  detail: (clusterId: string, kind: WorkloadKind, namespace: string, name: string) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, kindResource(kind)), 'detail', namespace, name],
      queryFn: () => commands.clustersWorkloadGet(clusterId, kind, namespace, name),
      retry: false,
    }),
  /** The Jobs a CronJob started, under the Jobs' kind. */
  jobsOf: (clusterId: string, namespace: string, name: string) =>
    queryOptions({
      queryKey: [...clusterKeys.kind(clusterId, 'job'), 'of', namespace, name],
      queryFn: () => commands.clustersCronJobJobs(clusterId, namespace, name),
      retry: false,
    }),
};
