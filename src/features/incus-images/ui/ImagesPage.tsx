import { useQuery } from '@tanstack/react-query';
import {
  DiscIcon,
  DownloadIcon,
  EllipsisVerticalIcon,
  RefreshCwIcon,
  TagIcon,
  Trash2Icon,
  XIcon,
} from 'lucide-react';
import { useState } from 'react';
import { useActionRequest, usePageQuery } from '@/core/feature';
import { IncusGate } from '@/core/host';
import type { IncusImage } from '@/shared/api';
import { formatBytes, formatDateTime, formatRelative } from '@/shared/lib';
import {
  Badge,
  Button,
  DataTable,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
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
        <span className="flex items-center gap-1 whitespace-nowrap">
          {row.original.aliases.map((alias) => (
            <Badge key={alias} variant="outline" className="gap-1 font-mono">
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
      cell: ({ row }) => (
        <span className="block max-w-72 truncate" title={row.original.description}>
          {row.original.description}
        </span>
      ),
    },
    {
      id: 'type',
      header: t('incus.images.column.type'),
      accessorFn: (i) => i.type,
      cell: ({ row }) => (
        <span className="text-muted-foreground whitespace-nowrap">
          {row.original.type === 'container'
            ? t('incus.type.container')
            : t('incus.type.virtualMachine')}
        </span>
      ),
    },
    {
      id: 'arch',
      header: t('incus.images.column.arch'),
      accessorFn: (i) => i.architecture,
      cell: ({ row }) => (
        <span className="text-muted-foreground font-mono text-xs">{row.original.architecture}</span>
      ),
    },
    {
      id: 'size',
      header: t('incus.images.column.size'),
      accessorFn: (i) => i.size,
      cell: ({ row }) => (
        <div className="text-end whitespace-nowrap tabular-nums">
          {formatBytes(row.original.size)}
        </div>
      ),
    },
    {
      id: 'uploaded',
      header: t('incus.images.column.uploaded'),
      accessorFn: (i) => seconds(i.uploadedAt),
      cell: ({ row }) => (
        <span
          className="text-muted-foreground whitespace-nowrap"
          title={formatDateTime(row.original.uploadedAt)}
        >
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
          <div className="flex justify-end gap-0.5">
            {image.source && (
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button
                    variant="ghost"
                    size="icon-sm"
                    aria-label={t('incus.images.refreshOne', { name: imageName(image) })}
                    disabled={refresh.isPending}
                    onClick={() => refresh.mutate(image)}
                  >
                    <RefreshCwIcon />
                  </Button>
                </TooltipTrigger>
                <TooltipContent>
                  {t('incus.images.refreshOne', { name: imageName(image) })}
                </TooltipContent>
              </Tooltip>
            )}
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('incus.instances.more', { name: imageName(image) })}
                >
                  <EllipsisVerticalIcon />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem onSelect={() => setAliasing(image)}>
                  <TagIcon /> {t('incus.images.alias.add')}
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem
                  variant="destructive"
                  disabled={remove.isPending}
                  onSelect={() =>
                    confirm({
                      title: t('incus.images.delete.title', { name: imageName(image) }),
                      description: t('incus.images.delete.description'),
                      confirmLabel: t('incus.delete'),
                      destructive: true,
                      action: () => remove.mutateAsync(image),
                    })
                  }
                >
                  <Trash2Icon /> {t('incus.delete')}
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
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
      {images.data.length > 0 && (
        <div className="mb-3">
          <SearchInput value={query} onChange={setQuery} placeholder={t('incus.images.search')} />
        </div>
      )}
      {images.data.length === 0 ? (
        <EmptyState
          icon={DiscIcon}
          title={t('incus.images.empty', { project })}
          description={t('incus.images.emptyDescription')}
          action={
            <Button onClick={() => setGetting(true)}>
              <DownloadIcon /> {t('incus.images.get')}
            </Button>
          }
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
