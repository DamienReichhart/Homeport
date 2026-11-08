import { RefreshCwIcon } from 'lucide-react';
import { connectionLabel, describeStatus, HostStatusBadge, statusError } from '@/core/host';
import { HostSwarmLink } from '@/features/swarm';
import type { HostView } from '@/shared/api';
import { Badge, Button, ErrorMessage, Label, PageHeader, Switch } from '@/shared/ui';
import { useHostConnection } from '../../api/mutations';
import { ConnectionCard } from './ConnectionCard';
import { RemoveHostCard } from './RemoveHostCard';
import { SshSetupCard } from './SshSetupCard';
import { TrustPrompt } from './TrustPrompt';
import { t } from '@/shared/i18n';

/** A Host's status, connection settings and removal. */
export function HostDetail({ view }: { view: HostView }) {
  const { reconnect, enable } = useHostConnection(view.host.id);
  const { detail } = describeStatus(view.status);

  return (
    <div className="grid max-w-3xl gap-4">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            {view.host.name}{' '}
            <Badge variant="outline">{connectionLabel(view.host.connection.kind)}</Badge>
          </span>
        }
        description={
          <span className="flex items-center gap-2">
            <HostStatusBadge status={view.status} />
            {detail && !statusError(view.status) && (
              <span className="text-muted-foreground">· {detail}</span>
            )}
            <HostSwarmLink hostId={view.host.id} />
          </span>
        }
        actions={
          <>
            <div className="flex items-center gap-2">
              <Switch
                id="enabled"
                checked={view.host.enabled}
                onCheckedChange={(v) => enable.mutate(v)}
              />
              <Label htmlFor="enabled">{t('hosts.detail.enabled')}</Label>
            </div>
            <Button
              variant="outline"
              disabled={reconnect.isPending}
              onClick={() => reconnect.mutate()}
            >
              <RefreshCwIcon /> {t('hosts.detail.reconnect')}
            </Button>
          </>
        }
      />
      <TrustPrompt view={view} />
      {statusError(view.status) && (
        <ErrorMessage error={statusError(view.status)} onRetry={() => reconnect.mutate()} />
      )}
      {view.host.connection.kind === 'ssh' && view.host.connection.publicKey && (
        <SshSetupCard view={view} />
      )}
      <ConnectionCard view={view} />
      <RemoveHostCard view={view} />
    </div>
  );
}
