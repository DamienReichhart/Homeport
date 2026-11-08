import { useNavigate, useParams } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { shortSwarmId, SwarmGate, swarmLabel } from '@/core/swarm';
import type { SwarmView } from '@/shared/api';
import { Badge, PageHeader, Tabs, TabsContent, TabsList, TabsTrigger } from '@/shared/ui';
import { NodesTab } from '../nodes/NodesTab';
import { ServicesTab } from '../services/ServicesTab';
import { TasksTab } from '../tasks/TasksTab';
import { SummaryTab } from './SummaryTab';
import { t, type MessageKey } from '@/shared/i18n';

const TABS = ['summary', 'nodes', 'services', 'tasks'] as const;
type Tab = (typeof TABS)[number];
const TAB_LABELS: Record<Tab, MessageKey> = {
  summary: 'swarm.tab.summary',
  nodes: 'swarm.tab.nodes',
  services: 'swarm.tab.services',
  tasks: 'swarm.tab.tasks',
};

function SwarmDetail({ swarm }: { swarm: SwarmView }) {
  const { tab = 'summary' } = useParams();
  const navigate = useNavigate();
  const hosts = useHosts();
  return (
    <div className="flex h-full min-h-0 flex-col">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            {swarmLabel(swarm, hosts.data)}
            <Badge variant="outline">{t('swarm.badge')}</Badge>
          </span>
        }
        description={<span className="font-mono text-xs">{shortSwarmId(swarm.id)}</span>}
      />
      <Tabs
        value={tab}
        onValueChange={(next) => void navigate(`/swarms/${swarm.id}/${next}`, { replace: true })}
        className="flex min-h-0 flex-1 flex-col"
      >
        <TabsList>
          {TABS.map((value) => (
            <TabsTrigger key={value} value={value}>
              {t(TAB_LABELS[value])}
            </TabsTrigger>
          ))}
        </TabsList>
        <TabsContent value="summary" className="overflow-auto">
          <SummaryTab swarm={swarm} />
        </TabsContent>
        <TabsContent value="nodes" className="overflow-auto">
          <NodesTab
            swarm={swarm}
            onShowTasks={(node) =>
              void navigate(`/swarms/${swarm.id}/tasks?node=${encodeURIComponent(node.id)}`)
            }
          />
        </TabsContent>
        <TabsContent value="services" className="overflow-auto">
          <ServicesTab swarm={swarm} />
        </TabsContent>
        <TabsContent value="tasks" className="overflow-auto">
          <TasksTab swarmId={swarm.id} />
        </TabsContent>
      </Tabs>
    </div>
  );
}

export function SwarmPage() {
  const { swarmId = '' } = useParams();
  return (
    <SwarmGate key={swarmId} swarmId={swarmId}>
      {(swarm) => <SwarmDetail swarm={swarm} />}
    </SwarmGate>
  );
}
