import { age, HealthBadge } from '@/core/cluster';
import type { WorkloadKind, WorkloadView } from '@/shared/api';
import { type ColumnDef } from '@/shared/ui';
import { readyText } from '../model/kinds';
import { t } from '@/shared/i18n';

/** The workloads table's columns for `kind`. */
export function workloadColumns(
  kind: WorkloadKind,
  showNamespace: boolean,
): ColumnDef<WorkloadView, unknown>[] {
  const columns: ColumnDef<WorkloadView, unknown>[] = [
    {
      id: 'name',
      header: t('workloads.column.name'),
      accessorFn: (w) => w.name,
      cell: ({ row }) => <span className="font-medium break-all">{row.original.name}</span>,
    },
  ];
  if (showNamespace)
    columns.push({
      id: 'namespace',
      header: t('workloads.column.namespace'),
      accessorFn: (w) => w.namespace,
    });
  columns.push(
    {
      id: 'health',
      header: t('workloads.column.health'),
      accessorFn: (w) => w.health,
      cell: ({ row }) => <HealthBadge workload={row.original.health} />,
    },
    {
      id: 'ready',
      header:
        kind === 'cronJob'
          ? t('workloads.column.active')
          : kind === 'job'
            ? t('workloads.column.completions')
            : t('workloads.column.ready'),
      accessorFn: (w) => readyText(w.ready, w.desired, kind),
    },
  );
  if (kind === 'cronJob') {
    columns.push(
      {
        id: 'schedule',
        header: t('workloads.column.schedule'),
        accessorFn: (w) => w.schedule ?? '',
        cell: ({ row }) => <span className="font-mono text-xs">{row.original.schedule}</span>,
      },
      {
        id: 'last',
        header: t('workloads.column.lastSchedule'),
        accessorFn: (w) => w.lastSchedule ?? '',
        cell: ({ row }) => age(row.original.lastSchedule),
      },
    );
  } else if (kind !== 'job') {
    columns.push({
      id: 'upToDate',
      header: t('workloads.column.upToDate'),
      accessorFn: (w) => w.upToDate ?? 0,
    });
  }
  columns.push(
    {
      id: 'images',
      header: t('workloads.column.images'),
      accessorFn: (w) => w.images.join(', '),
      cell: ({ row }) => (
        <span className="font-mono text-xs break-all">{row.original.images.join(', ')}</span>
      ),
    },
    {
      id: 'age',
      header: t('workloads.column.age'),
      accessorFn: (w) => w.created ?? '',
      cell: ({ row }) => age(row.original.created),
    },
  );
  return columns;
}
