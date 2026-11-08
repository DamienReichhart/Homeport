import type { UseQueryResult } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import { useNavigate } from 'react-router-dom';
import { usePageQuery } from '@/core/feature';
import type { ClusterView, ResourceKind } from '@/shared/api';
import {
  DataTable,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  Tabs,
  TabsList,
  TabsTrigger,
  type ColumnDef,
} from '@/shared/ui';
import { useResourceWatch } from '../live/useResourceWatch';
import { NamespacePicker } from './NamespacePicker';
import { t } from '@/shared/i18n';

/** Pages of one area, such as Services and Ingresses, as tabs that open them. */
export interface Section {
  path: string;
  label: string;
}

/**
 * A Cluster page listing one kind of resource: a Namespace picker (unless
 * the kind isn't namespaced), a search, the list kept fresh by a watch, and
 * a clear error when the credentials may not list it.
 */
export function ResourceList<T>({
  cluster,
  kind,
  title,
  summary,
  list,
  namespace,
  onNamespace,
  columns,
  getRowId,
  rowPath,
  search,
  sections,
  current,
  toolbar,
  empty,
}: {
  cluster: ClusterView;
  kind: ResourceKind;
  title: string;
  summary: (count: number) => string;
  list: UseQueryResult<T[]>;
  /** `undefined` for a kind that isn't in a Namespace. */
  namespace?: string | null;
  onNamespace?: (namespace: string | null) => void;
  columns: ColumnDef<T, unknown>[];
  getRowId: (row: T) => string;
  rowPath?: (row: T) => string;
  search: string;
  sections?: Section[];
  current?: string;
  toolbar?: ReactNode;
  empty?: string;
}) {
  const navigate = useNavigate();
  const [query, setQuery] = usePageQuery();
  const id = cluster.cluster.id;
  useResourceWatch(id, kind, namespace ?? null);
  return (
    <div>
      <PageHeader
        title={title}
        description={list.data && summary(list.data.length)}
        actions={
          onNamespace && (
            <NamespacePicker clusterId={id} value={namespace ?? null} onChange={onNamespace} />
          )
        }
      />
      <div className="mb-3 flex flex-wrap items-center gap-3">
        {sections && (
          <Tabs value={current} onValueChange={(path) => void navigate(path, { replace: true })}>
            <TabsList>
              {sections.map((s) => (
                <TabsTrigger key={s.path} value={s.path}>
                  {s.label}
                </TabsTrigger>
              ))}
            </TabsList>
          </Tabs>
        )}
        <SearchInput value={query} onChange={setQuery} placeholder={search} />
        {toolbar}
      </div>
      {list.isPending ? (
        <Spinner />
      ) : list.error ? (
        <ErrorMessage error={list.error} onRetry={() => void list.refetch()} />
      ) : (
        <DataTable<T>
          columns={columns}
          data={list.data ?? []}
          getRowId={getRowId}
          filter={query}
          pageSize={100}
          empty={empty ?? t('core.list.empty')}
          onRowClick={rowPath && ((row) => void navigate(rowPath(row)))}
        />
      )}
    </div>
  );
}
