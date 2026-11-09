import { RefreshCwIcon, ShipWheelIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import type { ClusterView } from '@/shared/api';
import { Button, EmptyState, ErrorMessage, Spinner } from '@/shared/ui';
import { useReconnectCluster } from '../api/mutations';
import { useClusters } from '../api/queries';
import { clusterStatusError, describeClusterStatus } from '../model/status';
import { useClusterSelection } from '../selection/useClusterSelection';
import { t } from '@/shared/i18n';

/**
 * Renders `children` with the selected Cluster once it is connected, and
 * explains what to do otherwise.
 */
export function ClusterGate({ children }: { children: (cluster: ClusterView) => ReactNode }) {
  const clusters = useClusters();
  const { cluster } = useClusterSelection();
  const reconnect = useReconnectCluster();

  if (clusters.isPending) return <Spinner />;
  if (clusters.error)
    return <ErrorMessage error={clusters.error} onRetry={() => void clusters.refetch()} />;
  if (!cluster) {
    return (
      <EmptyState
        icon={ShipWheelIcon}
        title={t('core.cluster.gate.none')}
        description={t('core.cluster.gate.noneDescription')}
        action={
          <Button asChild>
            <Link to="/clusters/new">{t('core.cluster.gate.add')}</Link>
          </Button>
        }
      />
    );
  }
  const { status } = cluster;
  if (status.state === 'connected') return <>{children(cluster)}</>;
  if (status.state === 'connecting')
    return <Spinner label={t('core.gate.connecting', { name: cluster.cluster.name })} />;

  const { label, detail } = describeClusterStatus(status, cluster.cluster.kind);
  const error = clusterStatusError(status);
  const retry = () => reconnect.mutate(cluster.cluster.id);
  return (
    <EmptyState
      icon={ShipWheelIcon}
      title={t('core.gate.statusTitle', { name: cluster.cluster.name, status: label })}
      description={
        error ? (
          <div className="grid gap-2">
            <ErrorMessage error={error} onRetry={retry} />
            {status.state === 'unreachable' && (
              <p>{t('core.gate.retryingOnItsOwn', { secs: status.retryInSecs })}</p>
            )}
          </div>
        ) : (
          detail
        )
      }
      action={
        <div className="flex gap-2">
          {!error && status.state !== 'disabled' && (
            <Button variant="outline" disabled={reconnect.isPending} onClick={retry}>
              <RefreshCwIcon /> {t('core.gate.retryNow')}
            </Button>
          )}
          <Button asChild>
            <Link to={`/clusters/${cluster.cluster.id}`}>
              {t('core.cluster.gate.openSettings')}
            </Link>
          </Button>
        </div>
      }
    />
  );
}
