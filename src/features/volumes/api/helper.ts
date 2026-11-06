import { useQueryClient } from '@tanstack/react-query';
import { useCallback, useEffect, useState } from 'react';
import { commands, subscribe, toAppError, type AppError, type PullEvent } from '@/shared/api';
import { volumeKeys } from './queries';

/**
 * Pulls the helper's image onto a Host (ADR 0015), once the user agreed.
 * The returned function starts the pull and returns its canceller.
 */
export function usePullHelperImage(hostId: string) {
  const client = useQueryClient();
  return useCallback(
    (onEvent: (event: PullEvent) => void, onError: (error: AppError) => void) =>
      subscribe<PullEvent>(
        (ch) => commands.volumesHelperPullSubscribe(hostId, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done')
            void client.invalidateQueries({ queryKey: volumeKeys.helperImage(hostId) });
        },
        onError,
      ),
    [client, hostId],
  );
}

/**
 * A browse session of `volume`: a helper container that mounts it read-only,
 * started when the component mounts and removed when it unmounts. `retry`
 * starts it again after a failure.
 */
export function useBrowseSession(hostId: string, volume: string, pulledImage: boolean) {
  const [session, setSession] = useState<string | null>(null);
  const [error, setError] = useState<AppError | null>(null);
  const [attempt, setAttempt] = useState(0);
  useEffect(() => {
    let ended = false;
    let started: string | null = null;
    commands.volumesBrowseStart(hostId, volume, pulledImage).then(
      (id) => {
        if (ended) void commands.volumesBrowseEnd(id);
        else {
          started = id;
          setSession(id);
        }
      },
      (e: unknown) => {
        if (!ended) setError(toAppError(e));
      },
    );
    return () => {
      ended = true;
      if (started) void commands.volumesBrowseEnd(started);
    };
  }, [hostId, volume, pulledImage, attempt]);
  const retry = useCallback(() => {
    setError(null);
    setSession(null);
    setAttempt((a) => a + 1);
  }, []);
  return { session, error, retry };
}
