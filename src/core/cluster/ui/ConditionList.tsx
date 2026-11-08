import type { ConditionView } from '@/shared/api';
import { DataTable, StatusDot, type ColumnDef } from '@/shared/ui';
import { age } from '../model/health';
import { t } from '@/shared/i18n';

const columns: ColumnDef<ConditionView, unknown>[] = [
  {
    id: 'type',
    header: t('core.conditions.type'),
    accessorFn: (c) => c.type,
    cell: ({ row }) => (
      <span className="inline-flex items-center gap-1.5 font-medium">
        <StatusDot
          tone={
            row.original.status === 'True'
              ? 'success'
              : row.original.status === 'False'
                ? 'danger'
                : 'muted'
          }
        />
        {row.original.type}
      </span>
    ),
  },
  { id: 'status', header: t('core.conditions.status'), accessorFn: (c) => c.status },
  { id: 'reason', header: t('core.conditions.reason'), accessorFn: (c) => c.reason ?? '' },
  {
    id: 'message',
    header: t('core.conditions.message'),
    accessorFn: (c) => c.message ?? '',
    cell: ({ row }) => <span className="text-sm break-words">{row.original.message}</span>,
  },
  {
    id: 'since',
    header: t('core.conditions.since'),
    accessorFn: (c) => c.lastTransition ?? '',
    cell: ({ row }) => age(row.original.lastTransition),
  },
];

/** A resource's status conditions. */
export function ConditionList({ conditions }: { conditions: ConditionView[] }) {
  return (
    <DataTable
      columns={columns}
      data={conditions}
      getRowId={(c) => c.type}
      empty={t('core.conditions.none')}
    />
  );
}
