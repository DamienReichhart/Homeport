import { useCallback, useMemo } from 'react';
import { useSelection } from '@/core/selection';
import type { HostView } from '@/shared/api';
import { useHosts } from '../api/queries';

export interface HostSelection {
  /** The selected Host, `null` when a Cluster or nothing is selected. */
  hostId: string | null;
  host: HostView | null;
  select: (hostId: string) => void;
}

/** The Host the resource pages show, and a way to pick another. */
export function useHostSelection(): HostSelection {
  const { selection, select: choose } = useSelection();
  const hosts = useHosts();
  const hostId = selection?.kind === 'host' ? selection.id : null;
  const select = useCallback((id: string) => choose({ kind: 'host', id }), [choose]);
  const list = hosts.data;
  return useMemo(
    () => ({ hostId, host: list?.find((h) => h.host.id === hostId) ?? null, select }),
    [hostId, list, select],
  );
}
