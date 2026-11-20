import { useClusters, useClusterSelection } from '@/core/cluster';
import { useHosts, useHostSelection } from '@/core/host';
import { SwarmCards } from '@/features/swarm';
import { ClusterCard } from './clusters/ClusterCard';
import { HostCard } from './hosts/HostCard';

/** A tile per Host, Swarm and Cluster, for the top of the Board. */
export function FleetCards() {
  const hosts = useHosts();
  const clusters = useClusters();
  const { hostId } = useHostSelection();
  const { clusterId } = useClusterSelection();
  if (!hosts.data?.length && !clusters.data?.length) return null;
  return (
    <div className="grid grid-cols-[repeat(auto-fill,minmax(15rem,1fr))] gap-3">
      {hosts.data?.map((view) => (
        <HostCard key={view.host.id} view={view} selected={view.host.id === hostId} />
      ))}
      <SwarmCards />
      {clusters.data?.map((view) => (
        <ClusterCard key={view.cluster.id} view={view} selected={view.cluster.id === clusterId} />
      ))}
    </div>
  );
}
