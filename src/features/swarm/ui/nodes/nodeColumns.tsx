import { Link } from 'react-router-dom';
import type { HostView, NodeChange, SwarmNode } from '@/shared/api';
import { Badge, StatusDot, type ColumnDef } from '@/shared/ui';
import { NodeMenu } from './NodeMenu';
import { t, type MessageKey } from '@/shared/i18n';

const AVAILABILITY: Record<SwarmNode['availability'], MessageKey> = {
  active: 'swarm.nodes.availability.active',
  pause: 'swarm.nodes.availability.pause',
  drain: 'swarm.nodes.availability.drain',
};

const MANAGER_STATUS: Record<NonNullable<SwarmNode['managerStatus']>, MessageKey> = {
  leader: 'swarm.nodes.manager.leader',
  reachable: 'swarm.nodes.manager.reachable',
  unreachable: 'swarm.nodes.manager.unreachable',
};

/** The Nodes table's columns; the last one holds the row's menu. */
export function nodeColumns({
  hosts,
  onChange,
  onEditLabels,
  onShowTasks,
  onRemove,
}: {
  hosts: HostView[] | undefined;
  onChange: (node: SwarmNode, change: NodeChange) => void;
  onEditLabels: (node: SwarmNode) => void;
  onShowTasks?: (node: SwarmNode) => void;
  onRemove: (node: SwarmNode) => void;
}): ColumnDef<SwarmNode, unknown>[] {
  return [
    {
      id: 'hostname',
      header: t('swarm.nodes.column.hostname'),
      accessorFn: (n) => n.hostname,
      cell: ({ row }) => {
        const node = row.original;
        const host = hosts?.find((h) => h.host.id === node.hostId);
        return (
          <span className="flex flex-wrap items-center gap-2 font-medium">
            <StatusDot tone={node.state === 'ready' ? 'success' : 'danger'} />
            {node.hostname}
            {host && (
              <Link
                to={`/hosts/${host.host.id}`}
                className="text-primary text-xs font-normal hover:underline"
                onClick={(e) => e.stopPropagation()}
              >
                {t('swarm.nodes.isHost', { name: host.host.name })}
              </Link>
            )}
          </span>
        );
      },
    },
    {
      id: 'role',
      header: t('swarm.nodes.column.role'),
      accessorFn: (n) => n.role,
      cell: ({ row }) => (
        <span className="flex items-center gap-2">
          {row.original.role === 'manager' ? t('swarm.role.manager') : t('swarm.role.worker')}
          {row.original.managerStatus && (
            <Badge
              variant={row.original.managerStatus === 'unreachable' ? 'destructive' : 'outline'}
            >
              {t(MANAGER_STATUS[row.original.managerStatus])}
            </Badge>
          )}
        </span>
      ),
    },
    {
      id: 'availability',
      header: t('swarm.nodes.column.availability'),
      accessorFn: (n) => n.availability,
      cell: ({ row }) => t(AVAILABILITY[row.original.availability]),
    },
    {
      id: 'state',
      header: t('swarm.nodes.column.state'),
      accessorFn: (n) => n.state,
      cell: ({ row }) => (
        <span title={row.original.message ?? undefined}>{row.original.state}</span>
      ),
    },
    {
      id: 'engine',
      header: t('swarm.nodes.column.engine'),
      accessorFn: (n) => n.engineVersion ?? '',
    },
    {
      id: 'address',
      header: t('swarm.nodes.column.address'),
      accessorFn: (n) => n.address ?? '',
      cell: ({ row }) => <span className="font-mono text-xs">{row.original.address ?? '—'}</span>,
    },
    {
      id: 'labels',
      header: t('swarm.nodes.column.labels'),
      accessorFn: (n) =>
        Object.entries(n.labels)
          .map(([k, v]) => `${k}=${v ?? ''}`)
          .join(' '),
      cell: ({ row }) => (
        <span className="flex flex-wrap gap-1">
          {Object.entries(row.original.labels).map(([k, v]) => (
            <Badge key={k} variant="secondary" className="font-mono text-xs">
              {k}={v}
            </Badge>
          ))}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <NodeMenu
            node={row.original}
            onChange={(change) => onChange(row.original, change)}
            onEditLabels={() => onEditLabels(row.original)}
            onShowTasks={onShowTasks && (() => onShowTasks(row.original))}
            onRemove={() => onRemove(row.original)}
          />
        </div>
      ),
    },
  ];
}
