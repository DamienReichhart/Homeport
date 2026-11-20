import { useQuery } from '@tanstack/react-query';
import {
  ArrowLeftIcon,
  EllipsisVerticalIcon,
  PencilIcon,
  ScaleIcon,
  Trash2Icon,
  Undo2Icon,
} from 'lucide-react';
import { useMemo, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { LogViewer } from '@/features/containers';
import { SwarmGate } from '@/core/swarm';
import type { SwarmView } from '@/shared/api';
import {
  Badge,
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  ErrorMessage,
  JsonView,
  PageHeader,
  Spinner,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import { swarmFeatureQueries } from '../../api/queries';
import { serviceLogs } from '../../api/streams';
import { canScale, updateTone } from '../../model/services';
import { TasksTab } from '../tasks/TasksTab';
import { EditServiceDialog } from './EditServiceDialog';
import { ReplicasBadge } from './ReplicasBadge';
import { UpdateState } from './UpdateState';
import { useServiceActions } from './useServiceActions';
import { t, type MessageKey } from '@/shared/i18n';

const TABS = ['tasks', 'logs', 'inspect'] as const;
const TAB_LABELS: Record<(typeof TABS)[number], MessageKey> = {
  tasks: 'swarm.service.tab.tasks',
  logs: 'swarm.service.tab.logs',
  inspect: 'swarm.service.tab.inspect',
};

function ServiceDetail({ swarm, id }: { swarm: SwarmView; id: string }) {
  const { tab = 'tasks' } = useParams();
  const navigate = useNavigate();
  const back = () => void navigate(`/swarms/${swarm.id}/services`);
  const services = useQuery(swarmFeatureQueries.services(swarm.id));
  const inspect = useQuery(swarmFeatureQueries.serviceInspect(swarm.id, id));
  const [dialogs, actions] = useServiceActions(swarm.id, back);
  const stream = useMemo(() => serviceLogs(swarm.id, id), [swarm.id, id]);
  const service = services.data?.find((s) => s.id === id);
  const [editing, setEditing] = useState(false);

  if (services.isPending) return <Spinner />;
  if (!service)
    return (
      <div className="grid gap-3">
        <ErrorMessage error={{ code: 'not_found', message: t('swarm.service.gone') }} />
        <Button variant="outline" className="justify-self-start" onClick={back}>
          {t('swarm.service.backToList')}
        </Button>
      </div>
    );
  return (
    <div className="flex h-full min-h-0 flex-col">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            <Button variant="ghost" size="icon-sm" aria-label={t('common.back')} onClick={back}>
              <ArrowLeftIcon className="rtl:-scale-x-100" />
            </Button>
            {service.name}
            <ReplicasBadge service={service} />
            {service.stack && <Badge variant="outline">{service.stack}</Badge>}
            {updateTone(service.updateState) && <UpdateState service={service} />}
          </span>
        }
        description={
          <span
            className="block truncate font-mono text-xs"
            title={service.updateMessage ?? undefined}
          >
            {service.image}
          </span>
        }
        actions={
          <>
            <Button onClick={() => setEditing(true)}>
              <PencilIcon /> {t('swarm.edit.open')}
            </Button>
            {canScale(service) && (
              <Button variant="outline" onClick={() => actions.onScale(service)}>
                <ScaleIcon /> {t('swarm.services.action.scale')}
              </Button>
            )}
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline" size="icon" aria-label={t('common.moreActions')}>
                  <EllipsisVerticalIcon />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem onSelect={() => actions.onRollback(service)}>
                  <Undo2Icon /> {t('swarm.services.action.rollback')}
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem variant="destructive" onSelect={() => actions.onRemove(service)}>
                  <Trash2Icon /> {t('swarm.services.action.remove')}
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </>
        }
      />
      <Tabs
        value={tab}
        onValueChange={(next) =>
          void navigate(`/swarms/${swarm.id}/services/${id}/${next}`, { replace: true })
        }
        className="flex min-h-0 flex-1 flex-col"
      >
        <TabsList>
          {TABS.map((value) => (
            <TabsTrigger key={value} value={value}>
              {t(TAB_LABELS[value])}
            </TabsTrigger>
          ))}
        </TabsList>
        <TabsContent value="tasks" className="overflow-auto">
          <TasksTab swarmId={swarm.id} serviceId={id} />
        </TabsContent>
        <TabsContent value="logs" className="min-h-[24rem] flex-1">
          <LogViewer stream={stream} streamKey={`${swarm.id}/${id}`} />
        </TabsContent>
        <TabsContent value="inspect" className="overflow-auto">
          {inspect.data ? (
            <JsonView value={inspect.data} />
          ) : inspect.error ? (
            <ErrorMessage error={inspect.error} onRetry={() => void inspect.refetch()} />
          ) : (
            <Spinner />
          )}
        </TabsContent>
      </Tabs>
      {dialogs}
      {editing && (
        <EditServiceDialog swarmId={swarm.id} service={service} onClose={() => setEditing(false)} />
      )}
    </div>
  );
}

export function ServiceDetailPage() {
  const { swarmId = '', serviceId = '' } = useParams();
  return (
    <SwarmGate key={swarmId} swarmId={swarmId}>
      {(swarm) => <ServiceDetail key={serviceId} swarm={swarm} id={serviceId} />}
    </SwarmGate>
  );
}
