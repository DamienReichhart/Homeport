import { EllipsisVerticalIcon, InfoIcon, Trash2Icon } from 'lucide-react';
import type { NetworkView } from '@/shared/api';
import { cn } from '@/shared/lib';
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
import { t } from '@/shared/i18n';

/** The networks table's columns; the last one holds the row's actions. */
export function networkColumns({
  onInspect,
  onRemove,
}: {
  onInspect: (network: NetworkView) => void;
  onRemove: (network: NetworkView) => void;
}): ColumnDef<NetworkView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('networks.column.name'),
      accessorFn: (n) => n.name,
      cell: ({ row }) => (
        <span className="flex max-w-sm min-w-0 items-center gap-2">
          <span className="truncate font-medium" title={row.original.name}>
            {row.original.name}
          </span>
          {row.original.builtIn && (
            <Badge variant="outline" className="text-muted-foreground">
              {t('networks.builtIn')}
            </Badge>
          )}
          {row.original.internal && (
            <Badge variant="outline" className="text-muted-foreground">
              {t('networks.internal')}
            </Badge>
          )}
        </span>
      ),
    },
    {
      id: 'stack',
      header: t('networks.column.stack'),
      accessorFn: (n) => n.stack ?? '',
      cell: ({ row }) =>
        row.original.stack && <Badge variant="outline">{row.original.stack}</Badge>,
    },
    { id: 'driver', header: t('networks.column.driver'), accessorFn: (n) => n.driver },
    {
      id: 'subnet',
      header: t('networks.column.subnet'),
      accessorFn: (n) => n.subnets.join(', '),
      cell: ({ row }) =>
        row.original.subnets.length ? (
          <span className="font-mono text-xs">{row.original.subnets.join(', ')}</span>
        ) : (
          <span className="text-muted-foreground">—</span>
        ),
    },
    {
      id: 'containers',
      header: t('networks.column.containers'),
      meta: { align: 'end' },
      accessorFn: (n) => n.containers.length,
      cell: ({ row }) => (
        <span
          className={cn(
            'tabular-nums',
            row.original.containers.length === 0 && 'text-muted-foreground',
          )}
        >
          {row.original.containers.length}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={t('networks.moreActions', { name: row.original.name })}
              >
                <EllipsisVerticalIcon />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-auto">
              <DropdownMenuItem onSelect={() => onInspect(row.original)}>
                <InfoIcon /> {t('networks.action.inspect')}
              </DropdownMenuItem>
              {/* Docker's own networks can't be removed: no dead entry for them. */}
              {!row.original.builtIn && (
                <>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem variant="destructive" onSelect={() => onRemove(row.original)}>
                    <Trash2Icon /> {t('common.remove')}
                  </DropdownMenuItem>
                </>
              )}
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      ),
    },
  ];
}
