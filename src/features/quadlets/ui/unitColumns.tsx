import type { QuadletUnit, QuadletUnits } from '@/shared/api';
import { Badge, StatusDot, type ColumnDef } from '@/shared/ui';
import { kindOf, unitStatusLabel, unitTone } from '../model/units';
import { UnitActions } from './UnitActions';
import { t } from '@/shared/i18n';

/** The Quadlet table's columns; the last one holds the row's action. */
export function unitColumns({
  hostId,
  units,
}: {
  hostId: string;
  units: QuadletUnits;
}): ColumnDef<QuadletUnit, unknown>[] {
  return [
    {
      id: 'name',
      header: t('quadlets.column.name'),
      accessorFn: (u) => u.name ?? u.unitName ?? '',
      cell: ({ row }) => (
        <span className="flex items-center gap-2 font-medium">
          <StatusDot tone={unitTone(row.original.status)} />
          <span className="truncate font-mono">{row.original.name ?? row.original.unitName}</span>
          {row.original.app && <Badge variant="outline">{row.original.app}</Badge>}
        </span>
      ),
    },
    {
      id: 'kind',
      header: t('quadlets.column.kind'),
      accessorFn: kindOf,
      cell: ({ row }) => (
        <span className="text-muted-foreground font-mono text-xs">{kindOf(row.original)}</span>
      ),
    },
    {
      id: 'service',
      header: t('quadlets.column.service'),
      accessorFn: (u) => u.unitName ?? '',
      cell: ({ row }) => (
        <span
          className="text-muted-foreground block max-w-64 truncate font-mono text-xs"
          title={row.original.unitName ?? undefined}
        >
          {row.original.unitName ?? t('quadlets.notGenerated')}
        </span>
      ),
    },
    {
      id: 'status',
      header: t('quadlets.column.status'),
      accessorFn: (u) => u.status,
      meta: { compact: true },
      cell: ({ row }) => t(unitStatusLabel(row.original.status)),
    },
    {
      id: 'containers',
      header: t('quadlets.column.containers'),
      accessorFn: (u) => u.containers.length,
      cell: ({ row }) => (
        <div className="text-end tabular-nums">{row.original.containers.length}</div>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <UnitActions hostId={hostId} units={units} unit={row.original} compact />
        </div>
      ),
    },
  ];
}
