import { Settings2Icon } from 'lucide-react';
import { Link } from 'react-router-dom';
import {
  CLUSTER_KIND_LABELS,
  clusterAddress,
  ClusterGate,
  ClusterStatusBadge,
  useClusterSelection,
} from '@/core/cluster';
import {
  connectionLabel,
  describeAddress,
  HostGate,
  HostStatusBadge,
  supports,
  useHostSelection,
} from '@/core/host';
import { HostSwarmLink } from '@/features/swarm';
import { Badge, Button, PageHeader } from '@/shared/ui';
import { ContainerdDashboard } from './dashboard/ContainerdDashboard';
import { ClusterDashboard } from './dashboard/ClusterDashboard';
import { HostDashboard } from './dashboard/HostDashboard';
import { IncusDashboard } from './dashboard/IncusDashboard';
import { NomadDashboard } from './dashboard/NomadDashboard';
import { t } from '@/shared/i18n';

/** The selected Host's or Cluster's own page: its Engine or API server, usage and events. */
export function OverviewPage() {
  const { host } = useHostSelection();
  const { clusterId, cluster } = useClusterSelection();
  if (clusterId) {
    return (
      <div className="grid gap-6">
        {cluster && (
          <PageHeader
            title={cluster.cluster.name}
            description={
              <span className="flex flex-wrap items-center gap-x-3 gap-y-1">
                <ClusterStatusBadge status={cluster.status} />
                <Badge variant="outline">{t(CLUSTER_KIND_LABELS[cluster.cluster.kind])}</Badge>
                <span className="font-mono text-xs">{clusterAddress(cluster.cluster)}</span>
              </span>
            }
            actions={
              <Button asChild variant="outline">
                <Link to={`/clusters/${cluster.cluster.id}`}>
                  <Settings2Icon /> {t('overview.clusterSettings')}
                </Link>
              </Button>
            }
          />
        )}
        <ClusterGate>
          {(view) =>
            view.cluster.kind === 'nomad' ? (
              <NomadDashboard key={view.cluster.id} cluster={view} />
            ) : (
              <ClusterDashboard key={view.cluster.id} cluster={view} />
            )
          }
        </ClusterGate>
      </div>
    );
  }
  return (
    <div className="grid gap-6">
      {host && (
        <PageHeader
          title={host.host.name}
          description={
            <span className="flex flex-wrap items-center gap-x-3 gap-y-1">
              <HostStatusBadge status={host.status} />
              <Badge variant="outline">{connectionLabel(host.host.connection.kind)}</Badge>
              <span className="font-mono text-xs">{describeAddress(host.host.connection)}</span>
              {host.status.state === 'connected' && <HostSwarmLink hostId={host.host.id} />}
            </span>
          }
          actions={
            <Button asChild variant="outline">
              <Link to={`/hosts/${host.host.id}`}>
                <Settings2Icon /> {t('overview.hostSettings')}
              </Link>
            </Button>
          }
        />
      )}
      <HostGate>
        {(id) =>
          host && supports(host, 'containerd') ? (
            <ContainerdDashboard key={id} host={host} />
          ) : host && supports(host, 'incus') ? (
            <IncusDashboard key={id} host={host} />
          ) : (
            <HostDashboard hostId={id} />
          )
        }
      </HostGate>
    </div>
  );
}
