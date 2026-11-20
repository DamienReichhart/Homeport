import { useQuery } from '@tanstack/react-query';
import { CameraIcon, HistoryIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import type { IncusInstance, IncusSnapshot } from '@/shared/api';
import { formatBytes, formatDateTime, formatRelative } from '@/shared/lib';
import {
  Badge,
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  Spinner,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  useConfirmedAction,
  type ColumnDef,
} from '@/shared/ui';
import { useSnapshots } from '../api/mutations';
import { incusInstanceQueries } from '../api/queries';
import { seconds } from '../model/instances';
import { TakeSnapshotDialog } from './TakeSnapshotDialog';
import { t } from '@/shared/i18n';

/** An Instance's snapshots: take one, restore the Instance to one, delete them. */
export function SnapshotsTab({
  hostId,
  project,
  instance,
  statefulAllowed,
}: {
  hostId: string;
  project: string;
  instance: IncusInstance;
  statefulAllowed: boolean;
}) {
  const snapshots = useQuery(incusInstanceQueries.snapshots(hostId, project, instance.name));
  const { restore, remove } = useSnapshots(hostId, project, instance.name);
  const [dialog, confirm] = useConfirmedAction();
  const [taking, setTaking] = useState(false);

  if (snapshots.isPending) return <Spinner />;
  if (snapshots.error)
    return <ErrorMessage error={snapshots.error} onRetry={() => void snapshots.refetch()} />;

  const columns: ColumnDef<IncusSnapshot, unknown>[] = [
    {
      id: 'name',
      header: t('incus.snapshots.column.name'),
      accessorFn: (s) => s.name,
      cell: ({ row }) => (
        <span className="flex items-center gap-2 font-medium">
          {row.original.name}
          {row.original.stateful && (
            <Badge variant="outline">{t('incus.snapshots.stateful')}</Badge>
          )}
        </span>
      ),
    },
    {
      id: 'created',
      header: t('incus.snapshots.column.created'),
      accessorFn: (s) => seconds(s.createdAt),
      cell: ({ row }) => (
        <span
          className="text-muted-foreground whitespace-nowrap"
          title={formatDateTime(row.original.createdAt)}
        >
          {formatRelative(seconds(row.original.createdAt))}
        </span>
      ),
    },
    {
      id: 'expires',
      header: t('incus.snapshots.column.expires'),
      accessorFn: (s) => seconds(s.expiresAt),
      cell: ({ row }) =>
        row.original.expiresAt ? (
          <span
            className="text-muted-foreground whitespace-nowrap"
            title={formatDateTime(row.original.expiresAt)}
          >
            {formatRelative(seconds(row.original.expiresAt))}
          </span>
        ) : (
          <span className="text-muted-foreground">—</span>
        ),
    },
    {
      id: 'size',
      header: t('incus.snapshots.column.size'),
      accessorFn: (s) => s.size ?? -1,
      cell: ({ row }) => (
        <div className="text-end whitespace-nowrap tabular-nums">
          {row.original.size === null ? '—' : formatBytes(row.original.size)}
        </div>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => {
        const snapshot = row.original;
        return (
          <div className="flex justify-end gap-0.5">
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('incus.snapshots.restoreOne', { snapshot: snapshot.name })}
                  disabled={restore.isPending}
                  onClick={() =>
                    confirm({
                      title: t('incus.snapshots.restore.title', {
                        name: instance.name,
                        snapshot: snapshot.name,
                      }),
                      description: t('incus.snapshots.restore.description', {
                        snapshot: snapshot.name,
                        time: formatDateTime(snapshot.createdAt),
                      }),
                      confirmLabel: t('incus.snapshots.restore'),
                      destructive: true,
                      action: () =>
                        restore.mutateAsync({
                          snapshot: snapshot.name,
                          stateful: snapshot.stateful,
                        }),
                    })
                  }
                >
                  <HistoryIcon />
                </Button>
              </TooltipTrigger>
              <TooltipContent>{t('incus.snapshots.restore')}</TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('incus.snapshots.deleteOne', { snapshot: snapshot.name })}
                  disabled={remove.isPending}
                  onClick={() =>
                    confirm({
                      title: t('incus.snapshots.delete.title', { snapshot: snapshot.name }),
                      description: t('incus.snapshots.delete.description'),
                      confirmLabel: t('incus.delete'),
                      destructive: true,
                      action: () => remove.mutateAsync(snapshot.name),
                    })
                  }
                >
                  <Trash2Icon />
                </Button>
              </TooltipTrigger>
              <TooltipContent>{t('incus.delete')}</TooltipContent>
            </Tooltip>
          </div>
        );
      },
    },
  ];

  return (
    <div className="grid content-start gap-3">
      {snapshots.data.length > 0 && (
        <div className="flex justify-end">
          <Button onClick={() => setTaking(true)}>
            <CameraIcon /> {t('incus.snapshots.take')}
          </Button>
        </div>
      )}
      {snapshots.data.length === 0 ? (
        <EmptyState
          icon={CameraIcon}
          title={t('incus.snapshots.empty')}
          description={t('incus.snapshots.emptyDescription')}
          action={
            <Button onClick={() => setTaking(true)}>
              <CameraIcon /> {t('incus.snapshots.take')}
            </Button>
          }
        />
      ) : (
        <DataTable columns={columns} data={snapshots.data} getRowId={(s) => s.name} />
      )}
      <TakeSnapshotDialog
        hostId={hostId}
        project={project}
        instance={instance}
        statefulAllowed={statefulAllowed}
        open={taking}
        onClose={() => setTaking(false)}
      />
      {dialog}
    </div>
  );
}
