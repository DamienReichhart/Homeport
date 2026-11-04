import { useQuery } from '@tanstack/react-query';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  KeyValue,
  StatusDot,
} from '@/shared/ui';
import { machineQuery } from '../api/queries';
import { ComposeStatus } from './ComposeStatus';
import { t } from '@/shared/i18n';

/** What Homeport can use on this machine: the OS keychain and the compose CLI. */
export function MachineCard() {
  const capabilities = useQuery(machineQuery());
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('settings.machine.title')}</CardTitle>
        <CardDescription>{t('settings.machine.description')}</CardDescription>
      </CardHeader>
      <CardContent>
        {capabilities.data && (
          <KeyValue
            items={[
              [
                t('settings.machine.keychain'),
                <span className="inline-flex items-center gap-1.5">
                  <StatusDot tone={capabilities.data.keychain ? 'success' : 'danger'} />
                  {capabilities.data.keychain
                    ? t('settings.machine.keychainAvailable')
                    : t('settings.machine.keychainUnavailable')}
                </span>,
              ],
              [
                t('settings.machine.compose'),
                <ComposeStatus compose={capabilities.data.compose} />,
              ],
            ]}
          />
        )}
      </CardContent>
    </Card>
  );
}
