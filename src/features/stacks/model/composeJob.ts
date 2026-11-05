import type { Candidate, ComposeAction, SourceKind } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** One compose command to run for a Managed or Linked Stack. */
export interface ComposeJob {
  /** Where the Stack's definition is: held by Homeport, or in a folder. */
  kind: SourceKind;
  /** Its record. */
  stackId: string;
  /** Roll a Managed Stack back to this recorded deploy, then deploy it. */
  rollbackTo?: string;
  /**
   * Show what changes before running: `null` compares the stored definition
   * with what is deployed. Left out for first deploys, which go straight on.
   */
  review?: Candidate | null;
  name: string;
  action: ComposeAction;
}

/** What a running job is doing, given the Stack's `name`. */
export const COMPOSE_TITLES: Record<ComposeAction, MessageKey> = {
  up: 'stacks.compose.up',
  rebuild: 'stacks.compose.rebuild',
  pull: 'stacks.compose.pull',
  down: 'stacks.compose.down',
  downWithVolumes: 'stacks.compose.downWithVolumes',
};
