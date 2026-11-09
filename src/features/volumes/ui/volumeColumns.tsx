import {
  ArchiveIcon,
  ArchiveRestoreIcon,
  CopyIcon,
  EllipsisIcon,
  FolderOpenIcon,
  InfoIcon,
  Trash2Icon,
} from 'lucide-react';
import type { VolumeView } from '@/shared/api';
import {
  Badge,
  Button,
  type ColumnDef,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/shared/ui';
import { VolumeSize, type VolumeSizes } from './VolumeSize';
import { t } from '@/shared/i18n';
import { formatDate } from '@/shared/lib';

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
      accessorFn: (v) => v.name,
      cell: ({ row }) => <span className="font-medium break-all">{row.original.name}</span>,
    },
    {
      id: 'stack',
      header: t('volumes.column.stack'),
      accessorFn: (v) => v.stack ?? '',
      cell: ({ row }) =>
        row.original.stack && <Badge variant="outline">{row.original.stack}</Badge>,
    },
    { id: 'driver', header: t('volumes.column.driver'), accessorFn: (v) => v.driver },
    {
      id: 'size',
      header: t('volumes.column.size'),
      // Unknown sizes sort below every known one.
      accessorFn: (v) => sizes?.[v.name] ?? -1,
      cell: ({ row }) => <VolumeSize sizes={sizes} name={row.original.name} />,
    },
    {
      id: 'usedBy',
      header: t('volumes.column.usedBy'),
      accessorFn: (v) => v.usedBy.join(' '),
      cell: ({ row }) =>
        row.original.usedBy.length ? (
          row.original.usedBy.join(', ')
        ) : (
          <Badge variant="outline">{t('volumes.unused')}</Badge>
        ),
    },
    {
      id: 'created',
      header: t('volumes.column.created'),
      accessorFn: (v) => v.createdAt ?? '',
      cell: ({ row }) => (row.original.createdAt ? formatDate(row.original.createdAt) : '—'),
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
            aria-label={t('volumes.inspectOne', { name: row.original.name })}
            onClick={() => onInspect(row.original)}
          >
            <InfoIcon />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t('volumes.browseOne', { name: row.original.name })}
            onClick={() => onBrowse(row.original)}
          >
            <FolderOpenIcon />
          </Button>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={t('volumes.moreActions', { name: row.original.name })}
              >
                <EllipsisIcon />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onSelect={() => onBackup(row.original)}>
                <ArchiveIcon /> {t('volumes.backupMenu')}
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onRestore(row.original)}>
                <ArchiveRestoreIcon /> {t('volumes.restoreMenu')}
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onClone(row.original)}>
                <CopyIcon /> {t('volumes.cloneMenu')}
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t('volumes.deleteOne', { name: row.original.name })}
            onClick={() => onRemove(row.original)}
          >
            <Trash2Icon />
          </Button>
        </div>
      ),
    },
  ];
}
