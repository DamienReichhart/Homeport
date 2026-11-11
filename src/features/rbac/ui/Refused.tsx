import type { AppErrorPayload } from '@/shared/api';
import { ErrorMessage } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** The lists the credentials couldn't read, which what is shown leaves out. */
export function Refused({ errors }: { errors: AppErrorPayload[] }) {
  if (!errors.length) return null;
  return (
    <div className="grid gap-2">
      <p className="text-muted-foreground text-sm">{t('rbac.refused')}</p>
      {errors.map((e) => (
        <ErrorMessage key={e.message} error={e} />
      ))}
    </div>
  );
}
