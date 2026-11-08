import { useQueryClient } from '@tanstack/react-query';
import { useEffect } from 'react';
import { toast } from 'sonner';
import { events, type StackOwner } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';
import { scopeKey, stacksKey } from './owner';
import { syncedKeys } from './synced';

/**
 * Keeps Synced Stacks fresh from the poller's pushes, and says when an
 * automatic deploy finishes or fails.
 */
export function useSyncedUpdates() {
  const client = useQueryClient();
  useEffect(() => {
    const refresh = (owner: StackOwner, stackId: string) => {
      void client.invalidateQueries({ queryKey: stacksKey(owner) });
      void client.invalidateQueries({ queryKey: syncedKeys.stack(stackId) });
    };
    const updates = events.syncedUpdateAvailable.listen(({ payload }) =>
      refresh(payload.owner, payload.stackId),
    );
    const deploys = events.syncedAutoDeployed.listen(({ payload }) => {
      refresh(payload.owner, payload.stackId);
      void client.invalidateQueries({ queryKey: scopeKey(payload.owner) });
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
