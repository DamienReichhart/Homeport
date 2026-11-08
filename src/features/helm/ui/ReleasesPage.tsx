import { useQuery } from '@tanstack/react-query';
import { RefreshCwIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { age, ClusterGate, NamespacePicker, useNamespace } from '@/core/cluster';
import { usePageQuery } from '@/core/feature';
import type { ClusterView, ReleaseSummary } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  Button,
  DataTable,
  ErrorMessage,
  SearchInput,
  Spinner,
  type ColumnDef,
} from '@/shared/ui';
import { helmQueries } from '../api/queries';
import { releasePath } from '../model/paths';
import { HelmGate } from './HelmGate';
import { HelmPageHeader } from './HelmPageHeader';
import { ReleaseStatus } from './ReleaseStatus';
import { t } from '@/shared/i18n';

function columns(showNamespace: boolean): ColumnDef<ReleaseSummary, unknown>[] {
  return [
    {
      id: 'name',
      header: t('helm.column.name'),
      accessorFn: (r) => r.name,
      cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
    },
    ...(showNamespace
      ? [
          {
            id: 'ns',
            header: t('helm.column.namespace'),
            accessorFn: (r: ReleaseSummary) => r.namespace,
          },
        ]
      : []),
    {
      id: 'chart',
      header: t('helm.column.chart'),
      accessorFn: (r) => r.chart,
      cell: ({ row }) => (
        <span>
          {row.original.chartName}{' '}
          <span className="text-muted-foreground font-mono text-xs">
            {row.original.chartVersion}
          </span>
        </span>
      ),
    },
    { id: 'app', header: t('helm.column.appVersion'), accessorFn: (r) => r.appVersion || '—' },
    { id: 'revision', header: t('helm.column.revision'), accessorFn: (r) => r.revision },
    {
      id: 'status',
      header: t('helm.column.status'),
      accessorFn: (r) => r.status,
      cell: ({ row }) => <ReleaseStatus status={row.original.status} />,
    },
    {
      id: 'updated',
      header: t('helm.column.updated'),
      accessorFn: (r) => r.updated ?? '',
      cell: ({ row }) => age(row.original.updated),
    },
  ];
}

function Releases({ cluster }: { cluster: ClusterView }) {
  const id = cluster.cluster.id;
  const navigate = useNavigate();
  const [query, setQuery] = usePageQuery();
  const [namespace, setNamespace] = useNamespace(cluster);
  const list = useQuery(helmQueries.releases(id, namespace));
  const only = list.data?.onlyNamespace ?? null;
  return (
    <div>
      <HelmPageHeader
        current="/helm"
        description={list.data && t('helm.summary', { count: list.data.releases.length })}
        actions={<NamespacePicker clusterId={id} value={namespace} onChange={setNamespace} />}
        toolbar={
          <>
            <SearchInput value={query} onChange={setQuery} placeholder={t('helm.search')} />
            <Button
              variant="outline"
              size="sm"
              disabled={list.isFetching}
              onClick={() => void list.refetch()}
            >
              <RefreshCwIcon /> {t('helm.refresh')}
            </Button>
          </>
        }
      />
      <div className="mb-3" />
      {only && (
        <Alert className="mb-3">
          <AlertDescription>{t('helm.onlyNamespace', { namespace: only })}</AlertDescription>
        </Alert>
      )}
      {list.isPending ? (
        <Spinner />
      ) : list.error ? (
        <ErrorMessage error={list.error} onRetry={() => void list.refetch()} />
      ) : (
        <DataTable<ReleaseSummary>
          columns={columns(namespace === null && only === null)}
          data={list.data.releases}
          getRowId={(r) => `${r.namespace}/${r.name}`}
          filter={query}
          pageSize={100}
          empty={t('helm.empty')}
          onRowClick={(r) => void navigate(releasePath(r.namespace, r.name))}
        />
      )}
    </div>
  );
}

export function ReleasesPage() {
  return (
    <ClusterGate>
      {(cluster) => (
        <HelmGate>
          <Releases key={cluster.cluster.id} cluster={cluster} />
        </HelmGate>
      )}
    </ClusterGate>
  );
}
