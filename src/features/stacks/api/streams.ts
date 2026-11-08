import { useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import {
  commands,
  subscribe,
  type AppError,
  type ComposeEvent,
  type StackOwner,
} from '@/shared/api';
import type { ComposeJob } from '../model/composeJob';
import { scopeKey } from './owner';
import { stackKeys } from './queries';

/**
 * Runs compose jobs for a Managed, Linked or Synced Stack and streams their output. When a run
 * ends, everything cached about the Host is refreshed, since compose can
 * touch any of it. The returned function starts a run and returns its
 * canceller.
 */
export function useComposeRun(owner: StackOwner) {
  const client = useQueryClient();
  return useCallback(
    (job: ComposeJob, onEvent: (event: ComposeEvent) => void, onError: (error: AppError) => void) =>
      subscribe<ComposeEvent>(
        (ch) =>
          job.rollbackTo
            ? commands.stacksRollbackSubscribe(job.rollbackTo, ch)
            : job.commit
              ? commands.stacksSyncedDeploySubscribe(job.stackId, job.commit, job.action, ch)
              : commands.stacksComposeSubscribe(job.kind, job.stackId, job.action, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'exit') {
            void client.invalidateQueries({ queryKey: scopeKey(owner) });
            // A rollback puts an earlier definition back.
            if (job.rollbackTo)
              void client.invalidateQueries({ queryKey: stackKeys.managed(job.stackId) });
            // A Synced Stack's deployed commit changed.
            if (job.kind === 'synced')
              void client.invalidateQueries({ queryKey: ['synced-stack', job.stackId] });
          }
        },
        onError,
      ),
    [client, owner],
  );
}
