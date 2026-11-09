import type { AppError, UpdateCheck } from '@/shared/api';
import { t } from '@/shared/i18n';

/** How often Homeport looks for a new version while it's open. */
export const CHECK_INTERVAL_MS = 6 * 60 * 60 * 1000;

export type InstallState =
  | { step: 'idle' }
  | { step: 'downloading'; percent: number | null }
  | { step: 'installing' }
  | { step: 'failed'; error: AppError };

export function describeProgress(state: InstallState): string | undefined {
  if (state.step === 'downloading')
    return state.percent === null
      ? t('updates.progress.downloading')
      : t('updates.progress.downloadingPercent', { percent: state.percent });
  if (state.step === 'installing') return t('updates.progress.installing');
  return undefined;
}

/** One line on how this copy gets updates. */
export function describeMethod(check: UpdateCheck): string {
  switch (check.method.kind) {
    case 'inApp':
      return t('updates.method.inApp');
    case 'packageManager':
      switch (check.method.manager) {
        case 'scoop':
          return t('updates.method.scoop');
        case 'aur':
          return t('updates.method.aur');
        default:
          return t('updates.method.packageManager', { manager: check.method.manager });
      }
    case 'manual':
      return t('updates.method.manual');
  }
}
