import { GitCommitHorizontalIcon } from 'lucide-react';
import { Badge } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** A Synced Stack's "Update available": a new commit changes its compose file's folder. */
export function SyncBadge() {
  return (
    <Badge variant="secondary" title={t('stacks.syncUpdate.description')}>
      <GitCommitHorizontalIcon /> {t('stacks.syncUpdate.available')}
    </Badge>
  );
}
