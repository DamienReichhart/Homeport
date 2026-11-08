import { useQueryClient } from '@tanstack/react-query';
import { useEffect } from 'react';
import { events } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { forwardKeys } from './queries';
import { t } from '@/shared/i18n';

/** Says when a port forward fails or ends because its target is gone, wherever the user is. */
export function useForwardUpdates() {
  const client = useQueryClient();
  useEffect(() => {
    const stop = events.portForwardChanged.listen(({ payload }) => {
      void client.invalidateQueries({ queryKey: forwardKeys.all });
      toastError(
        payload.event.error,
        payload.event.ended ? t('portforwards.ended') : t('portforwards.failed'),
      );
    });
    return () => void stop.then((unlisten) => unlisten());
  }, [client]);
}
