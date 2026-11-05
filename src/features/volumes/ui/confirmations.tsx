import type { VolumeView } from '@/shared/api';
import type { ConfirmedAction } from '@/shared/ui';
import { t, tRich } from '@/shared/i18n';

/** Deleting a volume, which destroys its data. */
export function removeVolumeConfirmation(
  volume: VolumeView,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: t('volumes.remove.title', { name: volume.name }),
    description: (
      <>
        {tRich('volumes.remove.warning', { strong: (chunks) => <strong>{chunks}</strong> })}
        {volume.usedBy.length > 0 &&
          t('volumes.remove.mounted', { containers: volume.usedBy.join(', ') })}
      </>
    ),
    confirmLabel: t('volumes.remove.confirm'),
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
    title: all ? t('volumes.prune.allTitle') : t('volumes.prune.anonymousTitle'),
    description: (
      <strong>{all ? t('volumes.prune.allWarning') : t('volumes.prune.anonymousWarning')}</strong>
    ),
    confirmLabel: t('volumes.prune.confirm'),
    destructive: true,
    action,
  };
}
