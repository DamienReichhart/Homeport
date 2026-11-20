import { LockIcon } from 'lucide-react';
import type { AppErrorPayload } from '@/shared/api';
import { Alert, AlertDescription } from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * The lists the credentials couldn't read, which what is shown leaves out.
 * Calm, not red: a narrower view is how RBAC is meant to work, not a failure.
 */
export function Refused({ errors }: { errors: AppErrorPayload[] }) {
  if (!errors.length) return null;
  return (
    <Alert>
      <LockIcon />
      <AlertDescription>
        <p>{t('rbac.refused')}</p>
        <ul className="grid gap-0.5 font-mono text-xs">
          {errors.map((e) => (
            <li key={e.message}>{e.message}</li>
          ))}
        </ul>
      </AlertDescription>
    </Alert>
  );
}
