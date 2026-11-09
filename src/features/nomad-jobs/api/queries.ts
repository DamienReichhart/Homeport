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
  allocation: (clusterId: string, allocId: string) =>
    queryOptions({
      queryKey: nomadKeys.allocation(clusterId, allocId),
      queryFn: () => commands.clustersNomadAllocation(clusterId, allocId),
    }),
};

/** A job's specification to edit: as submitted when Nomad kept it, else its JSON. */
export const jobSource = (job: { clusterId: string; namespace: string; jobId: string }) =>
  commands.clustersNomadJobSource(job.clusterId, job.namespace, job.jobId);
