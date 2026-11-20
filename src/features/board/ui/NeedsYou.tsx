import { useState } from 'react';
import { Link } from 'react-router-dom';
import { clusterStatusError, useClusters, useReconnectCluster } from '@/core/cluster';
import { statusError, useHosts, useReconnectHost } from '@/core/host';
import { Button, StatusDot, toastError } from '@/shared/ui';
import type { BoardRow } from '../model/board';
import { needsYou, type Need } from '../model/needsYou';
import { t } from '@/shared/i18n';

/** How many are listed before "Show all". */
const SHOWN = 6;

/**
 * What needs the user across every Host and Cluster, red first, each with
 * the one action that answers it. Quiet when nothing does.
 */
export function NeedsYou({ rows, onOpen }: { rows: BoardRow[]; onOpen: (row: BoardRow) => void }) {
  const hosts = useHosts();
  const clusters = useClusters();
  const [all, setAll] = useState(false);
  const needs = needsYou({
    hosts: (hosts.data ?? []).map((h) => ({
      id: h.host.id,
      name: h.host.name,
      unreachable: h.status.state === 'unreachable' ? (statusError(h.status)?.message ?? '') : null,
    })),
    clusters: (clusters.data ?? []).map((c) => ({
      id: c.cluster.id,
      name: c.cluster.name,
      unreachable:
        c.status.state === 'unreachable' ? (clusterStatusError(c.status)?.message ?? '') : null,
    })),
    rows,
  });
  const shown = all ? needs : needs.slice(0, SHOWN);
  return (
    <section aria-labelledby="needs-you" className="grid gap-2">
      <h2 id="needs-you" className="flex items-baseline gap-2 text-base font-semibold">
        {t('board.needs.title')}
        {needs.length > 0 && (
          <span className="text-muted-foreground text-sm font-normal tabular-nums">
            {needs.length}
          </span>
        )}
      </h2>
      {needs.length === 0 ? (
        <p className="text-muted-foreground flex items-center gap-2 text-sm">
          <StatusDot tone="success" />
          {t('board.needs.none')}
        </p>
      ) : (
        <ul className="bg-card divide-y rounded-md border">
          {shown.map((need) => (
            <NeedRow key={need.key} need={need} onOpen={onOpen} />
          ))}
          {needs.length > SHOWN && (
            <li className="px-3 py-1.5">
              <Button variant="link" size="sm" className="px-0" onClick={() => setAll(!all)}>
                {all ? t('board.needs.fewer') : t('board.needs.all', { count: needs.length })}
              </Button>
            </li>
          )}
        </ul>
      )}
    </section>
  );
}

function NeedRow({ need, onOpen }: { need: Need; onOpen: (row: BoardRow) => void }) {
  const { title, detail } = describe(need);
  return (
    <li className="flex min-h-11 items-center gap-3 px-3 py-1.5 text-sm">
      <StatusDot tone={need.tone} />
      <div className="grid min-w-0 flex-1">
        <span className="truncate">
          <span className="font-medium">{title}</span>
          <span className="text-muted-foreground">
            {' '}
            {t('board.needs.on', { name: need.targetName })}
          </span>
        </span>
        {detail && <span className="text-muted-foreground truncate text-xs">{detail}</span>}
      </div>
      <NeedAction need={need} onOpen={onOpen} />
    </li>
  );
}

function describe(need: Need): { title: string; detail?: string } {
  const name = need.row?.container.name ?? need.targetName;
  switch (need.reason.kind) {
    case 'host-unreachable':
      return { title: t('board.needs.hostUnreachable'), detail: need.reason.message };
    case 'cluster-unreachable':
      return { title: t('board.needs.clusterUnreachable'), detail: need.reason.message };
    case 'unhealthy':
      return { title: t('board.needs.unhealthy', { name }), detail: need.row?.container.status };
    case 'dead':
      return { title: t('board.needs.dead', { name }), detail: need.row?.container.status };
    case 'restarting':
      return { title: t('board.needs.restarting', { name }), detail: need.row?.container.status };
    case 'exited':
      return {
        title: t('board.needs.exited', { name, code: need.reason.code }),
        detail: need.row?.container.status,
      };
  }
}

function NeedAction({ need, onOpen }: { need: Need; onOpen: (row: BoardRow) => void }) {
  const reconnectHost = useReconnectHost();
  const reconnectCluster = useReconnectCluster();
  if (need.row) {
    const row = need.row;
    return (
      <Button variant="outline" size="sm" onClick={() => onOpen(row)}>
        {t('board.needs.logs')}
      </Button>
    );
  }
  if (need.reason.kind === 'host-unreachable') {
    return (
      <span className="flex gap-1.5">
        <Button variant="ghost" size="sm" asChild>
          <Link to={`/hosts/${need.targetId}`}>{t('board.needs.details')}</Link>
        </Button>
        <Button
          variant="outline"
          size="sm"
          disabled={reconnectHost.isPending}
          onClick={() => reconnectHost.mutate(need.targetId, { onError: (e) => toastError(e) })}
        >
          {t('board.needs.reconnect')}
        </Button>
      </span>
    );
  }
  return (
    <span className="flex gap-1.5">
      <Button variant="ghost" size="sm" asChild>
        <Link to={`/clusters/${need.targetId}`}>{t('board.needs.details')}</Link>
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={reconnectCluster.isPending}
        onClick={() => reconnectCluster.mutate(need.targetId)}
      >
        {t('board.needs.reconnect')}
      </Button>
    </span>
  );
}
