import { connectionLabel, describeAddress, describeStatus } from '@/core/host';
import type { HostView } from '@/shared/api';
import { StatusDot, type ColumnDef } from '@/shared/ui';
import { HostState } from './HostState';
import { t } from '@/shared/i18n';

/**
 * The Hosts table's columns. The status is a shape beside the name, so it
 * stays in a split view's narrow list; the Status column names it only when
 * something is wrong, and otherwise shows what the Host runs.
 */
export const hostColumns = (): ColumnDef<HostView, unknown>[] => [
  {
    id: 'name',
    header: t('hosts.column.name'),
    accessorFn: (v) => v.host.name,
    cell: ({ row }) => {
      const { label, tone } = describeStatus(row.original.status);
      return (
        <span className="flex min-w-0 items-center gap-2">
          <span title={label} className="inline-flex">
            <StatusDot tone={tone} pulse={row.original.status.state === 'connecting'} />
          </span>
          <span className="truncate font-medium">{row.original.host.name}</span>
        </span>
      );
    },
  },
  {
    id: 'kind',
    header: t('hosts.column.type'),
    accessorFn: (v) => connectionLabel(v.host.connection.kind),
    cell: ({ getValue }) => (
      <span className="text-muted-foreground whitespace-nowrap">{String(getValue())}</span>
    ),
  },
  {
    id: 'address',
    header: t('hosts.column.address'),
    accessorFn: (v) => describeAddress(v.host.connection),
    cell: ({ getValue }) => (
      <span className="block max-w-[22rem] truncate font-mono text-xs" title={String(getValue())}>
        {String(getValue())}
      </span>
    ),
  },
  {
    id: 'status',
    header: t('hosts.column.status'),
    accessorFn: (v) => v.status.state,
    cell: ({ row }) => <HostState view={row.original} />,
  },
];
