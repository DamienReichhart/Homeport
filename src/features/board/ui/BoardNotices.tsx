import { TriangleAlertIcon } from 'lucide-react';
import { Link } from 'react-router-dom';
import { describeStatus, type HostResult } from '@/core/host';
import type { Container, HostView } from '@/shared/api';
import { ErrorMessage } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** One line per Host the board can't show: not connected, or its list failed. */
export function BoardNotices({
  skipped,
  failed,
}: {
  skipped: HostView[];
  failed: HostResult<Container[]>[];
}) {
  if (skipped.length === 0 && failed.length === 0) return null;
  return (
    <div className="mb-3 grid gap-2">
      {skipped.map(({ host, status }) => (
        <p key={host.id} className="text-muted-foreground flex items-center gap-2 text-sm">
          <TriangleAlertIcon className="text-warning size-4" />
          {t('board.skipped', { name: host.name, status: describeStatus(status).label })}
          <Link to={`/hosts/${host.id}`} className="text-primary hover:underline">
            {t('board.openHost')}
          </Link>
        </p>
      ))}
      {failed.map((result) => (
        <div key={result.hostId} className="grid gap-1">
          <p className="text-sm font-medium">{t('board.listFailed', { name: result.hostName })}</p>
          {result.error && <ErrorMessage error={result.error} />}
        </div>
      ))}
    </div>
  );
}
