import { useQueryClient } from '@tanstack/react-query';
import { useCallback, useEffect, useMemo, useState, type ReactNode } from 'react';
import { patchSettings, settingsQuery } from '@/core/settings';
import { toastError } from '@/shared/ui';
import { useHosts } from '../api/queries';
import { HostSelectionContext, type HostSelection } from './context';

/**
 * The settings couldn't be read at start-up. The IPC gateway already wrote the
 * failure to the app log, and the provider falls back to the first Host, so
 * there is nothing more to tell the user here.
 */
function keepFirstHost() {}

/** Tracks the Host the resource pages show, remembered across launches. */
export function HostProvider({ children }: { children: ReactNode }) {
  const hosts = useHosts();
  const client = useQueryClient();
  const [hostId, setHostId] = useState<string | null>(null);

  useEffect(() => {
    client
      .fetchQuery(settingsQuery())
      .then((s) => setHostId((current) => current ?? s.lastHostId ?? null))
      .catch(keepFirstHost);
  }, [client]);

  const list = hosts.data;
  // Fall back to the first Host when the remembered one is gone.
  const effectiveId =
    list && (hostId === null || !list.some((h) => h.host.id === hostId))
      ? ((list.find((h) => h.status.state === 'connected') ?? list[0])?.host.id ?? null)
      : hostId;

  const select = useCallback(
    (id: string) => {
      setHostId(id);
      patchSettings(client, { lastHostId: id }).catch((e: unknown) =>
        toastError(e, 'Homeport couldn’t remember the selected Host'),
      );
    },
    [client],
  );

  const value = useMemo<HostSelection>(
    () => ({
      hostId: effectiveId,
      host: list?.find((h) => h.host.id === effectiveId) ?? null,
      select,
    }),
    [effectiveId, list, select],
  );
  return <HostSelectionContext.Provider value={value}>{children}</HostSelectionContext.Provider>;
}
