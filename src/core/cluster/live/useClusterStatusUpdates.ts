import { useQueryClient } from '@tanstack/react-query';
import { useEffect } from 'react';
import { events, type ClusterView } from '@/shared/api';
import { clusterKeys } from '../api/keys';

/** Keeps the cached Cluster list's statuses in step with the backend's pushes. */
export function useClusterStatusUpdates() {
  const client = useQueryClient();
  useEffect(() => {
    const unlisten = events.clusterStatusChanged.listen(({ payload }) => {
      let known = false;
      client.setQueryData<ClusterView[]>(clusterKeys.list, (clusters) =>
        clusters?.map((view) => {
          if (view.cluster.id !== payload.clusterId) return view;
          known = true;
          return { ...view, status: payload.status };
        }),
      );
      if (!known) void client.invalidateQueries({ queryKey: clusterKeys.list });
      if (payload.status.state === 'connected') {
        void client.invalidateQueries({ queryKey: clusterKeys.scope(payload.clusterId) });
      }
    });
    return () => {
      void unlisten.then((stop) => stop());
    };
  }, [client]);
}
