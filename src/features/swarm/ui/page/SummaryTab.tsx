import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { swarmQueries } from '@/core/swarm';
import type { SwarmView } from '@/shared/api';
import { Card, CardContent, CardHeader, CardTitle, KeyValue, Spinner } from '@/shared/ui';
import { JoinCommands } from './JoinCommands';
import { t } from '@/shared/i18n';
import { formatDateTime } from '@/shared/lib';

function HostLinks({ ids }: { ids: string[] }) {
  const hosts = useHosts();
  if (ids.length === 0) return <>—</>;
  return (
    <span className="flex flex-wrap gap-x-3">
      {ids.map((id) => (
        <Link key={id} to={`/hosts/${id}`} className="text-primary hover:underline">
          {hosts.data?.find((h) => h.host.id === id)?.host.name ?? id}
        </Link>
      ))}
    </span>
  );
}

/** What a Swarm is made of: its Hosts, node and service counts, and its ID. */
export function SummaryTab({ swarm }: { swarm: SwarmView }) {
  const summary = useQuery(swarmQueries.summary(swarm.id));
  if (!summary.data) return <Spinner />;
  const s = summary.data;
  return (
    <div className="grid gap-4">
      <Card>
        <CardHeader>
          <CardTitle>{t('swarm.summary.title')}</CardTitle>
        </CardHeader>
        <CardContent>
          <KeyValue
            items={[
              [t('swarm.summary.managerHosts'), <HostLinks key="m" ids={swarm.managers} />],
              [t('swarm.summary.workerHosts'), <HostLinks key="w" ids={swarm.workers} />],
              [
                t('swarm.summary.nodes'),
                t('swarm.summary.nodeCounts', { nodes: s.nodes, managers: s.managers }),
              ],
              [t('swarm.summary.services'), s.services],
              [t('swarm.summary.created'), s.created ? formatDateTime(s.created) : null],
              [
                t('swarm.summary.id'),
                <span key="id" className="font-mono text-xs">
                  {s.id}
                </span>,
              ],
            ]}
          />
        </CardContent>
      </Card>
      <JoinCommands swarmId={swarm.id} />
    </div>
  );
}
