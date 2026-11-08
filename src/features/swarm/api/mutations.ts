import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { swarmKeys } from '@/core/swarm';
import {
  commands,
  type NodeChange,
  type ObjectInput,
  type JoinRole,
  type JoinTarget,
  type ServiceChange,
  type ServiceSpecInput,
  type ServiceView,
  type SwarmNode,
  type SwarmObject,
} from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Refreshes everything of a Swarm, and the list of Swarms (roles may have changed). */
export function useRefreshSwarm(swarmId: string) {
  const client = useQueryClient();
  return () =>
    Promise.all([
      client.invalidateQueries({ queryKey: swarmKeys.scope(swarmId) }),
      client.invalidateQueries({ queryKey: swarmKeys.list }),
    ]);
}

/** Changes a node's availability, role or labels, reporting the outcome as a toast. */
export function useUpdateNode(swarmId: string) {
  const refresh = useRefreshSwarm(swarmId);
  return useMutation({
    mutationFn: ({ node, change }: { node: SwarmNode; change: NodeChange }) =>
      commands.swarmNodesUpdate(swarmId, node.id, node.version, change),
    onSuccess: (_, { node }) => toast.success(t('swarm.nodes.updated', { name: node.hostname })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

export function useRemoveNode(swarmId: string, onRemoved: () => void) {
  const refresh = useRefreshSwarm(swarmId);
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ node, force }: { node: SwarmNode; force: boolean }) =>
      commands.swarmNodesRemove(swarmId, node.id, force),
    onSuccess: async (_, { node }) => {
      toast.success(t('swarm.nodes.removed', { name: node.hostname }));
      await refresh();
      onRemoved();
    },
  });
}

/** Sets a replicated service's replicas; the dialog shows a failure in place. */
export function useScaleService(swarmId: string, onDone: () => void) {
  const refresh = useRefreshSwarm(swarmId);
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ service, replicas }: { service: ServiceView; replicas: number }) =>
      commands.swarmServicesScale(swarmId, service.id, service.version, replicas),
    onSuccess: async (_, { service, replicas }) => {
      toast.success(t('swarm.services.scaled', { name: service.name, count: replicas }));
      await refresh();
      onDone();
    },
  });
}

export function useRollbackService(swarmId: string) {
  const refresh = useRefreshSwarm(swarmId);
  return useMutation({
    mutationFn: (service: ServiceView) => commands.swarmServicesRollback(swarmId, service.id),
    onSuccess: (_, service) =>
      toast.success(t('swarm.services.rolledBack', { name: service.name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

export function useRemoveService(swarmId: string) {
  const refresh = useRefreshSwarm(swarmId);
  return useMutation({
    mutationFn: (service: ServiceView) => commands.swarmServicesRemove(swarmId, service.id),
    onSuccess: (_, service) => toast.success(t('swarm.services.removed', { name: service.name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** A secret or a config: the two are listed, created and removed alike. */
export type ObjectKind = 'secret' | 'config';

/**
 * Creates a secret or config; the dialog shows a failure in place. Its input
 * may hold a secret's value, so the mutation is dropped from the cache as
 * soon as nothing watches it (and the dialog resets it on closing).
 */
export function useCreateObject(swarmId: string, kind: ObjectKind, onCreated: () => void) {
  const refresh = useRefreshSwarm(swarmId);
  return useMutation({
    meta: { silent: true },
    gcTime: 0,
    mutationFn: (input: ObjectInput) =>
      kind === 'secret'
        ? commands.swarmSecretsCreate(swarmId, input)
        : commands.swarmConfigsCreate(swarmId, input),
    onSuccess: async () => {
      await refresh();
      onCreated();
    },
  });
}

export function useRemoveObject(swarmId: string, kind: ObjectKind) {
  const refresh = useRefreshSwarm(swarmId);
  return useMutation({
    mutationFn: (object: SwarmObject) =>
      kind === 'secret'
        ? commands.swarmSecretsRemove(swarmId, object.id)
        : commands.swarmConfigsRemove(swarmId, object.id),
    onSuccess: (_, object) => toast.success(t('swarm.objects.removed', { name: object.name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/** Creates a service; the dialog shows a failure in place. */
export function useCreateService(swarmId: string, onCreated: (id: string) => void) {
  const refresh = useRefreshSwarm(swarmId);
  return useMutation({
    meta: { silent: true },
    mutationFn: (input: ServiceSpecInput) => commands.swarmServicesCreate(swarmId, input),
    onSuccess: async (id, input) => {
      toast.success(t('swarm.services.created', { name: input.name }));
      await refresh();
      onCreated(id);
    },
  });
}

/** Updates a service from the version it was read at; the dialog shows a failure in place. */
export function useUpdateService(swarmId: string, onDone: () => void) {
  const refresh = useRefreshSwarm(swarmId);
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ service, change }: { service: ServiceView; change: ServiceChange }) =>
      commands.swarmServicesUpdate(swarmId, service.id, service.version, change),
    onSuccess: async (_, { service }) => {
      toast.success(t('swarm.services.updating', { name: service.name }));
      await refresh();
      onDone();
    },
  });
}

/** Refreshes the Hosts and Swarms after a Host's membership changed. */
function useRefreshMembership() {
  const client = useQueryClient();
  return () =>
    Promise.all([
      client.invalidateQueries({ queryKey: swarmKeys.list }),
      client.invalidateQueries({ queryKey: ['hosts'] }),
    ]);
}

/** Makes a Host the first manager of a new Swarm; the dialog shows a failure in place. */
export function useInitSwarm(hostId: string, onDone: () => void) {
  const refresh = useRefreshMembership();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ advertise, listen }: { advertise: string; listen: string }) =>
      commands.swarmInit(hostId, advertise.trim() || null, listen.trim() || null),
    onSuccess: async () => {
      toast.success(t('swarm.membership.created'));
      await refresh();
      onDone();
    },
  });
}

/** Joins a Host to a Swarm; the token, if pasted, leaves the cache with the dialog. */
export function useJoinSwarm(hostId: string, onDone: () => void) {
  const refresh = useRefreshMembership();
  return useMutation({
    meta: { silent: true },
    gcTime: 0,
    mutationFn: ({
      target,
      role,
      advertise,
    }: {
      target: JoinTarget;
      role: JoinRole;
      advertise: string;
    }) => commands.swarmJoin(hostId, target, role, advertise.trim() || null),
    onSuccess: async () => {
      toast.success(t('swarm.membership.joined'));
      await refresh();
      onDone();
    },
  });
}

export function useLeaveSwarm(hostId: string, onDone: () => void) {
  const refresh = useRefreshMembership();
  return useMutation({
    meta: { silent: true },
    mutationFn: (force: boolean) => commands.swarmLeave(hostId, force),
    onSuccess: async () => {
      toast.success(t('swarm.membership.left'));
      await refresh();
      onDone();
    },
  });
}
