import { Link } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { swarmLabel, swarmOfHost, useSwarms } from '@/core/swarm';
import { t } from '@/shared/i18n';

/** "Manager of …" or "Worker of …", linking to its node in the Swarm; nothing outside one. */
export function HostSwarmLink({ hostId }: { hostId: string }) {
  const swarms = useSwarms();
  const hosts = useHosts();
  const found = swarmOfHost(swarms.data, hostId);
  if (!found) return null;
  const name = swarmLabel(found.swarm, hosts.data);
  const status = hosts.data?.find((h) => h.host.id === hostId)?.status;
  const nodeId = status?.state === 'connected' ? status.swarm?.nodeId : undefined;
  return (
    <Link
      to={
        nodeId
          ? `/swarms/${found.swarm.id}/nodes?node=${encodeURIComponent(nodeId)}`
          : `/swarms/${found.swarm.id}`
      }
      className="text-primary text-sm hover:underline"
      onClick={(e) => e.stopPropagation()}
    >
      {found.role === 'manager'
        ? t('swarm.host.managerOf', { swarm: name })
        : t('swarm.host.workerOf', { swarm: name })}
    </Link>
  );
}
