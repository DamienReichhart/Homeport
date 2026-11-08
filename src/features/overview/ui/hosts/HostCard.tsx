import { useQuery } from '@tanstack/react-query';
import { Link, useNavigate } from 'react-router-dom';
import {
  connectionLabel,
  describeAddress,
  HostStatusBadge,
  containerdQueries,
  resourceQueries,
  supports,
  useHostSelection,
} from '@/core/host';
import { HostSwarmLink } from '@/features/swarm';
import type { HostView } from '@/shared/api';
import { cn } from '@/shared/lib';
import {
  Badge,
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/shared/ui';
import { HostUsage } from './HostUsage';
import { t, tRich } from '@/shared/i18n';

/** A Host at a glance; clicking it selects it and opens its containers. */
export function HostCard({ view, selected }: { view: HostView; selected: boolean }) {
  const { select } = useHostSelection();
  const navigate = useNavigate();
  const connected = view.status.state === 'connected';
  const docker = supports(view, 'docker');
  const containerd = supports(view, 'containerd');
  const info = useQuery({
    ...resourceQueries.systemInfo(view.host.id),
    enabled: connected && docker,
  });
  const namespaces = useQuery({
    ...containerdQueries.namespaces(view.host.id),
    enabled: connected && containerd,
  });
  const held = namespaces.data?.reduce(
    (sum, n) => ({ containers: sum.containers + n.containers, images: sum.images + n.images }),
    { containers: 0, images: 0 },
  );
  return (
    <Card
      className={cn(
        'cursor-pointer transition-colors hover:border-primary/60',
        selected && 'border-primary',
      )}
      onClick={() => {
        select(view.host.id);
        if (connected) navigate(containerd ? '/containerd/containers' : '/containers');
      }}
    >
      <CardHeader>
        <CardTitle className="truncate">{view.host.name}</CardTitle>
        <CardDescription className="truncate font-mono text-xs">
          {describeAddress(view.host.connection)}
        </CardDescription>
        <CardAction>
          <Badge variant="outline">{connectionLabel(view.host.connection.kind)}</Badge>
        </CardAction>
      </CardHeader>
      <CardContent className="grid gap-3">
        <HostStatusBadge status={view.status} />
        {connected && <HostSwarmLink hostId={view.host.id} />}
        {connected && info.data && (
          <div className="text-muted-foreground grid grid-cols-2 gap-2 text-sm">
            <span>
              {tRich('overview.host.running', {
                running: info.data.running,
                total: info.data.containers,
                strong: (chunks) => <span className="text-foreground font-medium">{chunks}</span>,
              })}
            </span>
            <span>{t('overview.host.images', { count: info.data.images })}</span>
            <HostUsage hostId={view.host.id} cpus={info.data.cpus} memory={info.data.memory} />
          </div>
        )}
        {connected && held && (
          <div className="text-muted-foreground grid grid-cols-2 gap-2 text-sm">
            <span>
              {t('overview.host.containerdContainers', {
                count: held.containers,
                namespaces: namespaces.data?.length ?? 0,
              })}
            </span>
            <span>{t('overview.host.images', { count: held.images })}</span>
          </div>
        )}
        {connected && docker && (
          <Link
            to={`/board?host=${encodeURIComponent(view.host.id)}`}
            className="text-primary justify-self-start text-sm hover:underline"
            onClick={(e) => e.stopPropagation()}
          >
            {t('overview.host.onBoard')}
          </Link>
        )}
      </CardContent>
    </Card>
  );
}
