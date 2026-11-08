import { useQuery } from '@tanstack/react-query';
import { Link, useNavigate } from 'react-router-dom';
import {
  connectionLabel,
  describeAddress,
  HostStatusBadge,
  resourceQueries,
  useHostSelection,
} from '@/core/host';
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
  const info = useQuery({ ...resourceQueries.systemInfo(view.host.id), enabled: connected });
  return (
    <Card
      className={cn(
        'cursor-pointer transition-colors hover:border-primary/60',
        selected && 'border-primary',
      )}
      onClick={() => {
        select(view.host.id);
        if (connected) navigate('/containers');
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
        {connected && (
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
