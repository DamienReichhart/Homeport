import { ChevronDownIcon, ChevronRightIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { useNavigate } from 'react-router-dom';
import type { ContainerUsage } from '@/shared/api';
import { formatBytes, formatPercent } from '@/shared/lib';
import { Badge, Button, Checkbox, StatusDot, TableCell, TableRow } from '@/shared/ui';
import type { StackGroup } from '../../model/grouping';
import { t } from '@/shared/i18n';

/** A Stack's header row with its totals, then its containers' rows when open. */
export function StackGroupRows({
  group: g,
  open,
  onToggleOpen,
  usage,
  allSelected,
  onSelect,
  swarmId,
  children,
}: {
  group: StackGroup;
  /** The Swarm the Host belongs to, where a swarm stack's page is. */
  swarmId?: string | null;
  open: boolean;
  onToggleOpen: () => void;
  usage: Map<string, ContainerUsage>;
  allSelected: boolean;
  onSelect: (on: boolean) => void;
  /** The containers' rows, shown while the group is open. */
  children: ReactNode;
}) {
  const navigate = useNavigate();
  const cpu = g.containers.reduce((s, c) => s + (usage.get(c.id)?.cpuPercent ?? 0), 0);
  const mem = g.containers.reduce((s, c) => s + (usage.get(c.id)?.memoryUsage ?? 0), 0);
  return (
    <>
      <TableRow className="bg-muted/30 cursor-pointer" onClick={onToggleOpen}>
        <TableCell onClick={(e) => e.stopPropagation()}>
          <Checkbox
            aria-label={t('containers.selectStack', { stack: g.stack })}
            checked={allSelected}
            onCheckedChange={(v) => onSelect(v === true)}
          />
        </TableCell>
        <TableCell colSpan={4}>
          <span className="flex items-center gap-2 font-medium">
            {open ? (
              <ChevronDownIcon className="size-4" />
            ) : (
              <ChevronRightIcon className="size-4" />
            )}
            <StatusDot
              tone={
                g.running === 0
                  ? 'muted'
                  : g.running === g.containers.length
                    ? 'success'
                    : 'warning'
              }
            />
            {g.stack}
            {g.swarm && <Badge variant="outline">{t('containers.stack.swarm')}</Badge>}
            <span className="text-muted-foreground text-xs font-normal">
              {t('containers.stack.running', { running: g.running, count: g.containers.length })}
            </span>
          </span>
        </TableCell>
        <TableCell className="text-right tabular-nums">
          {g.running ? formatPercent(cpu) : '—'}
        </TableCell>
        <TableCell className="text-right tabular-nums">
          {g.running ? formatBytes(mem) : '—'}
        </TableCell>
        <TableCell colSpan={2} className="text-right">
          {(!g.swarm || swarmId) && (
            <Button
              variant="link"
              size="sm"
              onClick={(e) => {
                e.stopPropagation();
                navigate(
                  g.swarm && swarmId
                    ? `/swarms/${swarmId}/stacks/${encodeURIComponent(g.stack)}`
                    : `/stacks/${encodeURIComponent(g.stack)}`,
                );
              }}
            >
              {t('containers.stack.open')}
            </Button>
          )}
        </TableCell>
      </TableRow>
      {open && children}
    </>
  );
}
