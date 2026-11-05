import { ErrorMessage } from '@/shared/ui';
import type { useUpdateCheck } from '../api/queries';
import { t } from '@/shared/i18n';

/** The outcome of the last check, in one line. */
export function UpdateStatus({ check }: { check: ReturnType<typeof useUpdateCheck> }) {
  if (check.isFetching) return null;
  if (check.error) return <ErrorMessage error={check.error} className="basis-full" />;
  if (!check.data) return null;
  const { available, currentVersion } = check.data;
  return (
    <span className="text-muted-foreground text-sm">
      {available
        ? t('updates.status.available', { version: available.version, current: currentVersion })
        : t('updates.status.latest', { current: currentVersion })}
    </span>
  );
}
