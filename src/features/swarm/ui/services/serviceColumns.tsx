import type { ServiceView } from '@/shared/api';
import { Badge, type ColumnDef } from '@/shared/ui';
import { portsLabel } from '../../model/services';
import { ReplicasBadge } from './ReplicasBadge';
import { ServiceMenu, type ServiceActions } from './ServiceMenu';
import { t, type MessageKey } from '@/shared/i18n';

const MODES: Record<ServiceView['mode'], MessageKey> = {
  replicated: 'swarm.services.mode.replicated',
  global: 'swarm.services.mode.global',
  replicatedJob: 'swarm.services.mode.replicatedJob',
  globalJob: 'swarm.services.mode.globalJob',
};

/** The Services table's columns; the last one holds the row's menu. */
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
        <span className="font-mono text-xs" title={row.original.imageDigest ?? undefined}>
          {row.original.image}
        </span>
      ),
    },
    {
      id: 'mode',
      header: t('swarm.services.column.mode'),
      accessorFn: (s) => s.mode,
      cell: ({ row }) =>
        row.original.mode === 'replicated'
          ? t('swarm.services.mode.replicatedCount', { count: row.original.replicas ?? 0 })
          : t(MODES[row.original.mode]),
    },
    {
      id: 'replicas',
      header: t('swarm.services.column.replicas'),
      accessorFn: (s) => s.running,
      cell: ({ row }) => <ReplicasBadge service={row.original} />,
    },
    {
      id: 'ports',
      header: t('swarm.services.column.ports'),
      accessorFn: portsLabel,
      cell: ({ row }) => (
        <span className="font-mono text-xs">{portsLabel(row.original) || '—'}</span>
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
      cell: ({ row }) => (
        <span className="text-muted-foreground text-xs" title={row.original.updateMessage ?? ''}>
          {row.original.updateState ?? '—'}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <ServiceMenu service={row.original} {...actions} />
        </div>
      ),
    },
  ];
}
