import { LayoutGridIcon, ServerIcon } from 'lucide-react';
import { useCallback, useState } from 'react';
import { useLocation, useNavigate, useSearchParams } from 'react-router-dom';
import { resourceQueries, useEveryHost, useHosts, useHostSelection } from '@/core/host';
import type { ContainerUsage } from '@/shared/api';
import { EmptyState, PageHeader, Spinner, useConfirm } from '@/shared/ui';
import {
  boardRows,
  filterRows,
  readFilter,
  sortRows,
  stacksOf,
  writeFilter,
  type BoardFilter,
  type BoardRow,
  type BoardSort,
} from '../model/board';
import { BoardFilters } from './BoardFilters';
import { BoardNotices } from './BoardNotices';
import { BoardSelection } from './BoardSelection';
import { BoardTable } from './BoardTable';
import { UsageFeed } from './UsageFeed';
import { t } from '@/shared/i18n';

/** The containers of every connected Host in one table. */
export function BoardPage() {
  const [params, setParams] = useSearchParams();
  const location = useLocation();
  const navigate = useNavigate();
  const hosts = useHosts();
  const { select } = useHostSelection();
  const every = useEveryHost(resourceQueries.containers);
  const [dialog, confirm] = useConfirm();
  const [sort, setSort] = useState<BoardSort>({ key: 'host', desc: false });
  const [selected, setSelected] = useState(new Set<string>());
  const [usage, setUsage] = useState(new Map<string, Map<string, ContainerUsage>>());
  const onUsage = useCallback(
    (hostId: string, list: Map<string, ContainerUsage>) =>
      setUsage((all) => new Map(all).set(hostId, list)),
    [],
  );

  const filter = readFilter(params);
  // Kept in the URL, so it's still there after opening a container and coming back.
  const setFilter = (next: BoardFilter) => void setParams(writeFilter(next), { replace: true });
  const shownHosts = (id: string) => filter.hosts.length === 0 || filter.hosts.includes(id);

  const rows = boardRows(
    every.results.map((r) => ({
      hostId: r.hostId,
      hostName: r.hostName,
      containers: r.data ?? [],
    })),
  );
  const usageOf = (row: BoardRow) => usage.get(row.hostId)?.get(row.container.id);
  const visible = sortRows(filterRows(rows, filter), sort, usageOf);
  const chosen = rows.filter((r) => selected.has(r.key));

  const toggle = (keys: string[], on: boolean) =>
    setSelected((was) => {
      const next = new Set(was);
      for (const key of keys) {
        if (on) next.add(key);
        else next.delete(key);
      }
      return next;
    });
  const open = (row: BoardRow) => {
    select(row.hostId);
    void navigate(`/containers/${row.container.id}/logs`, {
      state: { from: `${location.pathname}${location.search}` },
    });
  };

  const connected = every.results;
  const header = (
    <PageHeader
      title={t('board.title')}
      description={t('board.summary', {
        running: rows.filter((r) => r.container.state === 'running').length,
        count: rows.length,
        hosts: connected.length,
      })}
    />
  );
  if (hosts.isPending || (every.results.length > 0 && every.results.every((r) => r.isPending))) {
    return <Spinner />;
  }
  return (
    <div>
      {header}
      {connected
        .filter((r) => shownHosts(r.hostId))
        .map((r) => (
          <UsageFeed key={r.hostId} hostId={r.hostId} onUsage={onUsage} />
        ))}
      <BoardNotices
        skipped={every.skipped.filter((h) => shownHosts(h.host.id))}
        failed={connected.filter((r) => r.error && shownHosts(r.hostId))}
      />
      {connected.length === 0 ? (
        <EmptyState
          icon={ServerIcon}
          title={t('board.noHosts.title')}
          description={t('board.noHosts.description')}
        />
      ) : rows.length === 0 ? (
        <EmptyState
          icon={LayoutGridIcon}
          title={t('board.empty.title')}
          description={t('board.empty.description')}
        />
      ) : (
        <>
          <div className="flex flex-wrap items-start gap-2">
            <BoardFilters
              filter={filter}
              onChange={setFilter}
              hosts={hosts.data ?? []}
              stacks={stacksOf(rows)}
            />
            {chosen.length > 0 && (
              <BoardSelection
                selected={chosen}
                onRemoved={() => setSelected(new Set())}
                confirm={confirm}
              />
            )}
          </div>
          <BoardTable
            rows={visible}
            sort={sort}
            onSort={setSort}
            usage={usageOf}
            selected={selected}
            onSelect={toggle}
            onOpen={open}
          />
        </>
      )}
      {dialog}
    </div>
  );
}
