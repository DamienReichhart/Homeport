import { useCallback } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { visibleNavEntries, type HostCapability, type NavEntry } from '@/core/feature';
import { supports } from '@/core/host';
import { useSelection, type Selection } from '@/core/selection';
import type { ClusterView, HostView } from '@/shared/api';
import { navEntries } from '../router/features';

/** The sidebar entry a path belongs to: the longest entry path it starts with. */
export function entryOf(path: string, entries: NavEntry[]): NavEntry | undefined {
  return entries
    .filter((e) => e.path !== '/' && (path === e.path || path.startsWith(`${e.path}/`)))
    .sort((a, b) => b.path.length - a.path.length)[0];
}

/**
 * Where switching to another Host or Cluster leads from `path`: the same
 * kind of page when the new one has it (its list, since a container of one
 * Host isn't on another), otherwise its Overview.
 */
export function pathAfterSwitch(path: string, next: NavEntry[]): string {
  const page = entryOf(path, navEntries);
  if (page && page.scope !== 'app' && next.some((e) => e.path === page.path)) return page.path;
  return '/overview';
}

/** Selects a Host or a Cluster and opens the page of it that matches the current one. */
export function useSwitchTarget() {
  const { select } = useSelection();
  const navigate = useNavigate();
  const { pathname } = useLocation();
  return useCallback(
    (target: { kind: 'host'; view: HostView } | { kind: 'cluster'; view: ClusterView }) => {
      const selection: Selection =
        target.kind === 'host'
          ? { kind: 'host', id: target.view.host.id }
          : { kind: 'cluster', id: target.view.cluster.id };
      const next =
        target.kind === 'host'
          ? visibleNavEntries(navEntries, 'host', (c: HostCapability) => supports(target.view, c))
          : visibleNavEntries(navEntries, 'cluster', () => true, target.view.cluster.kind);
      select(selection);
      const to = pathAfterSwitch(pathname, next);
      if (to !== pathname) void navigate(to);
    },
    [select, navigate, pathname],
  );
}
