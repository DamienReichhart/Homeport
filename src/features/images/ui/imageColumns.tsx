import {
  CirclePlayIcon,
  DownloadIcon,
  EllipsisVerticalIcon,
  InfoIcon,
  SaveIcon,
  TagIcon,
  Trash2Icon,
  UploadIcon,
} from 'lucide-react';
import { newerTags, type FreshnessIndex } from '@/core/host';
import type { Image } from '@/shared/api';
import { formatBytes, formatDateTime, formatRelative, shortId } from '@/shared/lib';
import {
  Button,
  type ColumnDef,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/shared/ui';
import { imageName } from '../model/imageName';
import { ImageStatus } from './ImageStatus';
import { RowButton } from './RowButton';
import { t } from '@/shared/i18n';

/** The images table's columns; the last one holds the row's actions. */
export function imageColumns({
  onRun,
  onPullNewer,
  onInspect,
  onTag,
  onPush,
  onSave,
  onRemove,
  freshness,
}: {
  onRun: (image: Image) => void;
  /** Opens a pull of `reference`, a tag with a newer version published. */
  onPullNewer: (reference: string) => void;
  onInspect: (image: Image) => void;
  onTag: (image: Image) => void;
  onPush: (image: Image) => void;
  onSave: (image: Image) => void;
  onRemove: (image: Image) => void;
  freshness: FreshnessIndex;
}): ColumnDef<Image, unknown>[] {
  return [
    {
      id: 'name',
      header: t('images.column.name'),
      accessorFn: (i) => i.tags.join(' ') || '<none>',
      cell: ({ row }) => (
        <span className="flex max-w-md min-w-0 items-baseline gap-2">
          <span className="truncate font-medium" title={row.original.tags.join('\n') || undefined}>
            {imageName(row.original)}
          </span>
          {row.original.tags.length > 1 && (
            <span className="text-muted-foreground shrink-0 text-xs">
              {t('images.moreTags', { count: row.original.tags.length - 1 })}
            </span>
          )}
        </span>
      ),
    },
    {
      id: 'id',
      header: t('images.column.id'),
      accessorFn: (i) => i.id,
      cell: ({ row }) => (
        <span className="text-muted-foreground font-mono text-xs">{shortId(row.original.id)}</span>
      ),
    },
    {
      id: 'created',
      header: t('images.column.created'),
      accessorFn: (i) => i.created,
      cell: ({ row }) => (
        <span title={formatDateTime(row.original.created)}>
          {formatRelative(row.original.created)}
        </span>
      ),
    },
    {
      id: 'size',
      header: t('images.column.size'),
      meta: { align: 'end' },
      accessorFn: (i) => i.size,
      cell: ({ row }) => <span className="tabular-nums">{formatBytes(row.original.size)}</span>,
    },
    {
      id: 'usage',
      header: t('images.column.status'),
      // Images with a newer version first, then by how many containers use them.
      accessorFn: (i) => (newerTags(freshness, i).length > 0 ? 1e6 : 0) + i.containers,
      cell: ({ row }) => (
        <ImageStatus image={row.original} newer={newerTags(freshness, row.original)} />
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <RowButton
            label={t('images.runOne', { name: imageName(row.original) })}
            onClick={() => onRun(row.original)}
          >
            <CirclePlayIcon />
          </RowButton>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={t('images.moreActionsFor', { name: imageName(row.original) })}
              >
                <EllipsisVerticalIcon />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-auto">
              {newerTags(freshness, row.original).map((tag) => (
                <DropdownMenuItem key={tag} onSelect={() => onPullNewer(tag)}>
                  <DownloadIcon /> {t('images.action.pullNewer', { reference: tag })}
                </DropdownMenuItem>
              ))}
              <DropdownMenuItem onSelect={() => onInspect(row.original)}>
                <InfoIcon /> {t('images.action.inspect')}
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onTag(row.original)}>
                <TagIcon /> {t('images.action.tag')}
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onPush(row.original)}>
                <UploadIcon /> {t('images.action.push')}
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onSave(row.original)}>
                <SaveIcon /> {t('images.action.save')}
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem variant="destructive" onSelect={() => onRemove(row.original)}>
                <Trash2Icon /> {t('common.remove')}
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      ),
    },
  ];
}
