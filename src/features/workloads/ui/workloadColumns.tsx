import { age, HealthBadge } from '@/core/cluster';
import type { ClusterView, WorkloadKind, WorkloadView } from '@/shared/api';
import { type ColumnDef } from '@/shared/ui';
import { readyText } from '../model/kinds';
import { WorkloadActions } from './actions/WorkloadActions';
import { t } from '@/shared/i18n';

/** The workloads table's columns for `kind`. */
export function workloadColumns(
  kind: WorkloadKind,
  showNamespace: boolean,
  /** Adds each row's actions menu. */
  cluster?: ClusterView,
): ColumnDef<WorkloadView, unknown>[] {
  const columns: ColumnDef<WorkloadView, unknown>[] = [
    {
      id: 'name',
      header: t('workloads.column.name'),
      accessorFn: (w) => w.name,
      cell: ({ row }) => (
        <span
          className="inline-block max-w-72 truncate align-bottom font-medium"
          title={row.original.name}
        >
          {row.original.name}
        </span>
      ),
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
      // Beside an open workload, the list still shows which ones need a look.
      meta: { compact: true },
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
      cell: ({ getValue }) => <span className="tabular-nums">{getValue<string>()}</span>,
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
      cell: ({ getValue }) => <span className="tabular-nums">{getValue<number>()}</span>,
    });
  }
  columns.push(
    {
      id: 'images',
      header: t('workloads.column.images'),
      accessorFn: (w) => w.images.join(', '),
      cell: ({ row }) => {
        const images = row.original.images.join(', ');
        return (
          <span
            className="inline-block max-w-80 truncate align-bottom font-mono text-xs"
            title={images}
          >
            {images}
          </span>
        );
      },
    },
    {
      id: 'age',
      header: t('workloads.column.age'),
      accessorFn: (w) => w.created ?? '',
      cell: ({ row }) => age(row.original.created),
    },
  );
  if (cluster)
    columns.push({
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => <WorkloadActions cluster={cluster} workload={row.original} />,
    });
  return columns;
}
