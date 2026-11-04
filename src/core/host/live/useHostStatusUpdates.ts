import { useQueryClient } from '@tanstack/react-query';
import { useEffect } from 'react';
import { events, type HostView } from '@/shared/api';
import { hostKeys } from '../api/keys';

/** Keeps the cached Host list's statuses in step with the backend's pushes. */
export function useHostStatusUpdates() {
  const client = useQueryClient();
  useEffect(() => {
    const unlisten = events.hostStatusChanged.listen(({ payload }) => {
      let known = false;
      client.setQueryData<HostView[]>(hostKeys.list, (hosts) =>
        hosts?.map((view) => {
          if (view.host.id !== payload.hostId) return view;
          known = true;
          return { ...view, status: payload.status };
        }),
      );
      if (!known) void client.invalidateQueries({ queryKey: hostKeys.list });
      if (payload.status.state === 'connected') {
        void client.invalidateQueries({ queryKey: hostKeys.scope(payload.hostId) });
      }
    });
    return () => {
      void unlisten.then((stop) => stop());
    };
  }, [client]);
}
