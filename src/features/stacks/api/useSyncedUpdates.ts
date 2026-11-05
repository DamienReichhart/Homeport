import { useQueryClient } from '@tanstack/react-query';
import { useEffect } from 'react';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { events } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';
import { syncedKeys } from './synced';

/**
 * Keeps Synced Stacks fresh from the poller's pushes, and says when an
 * automatic deploy finishes or fails.
 */
export function useSyncedUpdates() {
  const client = useQueryClient();
  useEffect(() => {
    const refresh = (hostId: string, stackId: string) => {
      void client.invalidateQueries({ queryKey: hostKeys.stacks(hostId) });
      void client.invalidateQueries({ queryKey: syncedKeys.stack(stackId) });
    };
    const updates = events.syncedUpdateAvailable.listen(({ payload }) =>
      refresh(payload.hostId, payload.stackId),
    );
    const deploys = events.syncedAutoDeployed.listen(({ payload }) => {
      refresh(payload.hostId, payload.stackId);
      void client.invalidateQueries({ queryKey: hostKeys.scope(payload.hostId) });
      const commit = payload.commit.slice(0, 7);
      if (payload.error) {
        toastError(payload.error, t('stacks.autoDeploy.failed', { name: payload.name, commit }));
      } else {
        toast.success(t('stacks.autoDeploy.done', { name: payload.name, commit }));
      }
    });
    return () => {
      void updates.then((stop) => stop());
      void deploys.then((stop) => stop());
    };
  }, [client]);
}
