import { useQuery } from '@tanstack/react-query';
import { HardDriveIcon, PlusIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { IncusGate } from '@/core/host';
import type { IncusStoragePool, IncusVolume } from '@/shared/api';
import { cn, formatBytes, formatPercent } from '@/shared/lib';
import {
  Badge,
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  Progress,
  Spinner,
  useConfirmedAction,
  type ColumnDef,
} from '@/shared/ui';
import { useVolumes } from '../api/mutations';
import { incusStorageQueries } from '../api/queries';
import { NewVolumeDialog } from './NewVolumeDialog';
import { t } from '@/shared/i18n';

function PoolCard({
  pool,
  selected,
  onSelect,
}: {
  pool: IncusStoragePool;
  selected: boolean;
  onSelect: () => void;
}) {
  const share = pool.used !== null && pool.total ? pool.used / pool.total : null;
  return (
    <button
      type="button"
      aria-pressed={selected}
      onClick={onSelect}
      className={cn(
        'hover:bg-muted grid gap-2 rounded-lg border p-3 text-start',
        selected && 'border-primary bg-muted/60',
      )}
    >
      <span className="flex items-center justify-between gap-2">
        <span className="font-medium">{pool.name}</span>
        <Badge variant="secondary">{pool.driver}</Badge>
      </span>
      {share === null ? (
        <span className="text-muted-foreground text-xs">{t('incus.storage.spaceUnknown')}</span>
      ) : (
        <>
          <Progress
            value={share * 100}
            aria-label={t('incus.storage.usage', { pool: pool.name })}
          />
          <span className="text-muted-foreground text-xs">
            {t('incus.storage.space', {
              used: formatBytes(pool.used ?? 0),
              total: formatBytes(pool.total ?? 0),
              percent: formatPercent(share * 100),
            })}
          </span>
        </>
      )}
      {pool.status && pool.status !== 'Created' && (
        <span className="text-warning text-xs">{pool.status}</span>
      )}
    </button>
  );
}

function Volumes({ hostId, project, pool }: { hostId: string; project: string; pool: string }) {
  const volumes = useQuery(incusStorageQueries.volumes(hostId, project, pool));
  const { remove } = useVolumes(hostId, project, pool);
  const [dialog, confirm] = useConfirmedAction();
  const [creating, setCreating] = useState(false);
  if (volumes.isPending) return <Spinner />;
  if (volumes.error)
    return <ErrorMessage error={volumes.error} onRetry={() => void volumes.refetch()} />;
  const columns: ColumnDef<IncusVolume, unknown>[] = [
    {
      id: 'name',
      header: t('incus.storage.column.name'),
      accessorFn: (v) => v.name,
      cell: ({ row }) => <span className="font-mono text-xs">{row.original.name}</span>,
    },
    {
      id: 'type',
      header: t('incus.storage.column.type'),
      accessorFn: (v) => v.type,
      cell: ({ row }) => <Badge variant="outline">{row.original.type}</Badge>,
    },
    {
      id: 'content',
      header: t('incus.storage.column.content'),
      accessorFn: (v) => v.contentType,
    },
    {
      id: 'size',
      header: t('incus.storage.column.size'),
      accessorFn: (v) => v.size ?? '',
      cell: ({ row }) => row.original.size ?? '—',
    },
    {
      id: 'usedBy',
      header: t('incus.storage.column.usedBy'),
      accessorFn: (v) => v.usedBy.map((u) => u.name).join(' '),
      cell: ({ row }) => (
        <span className="text-muted-foreground text-xs">
          {row.original.usedBy.map((u) => u.name).join(', ') || '—'}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => {
        const volume = row.original;
        if (volume.type !== 'custom') return null;
        const inUse = volume.usedBy.length > 0;
        return (
          <div className="flex justify-end">
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('incus.storage.deleteOne', { name: volume.name })}
              title={
                inUse
                  ? t('incus.storage.inUse', { names: volume.usedBy.map((u) => u.name).join(', ') })
                  : undefined
              }
              disabled={inUse || remove.isPending}
              onClick={() =>
                confirm({
                  title: t('incus.storage.delete.title', { name: volume.name }),
                  description: t('incus.storage.delete.description', { pool }),
                  confirmLabel: t('incus.delete'),
                  destructive: true,
                  action: () => remove.mutateAsync(volume.name),
                })
              }
            >
              <Trash2Icon />
            </Button>
          </div>
        );
      },
    },
  ];
  return (
    <div className="grid gap-3">
      <div className="flex items-center justify-between gap-2">
        <h2 className="text-sm font-medium">{t('incus.storage.volumesOf', { pool })}</h2>
        <Button variant="outline" onClick={() => setCreating(true)}>
          <PlusIcon /> {t('incus.storage.new')}
        </Button>
      </div>
      {volumes.data.length === 0 ? (
        <p className="text-muted-foreground text-sm">{t('incus.storage.noVolumes')}</p>
      ) : (
        <DataTable columns={columns} data={volumes.data} getRowId={(v) => `${v.type}/${v.name}`} />
      )}
      <NewVolumeDialog
        hostId={hostId}
        project={project}
        pool={pool}
        open={creating}
        onClose={() => setCreating(false)}
      />
      {dialog}
    </div>
  );
}

function Storage({ hostId, project }: { hostId: string; project: string }) {
  const { pool: chosen } = useParams();
  const navigate = useNavigate();
  const pools = useQuery(incusStorageQueries.pools(hostId, project));
  if (pools.isPending) return <Spinner />;
  if (pools.error) return <ErrorMessage error={pools.error} onRetry={() => void pools.refetch()} />;
  const pool = chosen ? decodeURIComponent(chosen) : pools.data[0]?.name;
  return (
    <div className="grid gap-4">
      <PageHeader
        title={t('incus.storage.title')}
        description={t('incus.storage.summary', { count: pools.data.length, project })}
      />
      {pools.data.length === 0 ? (
        <EmptyState
          icon={HardDriveIcon}
          title={t('incus.storage.empty')}
          description={t('incus.storage.emptyDescription')}
        />
      ) : (
        <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {pools.data.map((p) => (
            <PoolCard
              key={p.name}
              pool={p}
              selected={p.name === pool}
              onSelect={() => void navigate(`/incus/storage/${encodeURIComponent(p.name)}`)}
            />
          ))}
        </div>
      )}
      {pool && <Volumes key={pool} hostId={hostId} project={project} pool={pool} />}
    </div>
  );
}

export function StoragePage() {
  return (
    <IncusGate>
      {(hostId, project) => (
        <Storage key={`${hostId}/${project}`} hostId={hostId} project={project} />
      )}
    </IncusGate>
  );
}
