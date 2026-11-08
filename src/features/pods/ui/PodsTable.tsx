import { useNavigate } from 'react-router-dom';
import { age, HealthBadge } from '@/core/cluster';
import type { PodView } from '@/shared/api';
import { DataTable, type ColumnDef } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Rows past this show a page at a time. */
const PAGE = 100;

function columns(showNamespace: boolean): ColumnDef<PodView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('pods.column.name'),
      accessorFn: (p) => p.name,
      cell: ({ row }) => <span className="font-medium break-all">{row.original.name}</span>,
    },
    ...(showNamespace
      ? [
          {
            id: 'namespace',
            header: t('pods.column.namespace'),
            accessorFn: (p: PodView) => p.namespace,
          },
        ]
      : []),
    {
      id: 'status',
      header: t('pods.column.status'),
      accessorFn: (p) => p.status,
      cell: ({ row }) => <HealthBadge pod={row.original.health} status={row.original.status} />,
    },
    {
      id: 'ready',
      header: t('pods.column.ready'),
      accessorFn: (p) => `${p.ready}/${p.containers}`,
    },
    { id: 'restarts', header: t('pods.column.restarts'), accessorFn: (p) => p.restarts },
    { id: 'node', header: t('pods.column.node'), accessorFn: (p) => p.node ?? '' },
    {
      id: 'age',
      header: t('pods.column.age'),
      accessorFn: (p) => p.created ?? '',
      cell: ({ row }) => age(row.original.created),
    },
  ];
}

/** Pods, each opening its page; long lists show a page at a time. */
export function PodsTable({
  pods,
  filter = '',
  showNamespace,
}: {
  pods: PodView[];
  filter?: string;
  showNamespace: boolean;
}) {
  const navigate = useNavigate();
  return (
    <DataTable
      columns={columns(showNamespace)}
      data={pods}
      getRowId={(p) => `${p.namespace}/${p.name}`}
      filter={filter}
      pageSize={PAGE}
      empty={t('pods.empty')}
      onRowClick={(p) =>
        void navigate(`/pods/${encodeURIComponent(p.namespace)}/${encodeURIComponent(p.name)}`)
      }
    />
  );
}
