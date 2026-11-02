import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useEffect } from 'react';
import { commands, events, keys, type HostView } from '@/shared/api';

export function useHosts() {
  return useQuery({ queryKey: keys.hosts, queryFn: () => commands.hostsList() });
}

export function useCapabilities() {
  return useQuery({
    queryKey: keys.capabilities,
    queryFn: () => commands.getCapabilities(),
    staleTime: Infinity,
  });
}

/** Keeps the cached Host list's statuses in step with the backend's pushes. */
export function useHostStatusUpdates() {
  const client = useQueryClient();
  useEffect(() => {
    const unlisten = events.hostStatusChanged.listen(({ payload }) => {
      let known = false;
      client.setQueryData<HostView[]>(keys.hosts, (hosts) =>
        hosts?.map((view) => {
          if (view.host.id !== payload.hostId) return view;
          known = true;
          return { ...view, status: payload.status };
        }),
      );
      if (!known) void client.invalidateQueries({ queryKey: keys.hosts });
      if (payload.status.state === 'connected') {
        void client.invalidateQueries({ queryKey: keys.host(payload.hostId) });
      }
    });
    return () => {
      void unlisten.then((stop) => stop());
    };
  }, [client]);
}
