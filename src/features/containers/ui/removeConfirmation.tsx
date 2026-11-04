import type { Container } from '@/shared/api';
import type { ConfirmOptions } from '@/shared/ui';

/** Confirmation for removing containers, naming what goes with them. */
export function removeConfirmation(
  containers: Container[],
  onConfirm: () => Promise<unknown>,
): ConfirmOptions {
  const running = containers.filter((c) => c.state === 'running').length;
  const what = containers.length === 1 ? containers[0]?.name : `${containers.length} containers`;
  return {
    title: `Remove ${what}?`,
    description: (
      <>
        The container{containers.length === 1 ? '' : 's'} and{' '}
        {containers.length === 1 ? 'its' : 'their'} writable layer and anonymous volumes are
        deleted. {running > 0 && <strong>{running} running will be stopped first. </strong>}
        Named volumes are kept.
      </>
    ),
    confirmLabel: 'Remove',
    destructive: true,
    onConfirm,
  };
}
