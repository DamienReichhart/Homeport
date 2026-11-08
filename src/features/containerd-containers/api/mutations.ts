import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type ContainerdContainer, type TaskAction } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t, type MessageKey } from '@/shared/i18n';

const DONE: Record<TaskAction, MessageKey> = {
  start: 'containerd.containers.done.start',
  stop: 'containerd.containers.done.stop',
  pause: 'containerd.containers.done.pause',
  resume: 'containerd.containers.done.resume',
};

function useRefresh(hostId: string) {
  const client = useQueryClient();
  return () =>
    Promise.all([
      client.invalidateQueries({ queryKey: hostKeys.containerdContainers(hostId) }),
      client.invalidateQueries({ queryKey: hostKeys.containerdSnapshots(hostId) }),
      client.invalidateQueries({ queryKey: hostKeys.containerdNamespaces(hostId) }),
    ]);
}

/** Starts, stops, pauses or resumes a container's task, with a toast for the outcome. */
export function useTaskAction(hostId: string, namespace: string) {
  const refresh = useRefresh(hostId);
  return useMutation({
    mutationFn: ({ container, action }: { container: ContainerdContainer; action: TaskAction }) =>
      commands.containerdContainerAct(hostId, namespace, container.id, action),
    onSuccess: (_, { container, action }) =>
      toast.success(t(DONE[action], { name: container.name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Sends a signal to a container's task. */
export function useKill(hostId: string, namespace: string) {
  const refresh = useRefresh(hostId);
  return useMutation({
    mutationFn: ({ container, signal }: { container: ContainerdContainer; signal: string }) =>
      commands.containerdContainerKill(hostId, namespace, container.id, signal),
    onSuccess: (_, { container, signal }) =>
      toast.success(t('containerd.containers.done.kill', { name: container.name, signal })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Removes a container, its task and, unless kept, its snapshot. */
export function useRemoveContainer(hostId: string, namespace: string) {
  const refresh = useRefresh(hostId);
  return useMutation({
    mutationFn: ({
      container,
      force,
      keepSnapshot,
    }: {
      container: ContainerdContainer;
      force: boolean;
      keepSnapshot: boolean;
    }) => commands.containerdContainerRemove(hostId, namespace, container.id, force, keepSnapshot),
    onSuccess: (_, { container }) =>
      toast.success(t('containerd.containers.removed', { name: container.name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}
