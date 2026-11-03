import { useQuery, useQueryClient } from '@tanstack/react-query';
import {
  ArrowLeftIcon,
  PauseIcon,
  PlayIcon,
  RotateCwIcon,
  SquareIcon,
  Trash2Icon,
} from 'lucide-react';
import { useNavigate, useParams } from 'react-router-dom';
import { HostGate, useContainers } from '@/core/host';
import { containerTone } from '../model/containerTone';
import { useContainerMutations } from '../api';
import { commands, keys } from '@/shared/api';
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
import type { ContainerInspect } from '../types';
import { removeConfirmation } from './ContainerMenu';
import { ExecTab } from './ExecTab';
import { FilesTab } from './FilesTab';
import { InspectTab } from './InspectTab';
import { LogViewer } from './LogViewer';
import { ProcessesTab } from './ProcessesTab';
import { StatsTab } from './StatsTab';

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
  const client = useQueryClient();
  const containers = useContainers(hostId);
  const { act, remove } = useContainerMutations(hostId);
  const [dialog, confirm] = useConfirm();
  const container = containers.data?.find((c) => c.id === id || c.id.startsWith(id));
  const inspect = useQuery({
    queryKey: [...keys.container(hostId, id), 'inspect', container?.state],
    queryFn: () =>
      commands.containersInspect(hostId, id) as Promise<unknown> as Promise<ContainerInspect>,
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
  const paused = container.state === 'paused';
  const ids = [container.id];

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
        description={`${container.image} · ${container.status || container.state}`}
        actions={
          <>
            {running || paused ? (
              <Button variant="outline" onClick={() => act.mutate({ ids, action: 'stop' })}>
                <SquareIcon /> Stop
              </Button>
            ) : (
              <Button variant="outline" onClick={() => act.mutate({ ids, action: 'start' })}>
                <PlayIcon /> Start
              </Button>
            )}
            <Button
              variant="outline"
              disabled={!running && !paused}
              onClick={() => act.mutate({ ids, action: 'restart' })}
            >
              <RotateCwIcon /> Restart
            </Button>
            <Button
              variant="outline"
              disabled={!running && !paused}
              onClick={() => act.mutate({ ids, action: paused ? 'unpause' : 'pause' })}
            >
              {paused ? <PlayIcon /> : <PauseIcon />} {paused ? 'Resume' : 'Pause'}
            </Button>
            <Button
              variant="destructive"
              onClick={() =>
                confirm(
                  removeConfirmation([container], async () => {
                    await remove.mutateAsync({ ids, volumes: true });
                    await client.invalidateQueries({ queryKey: keys.containers(hostId) });
                    navigate('/containers');
                  }),
                )
              }
            >
              <Trash2Icon /> Remove
            </Button>
          </>
        }
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
