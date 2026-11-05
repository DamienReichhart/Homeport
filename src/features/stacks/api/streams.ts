import { useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { hostKeys } from '@/core/host';
import { commands, subscribe, type AppError, type ComposeEvent } from '@/shared/api';
import type { ComposeJob } from '../model/composeJob';
import { stackKeys } from './queries';

/**
 * Runs compose jobs for a Managed or Linked Stack and streams their output. When a run
 * ends, everything cached about the Host is refreshed, since compose can
 * touch any of it. The returned function starts a run and returns its
 * canceller.
 */
export function useComposeRun(hostId: string) {
  const client = useQueryClient();
  return useCallback(
    (job: ComposeJob, onEvent: (event: ComposeEvent) => void, onError: (error: AppError) => void) =>
      subscribe<ComposeEvent>(
        (ch) =>
          job.rollbackTo
            ? commands.stacksRollbackSubscribe(job.rollbackTo, ch)
            : commands.stacksComposeSubscribe(job.kind, job.stackId, job.action, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'exit') {
            void client.invalidateQueries({ queryKey: hostKeys.scope(hostId) });
            // A rollback puts an earlier definition back.
            if (job.rollbackTo)
              void client.invalidateQueries({ queryKey: stackKeys.managed(job.stackId) });
          }
        },
        onError,
      ),
    [client, hostId],
  );
}
