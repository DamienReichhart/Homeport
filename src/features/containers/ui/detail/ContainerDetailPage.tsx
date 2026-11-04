import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon } from 'lucide-react';
import { useNavigate, useParams } from 'react-router-dom';
import {
  containerFreshness,
  HostGate,
  indexFreshness,
  NewerImage,
  newerLabel,
  resourceQueries,
  useContainers,
} from '@/core/host';
import {
  Badge,
  Button,
  ErrorMessage,
  PageHeader,
  Spinner,
  StatusDot,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  useConfirm,
} from '@/shared/ui';
import { containerQueries } from '../../api/queries';
import { containerTone } from '../../model/containerTone';
import { LogViewer } from '../logs/LogViewer';
import { ContainerActions } from './ContainerActions';
import { ExecTab } from './tabs/ExecTab';
import { FilesTab } from './tabs/FilesTab';
import { InspectTab } from './tabs/InspectTab';
import { ProcessesTab } from './tabs/ProcessesTab';
import { StatsTab } from './tabs/StatsTab';

const TABS = ['logs', 'inspect', 'exec', 'files', 'stats', 'processes'] as const;
const TAB_LABELS: Record<(typeof TABS)[number], string> = {
  logs: 'Logs',
  inspect: 'Inspect',
  exec: 'Exec',
  files: 'Files',
  stats: 'Stats',
  processes: 'Processes',
};

function ContainerDetail({ hostId, id }: { hostId: string; id: string }) {
  const { tab = 'logs' } = useParams();
  const navigate = useNavigate();
  const containers = useContainers(hostId);
  const [dialog, confirm] = useConfirm();
  const freshness = indexFreshness(useQuery(resourceQueries.freshness(hostId)).data);
  const container = containers.data?.find((c) => c.id === id || c.id.startsWith(id));
  const inspect = useQuery({
    ...containerQueries.inspect(hostId, id, container?.state),
    enabled: !!container,
  });

  if (containers.isPending) return <Spinner />;
  if (!container) {
    return (
      <div className="grid gap-3">
        <ErrorMessage error={{ code: 'not_found', message: 'This container no longer exists.' }} />
        <Button
          variant="outline"
          className="justify-self-start"
          onClick={() => navigate('/containers')}
        >
          Back to containers
        </Button>
      </div>
    );
  }
  const running = container.state === 'running';

  return (
    <div className="flex h-full min-h-0 flex-col">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label="Back"
              onClick={() => navigate('/containers')}
            >
              <ArrowLeftIcon />
            </Button>
            <StatusDot tone={containerTone(container.state, container.health)} />
            {container.name}
            {container.stack && <Badge variant="outline">{container.stack}</Badge>}
          </span>
        }
        description={
          <span className="inline-flex items-center gap-1.5">
            {container.image}
            {containerFreshness(freshness, container) === 'newer' && (
              <NewerImage label={newerLabel([container.image])} />
            )}
            <span>· {container.status || container.state}</span>
          </span>
        }
        actions={<ContainerActions hostId={hostId} container={container} confirm={confirm} />}
      />
      <Tabs
        value={tab}
        onValueChange={(t) => navigate(`/containers/${container.id}/${t}`, { replace: true })}
        className="flex min-h-0 flex-1 flex-col"
      >
        <TabsList>
          {TABS.map((t) => (
            <TabsTrigger key={t} value={t}>
              {TAB_LABELS[t]}
            </TabsTrigger>
          ))}
        </TabsList>
        <TabsContent value="logs" className="min-h-[24rem] flex-1">
          <LogViewer hostId={hostId} sources={[{ containerId: container.id, label: null }]} />
        </TabsContent>
        <TabsContent value="inspect" className="overflow-auto">
          {inspect.data ? (
            <InspectTab inspect={inspect.data} />
          ) : inspect.error ? (
            <ErrorMessage error={inspect.error} onRetry={() => void inspect.refetch()} />
          ) : (
            <Spinner />
          )}
        </TabsContent>
        <TabsContent value="exec" className="min-h-[24rem] flex-1">
          <ExecTab hostId={hostId} containerId={container.id} running={running} />
        </TabsContent>
        <TabsContent value="files" className="overflow-auto">
          <FilesTab hostId={hostId} containerId={container.id} running={running} />
        </TabsContent>
        <TabsContent value="stats" className="overflow-auto">
          <StatsTab hostId={hostId} containerId={container.id} running={running} />
        </TabsContent>
        <TabsContent value="processes" className="overflow-auto">
          <ProcessesTab hostId={hostId} containerId={container.id} running={running} />
        </TabsContent>
      </Tabs>
      {dialog}
    </div>
  );
}

export function ContainerDetailPage() {
  const { id = '' } = useParams();
  return (
    <HostGate>
      {(hostId) => <ContainerDetail key={`${hostId}/${id}`} hostId={hostId} id={id} />}
    </HostGate>
  );
}
