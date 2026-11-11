import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import {
  ClusterGate,
  DeleteSelected,
  NamespacePicker,
  stillListed,
  useMetrics,
  useNamespace,
  useResourceWatch,
} from '@/core/cluster';
import { usePageQuery } from '@/core/feature';
import type { ClusterView, PodView } from '@/shared/api';
import { ErrorMessage, PageHeader, SearchInput, Spinner } from '@/shared/ui';
import { podQueries } from '../api/queries';
import { PodsTable } from './PodsTable';
import { t } from '@/shared/i18n';

function PodsList({ cluster }: { cluster: ClusterView }) {
  const id = cluster.cluster.id;
  const [namespace, setNamespace] = useNamespace(cluster);
  const [query, setQuery] = usePageQuery();
  const pods = useQuery(podQueries.list(id, namespace));
  useResourceWatch(id, 'pod', namespace);
  // Without metrics-server there are no CPU and memory columns, and nothing to say.
  const metrics = useMetrics(id, { type: 'pods', namespace });
  const [selected, setSelected] = useState<PodView[]>([]);
  const picked = stillListed(selected, pods.data, (p) => `${p.namespace}/${p.name}`);
  return (
    <div>
      <PageHeader
        title={t('pods.title')}
        description={pods.data && t('pods.summary', { count: pods.data.length })}
        actions={<NamespacePicker clusterId={id} value={namespace} onChange={setNamespace} />}
      />
      <div className="mb-3 flex flex-wrap items-center gap-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('pods.search')} />
        <DeleteSelected
          cluster={cluster}
          targets={picked.map((p) => ({
            type: { kind: 'pod' },
            namespace: p.namespace,
            name: p.name,
            owner: p.owner,
          }))}
        />
      </div>
      {pods.isPending ? (
        <Spinner />
      ) : pods.error ? (
        <ErrorMessage error={pods.error} onRetry={() => void pods.refetch()} />
      ) : (
        <PodsTable
          pods={pods.data}
          filter={query}
          showNamespace={namespace === null}
          usage={metrics.pods}
          onSelectionChange={setSelected}
        />
      )}
    </div>
  );
}

export function PodsPage() {
  return (
    <ClusterGate>
      {(cluster) => <PodsList key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
