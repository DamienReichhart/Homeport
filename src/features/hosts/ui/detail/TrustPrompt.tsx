import { ShieldAlertIcon, ShieldQuestionIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import type { HostView } from '@/shared/api';
import { Alert, AlertDescription, AlertTitle, Button, useConfirm } from '@/shared/ui';
import { useTrustHostKey } from '../../api/mutations';
import { t, tRich } from '@/shared/i18n';

const mono = (chunks: ReactNode[]) => <span className="font-mono">{chunks}</span>;
const FINGERPRINT_COMMAND = 'ssh-keygen -lf /etc/ssh/ssh_host_ed25519_key.pub';

/** Asks the user to check an SSH server's key on first use, or after it changed. */
export function TrustPrompt({ view }: { view: HostView }) {
  const [dialog, confirm] = useConfirm();
  const trust = useTrustHostKey(view.host.id);
  const status = view.status;
  if (status.state === 'hostKeyUntrusted') {
    return (
      <Alert>
        <ShieldQuestionIcon />
        <AlertTitle>{t('hosts.trust.untrusted.title')}</AlertTitle>
        <AlertDescription className="grid gap-2">
          <p>
            {tRich('hosts.trust.untrusted.description', {
              fingerprint: status.fingerprint,
              command: FINGERPRINT_COMMAND,
              mono,
            })}
          </p>
          <Button size="sm" className="justify-self-start" onClick={() => trust.mutate(status.key)}>
            {t('hosts.trust.untrusted.button')}
          </Button>
        </AlertDescription>
      </Alert>
    );
  }
  if (status.state === 'hostKeyMismatch') {
    return (
      <Alert variant="destructive">
        <ShieldAlertIcon />
        <AlertTitle>{t('hosts.trust.mismatch.title')}</AlertTitle>
        <AlertDescription className="grid gap-2">
          <p>
            {tRich('hosts.trust.mismatch.description', {
              expected: status.expectedFingerprint,
              fingerprint: status.fingerprint,
              mono,
            })}
          </p>
          <Button
            size="sm"
            variant="destructive"
            className="justify-self-start"
            onClick={() =>
              confirm({
                title: t('hosts.trust.mismatch.confirmTitle'),
                description: t('hosts.trust.mismatch.confirmDescription', {
                  fingerprint: status.fingerprint,
                }),
                confirmLabel: t('hosts.trust.mismatch.confirm'),
                destructive: true,
                onConfirm: () => trust.mutateAsync(status.key),
              })
            }
          >
            {t('hosts.trust.mismatch.button')}
          </Button>
        </AlertDescription>
        {dialog}
      </Alert>
    );
  }
  return null;
}
