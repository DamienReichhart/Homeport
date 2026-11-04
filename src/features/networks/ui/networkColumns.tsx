import { InfoIcon, Trash2Icon } from 'lucide-react';
import type { NetworkView } from '@/shared/api';
import { Badge, Button, type ColumnDef } from '@/shared/ui';

/** The networks table's columns; the last one holds the row's actions. */
export function networkColumns({
  onInspect,
  onRemove,
}: {
  onInspect: (network: NetworkView) => void;
  onRemove: (network: NetworkView) => void;
}): ColumnDef<NetworkView, unknown>[] {
  return [
    {
      id: 'name',
      header: 'Name',
      accessorFn: (n) => n.name,
      cell: ({ row }) => (
        <span className="flex items-center gap-2 font-medium">
          {row.original.name}
          {row.original.builtIn && <Badge variant="secondary">built-in</Badge>}
          {row.original.internal && <Badge variant="outline">internal</Badge>}
        </span>
      ),
    },
    {
      id: 'stack',
      header: 'Stack',
      accessorFn: (n) => n.stack ?? '',
      cell: ({ row }) =>
        row.original.stack && <Badge variant="outline">{row.original.stack}</Badge>,
    },
    { id: 'driver', header: 'Driver', accessorFn: (n) => n.driver },
    {
      id: 'subnet',
      header: 'Subnet',
      accessorFn: (n) => n.subnets.join(', '),
      cell: ({ row }) => (
        <span className="font-mono text-xs">{row.original.subnets.join(', ') || '—'}</span>
      ),
    },
    { id: 'containers', header: 'Containers', accessorFn: (n) => n.containers.length },
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
          {!row.original.builtIn && (
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={`Remove ${row.original.name}`}
              onClick={() => onRemove(row.original)}
            >
              <Trash2Icon />
            </Button>
          )}
        </div>
      ),
    },
  ];
}
