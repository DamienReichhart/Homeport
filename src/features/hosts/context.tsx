import { useQueryClient } from '@tanstack/react-query';
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from 'react';
import { commands, keys, type HostView, type Settings } from '@/shared/api';
import { useHosts } from './api';

interface Selection {
  hostId: string | null;
  host: HostView | null;
  select: (hostId: string) => void;
}

const SelectionContext = createContext<Selection | null>(null);

/** Tracks the Host the resource pages show, remembered across launches. */
export function HostProvider({ children }: { children: ReactNode }) {
  const hosts = useHosts();
  const client = useQueryClient();
  const [hostId, setHostId] = useState<string | null>(null);

  useEffect(() => {
    commands
      .getSettings()
      .then((s) => setHostId((current) => current ?? s.lastHostId ?? null))
      .catch(() => {});
  }, []);

  const list = hosts.data;
  // Fall back to the first Host when the remembered one is gone.
  const effectiveId =
    list && (hostId === null || !list.some((h) => h.host.id === hostId))
      ? ((list.find((h) => h.status.state === 'connected') ?? list[0])?.host.id ?? null)
      : hostId;

  const select = useCallback(
    (id: string) => {
      setHostId(id);
      void commands
        .getSettings()
        .then((s) => commands.updateSettings({ ...s, lastHostId: id }))
        .then((s) => client.setQueryData<Settings>(keys.settings, s))
        .catch(() => {});
    },
    [client],
  );

  const value = useMemo<Selection>(
    () => ({
      hostId: effectiveId,
      host: list?.find((h) => h.host.id === effectiveId) ?? null,
      select,
    }),
    [effectiveId, list, select],
  );
  return <SelectionContext.Provider value={value}>{children}</SelectionContext.Provider>;
}

export function useHostSelection(): Selection {
  const value = useContext(SelectionContext);
  if (!value) throw new Error('useHostSelection needs a HostProvider');
  return value;
}
