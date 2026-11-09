import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon } from 'lucide-react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { age, ClusterGate } from '@/core/cluster';
import { allocationTone, NomadStatus, nomadKeys, shortId, useNomadWatch } from '@/core/nomad';
import type { ClusterView, NomadAllocationDetail } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  ErrorMessage,
  KeyValue,
  PageHeader,
  Spinner,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import { nomadJobQueries } from '../../api/queries';
import { jobPath } from '../../model/jobs';
import { AllocationLogs } from './AllocationLogs';
import { AllocationShell } from './AllocationShell';
import { t } from '@/shared/i18n';

const TABS = ['overview', 'logs', 'shell'] as const;
type Tab = (typeof TABS)[number];

function Overview({ detail }: { detail: NomadAllocationDetail }) {
  const a = detail.allocation;
  return (
    <div className="grid gap-4">
      <Card>
        <CardHeader>
          <CardTitle>{t('nomad.alloc.title')}</CardTitle>
        </CardHeader>
        <CardContent>
          <KeyValue
            items={[
              [
                t('nomad.alloc.job'),
                <Link className="underline" to={jobPath(a.namespace, a.jobId)}>
                  {a.jobId}
                </Link>,
              ],
              [t('nomad.alloc.group'), a.taskGroup],
              [t('nomad.alloc.node'), a.nodeName || shortId(a.nodeId)],
              [t('nomad.alloc.version'), a.jobVersion],
              [t('nomad.alloc.desired'), a.desiredStatus],
              [t('nomad.alloc.description'), detail.clientDescription],
              [t('nomad.alloc.created'), age(a.created)],
            ]}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>{t('nomad.alloc.tasks')}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-2">
          {a.tasks.map((task) => (
            <div key={task.name} className="flex flex-wrap items-center gap-3 text-sm">
              <span className="font-medium">{task.name}</span>
              <NomadStatus
                status={task.state}
                tone={task.failed ? 'danger' : allocationTone(task.state)}
              />
              <span className="text-muted-foreground">
                {t('nomad.alloc.restarts', { count: task.restarts })}
              </span>
            </div>
          ))}
        </CardContent>
      </Card>
      {detail.events.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>{t('nomad.alloc.events')}</CardTitle>
          </CardHeader>
          <CardContent>
            <ul className="grid gap-1 text-sm">
              {detail.events.map((e, i) => (
                <li key={i} className="flex gap-3">
                  <span className="text-muted-foreground w-12 shrink-0">{age(e.time)}</span>
                  <span className="w-24 shrink-0">{e.task}</span>
                  <span className="font-medium">{e.kind}</span>
                  <span className="text-muted-foreground">{e.message}</span>
                </li>
              ))}
            </ul>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

function AllocationLoader({ cluster }: { cluster: ClusterView }) {
  const { allocId = '', tab } = useParams();
  const navigate = useNavigate();
  const current: Tab = TABS.find((x) => x === tab) ?? 'overview';
  const id = cluster.cluster.id;
  const detail = useQuery(nomadJobQueries.allocation(id, allocId));
  useNomadWatch(id, { topic: 'allocation', allocId }, nomadKeys.allocation(id, allocId));
  if (detail.isPending) return <Spinner />;
  if (detail.error)
    return <ErrorMessage error={detail.error} onRetry={() => void detail.refetch()} />;
  const a = detail.data.allocation;
  const tasks = a.tasks.map((task) => task.name);
  const running = a.tasks.filter((task) => task.state === 'running').map((task) => task.name);
  return (
    <div className="grid gap-4">
      <PageHeader
        title={
          <span className="flex items-center gap-3">
            {a.name}
            <NomadStatus status={a.clientStatus} tone={allocationTone(a.clientStatus)} />
          </span>
        }
        description={t('nomad.alloc.headline', { id: shortId(a.id), node: a.nodeName })}
        actions={
          <Button asChild variant="ghost">
            <Link to={jobPath(a.namespace, a.jobId)}>
              <ArrowLeftIcon className="rtl:-scale-x-100" /> {a.jobId}
            </Link>
          </Button>
        }
      />
      <Tabs
        value={current}
        onValueChange={(next) =>
          void navigate(`/nomad/allocations/${allocId}/${next}`, { replace: true })
        }
      >
        <TabsList>
          <TabsTrigger value="overview">{t('nomad.alloc.tab.overview')}</TabsTrigger>
          <TabsTrigger value="logs">{t('nomad.alloc.tab.logs')}</TabsTrigger>
          <TabsTrigger value="shell">{t('nomad.alloc.tab.shell')}</TabsTrigger>
        </TabsList>
        <TabsContent value="overview" className="pt-3">
          <Overview detail={detail.data} />
        </TabsContent>
        <TabsContent value="logs" className="pt-3">
          {current === 'logs' && <AllocationLogs clusterId={id} allocId={a.id} tasks={tasks} />}
        </TabsContent>
        <TabsContent value="shell" className="pt-3">
          {current === 'shell' && (
            <AllocationShell clusterId={id} allocId={a.id} tasks={tasks} running={running} />
          )}
        </TabsContent>
      </Tabs>
    </div>
  );
}

/** An allocation: its tasks and their events, their logs, and a shell in one. */
export function AllocationPage() {
  return (
    <ClusterGate>
      {(cluster) => <AllocationLoader key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
