import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { clusterKeys } from '@/core/cluster';
import { commands, type WorkloadKind } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { kindResource } from '../model/resource';
import { t } from '@/shared/i18n';

/** A workload to act on. */
export interface WorkloadRef {
  kind: WorkloadKind;
  namespace: string;
  name: string;
}

/** Refreshes what is shown of the workload's kind, before its watch would. */
function useRefresh(clusterId: string) {
  const client = useQueryClient();
  return (kind: WorkloadKind) =>
    client.invalidateQueries({ queryKey: clusterKeys.kind(clusterId, kindResource(kind)) });
}

/** Sets how many Pods a Deployment or a StatefulSet runs. */
export function useScale(clusterId: string, onDone?: () => void) {
  const refresh = useRefresh(clusterId);
  return useMutation({
    mutationFn: ({ ref, replicas }: { ref: WorkloadRef; replicas: number }) =>
      commands.clustersScale(clusterId, ref.kind, ref.namespace, ref.name, replicas),
    onSuccess: async (_, { ref, replicas }) => {
      toast.success(t('workloads.scale.done', { name: ref.name, replicas }));
      await refresh(ref.kind);
      onDone?.();
    },
    onError: (e, { ref }) => toastError(e, t('workloads.scale.failed', { name: ref.name })),
  });
}

/** Rolls a workload's Pods out again. */
export function useRestart(clusterId: string) {
  const refresh = useRefresh(clusterId);
  return useMutation({
    mutationFn: (ref: WorkloadRef) =>
      commands.clustersRestart(clusterId, ref.kind, ref.namespace, ref.name),
    onSuccess: async (_, ref) => {
      toast.success(t('workloads.restart.done', { name: ref.name }));
      await refresh(ref.kind);
    },
    onError: (e, ref) => toastError(e, t('workloads.restart.failed', { name: ref.name })),
  });
}

/** Suspends a CronJob, or resumes it. */
export function useSuspend(clusterId: string) {
  const refresh = useRefresh(clusterId);
  return useMutation({
    mutationFn: ({ ref, suspended }: { ref: WorkloadRef; suspended: boolean }) =>
      commands.clustersCronJobSuspend(clusterId, ref.namespace, ref.name, suspended),
    onSuccess: async (_, { ref, suspended }) => {
      toast.success(
        suspended
          ? t('workloads.suspend.done', { name: ref.name })
          : t('workloads.resume.done', { name: ref.name }),
      );
      await refresh(ref.kind);
    },
    onError: (e, { ref, suspended }) =>
      toastError(
        e,
        suspended
          ? t('workloads.suspend.failed', { name: ref.name })
          : t('workloads.resume.failed', { name: ref.name }),
      ),
  });
}

/** Starts a Job from a CronJob now. */
export function useTrigger(clusterId: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (ref: WorkloadRef) =>
      commands.clustersCronJobTrigger(clusterId, ref.namespace, ref.name),
    onSuccess: async (job) => {
      toast.success(t('workloads.trigger.done', { job }));
      await client.invalidateQueries({ queryKey: clusterKeys.kind(clusterId, 'job') });
    },
    onError: (e, ref) => toastError(e, t('workloads.trigger.failed', { name: ref.name })),
  });
}
