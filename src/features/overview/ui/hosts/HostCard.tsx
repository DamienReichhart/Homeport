import { useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import {
  connectionLabel,
  describeAddress,
  HostStatusBadge,
  containerdQueries,
  incusQueries,
  resourceQueries,
  supports,
  useHostSelection,
} from '@/core/host';
import type { HostView } from '@/shared/api';
import { cn } from '@/shared/lib';
import { Badge } from '@/shared/ui';
import { HostUsage } from './HostUsage';
import { t, tRich } from '@/shared/i18n';

/** A Host at a glance on the Board; clicking it opens its Overview. */
export function HostCard({ view, selected }: { view: HostView; selected: boolean }) {
  const { select } = useHostSelection();
  const navigate = useNavigate();
  const connected = view.status.state === 'connected';
  const docker = supports(view, 'docker');
  const containerd = supports(view, 'containerd');
  const incus = supports(view, 'incus');
  const info = useQuery({
    ...resourceQueries.systemInfo(view.host.id),
    enabled: connected && docker,
  });
  const namespaces = useQuery({
    ...containerdQueries.namespaces(view.host.id),
    enabled: connected && containerd,
  });
  const projects = useQuery({
    ...incusQueries.projects(view.host.id),
    enabled: connected && incus,
  });
  const inProjects = projects.data?.reduce(
    (sum, p) => ({ instances: sum.instances + p.instances, images: sum.images + p.images }),
    { instances: 0, images: 0 },
  );
  const held = namespaces.data?.reduce(
    (sum, n) => ({ containers: sum.containers + n.containers, images: sum.images + n.images }),
    { containers: 0, images: 0 },
  );
  const problem = view.status.state !== 'connected';
  return (
    <button
      type="button"
      aria-current={selected || undefined}
      onClick={() => {
        select(view.host.id);
        void navigate('/overview');
      }}
      className={cn(
        'bg-card hover:border-foreground/40 focus-visible:ring-ring/50 grid min-w-0 gap-1.5 rounded-md border p-3 text-start text-sm transition-colors outline-none focus-visible:ring-3',
        selected && 'border-foreground/60',
        view.status.state === 'unreachable' && 'border-destructive/50',
      )}
    >
      <span className="flex min-w-0 items-center gap-2">
        <span className="min-w-0 flex-1 truncate font-semibold">{view.host.name}</span>
        <Badge variant="outline" className="text-muted-foreground font-normal">
          {connectionLabel(view.host.connection.kind)}
        </Badge>
      </span>
      <span className="text-muted-foreground truncate font-mono text-xs">
        {describeAddress(view.host.connection)}
      </span>
      {problem ? (
        <HostStatusBadge status={view.status} />
      ) : (
        <span className="text-muted-foreground flex flex-wrap gap-x-4 gap-y-1 tabular-nums">
          {info.data && (
            <>
              <span>
                {tRich('overview.host.running', {
                  running: info.data.running,
                  total: info.data.containers,
                  strong: (chunks) => <span className="text-foreground font-medium">{chunks}</span>,
                })}
              </span>
              <HostUsage hostId={view.host.id} cpus={info.data.cpus} memory={info.data.memory} />
            </>
          )}
          {held && (
            <span>
              {t('overview.host.containerdContainers', {
                count: held.containers,
                namespaces: namespaces.data?.length ?? 0,
              })}
            </span>
          )}
          {inProjects && (
            <span>
              {t('overview.host.incusInstances', {
                count: inProjects.instances,
                projects: projects.data?.length ?? 0,
              })}
            </span>
          )}
        </span>
      )}
    </button>
  );
}
