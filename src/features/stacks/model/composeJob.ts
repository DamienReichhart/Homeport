import type { ComposeAction } from '@/shared/api';

/** One compose command to run for a Managed Stack. */
export interface ComposeJob {
  stackId: string;
  name: string;
  action: ComposeAction;
}

/** What a running job is doing, before the Stack's name. */
export const COMPOSE_TITLES: Record<ComposeAction, string> = {
  up: 'Deploying',
  pull: 'Pulling images for',
  down: 'Taking down',
  downWithVolumes: 'Taking down (with volumes)',
};
