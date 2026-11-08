import { PlusIcon } from 'lucide-react';
import { Link } from 'react-router-dom';
import { ClusterGate, useClusters, useClusterSelection } from '@/core/cluster';
import { HostGate, useHosts, useHostSelection } from '@/core/host';
import { SwarmCards } from '@/features/swarm';
import { Button, PageHeader } from '@/shared/ui';
import { ClusterCard } from './clusters/ClusterCard';
import { ClusterDashboard } from './dashboard/ClusterDashboard';
import { HostDashboard } from './dashboard/HostDashboard';
import { HostCard } from './hosts/HostCard';
import { t } from '@/shared/i18n';

export function OverviewPage() {
  const hosts = useHosts();
  const clusters = useClusters();
  const { hostId, host } = useHostSelection();
  const { clusterId, cluster } = useClusterSelection();
  const any = (hosts.data?.length ?? 0) + (clusters.data?.length ?? 0) > 0;
  return (
    <div className="grid gap-6">
      <PageHeader
        title={t('overview.title')}
        actions={
          <>
            <Button asChild variant="outline">
              <Link to="/hosts/new">
                <PlusIcon /> {t('overview.addHost')}
              </Link>
            </Button>
            <Button asChild variant="outline">
              <Link to="/clusters/new">
                <PlusIcon /> {t('overview.addCluster')}
              </Link>
            </Button>
          </>
        }
      />
      {any && (
        <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {hosts.data?.map((view) => (
            <HostCard key={view.host.id} view={view} selected={view.host.id === hostId} />
          ))}
          {clusters.data?.map((view) => (
            <ClusterCard
              key={view.cluster.id}
              view={view}
              selected={view.cluster.id === clusterId}
            />
          ))}
        </div>
      )}
      <SwarmCards />
      <section className="grid gap-3">
        {clusterId ? (
          <>
            {cluster && <h2 className="text-lg font-semibold">{cluster.cluster.name}</h2>}
            <ClusterGate>
              {(view) => <ClusterDashboard key={view.cluster.id} cluster={view} />}
            </ClusterGate>
          </>
        ) : (
          <>
            {host && <h2 className="text-lg font-semibold">{host.host.name}</h2>}
            <HostGate>{(id) => <HostDashboard hostId={id} />}</HostGate>
          </>
        )}
      </section>
    </div>
  );
}
