import type { ComposeAction, DeploySummary } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

export type Outcome = 'succeeded' | 'failed' | 'unfinished';

/** How a recorded deploy ended; a cancelled one never finished. */
export function outcomeOf(deploy: DeploySummary): Outcome {
  if (deploy.finishedAt === null || deploy.success === null) return 'unfinished';
  return deploy.success ? 'succeeded' : 'failed';
}

/** The deploy running on the Host as far as Homeport knows: the newest that succeeded. */
export function currentDeploy(deploys: DeploySummary[]): string | null {
  return deploys.find((d) => outcomeOf(d) === 'succeeded')?.id ?? null;
}

/** The name of the action that deployed it. */
export const ACTION_LABEL: Record<ComposeAction, MessageKey> = {
  up: 'stacks.action.deploy',
  rebuild: 'stacks.action.rebuild',
  pull: 'stacks.action.pull',
  down: 'stacks.down.confirm',
  downWithVolumes: 'stacks.downWithVolumes.action',
};
