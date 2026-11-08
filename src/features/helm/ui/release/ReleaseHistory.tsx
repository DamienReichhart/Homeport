import type { ReactNode } from 'react';
import type { ReleaseRevision } from '@/shared/api';
import { DataTable } from '@/shared/ui';
import { ReleaseStatus } from '../ReleaseStatus';
import { t } from '@/shared/i18n';

/** Every revision of a release, newest first, each with what `action` offers. */
export function ReleaseHistory({
  history,
  action,
}: {
  history: ReleaseRevision[];
  action?: (revision: ReleaseRevision) => ReactNode;
}) {
  const newestFirst = [...history].reverse();
  return (
    <DataTable<ReleaseRevision>
      columns={[
        { id: 'revision', header: t('helm.column.revision'), accessorFn: (r) => r.revision },
        {
          id: 'updated',
          header: t('helm.column.updated'),
          accessorFn: (r) => r.updated ?? '',
          cell: ({ row }) =>
            row.original.updated ? new Date(row.original.updated).toLocaleString() : '—',
        },
        {
          id: 'status',
          header: t('helm.column.status'),
          accessorFn: (r) => r.status,
          cell: ({ row }) => <ReleaseStatus status={row.original.status} />,
        },
        {
          id: 'chart',
          header: t('helm.column.chart'),
          accessorFn: (r) => r.chart,
          cell: ({ row }) => <span className="font-mono text-xs">{row.original.chart}</span>,
        },
        {
          id: 'app',
          header: t('helm.column.appVersion'),
          accessorFn: (r) => r.appVersion || '—',
        },
        {
          id: 'description',
          header: t('helm.column.description'),
          accessorFn: (r) => r.description,
          cell: ({ row }) => (
            <span className="line-clamp-2 max-w-md text-xs">{row.original.description}</span>
          ),
        },
        ...(action
          ? [
              {
                id: 'action',
                header: '',
                enableSorting: false,
                cell: ({ row }: { row: { original: ReleaseRevision } }) => action(row.original),
              },
            ]
          : []),
      ]}
      data={newestFirst}
      getRowId={(r) => String(r.revision)}
    />
  );
}
