import { useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { hostKeys } from '@/core/host';
import { commands, subscribe, type AppError, type ComposeEvent } from '@/shared/api';
import type { ComposeJob } from '../model/composeJob';

/**
 * Runs compose jobs for a Managed Stack and streams their output. When a run
 * ends, everything cached about the Host is refreshed, since compose can
 * touch any of it. The returned function starts a run and returns its
 * canceller.
 */
export function useComposeRun(hostId: string) {
  const client = useQueryClient();
  return useCallback(
    (job: ComposeJob, onEvent: (event: ComposeEvent) => void, onError: (error: AppError) => void) =>
      subscribe<ComposeEvent>(
        (ch) => commands.stacksComposeSubscribe(job.stackId, job.action, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'exit')
            void client.invalidateQueries({ queryKey: hostKeys.scope(hostId) });
        },
        onError,
      ),
    [client, hostId],
  );
}
