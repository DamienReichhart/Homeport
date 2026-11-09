import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import {
  ClusterGate,
  DeleteSelected,
  NamespacePicker,
  stillListed,
  useNamespace,
  useResourceWatch,
} from '@/core/cluster';
import { usePageQuery } from '@/core/feature';
import type { ClusterView, WorkloadView } from '@/shared/api';
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
  const [selected, setSelected] = useState<WorkloadView[]>([]);
  const rowId = (w: WorkloadView) => `${w.kind}/${w.namespace}/${w.name}`;
  const picked = stillListed(selected, workloads.data, rowId);
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
              <TabsTrigger
                key={k.kind}
                value={k.kind}
                // A tab that switches what one view shows, without a panel of its own.
                aria-controls={undefined}
              >
                {t(k.label)}
              </TabsTrigger>
            ))}
          </TabsList>
        </Tabs>
        <SearchInput value={query} onChange={setQuery} placeholder={t('workloads.search')} />
        <DeleteSelected
          cluster={cluster}
          targets={picked.map((w) => ({
            type: { kind: w.kind },
            namespace: w.namespace,
            name: w.name,
          }))}
        />
      </div>
      {workloads.isPending ? (
        <Spinner />
      ) : workloads.error ? (
        <ErrorMessage error={workloads.error} onRetry={() => void workloads.refetch()} />
      ) : (
        <DataTable
          key={kind}
          columns={workloadColumns(kind, namespace === null, cluster)}
          data={workloads.data}
          getRowId={rowId}
          onSelectionChange={setSelected}
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
