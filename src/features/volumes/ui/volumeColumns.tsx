import { InfoIcon, Trash2Icon } from 'lucide-react';
import type { VolumeView } from '@/shared/api';
import { Badge, Button, type ColumnDef } from '@/shared/ui';

/** The volumes table's columns; the last one holds the row's actions. */
export function volumeColumns({
  onInspect,
  onRemove,
}: {
  onInspect: (volume: VolumeView) => void;
  onRemove: (volume: VolumeView) => void;
}): ColumnDef<VolumeView, unknown>[] {
  return [
    {
      id: 'name',
      header: 'Name',
      accessorFn: (v) => v.name,
      cell: ({ row }) => <span className="font-medium break-all">{row.original.name}</span>,
    },
    {
      id: 'stack',
      header: 'Stack',
      accessorFn: (v) => v.stack ?? '',
      cell: ({ row }) =>
        row.original.stack && <Badge variant="outline">{row.original.stack}</Badge>,
    },
    { id: 'driver', header: 'Driver', accessorFn: (v) => v.driver },
    {
      id: 'usedBy',
      header: 'Used by',
      accessorFn: (v) => v.usedBy.join(' '),
      cell: ({ row }) =>
        row.original.usedBy.length ? (
          row.original.usedBy.join(', ')
        ) : (
          <Badge variant="outline">Unused</Badge>
        ),
    },
    {
      id: 'created',
      header: 'Created',
      accessorFn: (v) => v.createdAt ?? '',
      cell: ({ row }) =>
        row.original.createdAt ? new Date(row.original.createdAt).toLocaleDateString() : '—',
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={`Inspect ${row.original.name}`}
            onClick={() => onInspect(row.original)}
          >
            <InfoIcon />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={`Delete ${row.original.name}`}
            onClick={() => onRemove(row.original)}
          >
            <Trash2Icon />
          </Button>
        </div>
      ),
    },
  ];
}
