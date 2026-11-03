import { useQuery } from '@tanstack/react-query';
import type { HostView } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  useConfirm,
} from '@/shared/ui';
import { useRegenerateKey } from '../../api/mutations';
import { sshSetupQuery } from '../../api/queries';
import { CommandBlock } from './CommandBlock';

/** How to authorize an SSH Host's generated key on its server. */
export function SshSetupCard({ view }: { view: HostView }) {
  const setup = useQuery(sshSetupQuery(view.host));
  const regenerate = useRegenerateKey(view.host.id);
  const [dialog, confirm] = useConfirm();
  if (!setup.data) return null;
  return (
    <Card>
      <CardHeader>
        <CardTitle>Authorize Homeport on the server</CardTitle>
        <CardDescription>
          Run these on{' '}
          <span className="font-mono">
            {view.host.connection.kind === 'ssh' && view.host.connection.host}
          </span>{' '}
          as the SSH user, then reconnect.
        </CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <CommandBlock label="1. Add the public key" command={setup.data.installCommand} />
        <CommandBlock
          label="2. Let the user reach Docker (if needed)"
          command={setup.data.dockerGroupCommand}
        />
        <div className="text-muted-foreground flex flex-wrap items-center justify-between gap-2 text-xs">
          <span>
            Key fingerprint <span className="font-mono">{setup.data.fingerprint}</span>
          </span>
          {view.host.connection.kind === 'ssh' && view.host.connection.auth === 'generated' && (
            <Button
              variant="outline"
              size="sm"
              onClick={() =>
                confirm({
                  title: 'Regenerate the SSH key?',
                  description:
                    'The current key stops working. You will need to add the new public key to the server.',
                  confirmLabel: 'Regenerate',
                  onConfirm: () => regenerate.mutateAsync(),
                })
              }
            >
              Regenerate key
            </Button>
          )}
        </div>
      </CardContent>
      {dialog}
    </Card>
  );
}
