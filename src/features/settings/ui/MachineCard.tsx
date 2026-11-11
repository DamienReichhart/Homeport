import { useQuery } from '@tanstack/react-query';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  ErrorMessage,
  KeyValue,
  StatusDot,
  useConfirm,
} from '@/shared/ui';
import { useKeychainStartOver } from '../api/mutations';
import { machineQuery } from '../api/queries';
import { ComposeStatus } from './ComposeStatus';
import { GitStatus } from './GitStatus';
import { HelmStatus } from './HelmStatus';
import { t } from '@/shared/i18n';

/** What Homeport can use on this machine: the OS keychain, the compose CLI, git and helm. */
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
                    : capabilities.data.keychainEntryMissing
                      ? t('settings.machine.keychainEntryMissing')
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
              [t('settings.machine.helm'), <HelmStatus helm={capabilities.data.helm} />],
            ]}
          />
        )}
        {capabilities.data?.keychainEntryMissing && <StartOver />}
      </CardContent>
    </Card>
  );
}

/**
 * Gives up on the key the stored secrets were sealed with, whose keychain
 * entry is missing, after saying what that costs.
 */
function StartOver() {
  const [dialog, confirm] = useConfirm();
  const startOver = useKeychainStartOver();
  return (
    <div className="mt-4 grid justify-items-start gap-2">
      <p className="text-muted-foreground text-sm">{t('settings.machine.startOverDescription')}</p>
      <Button
        variant="destructive"
        disabled={startOver.isPending}
        onClick={() =>
          confirm({
            title: t('settings.machine.startOverTitle'),
            description: t('settings.machine.startOverConfirmDescription'),
            confirmLabel: t('settings.machine.startOver'),
            destructive: true,
            onConfirm: () => startOver.mutateAsync(),
          })
        }
      >
        {t('settings.machine.startOver')}
      </Button>
      {startOver.error && <ErrorMessage error={startOver.error} />}
      {dialog}
    </div>
  );
}
