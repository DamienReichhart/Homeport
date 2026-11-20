import { ContainerRowMenu, containerTone, portLabels, statusText } from '@/features/containers';
import type { ContainerUsage } from '@/shared/api';
import { cn, formatBytes, formatPercent, shortId } from '@/shared/lib';
import { Badge, Checkbox, StatusDot, TableCell, TableRow } from '@/shared/ui';
import type { BoardRow } from '../model/board';
import { SHOWN_FROM } from './columns';
import { t } from '@/shared/i18n';

/** One container on the board; clicking it opens it on its Host. */
export function BoardRowView({
  row,
  usage,
  selected,
  onSelect,
  onOpen,
}: {
  row: BoardRow;
  usage?: ContainerUsage;
  selected: boolean;
  onSelect: (on: boolean) => void;
  onOpen: () => void;
}) {
  const c = row.container;
  const ports = portLabels(c);
  return (
    <TableRow
      className="cursor-pointer"
      data-state={selected ? 'selected' : undefined}
      onClick={onOpen}
    >
      <TableCell onClick={(e) => e.stopPropagation()}>
        <Checkbox
          aria-label={t('board.selectOne', { name: c.name, host: row.hostName })}
          checked={selected}
          onCheckedChange={(on) => onSelect(on === true)}
        />
      </TableCell>
      <TableCell className="text-muted-foreground max-w-28 truncate">{row.hostName}</TableCell>
      <TableCell>
        <div className="flex items-center gap-2">
          <StatusDot tone={containerTone(c.state, c.health)} />
          <span
            className="max-w-56 min-w-0 truncate font-medium"
            title={t('containers.idTitle', { id: shortId(c.id) })}
          >
            {c.name}
          </span>
        </div>
      </TableCell>
      <TableCell className={cn('max-w-36 truncate', SHOWN_FROM.image)} title={c.image}>
        {c.image}
      </TableCell>
      <TableCell className={cn('whitespace-nowrap', SHOWN_FROM.basic)}>
        {/* The health shows as a badge, not repeated in the status. */}
        <span className="text-sm">{statusText(c)}</span>
        {c.health && (
          <Badge variant={c.health === 'unhealthy' ? 'destructive' : 'outline'} className="ms-1.5">
            {c.health}
          </Badge>
        )}
      </TableCell>
      <TableCell className={cn('text-muted-foreground max-w-28 truncate', SHOWN_FROM.stack)}>
        {c.stack ?? '—'}
      </TableCell>
      <TableCell
        className={cn('max-w-28 truncate', SHOWN_FROM.ports)}
        title={ports.map((p) => p.label).join(', ')}
      >
        <span className="text-muted-foreground font-mono text-xs">
          {ports[0]?.label}
          {ports.length > 1 && ` +${ports.length - 1}`}
        </span>
      </TableCell>
      <TableCell className={cn('text-end tabular-nums', SHOWN_FROM.basic)}>
        {usage ? formatPercent(usage.cpuPercent ?? 0) : '—'}
      </TableCell>
      <TableCell className={cn('text-end tabular-nums', SHOWN_FROM.basic)}>
        {usage ? formatBytes(usage.memoryUsage) : '—'}
      </TableCell>
      <TableCell>
        <ContainerRowMenu hostId={row.hostId} container={c} />
      </TableCell>
    </TableRow>
  );
}
