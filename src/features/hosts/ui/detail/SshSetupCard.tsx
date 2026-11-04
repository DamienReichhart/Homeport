import { useQuery } from '@tanstack/react-query';
import type { ReactNode } from 'react';
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
import { t, tRich } from '@/shared/i18n';

const mono = (chunks: ReactNode[]) => <span className="font-mono">{chunks}</span>;

/** How to authorize an SSH Host's generated key on its server. */
export function SshSetupCard({ view }: { view: HostView }) {
  const setup = useQuery(sshSetupQuery(view.host));
  const regenerate = useRegenerateKey(view.host.id);
  const [dialog, confirm] = useConfirm();
  if (!setup.data) return null;
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('hosts.sshSetup.title')}</CardTitle>
        <CardDescription>
          {tRich('hosts.sshSetup.description', {
            server: view.host.connection.kind === 'ssh' ? view.host.connection.host : '',
            mono,
          })}
        </CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <CommandBlock label={t('hosts.sshSetup.addKey')} command={setup.data.installCommand} />
        <CommandBlock
          label={t('hosts.sshSetup.dockerGroup')}
          command={setup.data.dockerGroupCommand}
        />
        <div className="text-muted-foreground flex flex-wrap items-center justify-between gap-2 text-xs">
          <span>
            {tRich('hosts.sshSetup.fingerprint', { fingerprint: setup.data.fingerprint, mono })}
          </span>
          {view.host.connection.kind === 'ssh' && view.host.connection.auth === 'generated' && (
            <Button
              variant="outline"
              size="sm"
              onClick={() =>
                confirm({
                  title: t('hosts.regenerate.title'),
                  description: t('hosts.regenerate.description'),
                  confirmLabel: t('hosts.regenerate.confirm'),
                  onConfirm: () => regenerate.mutateAsync(),
                })
              }
            >
              {t('hosts.regenerate.button')}
            </Button>
          )}
        </div>
      </CardContent>
      {dialog}
    </Card>
  );
}
