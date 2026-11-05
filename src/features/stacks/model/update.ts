import type { Stack } from '@/shared/api';
import type { ServiceFreshness } from './freshness';

/** What a Stack with newer images can do about them. */
export type UpdateAction = 'none' | 'update' | 'adoptFirst';

/**
 * `update` for a deployed Managed or Linked Stack with a service whose image
 * moved on; `adoptFirst` for an External one, which can't be redeployed.
 */
export function updateAction(stack: Stack, newer: ServiceFreshness[]): UpdateAction {
  if (newer.length === 0) return 'none';
  if (stack.kind === 'external') return 'adoptFirst';
  return stack.id !== null && stack.state !== 'notDeployed' ? 'update' : 'none';
}

/** The images an update pulls, once each. */
export const imagesToUpdate = (newer: ServiceFreshness[]) => [
  ...new Set(newer.flatMap((s) => s.images)),
];
