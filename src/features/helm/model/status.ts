import type { Tone } from '@/shared/ui';

/** The colour of a release's or revision's status, as helm names it. */
export function statusTone(status: string): Tone {
  if (status === 'deployed') return 'success';
  if (status === 'failed') return 'danger';
  if (status.startsWith('pending-') || status === 'uninstalling') return 'warning';
  return 'muted';
}

/** Whether an install, upgrade or rollback is still holding the release. */
export const isPending = (status: string) => status.startsWith('pending-');
