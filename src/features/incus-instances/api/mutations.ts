import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type InstanceAction, type SnapshotRequest } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t, type MessageKey } from '@/shared/i18n';

const DONE: Record<InstanceAction, MessageKey> = {
  start: 'incus.instances.done.start',
  stop: 'incus.instances.done.stop',
  restart: 'incus.instances.done.restart',
  freeze: 'incus.instances.done.freeze',
  unfreeze: 'incus.instances.done.unfreeze',
};

function useRefresh(hostId: string) {
  const client = useQueryClient();
  return () =>
    Promise.all([
      client.invalidateQueries({ queryKey: hostKeys.incus(hostId, 'instances') }),
      client.invalidateQueries({ queryKey: hostKeys.incusProjects(hostId) }),
    ]);
}

/** Starts, stops, restarts, freezes or unfreezes an Instance, with a toast for the outcome. */
export function useInstanceAction(hostId: string, project: string) {
  const refresh = useRefresh(hostId);
  return useMutation({
    mutationFn: ({
      name,
      action,
      force,
    }: {
      name: string;
      action: InstanceAction;
      force: boolean;
    }) => commands.incusInstanceAct(hostId, project, name, action, force),
    onSuccess: (_, { name, action }) => toast.success(t(DONE[action], { name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Deletes an Instance and its snapshots, stopping it first if asked. */
export function useDeleteInstance(hostId: string, project: string) {
  const refresh = useRefresh(hostId);
  return useMutation({
    mutationFn: ({ name, stopFirst }: { name: string; stopFirst: boolean }) =>
      commands.incusInstanceDelete(hostId, project, name, stopFirst),
    onSuccess: (_, { name }) => toast.success(t('incus.instances.deleted', { name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Takes, restores and deletes an Instance's snapshots. Taking shows its failure in the dialog. */
export function useSnapshots(hostId: string, project: string, instance: string) {
  const refresh = useRefresh(hostId);
  const take = useMutation({
    meta: { silent: true },
    mutationFn: (request: SnapshotRequest) =>
      commands.incusSnapshotCreate(hostId, project, instance, request),
    onSuccess: () => toast.success(t('incus.snapshots.taken', { name: instance })),
    onSettled: () => refresh(),
  });
  const restore = useMutation({
    mutationFn: ({ snapshot, stateful }: { snapshot: string; stateful: boolean }) =>
      commands.incusSnapshotRestore(hostId, project, instance, snapshot, stateful),
    onSuccess: (_, { snapshot }) =>
      toast.success(t('incus.snapshots.restored', { name: instance, snapshot })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
  const remove = useMutation({
    mutationFn: (snapshot: string) =>
      commands.incusSnapshotDelete(hostId, project, instance, snapshot),
    onSuccess: (_, snapshot) => toast.success(t('incus.snapshots.deleted', { snapshot })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
  return { take, restore, remove };
}
