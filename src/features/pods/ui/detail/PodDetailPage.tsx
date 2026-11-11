import { useQuery } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import {
  age,
  ClusterGate,
  ConditionList,
  DeleteObjectButton,
  HealthBadge,
  Labels,
  ObjectEvents,
  ResourceYaml,
  useMetrics,
  useResourceWatch,
} from '@/core/cluster';
import type { ClusterView, OwnerView, PodDetail } from '@/shared/api';
import {
  Card,
  CardContent,
  ErrorMessage,
  KeyValue,
  PageHeader,
  Spinner,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import { podQueries } from '../../api/queries';
import { PodContainers } from './PodContainers';
import { PodLogs } from './PodLogs';
import { PodMetrics } from './PodMetrics';
import { PodTerminal } from './PodTerminal';
import { t, type MessageKey } from '@/shared/i18n';

/** The page of a Pod's owner, when Homeport shows that kind. */
function ownerLink(namespace: string, owner: OwnerView): string | null {
  const kinds: Partial<Record<string, string>> = {
    StatefulSet: 'statefulsets',
    DaemonSet: 'daemonsets',
    Job: 'jobs',
    CronJob: 'cronjobs',
  };
  const kind = kinds[owner.kind];
  return kind
    ? `/workloads/${kind}/${encodeURIComponent(namespace)}/${encodeURIComponent(owner.name)}`
    : null;
}

function Overview({ clusterId, pod }: { clusterId: string; pod: PodDetail }) {
  const v = pod.view;
  const link = v.owner && ownerLink(v.namespace, v.owner);
  const items: [string, ReactNode][] = [
    [t('pods.detail.namespace'), v.namespace],
    [
      t('pods.detail.node'),
      v.node ? <Link to={`/nodes/${encodeURIComponent(v.node)}`}>{v.node}</Link> : null,
    ],
    [t('pods.detail.ip'), v.ip],
    [t('pods.detail.hostIp'), pod.hostIp],
    [t('pods.detail.qos'), pod.qosClass],
    [t('pods.detail.serviceAccount'), pod.serviceAccount],
    [
      t('pods.detail.owner'),
      v.owner &&
        (link ? (
          <Link to={link}>{`${v.owner.kind} ${v.owner.name}`}</Link>
        ) : (
          `${v.owner.kind} ${v.owner.name}`
        )),
    ],
    [t('pods.detail.age'), age(v.created)],
    [t('pods.detail.labels'), <Labels values={pod.labels} />],
    [t('pods.detail.annotations'), <Labels values={pod.annotations} />],
  ];
  return (
    <div className="grid gap-4">
      <Card>
        <CardContent>
          <KeyValue items={items} />
        </CardContent>
      </Card>
      <PodContainers
        clusterId={clusterId}
        namespace={v.namespace}
        pod={v.name}
        containers={pod.containers}
      />
    </div>
  );
}

const TABS = ['overview', 'logs', 'terminal', 'metrics', 'conditions', 'events', 'yaml'] as const;
const TAB_LABELS: Record<(typeof TABS)[number], MessageKey> = {
  overview: 'pods.tab.overview',
  logs: 'pods.tab.logs',
  terminal: 'pods.tab.terminal',
  metrics: 'pods.tab.metrics',
  conditions: 'pods.tab.conditions',
  events: 'pods.tab.events',
  yaml: 'pods.tab.source',
};

function PodDetailView({ cluster }: { cluster: ClusterView }) {
  const { namespace = '', name = '', tab = 'overview' } = useParams();
  const navigate = useNavigate();
  const id = cluster.cluster.id;
  const pod = useQuery(podQueries.detail(id, namespace, name));
  useResourceWatch(id, 'pod', namespace);
  // Read while the page is open, whichever tab shows, so the charts have
  // the samples since it opened.
  const metrics = useMetrics(id, { type: 'pod', namespace, name });
  if (pod.isPending) return <Spinner />;
  if (pod.error) return <ErrorMessage error={pod.error} onRetry={() => void pod.refetch()} />;
  const base = `/pods/${encodeURIComponent(namespace)}/${encodeURIComponent(name)}`;
  return (
    <div className="flex h-full min-h-0 flex-col">
      <PageHeader
        title={name}
        description={
          <span className="flex items-center gap-2">
            <HealthBadge pod={pod.data.view.health} status={pod.data.view.status} />
            <span className="text-muted-foreground">
              ·{' '}
              {t('pods.detail.readyOf', {
                ready: pod.data.view.ready,
                total: pod.data.view.containers,
              })}
            </span>
          </span>
        }
        actions={
          <DeleteObjectButton
            cluster={cluster}
            target={{
              type: { kind: 'pod' },
              namespace,
              name,
              owner: pod.data.view.owner,
            }}
            onDeleted={() => void navigate('/pods', { replace: true })}
          />
        }
      />
      <Tabs
        value={tab}
        onValueChange={(next) => void navigate(`${base}/${next}`, { replace: true })}
      >
        <TabsList>
          {TABS.map((value) => (
            <TabsTrigger key={value} value={value}>
              {t(TAB_LABELS[value])}
            </TabsTrigger>
          ))}
        </TabsList>
        <TabsContent value="overview" className="pt-3">
          <Overview clusterId={id} pod={pod.data} />
        </TabsContent>
        <TabsContent value="logs" className="pt-3">
          {tab === 'logs' && <PodLogs clusterId={id} pod={pod.data} />}
        </TabsContent>
        <TabsContent value="terminal" className="pt-3">
          {tab === 'terminal' && <PodTerminal clusterId={id} pod={pod.data} />}
        </TabsContent>
        <TabsContent value="metrics" className="pt-3">
          {tab === 'metrics' && <PodMetrics pod={pod.data} metrics={metrics} />}
        </TabsContent>
        <TabsContent value="conditions" className="pt-3">
          <ConditionList conditions={pod.data.conditions} />
        </TabsContent>
        <TabsContent value="events" className="pt-3">
          <ObjectEvents clusterId={id} kind="pod" namespace={namespace} name={name} />
        </TabsContent>
        <TabsContent value="yaml" className="pt-3">
          <ResourceYaml clusterId={id} kind="pod" namespace={namespace} name={name} />
        </TabsContent>
      </Tabs>
    </div>
  );
}

export function PodDetailPage() {
  return <ClusterGate>{(cluster) => <PodDetailView cluster={cluster} />}</ClusterGate>;
}
