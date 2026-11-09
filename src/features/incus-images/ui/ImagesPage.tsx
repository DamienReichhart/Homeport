import { useQuery } from '@tanstack/react-query';
import { DiscIcon, DownloadIcon, RefreshCwIcon, TagIcon, Trash2Icon, XIcon } from 'lucide-react';
import { useState } from 'react';
import { useActionRequest, usePageQuery } from '@/core/feature';
import { IncusGate } from '@/core/host';
import type { IncusImage } from '@/shared/api';
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
import { imageName, useAliases, useDeleteImage, useRefreshImage } from '../api/mutations';
import { incusImageQueries } from '../api/queries';
import { AliasDialog } from './AliasDialog';
import { GetImageDialog } from './GetImageDialog';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

const seconds = (time: string | null) => (time ? Math.floor(Date.parse(time) / 1000) : 0);

function ImagesList({ hostId, project }: { hostId: string; project: string }) {
  const images = useQuery(incusImageQueries.list(hostId, project));
  const remove = useDeleteImage(hostId, project);
  const refresh = useRefreshImage(hostId, project);
  const aliases = useAliases(hostId, project);
  const [dialog, confirm] = useConfirmedAction();
  const [query, setQuery] = usePageQuery();
  const [getting, setGetting] = useState(false);
  const [aliasing, setAliasing] = useState<IncusImage | null>(null);
  usePageAction(t('incus.images.get'), () => setGetting(true));
  useActionRequest('get', () => setGetting(true));

  if (images.isPending) return <Spinner />;
  if (images.error)
    return <ErrorMessage error={images.error} onRetry={() => void images.refetch()} />;

  const columns: ColumnDef<IncusImage, unknown>[] = [
    {
      id: 'aliases',
      header: t('incus.images.column.aliases'),
      accessorFn: (i) => `${i.aliases.join(' ')} ${i.fingerprint}`,
      cell: ({ row }) => (
        <span className="flex flex-wrap items-center gap-1">
          {row.original.aliases.map((alias) => (
            <Badge key={alias} variant="secondary" className="gap-1 font-mono">
              {alias}
              <button
                type="button"
                aria-label={t('incus.images.aliasRemove', { name: alias })}
                className="hover:text-destructive"
                onClick={() => aliases.remove.mutate(alias)}
              >
                <XIcon className="size-3" />
              </button>
            </Badge>
          ))}
          <span className="text-muted-foreground font-mono text-xs">
            {row.original.fingerprint.slice(0, 12)}
          </span>
        </span>
      ),
    },
    {
      id: 'description',
      header: t('incus.images.column.description'),
      accessorFn: (i) => i.description,
    },
    {
      id: 'type',
      header: t('incus.images.column.type'),
      accessorFn: (i) => i.type,
      cell: ({ row }) => (
        <Badge variant="outline">
          {row.original.type === 'container'
            ? t('incus.type.container')
            : t('incus.type.virtualMachine')}
        </Badge>
      ),
    },
    { id: 'arch', header: t('incus.images.column.arch'), accessorFn: (i) => i.architecture },
    {
      id: 'size',
      header: t('incus.images.column.size'),
      accessorFn: (i) => i.size,
      cell: ({ row }) => formatBytes(row.original.size),
    },
    {
      id: 'uploaded',
      header: t('incus.images.column.uploaded'),
      accessorFn: (i) => seconds(i.uploadedAt),
      cell: ({ row }) => (
        <span className="text-muted-foreground whitespace-nowrap">
          {formatRelative(seconds(row.original.uploadedAt))}
          {row.original.cached && (
            <Badge variant="outline" className="ms-2" title={t('incus.images.cachedHint')}>
              {t('incus.images.cached')}
            </Badge>
          )}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => {
        const image = row.original;
        return (
          <div className="flex justify-end gap-1">
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('incus.images.aliasAdd', { name: imageName(image) })}
              onClick={() => setAliasing(image)}
            >
              <TagIcon />
            </Button>
            {image.source && (
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={t('incus.images.refreshOne', { name: imageName(image) })}
                disabled={refresh.isPending}
                onClick={() => refresh.mutate(image)}
              >
                <RefreshCwIcon />
              </Button>
            )}
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('incus.images.deleteOne', { name: imageName(image) })}
              disabled={remove.isPending}
              onClick={() =>
                confirm({
                  title: t('incus.images.delete.title', { name: imageName(image) }),
                  description: t('incus.images.delete.description'),
                  confirmLabel: t('incus.delete'),
                  destructive: true,
                  action: () => remove.mutateAsync(image),
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
    <div>
      <PageHeader
        title={t('incus.images.title')}
        description={t('incus.images.summary', { count: images.data.length, project })}
        actions={
          <WithShortcut keys={PAGE_ACTION_KEYS} label={t('incus.images.get')}>
            <Button onClick={() => setGetting(true)}>
              <DownloadIcon /> {t('incus.images.get')}
            </Button>
          </WithShortcut>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('incus.images.search')} />
      </div>
      {images.data.length === 0 ? (
        <EmptyState
          icon={DiscIcon}
          title={t('incus.images.empty', { project })}
          description={t('incus.images.emptyDescription')}
        />
      ) : (
        <DataTable
          columns={columns}
          data={images.data}
          getRowId={(i) => i.fingerprint}
          filter={query}
        />
      )}
      {getting && (
        <GetImageDialog
          hostId={hostId}
          project={project}
          open={getting}
          onClose={() => setGetting(false)}
        />
      )}
      <AliasDialog
        hostId={hostId}
        project={project}
        image={aliasing}
        onClose={() => setAliasing(null)}
      />
      {dialog}
    </div>
  );
}

export function ImagesPage() {
  return (
    <IncusGate>
      {(hostId, project) => (
        <ImagesList key={`${hostId}/${project}`} hostId={hostId} project={project} />
      )}
    </IncusGate>
  );
}
