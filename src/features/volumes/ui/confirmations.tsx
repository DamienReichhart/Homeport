import type { VolumeView } from '@/shared/api';
import type { ConfirmedAction } from '@/shared/ui';

/** Deleting a volume, which destroys its data. */
export function removeVolumeConfirmation(
  volume: VolumeView,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: `Delete volume ${volume.name}?`,
    description: (
      <>
        <strong>All data in this volume is permanently deleted.</strong>
        {volume.usedBy.length > 0 &&
          ` It is mounted by ${volume.usedBy.join(', ')}; remove those containers first.`}
      </>
    ),
    confirmLabel: 'Delete volume and data',
    destructive: true,
    action,
  };
}

/** Deleting unused anonymous volumes, or every unused one. */
export function pruneVolumesConfirmation(
  all: boolean,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: all ? 'Delete every unused volume?' : 'Delete unused anonymous volumes?',
    description: (
      <strong>
        {all
          ? 'The data in every volume no container uses is permanently deleted, named volumes included.'
          : 'The data in unnamed volumes no container uses is permanently deleted.'}
      </strong>
    ),
    confirmLabel: 'Delete volumes and data',
    destructive: true,
    action,
  };
}
