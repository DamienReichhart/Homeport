import type { ContainerdContainer } from '@/shared/api';
import { formatDateTime, formatRelative } from '@/shared/lib';
import { Badge, StatusDot, type ColumnDef } from '@/shared/ui';
import { seconds, STATE_LABELS, stateTone } from '../model/containers';
import { TaskActions } from './TaskActions';
import { t, type MessageKey } from '@/shared/i18n';

export const CREATED_BY: Record<ContainerdContainer['createdBy'], MessageKey> = {
  kubernetes: 'containerd.createdBy.kubernetes',
  nerdctl: 'containerd.createdBy.nerdctl',
  docker: 'containerd.createdBy.docker',
  other: 'containerd.createdBy.other',
};

/** The containerd Containers table's columns; the last one holds the row's actions. */
export function containerColumns({
  hostId,
  namespace,
  onRemove,
}: {
  hostId: string;
  namespace: string;
  onRemove: (container: ContainerdContainer) => void;
}): ColumnDef<ContainerdContainer, unknown>[] {
  return [
    {
      id: 'name',
      header: t('containerd.containers.column.name'),
      accessorFn: (c) => `${c.name} ${c.id}`,
      cell: ({ row }) => (
        <span className="flex min-w-0 items-center gap-2 font-medium">
          <StatusDot tone={stateTone(row.original.state)} />
          <span className="truncate">{row.original.name}</span>
          {row.original.sandbox && (
            <Badge variant="outline">{t('containerd.containers.sandbox')}</Badge>
          )}
        </span>
      ),
    },
    {
      id: 'image',
      header: t('containerd.containers.column.image'),
      accessorFn: (c) => c.image,
      cell: ({ row }) => (
        <span
          className="text-muted-foreground block max-w-72 truncate font-mono text-xs"
          title={row.original.image}
        >
          {row.original.image}
        </span>
      ),
    },
    {
      id: 'state',
      header: t('containerd.containers.column.state'),
      accessorFn: (c) => c.state,
      meta: { compact: true },
      cell: ({ row }) =>
        row.original.state === 'stopped' && row.original.exitStatus !== null
          ? t('containerd.state.exited', { code: row.original.exitStatus })
          : t(STATE_LABELS[row.original.state]),
    },
    {
      id: 'createdBy',
      header: t('containerd.containers.column.createdBy'),
      accessorFn: (c) => c.createdBy,
      cell: ({ row }) => (
        <span className="flex items-center gap-1.5 whitespace-nowrap">
          <Badge variant="outline">{t(CREATED_BY[row.original.createdBy])}</Badge>
          {row.original.pod && (
            <span
              className="text-muted-foreground max-w-48 truncate text-xs"
              title={row.original.pod}
            >
              {row.original.pod}
            </span>
          )}
        </span>
      ),
    },
    {
      id: 'created',
      header: t('containerd.containers.column.created'),
      accessorFn: (c) => seconds(c.createdAt),
      cell: ({ row }) => (
        <span
          className="text-muted-foreground whitespace-nowrap"
          title={row.original.createdAt ? formatDateTime(row.original.createdAt) : undefined}
        >
          {row.original.createdAt ? formatRelative(seconds(row.original.createdAt)) : '—'}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <TaskActions
          hostId={hostId}
          namespace={namespace}
          container={row.original}
          compact
          onRemove={() => onRemove(row.original)}
        />
      ),
    },
  ];
}
