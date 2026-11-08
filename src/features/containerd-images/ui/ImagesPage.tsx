import { useQuery } from '@tanstack/react-query';
import { DownloadIcon, LayersIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import { useActionRequest, usePageQuery } from '@/core/feature';
import { ContainerdGate } from '@/core/host';
import { PullDialog } from '@/features/images';
import type { ContainerdImage } from '@/shared/api';
import { formatBytes, formatRelative } from '@/shared/lib';
import {
  Badge,
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  useConfirmedAction,
  WithShortcut,
  type ColumnDef,
} from '@/shared/ui';
import { useRemoveImage } from '../api/mutations';
import { containerdImageQueries } from '../api/queries';
import { useContainerdPull } from '../api/streams';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

const seconds = (time: string | null) => (time ? Math.floor(Date.parse(time) / 1000) : 0);

function ImagesList({ hostId, namespace }: { hostId: string; namespace: string }) {
  const images = useQuery(containerdImageQueries.list(hostId, namespace));
  const remove = useRemoveImage(hostId, namespace);
  const puller = useContainerdPull(hostId, namespace);
  const [dialog, confirm] = useConfirmedAction();
  const [query, setQuery] = usePageQuery();
  const [pulling, setPulling] = useState(false);
  usePageAction(t('containerd.images.pull'), () => setPulling(true));
  useActionRequest('pull', () => setPulling(true));

  if (images.isPending) return <Spinner />;
  if (images.error)
    return <ErrorMessage error={images.error} onRetry={() => void images.refetch()} />;

  const columns: ColumnDef<ContainerdImage, unknown>[] = [
    {
      id: 'name',
      header: t('containerd.images.column.name'),
      accessorFn: (i) => i.name,
      cell: ({ row }) => <span className="font-mono text-xs">{row.original.name}</span>,
    },
    {
      id: 'platforms',
      header: t('containerd.images.column.platforms'),
      accessorFn: (i) => i.platforms.join(' '),
      cell: ({ row }) => (
        <span className="flex flex-wrap gap-1">
          {row.original.platforms.slice(0, 4).map((p) => (
            <Badge key={p} variant="outline">
              {p}
            </Badge>
          ))}
          {row.original.platforms.length > 4 && (
            <Badge variant="outline">+{row.original.platforms.length - 4}</Badge>
          )}
        </span>
      ),
    },
    {
      id: 'size',
      header: t('containerd.images.column.size'),
      accessorFn: (i) => i.size ?? -1,
      cell: ({ row }) =>
        row.original.size === null ? (
          <span className="text-muted-foreground" title={t('containerd.images.sizeUnknown')}>
            —
          </span>
        ) : (
          formatBytes(row.original.size)
        ),
    },
    {
      id: 'containers',
      header: t('containerd.images.column.containers'),
      accessorFn: (i) => i.containers,
    },
    {
      id: 'created',
      header: t('containerd.images.column.created'),
      accessorFn: (i) => seconds(i.createdAt),
      cell: ({ row }) => (
        <span className="text-muted-foreground whitespace-nowrap">
          {row.original.createdAt ? formatRelative(seconds(row.original.createdAt)) : '—'}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end">
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t('containerd.images.removeOne', { name: row.original.name })}
            title={row.original.containers > 0 ? t('containerd.images.inUse') : undefined}
            disabled={row.original.containers > 0 || remove.isPending}
            onClick={() =>
              confirm({
                title: t('containerd.images.remove.title', { name: row.original.name }),
                description: t('containerd.images.remove.description'),
                confirmLabel: t('common.remove'),
                destructive: true,
                action: () => remove.mutateAsync(row.original.name),
              })
            }
          >
            <Trash2Icon />
          </Button>
        </div>
      ),
    },
  ];

  return (
    <div>
      <PageHeader
        title={t('containerd.images.title')}
        description={t('containerd.images.summary', { count: images.data.length, namespace })}
        actions={
          <WithShortcut keys={PAGE_ACTION_KEYS} label={t('containerd.images.pull')}>
            <Button onClick={() => setPulling(true)}>
              <DownloadIcon /> {t('containerd.images.pull')}
            </Button>
          </WithShortcut>
        }
      />
      <div className="mb-3">
        <SearchInput
          value={query}
          onChange={setQuery}
          placeholder={t('containerd.images.search')}
        />
      </div>
      {images.data.length === 0 ? (
        <EmptyState
          icon={LayersIcon}
          title={t('containerd.images.empty', { namespace })}
          description={t('containerd.images.emptyDescription')}
        />
      ) : (
        <DataTable columns={columns} data={images.data} getRowId={(i) => i.name} filter={query} />
      )}
      <PullDialog
        hostId={hostId}
        open={pulling}
        onOpenChange={setPulling}
        puller={puller}
        search={false}
      />
      {dialog}
    </div>
  );
}

export function ImagesPage() {
  return (
    <ContainerdGate>
      {(hostId, namespace) => (
        <ImagesList key={`${hostId}/${namespace}`} hostId={hostId} namespace={namespace} />
      )}
    </ContainerdGate>
  );
}
