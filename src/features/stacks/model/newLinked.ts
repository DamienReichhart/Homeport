import type { Stack } from '@/shared/api';

/**
 * The External Stack a new Linked Stack named `name` would take over: one
 * running under that name with no record in Homeport.
 */
export function takenOver(stacks: Stack[] | undefined, name: string): Stack | undefined {
  return stacks?.find((s) => s.name === name && s.kind === 'external' && s.id === null);
}
