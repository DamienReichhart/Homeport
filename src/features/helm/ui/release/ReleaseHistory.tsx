import type { ReactNode } from 'react';
import type { ReleaseRevision } from '@/shared/api';
import { DataTable } from '@/shared/ui';
import { ReleaseStatus } from '../ReleaseStatus';
import { t } from '@/shared/i18n';
import { formatDateTime, formatRelative } from '@/shared/lib';

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
        {
          id: 'revision',
          header: t('helm.column.revision'),
          accessorFn: (r) => r.revision,
          cell: ({ row }) => <span className="tabular-nums">{row.original.revision}</span>,
        },
        {
          id: 'updated',
          header: t('helm.column.updated'),
          accessorFn: (r) => r.updated ?? '',
          cell: ({ row }) => {
            const updated = row.original.updated;
            return updated ? (
              <span title={formatDateTime(updated)}>
                {formatRelative(Date.parse(updated) / 1000)}
              </span>
            ) : (
              '—'
            );
          },
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
          // The app version a revision ran is on hover, so the table fits beside the list.
          cell: ({ row }) => (
            <span
              className="font-mono text-xs"
              title={
                row.original.appVersion
                  ? `${t('helm.column.appVersion')} ${row.original.appVersion}`
                  : undefined
              }
            >
              {row.original.chart}
            </span>
          ),
        },
        {
          id: 'description',
          header: t('helm.column.description'),
          accessorFn: (r) => r.description,
          cell: ({ row }) => (
            <span
              className="text-muted-foreground inline-block max-w-40 truncate align-bottom"
              title={row.original.description}
            >
              {row.original.description}
            </span>
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
