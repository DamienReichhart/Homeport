import { useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { chosenNamespace, ClusterStatusBadge, useClusterSelection } from '@/core/cluster';
import { settingsQuery } from '@/core/settings';
import type { ClusterView } from '@/shared/api';
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
import { overviewQueries } from '../../api/queries';
import { t } from '@/shared/i18n';

/** A Cluster at a glance; clicking it selects it. */
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
    <Card
      className={cn(
        'cursor-pointer transition-colors hover:border-primary/60',
        selected && 'border-primary',
      )}
      onClick={() => {
        select(view.cluster.id);
        void navigate('/');
      }}
    >
      <CardHeader>
        <CardTitle className="truncate">{view.cluster.name}</CardTitle>
        <CardDescription className="truncate font-mono text-xs">
          {view.cluster.connection.server}
        </CardDescription>
        <CardAction>
          <Badge variant="outline">{t('overview.cluster.kind')}</Badge>
        </CardAction>
      </CardHeader>
      <CardContent className="grid gap-3">
        <span className="flex items-center gap-2">
          <ClusterStatusBadge status={view.status} />
          {view.status.state === 'connected' && (
            <span className="text-muted-foreground text-sm">· {view.status.serverVersion}</span>
          )}
        </span>
        {connected && summary.data && (
          <div className="text-muted-foreground grid grid-cols-2 gap-2 text-sm">
            <span>
              {nodes?.state === 'read'
                ? t('overview.cluster.nodes', {
                    ready: nodes.value.ready,
                    total: nodes.value.total,
                  })
                : t('overview.cluster.nodesNotAllowed')}
            </span>
            <span>
              {pods?.state === 'read'
                ? t('overview.cluster.pods', {
                    running: pods.value.running,
                    total: pods.value.total,
                  })
                : t('overview.cluster.podsNotAllowed')}
            </span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
