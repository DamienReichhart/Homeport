import { ErrorMessage } from '@/shared/ui';
import type { useUpdateCheck } from '../api/queries';

/** The outcome of the last check, in one line. */
export function UpdateStatus({ check }: { check: ReturnType<typeof useUpdateCheck> }) {
  if (check.isFetching) return null;
  if (check.error) return <ErrorMessage error={check.error} className="basis-full" />;
  if (!check.data) return null;
  const { available, currentVersion } = check.data;
  return (
    <span className="text-muted-foreground text-sm">
      {available
        ? `Homeport ${available.version} is available (you have ${currentVersion}).`
        : `Homeport ${currentVersion} is the latest version.`}
    </span>
  );
}
