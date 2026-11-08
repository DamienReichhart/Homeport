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
import { GitStatus } from './GitStatus';
import { t } from '@/shared/i18n';

/** What Homeport can use on this machine: the OS keychain, the compose CLI and git. */
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
              [
                t('settings.machine.stackDeploy'),
                <ComposeStatus
                  compose={capabilities.data.stackDeploy}
                  missing={t('settings.machine.stackDeployMissing')}
                />,
              ],
              [t('settings.machine.git'), <GitStatus git={capabilities.data.git} />],
            ]}
          />
        )}
      </CardContent>
    </Card>
  );
}
