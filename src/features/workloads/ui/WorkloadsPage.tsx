import { useQuery } from '@tanstack/react-query';
import { useNavigate, useParams } from 'react-router-dom';
import { ClusterGate, NamespacePicker, useNamespace, useResourceWatch } from '@/core/cluster';
import { usePageQuery } from '@/core/feature';
import type { ClusterView } from '@/shared/api';
import {
  DataTable,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  Tabs,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import { workloadQueries } from '../api/queries';
import { kindOf, WORKLOAD_KINDS, workloadPath } from '../model/kinds';
import { kindResource } from '../model/resource';
import { workloadColumns } from './workloadColumns';
import { t } from '@/shared/i18n';

function WorkloadsList({ cluster }: { cluster: ClusterView }) {
  const { kind: segment } = useParams();
  const kind = kindOf(segment);
  const navigate = useNavigate();
  const id = cluster.cluster.id;
  const [namespace, setNamespace] = useNamespace(cluster);
  const [query, setQuery] = usePageQuery();
  const workloads = useQuery(workloadQueries.list(id, kind, namespace));
  useResourceWatch(id, kindResource(kind), namespace);
  return (
    <div>
      <PageHeader
        title={t('workloads.title')}
        description={workloads.data && t('workloads.summary', { count: workloads.data.length })}
        actions={<NamespacePicker clusterId={id} value={namespace} onChange={setNamespace} />}
      />
      <div className="mb-3 flex flex-wrap items-center gap-3">
        <Tabs
          value={kind}
          onValueChange={(next) => {
            const path = WORKLOAD_KINDS.find((k) => k.kind === next)?.path;
            if (path) void navigate(`/workloads/${path}`, { replace: true });
          }}
        >
          <TabsList>
            {WORKLOAD_KINDS.map((k) => (
              <TabsTrigger key={k.kind} value={k.kind}>
                {t(k.label)}
              </TabsTrigger>
            ))}
          </TabsList>
        </Tabs>
        <SearchInput value={query} onChange={setQuery} placeholder={t('workloads.search')} />
      </div>
      {workloads.isPending ? (
        <Spinner />
      ) : workloads.error ? (
        <ErrorMessage error={workloads.error} onRetry={() => void workloads.refetch()} />
      ) : (
        <DataTable
          columns={workloadColumns(kind, namespace === null)}
          data={workloads.data}
          getRowId={(w) => `${w.namespace}/${w.name}`}
          filter={query}
          pageSize={100}
          empty={t('workloads.empty')}
          onRowClick={(w) => void navigate(workloadPath(w.kind, w.namespace, w.name))}
        />
      )}
    </div>
  );
}

export function WorkloadsPage() {
  return (
    <ClusterGate>
      {(cluster) => <WorkloadsList key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
