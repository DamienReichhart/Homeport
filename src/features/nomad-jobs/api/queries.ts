import { queryOptions } from '@tanstack/react-query';
import { nomadKeys } from '@/core/nomad';
import { commands } from '@/shared/api';

export const nomadJobQueries = {
  /** The jobs of `namespace`: the token's default with `null`, every namespace with `*`. */
  list: (clusterId: string, namespace: string | null) =>
    queryOptions({
      queryKey: nomadKeys.jobs(clusterId, namespace),
      queryFn: () => commands.clustersNomadJobs(clusterId, namespace),
    }),
  detail: (clusterId: string, namespace: string, jobId: string) =>
    queryOptions({
      queryKey: nomadKeys.job(clusterId, namespace, jobId),
      queryFn: () => commands.clustersNomadJob(clusterId, namespace, jobId),
    }),
};
