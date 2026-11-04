import {
  EllipsisVerticalIcon,
  InfoIcon,
  PlayIcon,
  SaveIcon,
  TagIcon,
  Trash2Icon,
  UploadIcon,
} from 'lucide-react';
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

/** The images table's columns; the last one holds the row's actions. */
export function imageColumns({
  onRun,
  onInspect,
  onTag,
  onPush,
  onSave,
  onRemove,
}: {
  onRun: (image: Image) => void;
  onInspect: (image: Image) => void;
  onTag: (image: Image) => void;
  onPush: (image: Image) => void;
  onSave: (image: Image) => void;
  onRemove: (image: Image) => void;
}): ColumnDef<Image, unknown>[] {
  return [
    {
      id: 'name',
      header: 'Name',
      accessorFn: (i) => i.tags.join(' ') || '<none>',
      cell: ({ row }) => (
        <div className="min-w-0">
          <p className="truncate font-medium">{imageName(row.original)}</p>
          {row.original.tags.length > 1 && (
            <p className="text-muted-foreground text-xs">
              +{row.original.tags.length - 1} more tags
            </p>
          )}
        </div>
      ),
    },
    {
      id: 'id',
      header: 'ID',
      accessorFn: (i) => i.id,
      cell: ({ row }) => <span className="font-mono text-xs">{shortId(row.original.id)}</span>,
    },
    {
      id: 'created',
      header: 'Created',
      accessorFn: (i) => i.created,
      cell: ({ row }) => formatRelative(row.original.created),
    },
    {
      id: 'size',
      header: 'Size',
      accessorFn: (i) => i.size,
      cell: ({ row }) => formatBytes(row.original.size),
    },
    {
      id: 'usage',
      header: 'Status',
      accessorFn: (i) => i.containers,
      cell: ({ row }) =>
        row.original.containers > 0 ? (
          <Badge>In use · {row.original.containers}</Badge>
        ) : (
          <Badge variant="outline">Unused</Badge>
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
            aria-label={`Run ${imageName(row.original)}`}
            onClick={() => onRun(row.original)}
          >
            <PlayIcon />
          </Button>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="icon-sm" aria-label="More actions">
                <EllipsisVerticalIcon />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onSelect={() => onInspect(row.original)}>
                <InfoIcon /> Inspect
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onTag(row.original)}>
                <TagIcon /> Tag
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onPush(row.original)}>
                <UploadIcon /> Push
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => onSave(row.original)}>
                <SaveIcon /> Save…
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem variant="destructive" onSelect={() => onRemove(row.original)}>
                <Trash2Icon /> Remove
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      ),
    },
  ];
}
