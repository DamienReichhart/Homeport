import type { PodmanPod } from '@/shared/api';
import { formatRelative } from '@/shared/lib';
import { Badge, StatusDot, type ColumnDef, type ConfirmedAction } from '@/shared/ui';
import { createdSeconds, members, podStatusLabel, podTone, runningMembers } from '../model/pods';
import { PodActions } from './PodActions';
import { t } from '@/shared/i18n';

/** The Pods table's columns; the last one holds the row's actions. */
export function podColumns({
  hostId,
  confirm,
}: {
  hostId: string;
  confirm: (request: ConfirmedAction) => void;
}): ColumnDef<PodmanPod, unknown>[] {
  return [
    {
      id: 'name',
      header: t('podmanpods.column.name'),
      accessorFn: (p) => p.name,
      cell: ({ row }) => (
        <span className="flex items-center gap-2 font-medium">
          <StatusDot tone={podTone(row.original.status)} />
          {row.original.name}
        </span>
      ),
    },
    {
      id: 'status',
      header: t('podmanpods.column.status'),
      accessorFn: (p) => p.status,
      cell: ({ row }) => t(podStatusLabel(row.original.status)),
    },
    {
      id: 'containers',
      header: t('podmanpods.column.containers'),
      accessorFn: (p) => members(p).length,
      cell: ({ row }) =>
        t('podmanpods.running', {
          running: runningMembers(row.original),
          count: members(row.original).length,
        }),
    },
    {
      id: 'networks',
      header: t('podmanpods.column.networks'),
      accessorFn: (p) => p.networks.join(', '),
      cell: ({ row }) => (
        <span className="flex flex-wrap gap-1">
          {row.original.networks.map((n) => (
            <Badge key={n} variant="outline">
              {n}
            </Badge>
          ))}
        </span>
      ),
    },
    {
      id: 'created',
      header: t('podmanpods.column.created'),
      accessorFn: createdSeconds,
      cell: ({ row }) => (
        <span className="text-muted-foreground whitespace-nowrap">
          {formatRelative(createdSeconds(row.original))}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <PodActions hostId={hostId} pod={row.original} compact confirm={confirm} />
        </div>
      ),
    },
  ];
}
