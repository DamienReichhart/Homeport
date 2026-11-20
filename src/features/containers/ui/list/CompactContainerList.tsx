import type { KeyboardEvent } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import type { Container } from '@/shared/api';
import { cn, shortId } from '@/shared/lib';
import { StatusDot } from '@/shared/ui';
import { containerTone } from '../../model/containerTone';
import type { Grouped } from '../../model/grouping';
import { t } from '@/shared/i18n';

/**
 * The containers as a narrow list beside the open one: grouped by Stack, one
 * line each, the open one marked. Up and Down open the one above or below.
 */
export function CompactContainerList({
  grouped: { groups, standalone },
  openId,
}: {
  grouped: Grouped;
  openId: string | null;
}) {
  const navigate = useNavigate();
  const order = [...groups.flatMap((g) => g.containers), ...standalone];
  const isOpen = (c: Container) => !!openId && (c.id === openId || c.id.startsWith(openId));
  const onKeyDown = (event: KeyboardEvent) => {
    if (event.key !== 'ArrowDown' && event.key !== 'ArrowUp') return;
    const at = order.findIndex(isOpen);
    const next = order[at + (event.key === 'ArrowDown' ? 1 : -1)];
    if (!next) return;
    event.preventDefault();
    void navigate(`/containers/${next.id}/logs`);
  };
  const item = (c: Container, label: string) => (
    <li key={c.id}>
      <Link
        to={`/containers/${c.id}/logs`}
        aria-current={isOpen(c) ? 'true' : undefined}
        title={t('containers.idTitle', { id: shortId(c.id) })}
        ref={isOpen(c) ? (el) => el?.scrollIntoView?.({ block: 'nearest' }) : undefined}
        className={cn(
          'hover:bg-selected focus-visible:ring-ring/50 flex h-8 items-center gap-2 rounded-md px-2 text-sm transition-colors outline-none focus-visible:ring-2',
          isOpen(c) && 'bg-muted font-medium',
        )}
      >
        <StatusDot tone={containerTone(c.state, c.health)} />
        <span className="min-w-0 flex-1 truncate">{label}</span>
      </Link>
    </li>
  );
  return (
    <div className="grid gap-3" onKeyDown={onKeyDown}>
      {groups.map((g) => (
        <section key={`${g.swarm}:${g.stack}`} aria-label={g.stack}>
          <h3 className="text-muted-foreground flex items-center justify-between px-2 pb-1 text-xs font-medium">
            <span className="truncate">{g.stack}</span>
            <span className="tabular-nums">
              {g.running}/{g.containers.length}
            </span>
          </h3>
          <ul className="grid gap-px">
            {g.containers.map((c) => {
              // Replicas share a service name; their own names tell them apart.
              const shared = g.containers.filter((o) => o.service === c.service).length > 1;
              return item(c, c.service && !shared ? c.service : c.name);
            })}
          </ul>
        </section>
      ))}
      {standalone.length > 0 && (
        <ul className="grid gap-px">{standalone.map((c) => item(c, c.name))}</ul>
      )}
    </div>
  );
}
