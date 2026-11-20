import { useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import {
  chosenNamespace,
  CLUSTER_KIND_LABELS,
  clusterAddress,
  ClusterStatusBadge,
  useClusterSelection,
} from '@/core/cluster';
import { settingsQuery } from '@/core/settings';
import type { ClusterView } from '@/shared/api';
import { cn } from '@/shared/lib';
import { Badge } from '@/shared/ui';
import { overviewQueries } from '../../api/queries';
import { t } from '@/shared/i18n';

/** A Cluster at a glance on the Board; clicking it opens its Overview. */
export function ClusterCard({ view, selected }: { view: ClusterView; selected: boolean }) {
  const { select } = useClusterSelection();
  const navigate = useNavigate();
  const settings = useQuery(settingsQuery());
  const connected = view.status.state === 'connected';
  const namespace = chosenNamespace(view, settings.data?.clusterNamespaces);
  const summary = useQuery({
    ...overviewQueries.clusterSummary(view.cluster.id, namespace),
    enabled: connected,
  });
  const nodes = summary.data?.nodes;
  const pods = summary.data?.pods;
  return (
    <button
      type="button"
      aria-current={selected || undefined}
      onClick={() => {
        select(view.cluster.id);
        void navigate('/overview');
      }}
      className={cn(
        'bg-card hover:border-foreground/40 focus-visible:ring-ring/50 grid min-w-0 gap-1.5 rounded-md border p-3 text-start text-sm transition-colors outline-none focus-visible:ring-3',
        selected && 'border-foreground/60',
        view.status.state === 'unreachable' && 'border-destructive/50',
      )}
    >
      <span className="flex min-w-0 items-center gap-2">
        <span className="min-w-0 flex-1 truncate font-semibold">{view.cluster.name}</span>
        <Badge variant="outline" className="text-muted-foreground font-normal">
          {t(CLUSTER_KIND_LABELS[view.cluster.kind])}
        </Badge>
      </span>
      <span className="text-muted-foreground truncate font-mono text-xs">
        {clusterAddress(view.cluster)}
      </span>
      {!connected ? (
        <ClusterStatusBadge status={view.status} />
      ) : (
        <span className="text-muted-foreground flex flex-wrap gap-x-4 gap-y-1 tabular-nums">
          {nodes && (
            <span>
              {nodes.state === 'read'
                ? t('overview.cluster.nodes', {
                    ready: nodes.value.ready,
                    total: nodes.value.total,
                  })
                : t('overview.cluster.nodesNotAllowed')}
            </span>
          )}
          {pods && (
            <span>
              {pods.state === 'read'
                ? t('overview.cluster.pods', {
                    running: pods.value.running,
                    total: pods.value.total,
                  })
                : t('overview.cluster.podsNotAllowed')}
            </span>
          )}
          {!summary.data && view.status.state === 'connected' && (
            <span>{view.status.serverVersion}</span>
          )}
        </span>
      )}
    </button>
  );
}
