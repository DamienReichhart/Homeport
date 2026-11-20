import type { VolumeView } from '@/shared/api';
import type { ConfirmedAction } from '@/shared/ui';
import { t, tRich } from '@/shared/i18n';

/** Deleting a volume from `host`, which destroys its data: its name is typed first. */
export function removeVolumeConfirmation(
  volume: VolumeView,
  host: string,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: t('volumes.remove.title', { name: volume.name, host }),
    description: (
      <>
        {tRich('volumes.remove.warning', { strong: (chunks) => <strong>{chunks}</strong> })}
        {volume.usedBy.length > 0 &&
          t('volumes.remove.mounted', { containers: volume.usedBy.join(', ') })}
      </>
    ),
    confirmLabel: t('volumes.remove.confirm'),
    destructive: true,
    typeToConfirm: volume.name,
    action,
  };
}

/**
 * Deleting the unused anonymous volumes of `host`, or every unused one,
 * named ones included, which asks for the Host's name to be typed.
 */
export function pruneVolumesConfirmation(
  all: boolean,
  host: string,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: all
      ? t('volumes.prune.allTitle', { host })
      : t('volumes.prune.anonymousTitle', { host }),
    description: (
      <strong>{all ? t('volumes.prune.allWarning') : t('volumes.prune.anonymousWarning')}</strong>
    ),
    confirmLabel: t('volumes.prune.confirm'),
    destructive: true,
    typeToConfirm: all ? host : undefined,
    action,
  };
}
