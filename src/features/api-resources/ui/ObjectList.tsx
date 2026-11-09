import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { age, clusterKeys, DeleteSelected, NamespacePicker, stillListed } from '@/core/cluster';
import { usePageQuery } from '@/core/feature';
import type { ApiResourceInfo, ClusterView, TableRow } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  DataTable,
  ErrorMessage,
  SearchInput,
  Spinner,
  type ColumnDef,
} from '@/shared/ui';
import { apiQueries } from '../api/queries';
import { useApiWatch } from '../api/useApiWatch';
import { notListable, objectPagePath, qualifiedName } from '../model/types';
import { t } from '@/shared/i18n';

const rowId = (r: TableRow) => `${r.namespace ?? ''}/${r.name}`;

/** The objects of one type, with the columns `kubectl get` shows, kept live by a watch. */
export function ObjectList({
  cluster,
  info,
  namespace,
  onNamespace,
}: {
  cluster: ClusterView;
  info: ApiResourceInfo;
  namespace: string | null;
  onNamespace: (namespace: string | null) => void;
}) {
  const id = cluster.cluster.id;
  const navigate = useNavigate();
  const [query, setQuery] = usePageQuery();
  const api = info.api;
  const ns = api.namespaced ? namespace : null;
  const blocked = notListable(info);
  const list = useQuery({ ...apiQueries.list(id, api, ns), enabled: blocked !== 'verb' });
  useApiWatch(id, blocked ? undefined : api, ns, clusterKeys.api(id, api));
  const [selected, setSelected] = useState<TableRow[]>([]);
  const picked = stillListed(selected, list.data?.rows, rowId);
  const canDelete = info.verbs.includes('delete');

  const header = (
    <div className="flex flex-wrap items-center gap-3">
      <h2 className="font-medium">
        {api.kind}{' '}
        <span className="text-muted-foreground font-mono text-xs">{qualifiedName(api)}</span>
      </h2>
      <SearchInput
        value={query}
        onChange={setQuery}
        placeholder={t('apiresources.searchObjects')}
      />
      {canDelete && (
        <DeleteSelected
          cluster={cluster}
          targets={picked.map((r) => ({ type: { api }, namespace: r.namespace, name: r.name }))}
        />
      )}
      {api.namespaced && (
        <div className="ms-auto">
          <NamespacePicker clusterId={id} value={namespace} onChange={onNamespace} />
        </div>
      )}
    </div>
  );

  if (blocked === 'verb')
    return (
      <div className="grid gap-3">
        {header}
        <Alert>
          <AlertDescription>{t('apiresources.cantListWhy', { kind: api.kind })}</AlertDescription>
        </Alert>
      </div>
    );

  const columns: ColumnDef<TableRow, unknown>[] = [
    {
      id: 'name',
      header: t('apiresources.column.name'),
      accessorFn: (r) => r.name,
      cell: ({ row }) => <span className="font-medium break-all">{row.original.name}</span>,
    },
    ...(api.namespaced && namespace === null
      ? [
          {
            id: 'namespace',
            header: t('apiresources.column.namespace'),
            accessorFn: (r: TableRow) => r.namespace ?? '',
          },
        ]
      : []),
    ...(list.data?.columns ?? [])
      .map((c, i) => ({ c, i }))
      .filter(({ c }) => c.priority === 0 && c.name !== 'Age')
      .map(({ c, i }) => ({
        id: `c${i}`,
        header: c.name,
        accessorFn: (r: TableRow) => r.cells[i] ?? '',
      })),
    {
      id: 'age',
      header: t('apiresources.column.age'),
      accessorFn: (r) => r.created ?? '',
      cell: ({ row }) => age(row.original.created),
    },
  ];

  return (
    <div className="grid gap-3">
      {header}
      {list.isPending ? (
        <Spinner />
      ) : list.error ? (
        <ErrorMessage error={list.error} onRetry={() => void list.refetch()} />
      ) : (
        <DataTable
          key={`${api.group}/${api.plural}`}
          columns={columns}
          data={list.data.rows}
          getRowId={rowId}
          filter={query}
          pageSize={100}
          empty={t('apiresources.empty')}
          onRowClick={(r) => void navigate(objectPagePath(api, r.namespace, r.name))}
          onSelectionChange={canDelete ? setSelected : undefined}
        />
      )}
    </div>
  );
}
