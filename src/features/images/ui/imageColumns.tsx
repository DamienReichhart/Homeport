import {
  EllipsisVerticalIcon,
  InfoIcon,
  PlayIcon,
  SaveIcon,
  TagIcon,
  Trash2Icon,
  UploadIcon,
} from 'lucide-react';
import { NewerImage, newerLabel, newerTags, type FreshnessIndex } from '@/core/host';
import type { Image } from '@/shared/api';
import { formatBytes, formatRelative, shortId } from '@/shared/lib';
import {
  Badge,
  Button,
  type ColumnDef,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/shared/ui';
import { imageName } from '../model/imageName';
import { t } from '@/shared/i18n';

/** The images table's columns; the last one holds the row's actions. */
export function imageColumns({
  onRun,
  onInspect,
  onTag,
  onPush,
  onSave,
  onRemove,
  freshness,
}: {
  onRun: (image: Image) => void;
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
        <div className="min-w-0">
          <p className="flex items-center gap-1.5 font-medium">
            <span className="truncate">{imageName(row.original)}</span>
            {newerTags(freshness, row.original).length > 0 && (
              <NewerImage label={newerLabel(newerTags(freshness, row.original))} />
            )}
          </p>
          {row.original.tags.length > 1 && (
            <p className="text-muted-foreground text-xs">
              {t('images.moreTags', { count: row.original.tags.length - 1 })}
            </p>
          )}
        </div>
      ),
    },
    {
      id: 'id',
      header: t('images.column.id'),
      accessorFn: (i) => i.id,
      cell: ({ row }) => <span className="font-mono text-xs">{shortId(row.original.id)}</span>,
    },
    {
      id: 'created',
      header: t('images.column.created'),
      accessorFn: (i) => i.created,
      cell: ({ row }) => formatRelative(row.original.created),
    },
    {
      id: 'size',
      header: t('images.column.size'),
      accessorFn: (i) => i.size,
      cell: ({ row }) => formatBytes(row.original.size),
    },
    {
      id: 'usage',
      header: t('images.column.status'),
      accessorFn: (i) => i.containers,
      cell: ({ row }) =>
        row.original.containers > 0 ? (
          <Badge>{t('images.inUse', { count: row.original.containers })}</Badge>
        ) : (
          <Badge variant="outline">{t('images.unused')}</Badge>
        ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t('images.runOne', { name: imageName(row.original) })}
            onClick={() => onRun(row.original)}
          >
            <PlayIcon />
          </Button>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="icon-sm" aria-label={t('common.moreActions')}>
                <EllipsisVerticalIcon />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
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
