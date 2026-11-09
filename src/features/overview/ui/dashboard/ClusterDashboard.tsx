import { useQuery } from '@tanstack/react-query';
import { clusterAddress, useNamespace } from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  ErrorMessage,
  KeyValue,
  Spinner,
} from '@/shared/ui';
import { overviewQueries } from '../../api/queries';
import { NodesCard, PodsCard, UnreadyCard, WarningsCard } from './ClusterParts';
import { t } from '@/shared/i18n';

/** The selected Cluster at a glance: its version, Nodes, Pods, workloads short of Pods and warnings. */
export function ClusterDashboard({ cluster }: { cluster: ClusterView }) {
  const [namespace] = useNamespace(cluster);
  const summary = useQuery(overviewQueries.clusterSummary(cluster.cluster.id, namespace));
  const status = cluster.status;
  return (
    <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
      <Card>
        <CardHeader>
          <CardTitle>{t('overview.cluster.versionTitle')}</CardTitle>
        </CardHeader>
        <CardContent>
          <KeyValue
            items={[
              [
                t('overview.cluster.version'),
                status.state === 'connected' ? status.serverVersion : null,
              ],
              [
                t('overview.cluster.platform'),
                status.state === 'connected' ? status.platform : null,
              ],
              [
                t('overview.cluster.server'),
                <span className="font-mono text-xs">{clusterAddress(cluster.cluster)}</span>,
              ],
              [t('overview.cluster.namespace'), namespace ?? t('overview.cluster.allNamespaces')],
            ]}
          />
        </CardContent>
      </Card>
      {summary.isPending ? (
        <Spinner />
      ) : summary.error ? (
        <ErrorMessage error={summary.error} onRetry={() => void summary.refetch()} />
      ) : (
        <>
          <NodesCard part={summary.data.nodes} />
          <PodsCard part={summary.data.pods} />
          <UnreadyCard part={summary.data.unready} />
          <WarningsCard part={summary.data.warnings} />
        </>
      )}
    </div>
  );
}
