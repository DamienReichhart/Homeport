import type { IncusInstance } from '@/shared/api';
import { formatBytes, formatRelative } from '@/shared/lib';
import { Badge, StatusDot, type ColumnDef } from '@/shared/ui';
import { seconds, STATUS_LABELS, statusTone, TYPE_LABELS } from '../model/instances';
import { InstanceActions } from './InstanceActions';
import { t } from '@/shared/i18n';

/** The Instances table's columns; the last one holds the row's actions. */
export function instanceColumns({
  hostId,
  project,
  onDelete,
}: {
  hostId: string;
  project: string;
  onDelete: (instance: IncusInstance) => void;
}): ColumnDef<IncusInstance, unknown>[] {
  return [
    {
      id: 'name',
      header: t('incus.instances.column.name'),
      accessorFn: (i) => `${i.name} ${i.description}`,
      cell: ({ row }) => (
        <span className="flex min-w-0 items-center gap-2 font-medium">
          <StatusDot tone={statusTone(row.original.status)} />
          <span className="truncate">{row.original.name}</span>
        </span>
      ),
    },
    {
      id: 'type',
      header: t('incus.instances.column.type'),
      accessorFn: (i) => i.type,
      cell: ({ row }) => <Badge variant="secondary">{t(TYPE_LABELS[row.original.type])}</Badge>,
    },
    {
      id: 'status',
      header: t('incus.instances.column.status'),
      accessorFn: (i) => i.status,
      cell: ({ row }) => t(STATUS_LABELS[row.original.status]),
    },
    {
      id: 'image',
      header: t('incus.instances.column.image'),
      accessorFn: (i) => i.image,
      cell: ({ row }) => (
        <span className="text-muted-foreground text-xs">{row.original.image || '—'}</span>
      ),
    },
    {
      id: 'addresses',
      header: t('incus.instances.column.addresses'),
      accessorFn: (i) => i.addresses.join(' '),
      cell: ({ row }) => (
        <span className="font-mono text-xs">{row.original.addresses.join(', ') || '—'}</span>
      ),
    },
    {
      id: 'memory',
      header: t('incus.instances.column.memory'),
      accessorFn: (i) => i.memoryBytes ?? -1,
      cell: ({ row }) =>
        row.original.memoryBytes === null ? '—' : formatBytes(row.original.memoryBytes),
    },
    {
      id: 'created',
      header: t('incus.instances.column.created'),
      accessorFn: (i) => seconds(i.createdAt),
      cell: ({ row }) => (
        <span className="text-muted-foreground whitespace-nowrap">
          {formatRelative(seconds(row.original.createdAt))}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <InstanceActions
            hostId={hostId}
            project={project}
            instance={row.original}
            compact
            onDelete={() => onDelete(row.original)}
          />
        </div>
      ),
    },
  ];
}
