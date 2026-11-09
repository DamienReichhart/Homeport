import { ContainerRowMenu, containerTone, portLabels } from '@/features/containers';
import type { ContainerUsage } from '@/shared/api';
import { formatBytes, formatPercent, formatRelative, shortId } from '@/shared/lib';
import { Badge, Checkbox, StatusDot, TableCell, TableRow } from '@/shared/ui';
import type { BoardRow } from '../model/board';
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
      <TableCell className="max-w-28 truncate">{row.hostName}</TableCell>
      <TableCell>
        <div className="flex items-center gap-2">
          <StatusDot tone={containerTone(c.state, c.health)} />
          <div className="max-w-48 min-w-0">
            <p className="truncate font-medium">{c.name}</p>
            <p className="text-muted-foreground font-mono text-xs">{shortId(c.id)}</p>
          </div>
        </div>
      </TableCell>
      <TableCell className="max-w-36 truncate" title={c.image}>
        {c.image}
      </TableCell>
      <TableCell className="whitespace-nowrap">
        {/* The health shows as a badge, not repeated in the status. */}
        <span className="text-sm">
          {(c.status || c.state).replace(/\s*\((healthy|unhealthy|health: starting)\)/, '')}
        </span>
        {c.health && (
          <Badge variant={c.health === 'unhealthy' ? 'destructive' : 'outline'} className="ms-1.5">
            {c.health}
          </Badge>
        )}
      </TableCell>
      <TableCell className="text-muted-foreground max-w-28 truncate">{c.stack ?? '—'}</TableCell>
      <TableCell className="max-w-28 truncate" title={ports.map((p) => p.label).join(', ')}>
        <span className="text-muted-foreground font-mono text-xs">
          {ports[0]?.label}
          {ports.length > 1 && ` +${ports.length - 1}`}
        </span>
      </TableCell>
      <TableCell className="text-end tabular-nums">
        {usage ? formatPercent(usage.cpuPercent ?? 0) : '—'}
      </TableCell>
      <TableCell className="text-end tabular-nums">
        {usage ? formatBytes(usage.memoryUsage) : '—'}
      </TableCell>
      <TableCell className="text-muted-foreground whitespace-nowrap">
        {formatRelative(c.created)}
      </TableCell>
      <TableCell>
        <ContainerRowMenu hostId={row.hostId} container={c} />
      </TableCell>
    </TableRow>
  );
}
