import { ShieldAlertIcon, ShieldQuestionIcon } from 'lucide-react';
import type { HostView } from '@/shared/api';
import { Alert, AlertDescription, AlertTitle, Button, useConfirm } from '@/shared/ui';
import { useTrustHostKey } from '../../api/mutations';

/** Asks the user to check an SSH server's key on first use, or after it changed. */
export function TrustPrompt({ view }: { view: HostView }) {
  const [dialog, confirm] = useConfirm();
  const trust = useTrustHostKey(view.host.id);
  const status = view.status;
  if (status.state === 'hostKeyUntrusted') {
    return (
      <Alert>
        <ShieldQuestionIcon />
        <AlertTitle>Is this the right server?</AlertTitle>
        <AlertDescription className="grid gap-2">
          <p>
            First connection. The server identifies itself with key fingerprint{' '}
            <span className="font-mono">{status.fingerprint}</span>. Compare it with{' '}
            <span className="font-mono">ssh-keygen -lf /etc/ssh/ssh_host_ed25519_key.pub</span> on
            the server.
          </p>
          <Button size="sm" className="justify-self-start" onClick={() => trust.mutate(status.key)}>
            Trust this key
          </Button>
        </AlertDescription>
      </Alert>
    );
  }
  if (status.state === 'hostKeyMismatch') {
    return (
      <Alert variant="destructive">
        <ShieldAlertIcon />
        <AlertTitle>The server&apos;s key changed</AlertTitle>
        <AlertDescription className="grid gap-2">
          <p>
            Expected <span className="font-mono">{status.expectedFingerprint}</span> but the server
            presented <span className="font-mono">{status.fingerprint}</span>. This happens after
            reinstalling the server, but can also mean someone is intercepting the connection.
          </p>
          <Button
            size="sm"
            variant="destructive"
            className="justify-self-start"
            onClick={() =>
              confirm({
                title: 'Trust the new server key?',
                description: `Only do this if you know why the key changed. New fingerprint: ${status.fingerprint}.`,
                confirmLabel: 'Trust new key',
                destructive: true,
                onConfirm: () => trust.mutateAsync(status.key),
              })
            }
          >
            Trust the new key
          </Button>
        </AlertDescription>
        {dialog}
      </Alert>
    );
  }
  return null;
}
