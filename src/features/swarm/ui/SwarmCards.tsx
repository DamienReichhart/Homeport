import { useNavigate } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { shortSwarmId, swarmLabel, useSwarms } from '@/core/swarm';
import {
  Badge,
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  StatusDot,
} from '@/shared/ui';
import { t } from '@/shared/i18n';

/** A card per Swarm, for the Overview; nothing when no Host is in one. */
export function SwarmCards() {
  const swarms = useSwarms();
  const hosts = useHosts();
  const navigate = useNavigate();
  if (!swarms.data?.length) return null;
  return (
    <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
      {swarms.data.map((swarm) => (
        <Card
          key={swarm.id}
          className="hover:border-primary/60 cursor-pointer transition-colors"
          onClick={() => void navigate(`/swarms/${swarm.id}`)}
        >
          <CardHeader>
            <CardTitle className="truncate">{swarmLabel(swarm, hosts.data)}</CardTitle>
            <CardDescription className="font-mono text-xs">
              {shortSwarmId(swarm.id)}
            </CardDescription>
            <CardAction>
              <Badge variant="outline">{t('swarm.badge')}</Badge>
            </CardAction>
          </CardHeader>
          <CardContent className="grid gap-2 text-sm">
            <span className="flex items-center gap-2">
              <StatusDot tone={swarm.reachable ? 'success' : 'warning'} />
              {swarm.reachable ? t('swarm.reachable') : t('swarm.unreachable')}
            </span>
            <span className="text-muted-foreground">
              {t('swarm.hostCounts', {
                managers: swarm.managers.length,
                workers: swarm.workers.length,
              })}
            </span>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
