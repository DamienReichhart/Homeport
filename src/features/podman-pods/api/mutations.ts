import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type PodmanPod, type PodmanPodAction, type PodmanPodSpec } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t, type MessageKey } from '@/shared/i18n';

/** Pods and containers change together: a pod's status is its containers'. */
function useRefreshPods(hostId: string) {
  const client = useQueryClient();
  return () =>
    Promise.all([
      client.invalidateQueries({ queryKey: hostKeys.pods(hostId) }),
      client.invalidateQueries({ queryKey: hostKeys.containers(hostId) }),
    ]);
}

const DONE: Record<PodmanPodAction, MessageKey> = {
  start: 'podmanpods.done.start',
  stop: 'podmanpods.done.stop',
  restart: 'podmanpods.done.restart',
  pause: 'podmanpods.done.pause',
  unpause: 'podmanpods.done.unpause',
};

/** Starts, stops, restarts, pauses or resumes a pod, reporting the outcome as a toast. */
export function usePodAction(hostId: string) {
  const refresh = useRefreshPods(hostId);
  return useMutation({
    mutationFn: ({ pod, action }: { pod: PodmanPod; action: PodmanPodAction }) =>
      commands.podsAct(hostId, pod.id, action),
    onSuccess: (_, { pod, action }) => toast.success(t(DONE[action], { name: pod.name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Removes a pod and its containers; `force` stops them first. */
export function useRemovePod(hostId: string) {
  const refresh = useRefreshPods(hostId);
  return useMutation({
    mutationFn: ({ pod, force }: { pod: PodmanPod; force: boolean }) =>
      commands.podsRemove(hostId, pod.id, force),
    onSuccess: (removed, { pod }) =>
      toast.success(t('podmanpods.removed', { name: pod.name, count: removed })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Creates a pod; the dialog shows a failure in place. */
export function useCreatePod(hostId: string, onCreated: (id: string) => void) {
  const refresh = useRefreshPods(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: (spec: PodmanPodSpec) => commands.podsCreate(hostId, spec),
    onSuccess: async (id, spec) => {
      await refresh();
      toast.success(t('podmanpods.created', { name: spec.name }));
      onCreated(id);
    },
  });
}
