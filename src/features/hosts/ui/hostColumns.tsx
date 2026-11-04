import { connectionLabel, describeAddress, HostStatusBadge } from '@/core/host';
import type { HostView } from '@/shared/api';
import { Badge, type ColumnDef } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** The Hosts table's columns. */
export const hostColumns = (): ColumnDef<HostView, unknown>[] => [
  {
    id: 'name',
    header: t('hosts.column.name'),
    accessorFn: (v) => v.host.name,
    cell: ({ row }) => <span className="font-medium">{row.original.host.name}</span>,
  },
  {
    id: 'kind',
    header: t('hosts.column.type'),
    accessorFn: (v) => v.host.connection.kind,
    cell: ({ row }) => (
      <Badge variant="outline">{connectionLabel(row.original.host.connection.kind)}</Badge>
    ),
  },
  {
    id: 'address',
    header: t('hosts.column.address'),
    accessorFn: (v) => describeAddress(v.host.connection),
    cell: ({ getValue }) => <span className="font-mono text-xs">{String(getValue())}</span>,
  },
  {
    id: 'status',
    header: t('hosts.column.status'),
    accessorFn: (v) => v.status.state,
    cell: ({ row }) => <HostStatusBadge status={row.original.status} />,
  },
];
