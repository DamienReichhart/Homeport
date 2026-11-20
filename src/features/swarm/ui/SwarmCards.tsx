import { useNavigate } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { shortSwarmId, swarmLabel, useSwarms } from '@/core/swarm';
import { Badge, StatusDot } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** A tile per Swarm, for the Board's fleet; nothing when no Host is in one. */
export function SwarmCards() {
  const swarms = useSwarms();
  const hosts = useHosts();
  const navigate = useNavigate();
  if (!swarms.data?.length) return null;
  return swarms.data.map((swarm) => (
    <button
      key={swarm.id}
      type="button"
      onClick={() => void navigate(`/swarms/${swarm.id}`)}
      className="bg-card hover:border-foreground/40 focus-visible:ring-ring/50 grid min-w-0 gap-1.5 rounded-md border p-3 text-start text-sm transition-colors outline-none focus-visible:ring-3"
    >
      <span className="flex min-w-0 items-center gap-2">
        <span className="min-w-0 flex-1 truncate font-semibold">
          {swarmLabel(swarm, hosts.data)}
        </span>
        <Badge variant="outline" className="text-muted-foreground font-normal">
          {t('swarm.badge')}
        </Badge>
      </span>
      <span className="text-muted-foreground truncate font-mono text-xs">
        {shortSwarmId(swarm.id)}
      </span>
      <span className="text-muted-foreground flex flex-wrap items-center gap-x-4 gap-y-1">
        {!swarm.reachable && (
          <span className="text-foreground flex items-center gap-1.5">
            <StatusDot tone="warning" />
            {t('swarm.unreachable')}
          </span>
        )}
        <span>
          {t('swarm.hostCounts', {
            managers: swarm.managers.length,
            workers: swarm.workers.length,
          })}
        </span>
      </span>
    </button>
  ));
}
