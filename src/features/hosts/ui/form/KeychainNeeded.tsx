import { TriangleAlertIcon } from 'lucide-react';
import { Alert, AlertDescription, AlertTitle } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Why keys and certificates can't be entered on a machine without an OS keychain. */
export function KeychainNeeded() {
  return (
    <Alert variant="destructive">
      <TriangleAlertIcon />
      <AlertTitle>{t('hosts.keychain.title')}</AlertTitle>
      <AlertDescription>{t('hosts.keychain.description')}</AlertDescription>
    </Alert>
  );
}
