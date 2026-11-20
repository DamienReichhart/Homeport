import {
  ArchiveIcon,
  ArchiveRestoreIcon,
  CopyIcon,
  EllipsisVerticalIcon,
  FolderOpenIcon,
  InfoIcon,
  Trash2Icon,
} from 'lucide-react';
import type { VolumeView } from '@/shared/api';
import { cn, formatDateTime, formatRelative } from '@/shared/lib';
import {
  Badge,
  Button,
  type ColumnDef,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/shared/ui';
import { VolumeSize, type VolumeSizes } from './VolumeSize';
import { t } from '@/shared/i18n';

/** The driver every Docker Engine has; only others are worth naming. */
const LOCAL = 'local';
/** An anonymous volume's name is a 64-digit hex ID. */
const ANONYMOUS = /^[0-9a-f]{64}$/;

/** When a volume was made, relative, with the date on hover. */
function created(at: string | null) {
  if (!at) return <span className="text-muted-foreground">—</span>;
  const date = new Date(at);
  if (Number.isNaN(date.getTime())) return <span className="text-muted-foreground">—</span>;
  return <span title={formatDateTime(date)}>{formatRelative(date.getTime() / 1000)}</span>;
}

/** The volumes table's columns; the last one holds the row's actions. */
export function volumeColumns({
  sizes,
  onInspect,
  onBrowse,
  onBackup,
  onRestore,
  onClone,
  onRemove,
}: {
  sizes: VolumeSizes;
  onInspect: (volume: VolumeView) => void;
  onBrowse: (volume: VolumeView) => void;
  onBackup: (volume: VolumeView) => void;
  onRestore: (volume: VolumeView) => void;
  onClone: (volume: VolumeView) => void;
  onRemove: (volume: VolumeView) => void;
}): ColumnDef<VolumeView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('volumes.column.name'),
      // The driver is searchable too, though only an unusual one is shown.
      accessorFn: (v) => (v.driver === LOCAL ? v.name : `${v.name} ${v.driver}`),
      cell: ({ row }) => (
        <span className="flex max-w-sm min-w-0 items-baseline gap-2">
          <span
            className={cn(
              'truncate font-medium',
              ANONYMOUS.test(row.original.name) && 'font-mono text-xs',
            )}
            title={row.original.name}
          >
            {row.original.name}
          </span>
          {row.original.driver !== LOCAL && (
            <span className="text-muted-foreground shrink-0 font-mono text-xs">
              {row.original.driver}
            </span>
          )}
        </span>
      ),
    },
    {
      id: 'stack',
      header: t('volumes.column.stack'),
      accessorFn: (v) => v.stack ?? '',
      cell: ({ row }) =>
        row.original.stack && <Badge variant="outline">{row.original.stack}</Badge>,
    },
    {
      id: 'size',
      header: t('volumes.column.size'),
      meta: { align: 'end' },
      // Unknown sizes sort below every known one.
      accessorFn: (v) => sizes?.[v.name] ?? -1,
      cell: ({ row }) => (
        <span className="tabular-nums">
          <VolumeSize sizes={sizes} name={row.original.name} />
        </span>
      ),
    },
    {
      id: 'usedBy',
      header: t('volumes.column.usedBy'),
      accessorFn: (v) => v.usedBy.join(' '),
      cell: ({ row }) =>
        row.original.usedBy.length ? (
          <span className="block max-w-xs truncate" title={row.original.usedBy.join('\n')}>
            {row.original.usedBy.join(', ')}
          </span>
        ) : (
          <Badge variant="outline">{t('volumes.unused')}</Badge>
        ),
    },
    {
      id: 'created',
      header: t('volumes.column.created'),
      accessorFn: (v) => v.createdAt ?? '',
      cell: ({ row }) => created(row.original.createdAt),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={t('volumes.browseOne', { name: row.original.name })}
                onClick={() => onBrowse(row.original)}
              >
                <FolderOpenIcon />
              </Button>
            </TooltipTrigger>
            <TooltipContent>{t('volumes.browseOne', { name: row.original.name })}</TooltipContent>
          </Tooltip>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={t('volumes.moreActions', { name: row.original.name })}
              >
                <EllipsisVerticalIcon />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-auto">
              <DropdownMenuItem onSelect={() => onInspect(row.original)}>
                <InfoIcon /> {t('volumes.inspectMenu')}
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onBackup(row.original)}>
                <ArchiveIcon /> {t('volumes.backupMenu')}
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onRestore(row.original)}>
                <ArchiveRestoreIcon /> {t('volumes.restoreMenu')}
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onClone(row.original)}>
                <CopyIcon /> {t('volumes.cloneMenu')}
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem variant="destructive" onSelect={() => onRemove(row.original)}>
                <Trash2Icon /> {t('volumes.deleteMenu')}
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      ),
    },
  ];
}
