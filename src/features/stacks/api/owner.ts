import type { QueryKey } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { swarmKeys } from '@/core/swarm';
import type { StackOwner } from '@/shared/api';

/** The Host `hostId` as a Stack's owner. */
export const hostOwner = (hostId: string): StackOwner => ({ kind: 'host', id: hostId });

/** The key of an owner's Stack list. */
export const stacksKey = (owner: StackOwner): QueryKey =>
  owner.kind === 'host' ? hostKeys.stacks(owner.id) : swarmKeys.stacks(owner.id);

/** The key of everything an owner has, refreshed after a deploy. */
export const scopeKey = (owner: StackOwner): QueryKey =>
  owner.kind === 'host' ? hostKeys.scope(owner.id) : swarmKeys.scope(owner.id);

/** A Stack's page: under the Stacks of a Host, or on its Swarm's page. */
export const stackPath = (owner: StackOwner, name: string): string =>
  owner.kind === 'host'
    ? `/stacks/${encodeURIComponent(name)}`
    : `/swarms/${owner.id}/stacks/${encodeURIComponent(name)}`;
