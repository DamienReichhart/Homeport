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

/** What Homeport can use on this machine: the OS keychain and the compose CLI. */
export function MachineCard() {
  const capabilities = useQuery(machineQuery());
  return (
    <Card>
      <CardHeader>
        <CardTitle>This machine</CardTitle>
        <CardDescription>What Homeport can use here.</CardDescription>
      </CardHeader>
      <CardContent>
        {capabilities.data && (
          <KeyValue
            items={[
              [
                'OS keychain',
                <span className="inline-flex items-center gap-1.5">
                  <StatusDot tone={capabilities.data.keychain ? 'success' : 'danger'} />
                  {capabilities.data.keychain
                    ? 'Available — SSH keys, certificates and stack environments are stored encrypted.'
                    : 'Unavailable — SSH keys, TLS certificates and stack environments can’t be stored.'}
                </span>,
              ],
              ['Compose CLI', <ComposeStatus compose={capabilities.data.compose} />],
            ]}
          />
        )}
      </CardContent>
    </Card>
  );
}
