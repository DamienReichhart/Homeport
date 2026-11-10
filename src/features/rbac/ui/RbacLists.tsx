import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { age } from '@/core/cluster';
import type { BindingView, RbacIndex, RoleView, SubjectEntry } from '@/shared/api';
import { Badge, DataTable, SearchInput, type ColumnDef } from '@/shared/ui';
import { boundRolePath, rolePath, subjectName, subjectPath } from '../model/paths';
import { t } from '@/shared/i18n';

const scopeOf = (namespace: string | null) => namespace ?? t('rbac.clusterWide');

const roleColumns: ColumnDef<RoleView, unknown>[] = [
  {
    id: 'name',
    header: t('rbac.column.name'),
    accessorFn: (r) => r.name,
    cell: ({ row }) => (
      <span className="flex items-center gap-2">
        <span className="font-medium">{row.original.name}</span>
        {row.original.aggregation.length > 0 && (
          <Badge variant="outline">{t('rbac.aggregated')}</Badge>
        )}
      </span>
    ),
  },
  { id: 'kind', header: t('rbac.column.kind'), accessorFn: (r) => r.kind },
  { id: 'namespace', header: t('rbac.column.namespace'), accessorFn: (r) => scopeOf(r.namespace) },
  { id: 'rules', header: t('rbac.column.rules'), accessorFn: (r) => r.rules.length },
  {
    id: 'age',
    header: t('rbac.column.age'),
    accessorFn: (r) => r.created ?? '',
    cell: ({ row }) => age(row.original.created),
  },
];

/** A binding's role, linked, flagged when it wasn't found. */
export function BoundRole({ binding }: { binding: BindingView }) {
  return (
    <span className="flex items-center gap-2">
      <Link className="text-primary" to={boundRolePath(binding)}>
        {binding.roleKind}/{binding.roleName}
      </Link>
      {!binding.roleFound && <Badge variant="destructive">{t('rbac.roleMissing')}</Badge>}
    </span>
  );
}

const bindingColumns: ColumnDef<BindingView, unknown>[] = [
  { id: 'name', header: t('rbac.column.name'), accessorFn: (b) => b.name },
  { id: 'kind', header: t('rbac.column.kind'), accessorFn: (b) => b.kind },
  { id: 'namespace', header: t('rbac.column.namespace'), accessorFn: (b) => scopeOf(b.namespace) },
  {
    id: 'role',
    header: t('rbac.column.role'),
    accessorFn: (b) => `${b.roleKind}/${b.roleName}`,
    cell: ({ row }) => <BoundRole binding={row.original} />,
  },
  {
    id: 'subjects',
    header: t('rbac.column.subjects'),
    accessorFn: (b) => b.subjects.map(subjectName).join(' '),
    cell: ({ row }) => (
      <span className="flex flex-wrap gap-x-3 gap-y-1">
        {row.original.subjects.map((s) => (
          <Link key={subjectPath(s)} className="text-primary" to={subjectPath(s)}>
            {s.kind}/{subjectName(s)}
          </Link>
        ))}
      </span>
    ),
  },
  {
    id: 'age',
    header: t('rbac.column.age'),
    accessorFn: (b) => b.created ?? '',
    cell: ({ row }) => age(row.original.created),
  },
];

const subjectColumns: ColumnDef<SubjectEntry, unknown>[] = [
  {
    id: 'name',
    header: t('rbac.column.name'),
    accessorFn: (s) => s.subject.name,
    cell: ({ row }) => <span className="font-medium">{row.original.subject.name}</span>,
  },
  { id: 'kind', header: t('rbac.column.kind'), accessorFn: (s) => s.subject.kind },
  {
    id: 'namespace',
    header: t('rbac.column.namespace'),
    accessorFn: (s) => s.subject.namespace ?? '',
  },
  { id: 'bindings', header: t('rbac.column.bindings'), accessorFn: (s) => s.bindings },
];

function Searchable<T>({
  rows,
  columns,
  getRowId,
  search,
  open,
}: {
  rows: T[];
  columns: ColumnDef<T, unknown>[];
  getRowId: (row: T) => string;
  search: string;
  open?: (row: T) => string;
}) {
  const navigate = useNavigate();
  const [query, setQuery] = useState('');
  return (
    <div className="grid gap-3">
      <SearchInput value={query} onChange={setQuery} placeholder={search} />
      <DataTable<T>
        columns={columns}
        data={rows}
        getRowId={getRowId}
        filter={query}
        pageSize={100}
        empty={t('rbac.empty')}
        onRowClick={open && ((row) => void navigate(open(row)))}
      />
    </div>
  );
}

const roleId = (r: RoleView) => `${r.kind}/${r.namespace ?? ''}/${r.name}`;

export function RolesList({ index }: { index: RbacIndex }) {
  return (
    <Searchable
      rows={index.roles}
      columns={roleColumns}
      getRowId={roleId}
      search={t('rbac.search.roles')}
      open={rolePath}
    />
  );
}

/** Bindings, each linking to its role and its subjects. */
export function BindingsList({ bindings }: { bindings: BindingView[] }) {
  return (
    <Searchable
      rows={bindings}
      columns={bindingColumns}
      getRowId={(b) => `${b.kind}/${b.namespace ?? ''}/${b.name}`}
      search={t('rbac.search.bindings')}
    />
  );
}

export function SubjectsList({ index }: { index: RbacIndex }) {
  return (
    <Searchable
      rows={index.subjects}
      columns={subjectColumns}
      getRowId={(s) => subjectPath(s.subject)}
      search={t('rbac.search.subjects')}
      open={(s) => subjectPath(s.subject)}
    />
  );
}
