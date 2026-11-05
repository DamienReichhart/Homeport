import type { ComposeAction } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** One compose command to run for a Managed Stack. */
export interface ComposeJob {
  stackId: string;
  name: string;
  action: ComposeAction;
}

/** What a running job is doing, given the Stack's `name`. */
export const COMPOSE_TITLES: Record<ComposeAction, MessageKey> = {
  up: 'stacks.compose.up',
  pull: 'stacks.compose.pull',
  down: 'stacks.compose.down',
  downWithVolumes: 'stacks.compose.downWithVolumes',
};
