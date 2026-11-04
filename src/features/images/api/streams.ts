import { useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, subscribe, type AppError, type PullEvent } from '@/shared/api';

/**
 * Pulls images onto a Host and streams their progress. A finished pull is
 * announced and refreshes the image list. The returned function starts a
 * pull and returns its canceller.
 */
export function usePullImage(hostId: string) {
  const client = useQueryClient();
  return useCallback(
    (reference: string, onEvent: (event: PullEvent) => void, onError: (error: AppError) => void) =>
      subscribe<PullEvent>(
        (ch) => commands.imagesPullSubscribe(hostId, reference, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done') {
            toast.success(`Pulled ${reference}`);
            void client.invalidateQueries({ queryKey: hostKeys.images(hostId) });
          }
        },
        onError,
      ),
    [client, hostId],
  );
}

/**
 * Pushes an image name to its registry and streams the progress. A finished
 * push is announced. The returned function starts a push and returns its canceller.
 */
export function usePushImage(hostId: string) {
  return useCallback(
    (reference: string, onEvent: (event: PullEvent) => void, onError: (error: AppError) => void) =>
      subscribe<PullEvent>(
        (ch) => commands.imagesPushSubscribe(hostId, reference, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done') toast.success(`Pushed ${reference}`);
        },
        onError,
      ),
    [hostId],
  );
}
