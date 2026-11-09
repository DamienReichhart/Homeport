import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { nomadKeys } from '@/core/nomad';
import { commands, type NomadPlan, type NomadRunResult } from '@/shared/api';
import { t } from '@/shared/i18n';

/** A job on a Nomad Cluster, as its pages name it. */
export interface JobRef {
  clusterId: string;
  namespace: string;
  jobId: string;
}

/** Stops a job, purging it when asked; refreshes the Cluster's Nomad pages. */
export function useStopJob(job: JobRef, onDone: () => void) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (purge: boolean) =>
      commands.clustersNomadJobStop(job.clusterId, job.namespace, job.jobId, purge),
    onSuccess: async (_, purge) => {
      await client.invalidateQueries({ queryKey: nomadKeys.cluster(job.clusterId) });
      toast.success(t(purge ? 'nomad.stop.purged' : 'nomad.stop.done', { job: job.jobId }));
      onDone();
    },
  });
}

/** Sets a task group's count; the dialog shows a refusal in place. */
export function useScaleGroup(job: JobRef, onDone: () => void) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ group, count }: { group: string; count: number }) =>
      commands.clustersNomadJobScale(job.clusterId, job.namespace, job.jobId, group, count),
    onSuccess: async (_, { group, count }) => {
      await client.invalidateQueries({ queryKey: nomadKeys.cluster(job.clusterId) });
      toast.success(t('nomad.scale.done', { group, count }));
      onDone();
    },
  });
}

/** Parses and plans a job specification; the dialog shows a failure in place. */
export function usePlanJob(clusterId: string) {
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ spec, namespace }: { spec: string; namespace: string | null }) =>
      commands.clustersNomadJobPlan(clusterId, namespace, spec),
  });
}

/** Runs a planned job; the dialog shows a failure, such as a job changed since, in place. */
export function useRunJob(clusterId: string, onDone: (result: NomadRunResult) => void) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ plan, source }: { plan: NomadPlan; source: string }) =>
      commands.clustersNomadJobRun(
        clusterId,
        {
          jobId: plan.jobId,
          namespace: plan.namespace,
          format: plan.format,
          job: plan.job,
          jobModifyIndex: plan.jobModifyIndex,
        },
        source,
      ),
    onSuccess: async (result) => {
      await client.invalidateQueries({ queryKey: nomadKeys.cluster(clusterId) });
      toast.success(t('nomad.run.done', { job: result.jobId }));
      onDone(result);
    },
  });
}
