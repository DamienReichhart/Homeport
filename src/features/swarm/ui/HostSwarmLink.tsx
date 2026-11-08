import { Link } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { swarmLabel, swarmOfHost, useSwarms } from '@/core/swarm';
import { t } from '@/shared/i18n';

/** "Manager of …" or "Worker of …", linking to the Swarm; nothing outside one. */
export function HostSwarmLink({ hostId }: { hostId: string }) {
  const swarms = useSwarms();
  const hosts = useHosts();
  const found = swarmOfHost(swarms.data, hostId);
  if (!found) return null;
  const name = swarmLabel(found.swarm, hosts.data);
  return (
    <Link
      to={`/swarms/${found.swarm.id}`}
      className="text-primary text-sm hover:underline"
      onClick={(e) => e.stopPropagation()}
    >
      {found.role === 'manager'
        ? t('swarm.host.managerOf', { swarm: name })
        : t('swarm.host.workerOf', { swarm: name })}
    </Link>
  );
}
