import type { ServiceView } from '@/shared/api';
import { Badge, type ColumnDef } from '@/shared/ui';
import { portsLabel } from '../../model/services';
import { ReplicasBadge } from './ReplicasBadge';
import { ServiceMenu, type ServiceActions } from './ServiceMenu';
import { UpdateState } from './UpdateState';
import { t, type MessageKey } from '@/shared/i18n';

const MODES: Record<ServiceView['mode'], MessageKey> = {
  replicated: 'swarm.services.mode.replicated',
  global: 'swarm.services.mode.global',
  replicatedJob: 'swarm.services.mode.replicatedJob',
  globalJob: 'swarm.services.mode.globalJob',
};

/** The Services table's columns; the last one holds the row's actions. */
export function serviceColumns(actions: ServiceActions): ColumnDef<ServiceView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('swarm.services.column.name'),
      accessorFn: (s) => s.name,
      cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
    },
    {
      id: 'image',
      header: t('swarm.services.column.image'),
      accessorFn: (s) => s.image,
      cell: ({ row }) => (
        <span
          className="block max-w-56 truncate font-mono text-xs"
          title={row.original.imageDigest ?? row.original.image}
        >
          {row.original.image}
        </span>
      ),
    },
    {
      id: 'replicas',
      header: t('swarm.services.column.replicas'),
      accessorFn: (s) => s.running,
      meta: { compact: true },
      // Replicated is the norm; a global service or a job says so beside its count.
      cell: ({ row }) => (
        <span className="flex items-center gap-1.5 whitespace-nowrap">
          <ReplicasBadge service={row.original} />
          {row.original.mode !== 'replicated' && (
            <span className="text-muted-foreground text-xs">{t(MODES[row.original.mode])}</span>
          )}
        </span>
      ),
    },
    {
      id: 'ports',
      header: t('swarm.services.column.ports'),
      accessorFn: portsLabel,
      cell: ({ row }) => (
        <span
          className="block max-w-40 truncate font-mono text-xs"
          title={portsLabel(row.original) || undefined}
        >
          {portsLabel(row.original) || '—'}
        </span>
      ),
    },
    {
      id: 'stack',
      header: t('swarm.services.column.stack'),
      accessorFn: (s) => s.stack ?? '',
      cell: ({ row }) =>
        row.original.stack && <Badge variant="outline">{row.original.stack}</Badge>,
    },
    {
      id: 'update',
      header: t('swarm.services.column.update'),
      accessorFn: (s) => s.updateState ?? '',
      cell: ({ row }) => <UpdateState service={row.original} />,
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => <ServiceMenu service={row.original} {...actions} />,
    },
  ];
}
