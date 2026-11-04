import type { Loss } from '@/shared/api';
import { t } from '@/shared/i18n';

/** One line of Edit's confirmation per thing the recreated container won't have. */
export function lossText(loss: Loss): string {
  switch (loss.kind) {
    case 'anonymousVolume':
      return t('containers.loss.anonymousVolume', { path: loss.path });
    case 'filesystemChanges':
      return t('containers.loss.filesystemChanges', { count: loss.count });
    case 'downtime':
      return t('containers.loss.downtime');
    case 'stackRedeploy':
      return t('containers.loss.stackRedeploy', { stack: loss.stack });
  }
}
