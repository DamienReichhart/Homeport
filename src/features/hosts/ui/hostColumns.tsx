import { connectionLabel, describeAddress, HostStatusBadge } from '@/core/host';
import type { HostView } from '@/shared/api';
import { Badge, type ColumnDef } from '@/shared/ui';

/** The Hosts table's columns. */
export const hostColumns: ColumnDef<HostView, unknown>[] = [
  {
    id: 'name',
    header: 'Name',
    accessorFn: (v) => v.host.name,
    cell: ({ row }) => <span className="font-medium">{row.original.host.name}</span>,
  },
  {
    id: 'kind',
    header: 'Type',
    accessorFn: (v) => v.host.connection.kind,
    cell: ({ row }) => (
      <Badge variant="outline">{connectionLabel(row.original.host.connection.kind)}</Badge>
    ),
  },
  {
    id: 'address',
    header: 'Address',
    accessorFn: (v) => describeAddress(v.host.connection),
    cell: ({ getValue }) => <span className="font-mono text-xs">{String(getValue())}</span>,
  },
  {
    id: 'status',
    header: 'Status',
    accessorFn: (v) => v.status.state,
    cell: ({ row }) => <HostStatusBadge status={row.original.status} />,
  },
];
