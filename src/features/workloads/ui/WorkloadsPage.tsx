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
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Spinner,
  Tabs,
  TabsList,
  TabsTrigger,
  useSplitPane,
} from '@/shared/ui';
import { workloadQueries } from '../api/queries';
import { kindLabel, kindOf, WORKLOAD_KINDS, workloadPath } from '../model/kinds';
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
  const { compact } = useSplitPane();
  const showKind = (next: string) => {
    const path = WORKLOAD_KINDS.find((k) => k.kind === next)?.path;
    if (path) void navigate(`/workloads/${path}`, { replace: true });
  };
  return (
    <div>
      <PageHeader
        title={t('workloads.title')}
        description={workloads.data && t('workloads.summary', { count: workloads.data.length })}
        actions={<NamespacePicker clusterId={id} value={namespace} onChange={setNamespace} />}
      />
      <div className="mb-3 flex flex-wrap items-center gap-3">
        {compact ? (
          // Beside an open workload the five kinds don't fit as tabs.
          <Select value={kind} onValueChange={showKind}>
            <SelectTrigger className="w-full" aria-label={t('workloads.kindLabel')}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {WORKLOAD_KINDS.map((k) => (
                <SelectItem key={k.kind} value={k.kind}>
                  {t(k.label)}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        ) : (
          <Tabs value={kind} onValueChange={showKind}>
            <TabsList variant="default" aria-label={t('workloads.kindLabel')}>
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
        )}
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
          empty={
            namespace === null
              ? t('workloads.emptyAll', { kind: t(kindLabel(kind)) })
              : t('workloads.emptyIn', { kind: t(kindLabel(kind)), namespace })
          }
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
