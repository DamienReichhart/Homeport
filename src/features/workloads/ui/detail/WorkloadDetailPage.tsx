import { useQuery } from '@tanstack/react-query';
import { useNavigate, useParams } from 'react-router-dom';
import {
  ClusterGate,
  ConditionList,
  HealthBadge,
  ObjectEvents,
  ResourceYaml,
  useResourceWatch,
} from '@/core/cluster';
import { RelatedPods } from '@/features/pods';
import type { ClusterView } from '@/shared/api';
import {
  Badge,
  ErrorMessage,
  PageHeader,
  Spinner,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import { workloadQueries } from '../../api/queries';
import { kindOf, pathOf, WORKLOAD_KINDS, workloadPath } from '../../model/kinds';
import { kindResource } from '../../model/resource';
import { WorkloadActions } from '../actions/WorkloadActions';
import { CronJobJobs } from './CronJobJobs';
import { WorkloadOverview } from './WorkloadOverview';
import { t, type MessageKey } from '@/shared/i18n';

const TAB_LABELS: Record<
  'overview' | 'pods' | 'jobs' | 'conditions' | 'events' | 'yaml',
  MessageKey
> = {
  overview: 'workloads.tab.overview',
  pods: 'workloads.tab.pods',
  jobs: 'workloads.tab.jobs',
  conditions: 'workloads.tab.conditions',
  events: 'workloads.tab.events',
  yaml: 'workloads.tab.source',
};

function WorkloadDetailView({ cluster }: { cluster: ClusterView }) {
  const { kind: segment, namespace = '', name = '', tab = 'overview' } = useParams();
  const kind = kindOf(segment);
  const navigate = useNavigate();
  const id = cluster.cluster.id;
  const detail = useQuery(workloadQueries.detail(id, kind, namespace, name));
  useResourceWatch(id, kindResource(kind), namespace);
  if (detail.isPending) return <Spinner />;
  if (detail.error)
    return <ErrorMessage error={detail.error} onRetry={() => void detail.refetch()} />;
  const label = WORKLOAD_KINDS.find((k) => k.kind === kind)?.one;
  const children = kind === 'cronJob' ? 'jobs' : 'pods';
  const tabs = ['overview', children, 'conditions', 'events', 'yaml'] as const;
  return (
    <div className="flex h-full min-h-0 flex-col">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            {name}
            {label && <Badge variant="outline">{t(label)}</Badge>}
          </span>
        }
        description={<HealthBadge workload={detail.data.view.health} />}
        actions={
          <WorkloadActions
            cluster={cluster}
            workload={detail.data.view}
            header
            onDeleted={() => void navigate(`/workloads/${pathOf(kind)}`, { replace: true })}
          />
        }
      />
      <Tabs
        value={tab}
        onValueChange={(next) =>
          void navigate(`${workloadPath(kind, namespace, name)}/${next}`, { replace: true })
        }
      >
        <TabsList>
          {tabs.map((value) => (
            <TabsTrigger key={value} value={value}>
              {t(TAB_LABELS[value])}
            </TabsTrigger>
          ))}
        </TabsList>
        <TabsContent value="overview" className="pt-3">
          <WorkloadOverview detail={detail.data} />
        </TabsContent>
        <TabsContent value="pods" className="pt-3">
          {detail.data.selector && (
            <RelatedPods clusterId={id} namespace={namespace} selector={detail.data.selector} />
          )}
        </TabsContent>
        <TabsContent value="jobs" className="pt-3">
          <CronJobJobs clusterId={id} namespace={namespace} name={name} />
        </TabsContent>
        <TabsContent value="conditions" className="pt-3">
          <ConditionList conditions={detail.data.conditions} />
        </TabsContent>
        <TabsContent value="events" className="pt-3">
          <ObjectEvents
            clusterId={id}
            kind={kindResource(kind)}
            namespace={namespace}
            name={name}
          />
        </TabsContent>
        <TabsContent value="yaml" className="pt-3">
          <ResourceYaml
            clusterId={id}
            kind={kindResource(kind)}
            namespace={namespace}
            name={name}
          />
        </TabsContent>
      </Tabs>
    </div>
  );
}

export function WorkloadDetailPage() {
  return <ClusterGate>{(cluster) => <WorkloadDetailView cluster={cluster} />}</ClusterGate>;
}
